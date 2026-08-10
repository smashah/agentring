//! Device-profile matching rule tests — the safety gate against the
//! spoofed-Apple-keyboard trap.

use agentring::profile::{DeviceId, DeviceProfile, MatchResult, RejectReason, Transport};

#[test]
fn spoofed_apple_descriptor_is_rejected_as_vid_pid_only() {
    // The WX02 spoofs VID 0x05AC / PID 0x0220 (a real Apple keyboard). A
    // descriptor that exposes nothing but those IDs MUST be rejected, and the
    // rejection reason MUST call out VID/PID-only matching.
    let spoof = DeviceId::apple_keyboard_spoof();
    assert_eq!(spoof.vendor_id, Some(0x05AC));
    assert_eq!(spoof.product_id, Some(0x0220));
    assert!(spoof.product_string.is_none());

    let result = DeviceProfile::wx02().matches(&spoof);
    assert_eq!(result, MatchResult::Reject(RejectReason::VendorPidOnly));
}

#[test]
fn real_wx02_descriptor_matches() {
    let real = DeviceId::wx02_bluetooth();
    let result = DeviceProfile::wx02().matches(&real);
    assert_eq!(result, MatchResult::Match);
}

#[test]
fn matching_is_satisfied_even_with_spoofed_vid_pid_present() {
    // A real WX02 also reports the spoofed VID/PID — the matching rule must
    // not be confused by that. The honest structural signals (product string,
    // Bluetooth, usage pages) drive the match.
    let mut dev = DeviceId::wx02_bluetooth();
    dev.vendor_id = Some(0x05AC);
    dev.product_id = Some(0x0220);
    assert_eq!(DeviceProfile::wx02().matches(&dev), MatchResult::Match);
}

#[test]
fn usb_transport_rejected_for_bluetooth_profile() {
    let mut dev = DeviceId::wx02_bluetooth();
    dev.transport = Some(Transport::Usb);
    assert_eq!(
        DeviceProfile::wx02().matches(&dev),
        MatchResult::Reject(RejectReason::TransportMismatch)
    );
}

#[test]
fn wrong_product_string_rejected() {
    let mut dev = DeviceId::wx02_bluetooth();
    dev.product_string = Some("Magic Keyboard".to_string());
    assert_eq!(
        DeviceProfile::wx02().matches(&dev),
        MatchResult::Reject(RejectReason::ProductStringMismatch)
    );
}

#[test]
fn missing_digitizer_usage_page_rejected() {
    let mut dev = DeviceId::wx02_bluetooth();
    // Generic desktop + consumer only, no digitizer (0x0D).
    dev.usage_pages = vec![0x01, 0x0C];
    match DeviceProfile::wx02().matches(&dev) {
        MatchResult::Reject(RejectReason::MissingUsagePages(missing)) => {
            assert_eq!(missing, vec![0x0D]);
        }
        other => panic!("expected MissingUsagePages, got {other:?}"),
    }
}

#[test]
fn empty_descriptor_is_vid_pid_only() {
    let empty = DeviceId::default();
    assert!(empty.vendor_id.is_none());
    // Nothing matches at all — degenerates to VendorPidOnly (the safety gate).
    assert_eq!(
        DeviceProfile::wx02().matches(&empty),
        MatchResult::Reject(RejectReason::VendorPidOnly)
    );
}
