//! macOS HID reader — IOHIDManager value callbacks, a faithful port of the
//! validated Python `wx02-remap` prototype. Matches the ring by VID/PID for the
//! manager filter, but gesture logic runs through the profile-checked classifier.
use crate::gestures::{Gesture, GestureClassifier, RawEvent};
use crate::profile::{DeviceId, DeviceProfile, MatchResult, Transport};
use crate::state::SharedState;
use core_foundation::base::TCFType;
use core_foundation::runloop::{kCFRunLoopDefaultMode, CFRunLoop};
use core_foundation::string::CFString;
use core_foundation_sys::base::CFAllocatorRef;
use core_foundation_sys::dictionary::CFMutableDictionaryRef;
use std::collections::HashSet;
use std::ffi::c_void;
use std::os::raw::c_int;
use std::sync::atomic::Ordering;
use std::sync::mpsc::SyncSender;

// The ring spoofs a real Apple keyboard's IDs. We use them only for the coarse
// IOKit device-matching filter; the digitizer usage pages below are what
// actually identify the WX02's synthetic-touch reports.
const APPLE_VID: i32 = 0x05AC;
const APPLE_PID: i32 = 0x0220;

#[allow(non_camel_case_types)]
type IOHIDManagerRef = *mut c_void;
#[allow(non_camel_case_types)]
type IOHIDDeviceRef = *mut c_void;
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
    fn IOHIDDeviceGetProperty(device: IOHIDDeviceRef, key: *const c_void) -> *const c_void;
    fn IOHIDDeviceCopyMatchingElements(
        device: IOHIDDeviceRef,
        matching: *const c_void,
        options: u32,
    ) -> *const c_void;
    fn IOHIDValueGetElement(value: IOHIDValueRef) -> IOHIDElementRef;
    fn IOHIDElementGetDevice(elem: IOHIDElementRef) -> IOHIDDeviceRef;
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
    fn CFSetGetValues(set: *const c_void, values: *mut *const c_void);
    fn CFArrayGetCount(array: *const c_void) -> isize;
    fn CFArrayGetValueAtIndex(array: *const c_void, index: isize) -> *const c_void;
    fn CFStringGetCString(
        string: *const c_void,
        buffer: *mut i8,
        buffer_size: isize,
        encoding: u32,
    ) -> bool;
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
    tx: SyncSender<Gesture>,
    state: SharedState,
    matched_devices: HashSet<usize>,
}

extern "C" fn on_matched(ctx: *mut c_void, _r: c_int, _s: *mut c_void, device: *mut c_void) {
    if ctx.is_null() || device.is_null() {
        return;
    }
    let ctx = unsafe { &mut *(ctx as *mut Ctx) };
    if unsafe { is_wx02_device(device) } {
        ctx.matched_devices.insert(device as usize);
        ctx.state.ring_connected.store(true, Ordering::Relaxed);
    }
}
extern "C" fn on_removed(ctx: *mut c_void, _r: c_int, _s: *mut c_void, device: *mut c_void) {
    if ctx.is_null() || device.is_null() {
        return;
    }
    let ctx = unsafe { &mut *(ctx as *mut Ctx) };
    ctx.matched_devices.remove(&(device as usize));
    ctx.state
        .ring_connected
        .store(!ctx.matched_devices.is_empty(), Ordering::Relaxed);
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
    let (page, usage, val) = unsafe {
        let elem = IOHIDValueGetElement(value);
        if elem.is_null()
            || !ctx
                .matched_devices
                .contains(&(IOHIDElementGetDevice(elem) as usize))
        {
            return;
        }
        (
            IOHIDElementGetUsagePage(elem),
            IOHIDElementGetUsage(elem),
            IOHIDValueGetIntegerValue(value) as i32,
        )
    };
    ctx.state.ring_connected.store(true, Ordering::Relaxed);
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
        let _ = ctx.tx.try_send(gesture);
    }
}

fn cfnum(n: i32) -> *const c_void {
    // kCFNumberSInt32Type = 3
    unsafe { CFNumberCreate(std::ptr::null(), 3, &n as *const i32 as *const c_void) }
}

unsafe fn string_property(device: IOHIDDeviceRef, key: &str) -> Option<String> {
    const UTF8: u32 = 0x0800_0100;
    let key = CFString::new(key);
    let value = IOHIDDeviceGetProperty(device, key.as_concrete_TypeRef() as *const c_void);
    if value.is_null() {
        return None;
    }
    let mut buffer = [0_u8; 512];
    if !CFStringGetCString(
        value,
        buffer.as_mut_ptr().cast(),
        buffer.len() as isize,
        UTF8,
    ) {
        return None;
    }
    let end = buffer.iter().position(|byte| *byte == 0)?;
    Some(String::from_utf8_lossy(&buffer[..end]).into_owned())
}

