//! macOS HID reader — IOHIDManager value callbacks, a faithful port of the
//! validated Python `wx02-remap` prototype. Matches the ring by VID/PID for the
//! manager filter, but gesture logic runs through the profile-checked classifier.
use crate::gestures::{Gesture, GestureClassifier, RawEvent};
use crate::state::SharedState;
use core_foundation::base::TCFType;
use core_foundation::runloop::{kCFRunLoopDefaultMode, CFRunLoop};
use core_foundation::string::CFString;
use core_foundation_sys::base::CFAllocatorRef;
use core_foundation_sys::dictionary::CFMutableDictionaryRef;
use std::ffi::c_void;
use std::os::raw::c_int;
use std::sync::atomic::Ordering;
use std::sync::mpsc::Sender;

// The ring spoofs a real Apple keyboard's IDs. We use them only for the coarse
// IOKit device-matching filter; the digitizer usage pages below are what
// actually identify the WX02's synthetic-touch reports.
const APPLE_VID: i32 = 0x05AC;
const APPLE_PID: i32 = 0x0220;

#[allow(non_camel_case_types)]
type IOHIDManagerRef = *mut c_void;
#[allow(non_camel_case_types)]
type IOHIDValueRef = *mut c_void;
#[allow(non_camel_case_types)]
type IOHIDElementRef = *mut c_void;
type IOHIDValueCallback = extern "C" fn(*mut c_void, c_int, *mut c_void, IOHIDValueRef);
type IOHIDDeviceCallback = extern "C" fn(*mut c_void, c_int, *mut c_void, *mut c_void);

#[link(name = "IOKit", kind = "framework")]
extern "C" {
    fn IOHIDManagerCreate(allocator: CFAllocatorRef, options: u32) -> IOHIDManagerRef;
    fn IOHIDManagerSetDeviceMatching(mgr: IOHIDManagerRef, matching: CFMutableDictionaryRef);
    fn IOHIDManagerRegisterInputValueCallback(
        mgr: IOHIDManagerRef,
        cb: IOHIDValueCallback,
        context: *mut c_void,
    );
    fn IOHIDManagerRegisterDeviceMatchingCallback(
        mgr: IOHIDManagerRef,
        cb: IOHIDDeviceCallback,
        context: *mut c_void,
    );
    fn IOHIDManagerRegisterDeviceRemovalCallback(
        mgr: IOHIDManagerRef,
        cb: IOHIDDeviceCallback,
        context: *mut c_void,
    );
    fn IOHIDManagerScheduleWithRunLoop(
        mgr: IOHIDManagerRef,
        run_loop: *mut c_void,
        mode: core_foundation_sys::string::CFStringRef,
    );
    fn IOHIDManagerOpen(mgr: IOHIDManagerRef, options: u32) -> c_int;
    fn IOHIDManagerClose(mgr: IOHIDManagerRef, options: u32) -> c_int;
    fn IOHIDManagerCopyDevices(mgr: IOHIDManagerRef) -> *const c_void;
    fn IOHIDValueGetElement(value: IOHIDValueRef) -> IOHIDElementRef;
    fn IOHIDValueGetIntegerValue(value: IOHIDValueRef) -> isize;
    fn IOHIDElementGetUsagePage(elem: IOHIDElementRef) -> u32;
    fn IOHIDElementGetUsage(elem: IOHIDElementRef) -> u32;
}

#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
    fn CFDictionaryCreateMutable(
        allocator: CFAllocatorRef,
        capacity: isize,
        key_cb: *const c_void,
        val_cb: *const c_void,
    ) -> CFMutableDictionaryRef;
    fn CFDictionarySetValue(dict: CFMutableDictionaryRef, key: *const c_void, value: *const c_void);
    fn CFNumberCreate(
        allocator: CFAllocatorRef,
        the_type: c_int,
        value_ptr: *const c_void,
    ) -> *const c_void;
    fn CFSetGetCount(set: *const c_void) -> isize;
    fn CFRelease(cf: *const c_void);
    static kCFTypeDictionaryKeyCallBacks: c_void;
    static kCFTypeDictionaryValueCallBacks: c_void;
}

// Callback context: the classifier plus the channel that carries recognised
// gestures out to the app thread. Boxed and leaked for the lifetime of the run.
struct Ctx {
    classifier: GestureClassifier,
    last_x: Option<i32>,
    last_y: Option<i32>,
    tx: Sender<Gesture>,
    state: SharedState,
}

extern "C" fn on_matched(ctx: *mut c_void, _r: c_int, _s: *mut c_void, _d: *mut c_void) {
    if !ctx.is_null() {
        unsafe { &*(ctx as *const Ctx) }
            .state
            .ring_connected
            .store(true, Ordering::Relaxed);
    }
}
extern "C" fn on_removed(ctx: *mut c_void, _r: c_int, _s: *mut c_void, _d: *mut c_void) {
    if !ctx.is_null() {
        unsafe { &*(ctx as *const Ctx) }
            .state
            .ring_connected
            .store(false, Ordering::Relaxed);
    }
}

