//! Gesture -> keystroke dispatch via enigo. macOS needs Accessibility permission
//! or the presses are silently dropped.
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

    pub fn dispatch(&mut self, action: &Action) {
        match action {
            Action::None => {}
            Action::KeyCombo(kc) => {
                for m in &kc.modifiers {
                    let _ = self.enigo.key(modifier_key(m), Direction::Press);
                }
                let _ = self.enigo.key(main_key(&kc.key), Direction::Click);
                for m in kc.modifiers.iter().rev() {
                    let _ = self.enigo.key(modifier_key(m), Direction::Release);
                }
            }
            Action::MediaKey(mk) => {
                let _ = self.enigo.key(media_key(mk), Direction::Click);
            }
        }
    }
}

fn modifier_key(m: &Modifier) -> EKey {
    match m {
        Modifier::Ctrl => EKey::Control,
        Modifier::Shift => EKey::Shift,
        Modifier::Alt => EKey::Alt,
        Modifier::Cmd => EKey::Meta,
        Modifier::Fn => EKey::Other(0x3F), // kVK_Function
    }
}

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