unsafe fn device_id(device: IOHIDDeviceRef) -> DeviceId {
    let product_string = string_property(device, "Product");
    let transport = string_property(device, "Transport").and_then(|value| {
        let value = value.to_ascii_lowercase();
        if value.contains("bluetooth") {
            Some(Transport::Bluetooth)
        } else if value.contains("usb") {
            Some(Transport::Usb)
        } else {
            None
        }
    });

    let mut usage_pages = Vec::new();
    let elements = IOHIDDeviceCopyMatchingElements(device, std::ptr::null(), 0);
    if !elements.is_null() {
        let count = CFArrayGetCount(elements);
        for index in 0..count {
            let element = CFArrayGetValueAtIndex(elements, index) as IOHIDElementRef;
            if !element.is_null() {
                usage_pages.push(IOHIDElementGetUsagePage(element) as u16);
            }
        }
        CFRelease(elements);
    }
    usage_pages.sort_unstable();
    usage_pages.dedup();

    DeviceId {
        vendor_id: Some(APPLE_VID as u16),
        product_id: Some(APPLE_PID as u16),
        product_string,
        transport,
        usage_pages,
    }
}

unsafe fn is_wx02_device(device: IOHIDDeviceRef) -> bool {
    matches!(
        DeviceProfile::wx02().matches(&device_id(device)),
        MatchResult::Match
    )
}

unsafe fn matching_dictionary() -> CFMutableDictionaryRef {
    let matching = CFDictionaryCreateMutable(
        std::ptr::null(),
        0,
        &kCFTypeDictionaryKeyCallBacks as *const _,
        &kCFTypeDictionaryValueCallBacks as *const _,
    );
    if matching.is_null() {
        return matching;
    }
    let vid_key = CFString::new("VendorID");
    let pid_key = CFString::new("ProductID");
    let vid = cfnum(APPLE_VID);
    let pid = cfnum(APPLE_PID);
    if !vid.is_null() {
        CFDictionarySetValue(matching, vid_key.as_concrete_TypeRef().cast(), vid);
        CFRelease(vid);
    }
    if !pid.is_null() {
        CFDictionarySetValue(matching, pid_key.as_concrete_TypeRef().cast(), pid);
        CFRelease(pid);
    }
    matching
}

/// Start the IOHIDManager on the CURRENT thread's run loop. Blocks in CFRunLoopRun.
/// Recognised gestures are sent on `tx`. Returns Err if the manager can't open
/// (almost always missing Input Monitoring permission).
pub fn run(tx: SyncSender<Gesture>, state: SharedState) -> Result<(), String> {
    unsafe {
        let matching = matching_dictionary();
        if matching.is_null() {
            return Err("CFDictionaryCreateMutable returned null".into());
        }

        let mgr = IOHIDManagerCreate(std::ptr::null(), 0);
        if mgr.is_null() {
            CFRelease(matching as *const c_void);
            return Err("IOHIDManagerCreate returned null".into());
        }
        IOHIDManagerSetDeviceMatching(mgr, matching);

        let ctx = Box::new(Ctx {
            classifier: GestureClassifier::new(),
            last_x: None,
            last_y: None,
            tx,
            state: state.clone(),
            matched_devices: HashSet::new(),
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
            drop(Box::from_raw(ctx_ptr as *mut Ctx));
            CFRelease(mgr as *const c_void);
            CFRelease(matching as *const c_void);
            return Err(format!(
                "IOHIDManagerOpen failed (0x{ret:08X}) — grant Input Monitoring to agentring in System Settings > Privacy & Security"
            ));
        }
        state.input_monitoring_ok.store(true, Ordering::Relaxed);
        CFRunLoop::run_current();
        let _ = IOHIDManagerClose(mgr, 0);
        drop(Box::from_raw(ctx_ptr as *mut Ctx));
        CFRelease(mgr as *const c_void);
        CFRelease(matching as *const c_void);
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
        let matching = matching_dictionary();
        if matching.is_null() {
            return false;
        }

        let mgr = IOHIDManagerCreate(std::ptr::null(), 0);
        if mgr.is_null() {
            CFRelease(matching as *const c_void);
            return false;
        }
        IOHIDManagerSetDeviceMatching(mgr, matching);
        let _ = IOHIDManagerOpen(mgr, 0);
        let devices = IOHIDManagerCopyDevices(mgr);
        let found = if devices.is_null() {
            false
        } else {
            let count = CFSetGetCount(devices);
            let mut values = vec![std::ptr::null(); count.max(0) as usize];
            if !values.is_empty() {
                CFSetGetValues(devices, values.as_mut_ptr());
            }
            let found = values
                .into_iter()
                .any(|device| !device.is_null() && is_wx02_device(device as IOHIDDeviceRef));
            CFRelease(devices);
            found
        };
        let _ = IOHIDManagerClose(mgr, 0);
        CFRelease(mgr as *const c_void);
        CFRelease(matching as *const c_void);
        found
    }
}