extern "C" fn on_value(
    context: *mut c_void,
    _res: c_int,
    _sender: *mut c_void,
    value: IOHIDValueRef,
) {
    if context.is_null() || value.is_null() {
        return;
    }
    let ctx = unsafe { &mut *(context as *mut Ctx) };
    // Any value from the ring proves it is connected — the strongest liveness
    // signal we have, independent of whether the match callback ever fired.
    ctx.state.ring_connected.store(true, Ordering::Relaxed);
    let (page, usage, val) = unsafe {
        let elem = IOHIDValueGetElement(value);
        (
            IOHIDElementGetUsagePage(elem),
            IOHIDElementGetUsage(elem),
            IOHIDValueGetIntegerValue(value) as i32,
        )
    };
    // page 1 usage 0x30 = X, 0x31 = Y ; page 13 (0x0D) usage 0x42 = TipSwitch.
    let event = match (page, usage) {
        (1, 0x30) => {
            ctx.last_x = Some(val);
            RawEvent::x(val)
        }
        (1, 0x31) => {
            ctx.last_y = Some(val);
            RawEvent::y(val)
        }
        (13, 0x42) => RawEvent::tip(val),
        _ => return,
    };
    if let Some(gesture) = ctx.classifier.feed(event) {
        let _ = ctx.tx.send(gesture);
    }
}

fn cfnum(n: i32) -> *const c_void {
    // kCFNumberSInt32Type = 3
    unsafe { CFNumberCreate(std::ptr::null(), 3, &n as *const i32 as *const c_void) }
}

/// Start the IOHIDManager on the CURRENT thread's run loop. Blocks in CFRunLoopRun.
/// Recognised gestures are sent on `tx`. Returns Err if the manager can't open
/// (almost always missing Input Monitoring permission).
pub fn run(tx: Sender<Gesture>, state: SharedState) -> Result<(), String> {
    unsafe {
        let matching = CFDictionaryCreateMutable(
            std::ptr::null(),
            0,
            &kCFTypeDictionaryKeyCallBacks as *const _,
            &kCFTypeDictionaryValueCallBacks as *const _,
        );
        let vid_key = CFString::new("VendorID");
        let pid_key = CFString::new("ProductID");
        CFDictionarySetValue(
            matching,
            vid_key.as_concrete_TypeRef() as *const c_void,
            cfnum(APPLE_VID),
        );
        CFDictionarySetValue(
            matching,
            pid_key.as_concrete_TypeRef() as *const c_void,
            cfnum(APPLE_PID),
        );

        let mgr = IOHIDManagerCreate(std::ptr::null(), 0);
        if mgr.is_null() {
            return Err("IOHIDManagerCreate returned null".into());
        }
        IOHIDManagerSetDeviceMatching(mgr, matching);

        let ctx = Box::new(Ctx {
            classifier: GestureClassifier::new(),
            last_x: None,
            last_y: None,
            tx,
            state: state.clone(),
        });
        let ctx_ptr = Box::into_raw(ctx) as *mut c_void;
        IOHIDManagerRegisterInputValueCallback(mgr, on_value, ctx_ptr);
        IOHIDManagerRegisterDeviceMatchingCallback(mgr, on_matched, ctx_ptr);
        IOHIDManagerRegisterDeviceRemovalCallback(mgr, on_removed, ctx_ptr);

        let run_loop = CFRunLoop::get_current();
        IOHIDManagerScheduleWithRunLoop(
            mgr,
            run_loop.as_concrete_TypeRef() as *mut c_void,
            kCFRunLoopDefaultMode,
        );

        let ret = IOHIDManagerOpen(mgr, 0); // shared — seize broke value-callback delivery via the manager
        if ret != 0 {
            return Err(format!(
                "IOHIDManagerOpen failed (0x{ret:08X}) — grant Input Monitoring to agentring in System Settings > Privacy & Security"
            ));
        }
        state.input_monitoring_ok.store(true, Ordering::Relaxed);
        CFRunLoop::run_current();
    }
    Ok(())
}

/// Synchronous check: is the ring currently present on the system?
///
/// Creates a short-lived IOHIDManager matching the ring's (Apple-spoofed)
/// VID/PID — the same identifier the reader uses and which resolves to the WX02
/// in practice — enumerates matched devices, and tears the manager down. Safe to
/// call from the UI thread on demand (e.g. a Refresh button); it opens in shared
/// mode so it never disturbs the reader's own manager. Returns false if the
/// device set is empty or Input Monitoring is not yet granted.
pub fn ring_present() -> bool {
    unsafe {
        let matching = CFDictionaryCreateMutable(
            std::ptr::null(),
            0,
            &kCFTypeDictionaryKeyCallBacks as *const _,
            &kCFTypeDictionaryValueCallBacks as *const _,
        );
        if matching.is_null() {
            return false;
        }
        let vid_key = CFString::new("VendorID");
        let pid_key = CFString::new("ProductID");
        CFDictionarySetValue(
            matching,
            vid_key.as_concrete_TypeRef() as *const c_void,
            cfnum(APPLE_VID),
        );
        CFDictionarySetValue(
            matching,
            pid_key.as_concrete_TypeRef() as *const c_void,
            cfnum(APPLE_PID),
        );

        let mgr = IOHIDManagerCreate(std::ptr::null(), 0);
        if mgr.is_null() {
            CFRelease(matching as *const c_void);
            return false;
        }
        IOHIDManagerSetDeviceMatching(mgr, matching);
        let _ = IOHIDManagerOpen(mgr, 0);
        let devices = IOHIDManagerCopyDevices(mgr);
        let count = if devices.is_null() {
            0
        } else {
            let c = CFSetGetCount(devices);
            CFRelease(devices);
            c
        };
        let _ = IOHIDManagerClose(mgr, 0);
        CFRelease(mgr as *const c_void);
        CFRelease(matching as *const c_void);
        count > 0
    }
}
