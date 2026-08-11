//! macOS permission requests. The difference that matters: OPENING the settings
//! pane does nothing — an app only appears in the Accessibility / Input
//! Monitoring lists once it has actually REQUESTED the permission through the
//! system API, which also shows the grant prompt. These call the real request
//! APIs so the app populates the list on first click.
#![cfg(target_os = "macos")]
use core_foundation::base::TCFType;
use core_foundation::boolean::CFBoolean;
use core_foundation::dictionary::CFDictionary;
use core_foundation::string::CFString;
use std::os::raw::c_int;

#[link(name = "ApplicationServices", kind = "framework")]
extern "C" {
    fn AXIsProcessTrusted() -> bool;
    fn AXIsProcessTrustedWithOptions(
        options: core_foundation_sys::dictionary::CFDictionaryRef,
    ) -> bool;
    static kAXTrustedCheckOptionPrompt: core_foundation_sys::string::CFStringRef;
}

#[link(name = "IOKit", kind = "framework")]
extern "C" {
    fn IOHIDRequestAccess(request_type: c_int) -> bool;
    fn IOHIDCheckAccess(request_type: c_int) -> c_int;
}

// kIOHIDRequestTypeListenEvent = 1
const LISTEN_EVENT: c_int = 1;
// kIOHIDAccessTypeGranted = 0
const ACCESS_GRANTED: c_int = 0;

pub fn accessibility_granted() -> bool {
    unsafe { AXIsProcessTrusted() }
}

/// Request Accessibility. Shows the system prompt AND registers the app in the
/// Accessibility list — this is what makes it appear there. Returns current state.
pub fn request_accessibility() -> bool {
    unsafe {
        let key = CFString::wrap_under_get_rule(kAXTrustedCheckOptionPrompt);
        let val = CFBoolean::true_value();
        let opts = CFDictionary::from_CFType_pairs(&[(key.as_CFType(), val.as_CFType())]);
        AXIsProcessTrustedWithOptions(opts.as_concrete_TypeRef())
    }
}

pub fn input_monitoring_granted() -> bool {
    unsafe { IOHIDCheckAccess(LISTEN_EVENT) == ACCESS_GRANTED }
}

/// Request Input Monitoring. Shows the prompt and registers the app in the list.
pub fn request_input_monitoring() -> bool {
    unsafe { IOHIDRequestAccess(LISTEN_EVENT) }
}
