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
            Key::Code(c) => name_for_macos_keycode(*c)
                .map(|n| n.to_string())
                .unwrap_or_else(|| format!("code_{c}")),
        }
    }

    /// macOS virtual keycode for this key, used by the raw-CGEvent injector.
    pub fn macos_keycode(&self) -> u16 {
        match self {
            Key::Space => 49,
            Key::Enter => 36,
            Key::Code(c) => *c,
        }
    }

    /// The Python prototype's raw keycodes, kept here for documentation.
    pub const SPACE_CODE: u16 = 49;
    pub const ENTER_CODE: u16 = 36;
}

/// Map a key name (case-insensitive) to its macOS virtual keycode.
///
/// Covers a-z, 0-9, the common named keys, arrows, function keys, and the ANSI
/// symbol keys — enough for the in-app combo editor. Returns `None` for an
/// unknown name so the parser can report it.
pub fn macos_keycode_for_name(name: &str) -> Option<u16> {
    let n = name.trim().to_ascii_lowercase();
    let code = match n.as_str() {
        // letters
        "a" => 0, "b" => 11, "c" => 8, "d" => 2, "e" => 14, "f" => 3, "g" => 5,
        "h" => 4, "i" => 34, "j" => 38, "k" => 40, "l" => 37, "m" => 46, "n" => 45,
        "o" => 31, "p" => 35, "q" => 12, "r" => 15, "s" => 1, "t" => 17, "u" => 32,
        "v" => 9, "w" => 13, "x" => 7, "y" => 16, "z" => 6,
        // digits (top row)
        "0" => 29, "1" => 18, "2" => 19, "3" => 20, "4" => 21, "5" => 23,
        "6" => 22, "7" => 26, "8" => 28, "9" => 25,
        // named
        "space" => 49, "enter" | "return" => 36, "tab" => 48,
        "escape" | "esc" => 53, "delete" | "backspace" => 51,
        // arrows
        "up" => 126, "down" => 125, "left" => 123, "right" => 124,
        // function keys
        "f1" => 122, "f2" => 120, "f3" => 99, "f4" => 118, "f5" => 96, "f6" => 97,
        "f7" => 98, "f8" => 100, "f9" => 101, "f10" => 109, "f11" => 103, "f12" => 111,
        // symbols
        "-" | "minus" => 27, "=" | "equal" => 24, "[" => 33, "]" => 30, ";" => 41,
        "'" => 39, "," => 43, "." => 47, "/" | "slash" => 44, "\\" => 42, "`" => 50,
        _ => return None,
    };
    Some(code)
}

/// Reverse of [`macos_keycode_for_name`] for a handful of codes, used by labels.
fn name_for_macos_keycode(code: u16) -> Option<&'static str> {
    Some(match code {
        49 => "space", 36 => "enter", 48 => "tab", 53 => "esc", 51 => "delete",
        126 => "up", 125 => "down", 123 => "left", 124 => "right",
        0 => "a", 11 => "b", 8 => "c", 2 => "d", 14 => "e", 3 => "f", 5 => "g",
        4 => "h", 34 => "i", 38 => "j", 40 => "k", 37 => "l", 46 => "m", 45 => "n",
        31 => "o", 35 => "p", 12 => "q", 15 => "r", 1 => "s", 17 => "t", 32 => "u",
        9 => "v", 13 => "w", 7 => "x", 16 => "y", 6 => "z",
        _ => return None,
    })
}

/// Parse a human combo string ("cmd+enter", "option+space", "ctrl+shift+4",
/// "space", "a", "none") into an [`Action`].
///
/// Modifiers are `cmd`/`command`/`meta`, `opt`/`option`/`alt`, `ctrl`/`control`,
/// `shift`, `fn`. The final token is the key. Empty or "none" yields
/// [`Action::None`]. Returns `Err(message)` on an unknown token so the editor
/// can show why it was rejected.
pub fn parse_combo(input: &str) -> Result<Action, String> {
    let s = input.trim();
    if s.is_empty() || s.eq_ignore_ascii_case("none") {
        return Ok(Action::None);
    }
    let parts: Vec<&str> = s.split('+').map(|p| p.trim()).filter(|p| !p.is_empty()).collect();
    if parts.is_empty() {
        return Ok(Action::None);
    }
    let (key_str, mod_strs) = parts.split_last().unwrap();
    let mut modifiers = Vec::new();
    for m in mod_strs {
        let modifier = match m.to_ascii_lowercase().as_str() {
            "cmd" | "command" | "meta" | "super" | "win" => Modifier::Cmd,
            "opt" | "option" | "alt" => Modifier::Alt,
            "ctrl" | "control" => Modifier::Ctrl,
            "shift" => Modifier::Shift,
            "fn" => Modifier::Fn,
            other => return Err(format!("unknown modifier '{other}'")),
        };
        modifiers.push(modifier);
    }
    let key = match key_str.to_ascii_lowercase().as_str() {
        "space" => Key::Space,
        "enter" | "return" => Key::Enter,
        _ => {
            let code = macos_keycode_for_name(key_str)
                .ok_or_else(|| format!("unknown key '{key_str}'"))?;
            Key::Code(code)
        }
    };
    Ok(Action::KeyCombo(KeyCombo { modifiers, key }))
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
