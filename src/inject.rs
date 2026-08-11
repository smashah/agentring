//! Gesture -> keystroke dispatch. macOS needs Accessibility permission or the
//! presses are silently dropped.
//!
//! On macOS, key combos are injected as **raw CGEvents with explicit flags**
//! rather than through enigo's press/release dance. This guarantees the injected
//! event carries exactly the modifiers we asked for and nothing else — enigo was
//! observed fusing an unwanted Command onto every combo (a plain Enter arrived as
//! Cmd+Enter, Option+Space as Cmd+Option+Space). Setting the CGEvent flags field
//! directly removes that whole class of bug because the receiving app reads the
//! event's own flags. Media keys still go through enigo (raw NSSystemDefined
//! events are a separate, heavier path we don't need yet).
use crate::actions::{Action, Key, MediaKey, Modifier};
use enigo::{Direction, Enigo, Key as EKey, Keyboard, Settings};

pub struct Injector {
    enigo: Enigo,
}

impl Injector {
    pub fn new() -> Result<Self, String> {
        Enigo::new(&Settings::default())
            .map(|enigo| Self { enigo })
            .map_err(|e| format!("enigo init failed: {e}"))
    }

    pub fn dispatch(&mut self, action: &Action) -> Result<(), String> {
        match action {
            Action::None => Ok(()),
            Action::KeyCombo(kc) => {
                #[cfg(target_os = "macos")]
                {
                    macos_post_combo(&kc.modifiers, &kc.key)
                }
                #[cfg(not(target_os = "macos"))]
                {
                    let mut pressed = Vec::new();
                    let mut first_error = None;
                    for m in &kc.modifiers {
                        match self.enigo.key(modifier_key(m), Direction::Press) {
                            Ok(()) => pressed.push(m),
                            Err(error) => {
                                first_error = Some(error.to_string());
                                break;
                            }
                        }
                    }
                    if first_error.is_none() {
                        if let Err(error) = self.enigo.key(main_key(&kc.key), Direction::Click) {
                            first_error = Some(error.to_string());
                        }
                    }
                    for m in pressed.into_iter().rev() {
                        if let Err(error) = self.enigo.key(modifier_key(m), Direction::Release) {
                            if first_error.is_none() {
                                first_error = Some(error.to_string());
                            }
                        }
                    }
                    match first_error {
                        Some(error) => Err(error),
                        None => Ok(()),
                    }
                }
            }
            Action::MediaKey(mk) => self
                .enigo
                .key(media_key(mk), Direction::Click)
                .map_err(|e| e.to_string()),
        }
    }
}

/// macOS CGEvent flag masks.
#[cfg(target_os = "macos")]
mod cg_flags {
    pub const SHIFT: u64 = 0x0002_0000;
    pub const CONTROL: u64 = 0x0004_0000;
    pub const ALTERNATE: u64 = 0x0008_0000; // Option
    pub const COMMAND: u64 = 0x0010_0000;
    pub const SECONDARY_FN: u64 = 0x0080_0000;
}

#[cfg(target_os = "macos")]
fn macos_flags(modifiers: &[Modifier]) -> u64 {
    let mut f = 0u64;
    for m in modifiers {
        f |= match m {
            Modifier::Shift => cg_flags::SHIFT,
            Modifier::Ctrl => cg_flags::CONTROL,
            Modifier::Alt => cg_flags::ALTERNATE,
            Modifier::Cmd => cg_flags::COMMAND,
            Modifier::Fn => cg_flags::SECONDARY_FN,
        };
    }
    f
}

#[cfg(target_os = "macos")]
mod cg {
    use std::ffi::c_void;
    pub type CGEventRef = *mut c_void;
    pub type CGEventSourceRef = *mut c_void;

    // kCGEventSourceStateHIDSystemState = 1, kCGHIDEventTap = 0.
    pub const HID_SYSTEM_STATE: i32 = 1;
    pub const HID_EVENT_TAP: u32 = 0;

    #[link(name = "CoreGraphics", kind = "framework")]
    extern "C" {
        pub fn CGEventSourceCreate(state_id: i32) -> CGEventSourceRef;
        pub fn CGEventCreateKeyboardEvent(
            source: CGEventSourceRef,
            keycode: u16,
            key_down: bool,
        ) -> CGEventRef;
        pub fn CGEventSetFlags(event: CGEventRef, flags: u64);
        pub fn CGEventPost(tap: u32, event: CGEventRef);
    }

    #[link(name = "CoreFoundation", kind = "framework")]
    extern "C" {
        pub fn CFRelease(cf: *const c_void);
    }
}

/// Post a key combo as an explicit-flags CGEvent pair. The flags field carries
/// exactly the requested modifiers; nothing is inferred from ambient state.
#[cfg(target_os = "macos")]
fn macos_post_combo(modifiers: &[Modifier], key: &Key) -> Result<(), String> {
    use cg::*;
    let keycode = key.macos_keycode();
    let flags = macos_flags(modifiers);
    unsafe {
        let source = CGEventSourceCreate(HID_SYSTEM_STATE);
        // A null source is still valid for CGEventCreateKeyboardEvent (it just
        // uses a private source), so we proceed either way.
        let down = CGEventCreateKeyboardEvent(source, keycode, true);
        let up = CGEventCreateKeyboardEvent(source, keycode, false);
        if down.is_null() || up.is_null() {
            if !down.is_null() {
                CFRelease(down);
            }
            if !up.is_null() {
                CFRelease(up);
            }
            if !source.is_null() {
                CFRelease(source);
            }
            return Err("CoreGraphics could not create keyboard events".to_string());
        }
        CGEventSetFlags(down, flags);
        CGEventSetFlags(up, flags);
        CGEventPost(HID_EVENT_TAP, down);
        CGEventPost(HID_EVENT_TAP, up);
        CFRelease(down);
        CFRelease(up);
        if !source.is_null() {
            CFRelease(source);
        }
    }
    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn modifier_key(m: &Modifier) -> EKey {
    match m {
        Modifier::Ctrl => EKey::Control,
        Modifier::Shift => EKey::Shift,
        Modifier::Alt => EKey::Alt,
        Modifier::Cmd => EKey::Meta,
        Modifier::Fn => EKey::Other(0x3F), // kVK_Function
    }
}

#[cfg(not(target_os = "macos"))]
fn main_key(k: &Key) -> EKey {
    match k {
        Key::Space => EKey::Space,
        Key::Enter => EKey::Return,
        Key::Code(c) => EKey::Other(*c as u32),
    }
}

fn media_key(mk: &MediaKey) -> EKey {
    match mk {
        MediaKey::VolumeUp => EKey::VolumeUp,
        MediaKey::VolumeDown => EKey::VolumeDown,
        MediaKey::Mute => EKey::VolumeMute,
        MediaKey::PlayPause => EKey::MediaPlayPause,
        MediaKey::NextTrack => EKey::MediaNextTrack,
        MediaKey::PrevTrack => EKey::MediaPrevTrack,
        MediaKey::Power => EKey::Other(0x7F),
    }
}
