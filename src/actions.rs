//! Action model — pure data, no enigo, no platform code.
//!
//! The dispatch layer (mapping [`Action`] to real OS key events via enigo) is
//! M1. M0 only models the data so the config layer and tests can round-trip it.

use serde::{Deserialize, Serialize};

/// Keyboard modifier. Named to match common macOS/Win/Linux usage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Modifier {
    Ctrl,
    Shift,
    Alt,
    Cmd,
    Fn,
}

impl Modifier {
    /// Short label used by [`Action::label`]: ctrl/shift/alt/cmd/fn.
    pub fn label(&self) -> &'static str {
        match self {
            Modifier::Ctrl => "ctrl",
            Modifier::Shift => "shift",
            Modifier::Alt => "option",
            Modifier::Cmd => "cmd",
            Modifier::Fn => "fn",
        }
    }
}

/// A named key plus a raw-code escape hatch.
///
/// `Code(u16)` lets a profile express any raw keycode (e.g. the Python
/// prototype used keycode 49 for space, 36 for enter) without enumerating every
/// possible key in M0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", content = "value", rename_all = "snake_case")]
pub enum Key {
    Space,
    Enter,
    /// Raw keycode (HID or platform-specific).
    Code(u16),
}

impl Key {
    pub fn label(&self) -> String {
        match self {
            Key::Space => "space".to_string(),
            Key::Enter => "enter".to_string(),
            Key::Code(c) => format!("code_{c}"),
        }
    }

    /// The Python prototype's raw keycodes, kept here for documentation.
    pub const SPACE_CODE: u16 = 49;
    pub const ENTER_CODE: u16 = 36;
}

/// A key + modifier combination.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct KeyCombo {
    #[serde(default)]
    pub modifiers: Vec<Modifier>,
    pub key: Key,
}

/// Consumer-page media keys for the WX02 long-press events (future, M1+).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MediaKey {
    VolumeUp,
    VolumeDown,
    Mute,
    PlayPause,
    NextTrack,
    PrevTrack,
    Power,
}

impl MediaKey {
    pub fn label(&self) -> &'static str {
        match self {
            MediaKey::VolumeUp => "volume_up",
            MediaKey::VolumeDown => "volume_down",
            MediaKey::Mute => "mute",
            MediaKey::PlayPause => "play_pause",
            MediaKey::NextTrack => "next_track",
            MediaKey::PrevTrack => "prev_track",
            MediaKey::Power => "power",
        }
    }
}

/// An action bound to a gesture.
///
/// Uses serde's adjacently-tagged representation (`tag = "type", content =
/// "value"`) rather than internally-tagged because `MediaKey(MediaKey)` wraps
/// an enum, and serde's internally-tagged mode cannot merge a non-struct
/// newtype variant into the tag map. Adjacent tagging keeps the `type`
/// discriminant the brief requires (`none`, `key_combo`, `media_key`) while
/// uniformly handling all three variants.
#[derive(Debug, Clone, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub enum Action {
    #[default]
    None,
    KeyCombo(KeyCombo),
    MediaKey(MediaKey),
}

impl Action {
    /// Build a key-combo action.
    pub fn key_combo(modifiers: &[Modifier], key: Key) -> Self {
        Action::KeyCombo(KeyCombo {
            modifiers: modifiers.to_vec(),
            key,
        })
    }

    /// The null action.
    pub fn none() -> Self {
        Action::None
    }

    /// Human-friendly label for UI/debug.
    ///
    /// Examples: `(none)`, `option+space`, `enter`, `volume_up`.
    pub fn label(&self) -> String {
        match self {
            Action::None => "(none)".to_string(),
            Action::KeyCombo(kc) => {
                let mut parts: Vec<String> =
                    kc.modifiers.iter().map(|m| m.label().to_string()).collect();
                parts.push(kc.key.label());
                parts.join("+")
            }
            Action::MediaKey(mk) => mk.label().to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn option_plus_space_label() {
        let a = Action::key_combo(&[Modifier::Alt], Key::Space);
        assert_eq!(a.label(), "option+space");
    }

    #[test]
    fn enter_label() {
        let a = Action::key_combo(&[], Key::Enter);
        assert_eq!(a.label(), "enter");
    }

    #[test]
    fn none_label() {
        assert_eq!(Action::none().label(), "(none)");
        assert_eq!(Action::default(), Action::None);
    }

    #[test]
    fn media_key_label() {
        assert_eq!(Action::MediaKey(MediaKey::VolumeUp).label(), "volume_up");
    }

    #[test]
    fn action_round_trips_toml() {
        // Round-trip each action via a full Config — TOML requires a table at
        // the root. All five Action variants (None, KeyCombo with/without
        // modifiers, MediaKey) are exercised.
        use crate::config::Config;
        let actions = vec![
            Action::none(),
            Action::key_combo(&[Modifier::Alt], Key::Space),
            Action::key_combo(&[], Key::Enter),
            Action::MediaKey(MediaKey::Mute),
            Action::MediaKey(MediaKey::VolumeUp),
            Action::key_combo(&[Modifier::Ctrl, Modifier::Shift], Key::Code(49)),
        ];
        for a in actions {
            let mut cfg = Config::default_for_wx02();
            cfg.mappings.tap = a.clone();
            cfg.mappings.swipe_up = a.clone();
            cfg.mappings.swipe_down = a.clone();
            cfg.mappings.swipe_left = a.clone();
            cfg.mappings.swipe_right = a.clone();
            let s = cfg.to_toml().expect("serialize");
            let back = Config::from_toml(&s).expect("deserialize");
            assert_eq!(back.mappings.tap, a, "tap round-trip failed for:\n{s}");
            assert_eq!(
                back.mappings.swipe_up, a,
                "swipe_up round-trip failed for:\n{s}"
            );
            assert_eq!(
                back.mappings.swipe_down, a,
                "swipe_down round-trip failed for:\n{s}"
            );
            assert_eq!(
                back.mappings.swipe_right, a,
                "swipe_right round-trip failed for:\n{s}"
            );
        }
    }
}
