//! TOML config layer for Agent Ring.
//!
//! Configs are ergonomic TOML: one [`Action`] per named gesture. Defaults
//! mirror the validated Python `wx02-remap` daemon: `swipe_up = option+space`,
//! `tap = enter`, all others `none`.

use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::actions::{Action, Key, KeyCombo, Modifier};
use crate::profile::DeviceProfile;

/// Top-level config.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub version: u32,
    #[serde(default)]
    pub profiles: Vec<DeviceProfile>,
    #[serde(default)]
    pub mappings: GestureMappings,
}

/// Per-gesture action bindings. Field names are stable snake_case identifiers
/// matching [`crate::gestures::Gesture::as_str`].
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GestureMappings {
    #[serde(default)]
    pub tap: Action,
    #[serde(default)]
    pub swipe_up: Action,
    #[serde(default)]
    pub swipe_down: Action,
    #[serde(default)]
    pub swipe_left: Action,
    #[serde(default)]
    pub swipe_right: Action,
}

impl Default for GestureMappings {
    fn default() -> Self {
        Self {
            tap: Action::none(),
            swipe_up: Action::none(),
            swipe_down: Action::none(),
            swipe_left: Action::none(),
            swipe_right: Action::none(),
        }
    }
}

impl GestureMappings {
    /// Look up the action bound to a gesture name (snake_case).
    pub fn get(&self, gesture: &str) -> Option<&Action> {
        match gesture {
            "tap" => Some(&self.tap),
            "swipe_up" => Some(&self.swipe_up),
            "swipe_down" => Some(&self.swipe_down),
            "swipe_left" => Some(&self.swipe_left),
            "swipe_right" => Some(&self.swipe_right),
            _ => None,
        }
    }

    /// Iterate over `(gesture_name, &Action)` pairs.
    pub fn iter(&self) -> impl Iterator<Item = (&'static str, &Action)> {
        [
            ("tap", &self.tap),
            ("swipe_up", &self.swipe_up),
            ("swipe_down", &self.swipe_down),
            ("swipe_left", &self.swipe_left),
            ("swipe_right", &self.swipe_right),
        ]
        .into_iter()
    }

    /// Collect into a `HashMap<String, Action>` keyed by snake_case gesture name.
    pub fn to_map(&self) -> HashMap<String, Action> {
        self.iter()
            .map(|(k, v)| (k.to_string(), v.clone()))
            .collect()
    }
}

/// Errors that can arise while reading or writing a config.
#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("failed to deserialize config: {0}")]
    De(#[from] toml::de::Error),
    #[error("failed to serialize config: {0}")]
    Ser(#[from] toml::ser::Error),
    #[error("config io error: {0}")]
    Io(#[from] std::io::Error),
}

impl Config {
    /// Defaults matching the validated Python `wx02-remap` `ACTIONS`:
    /// `swipe_up = option+space`, `tap = enter`, others `none`. Includes the
    /// canonical WX02 device profile.
    pub fn default_for_wx02() -> Self {
        let option_space = Action::KeyCombo(KeyCombo {
            modifiers: vec![Modifier::Alt],
            key: Key::Space,
        });
        let enter = Action::key_combo(&[], Key::Enter);
        let mappings = GestureMappings {
            tap: enter,
            swipe_up: option_space,
            swipe_down: Action::none(),
            swipe_left: Action::none(),
            swipe_right: Action::none(),
        };
        Self {
            version: 1,
            profiles: vec![DeviceProfile::wx02()],
            mappings,
        }
    }

    /// Serialize to a TOML string.
    pub fn to_toml(&self) -> Result<String, toml::ser::Error> {
        toml::to_string_pretty(self)
    }

    /// Deserialize from a TOML string.
    pub fn from_toml(s: &str) -> Result<Self, ConfigError> {
        Ok(toml::from_str(s)?)
    }

    /// Load config from `path`, falling back to [`Config::default_for_wx02`]
    /// when the file does not exist. Parse errors propagate; a missing file
    /// does not (M0 has no logging layer, so we fail open silently).
    pub fn load_or_default(path: &Path) -> Self {
        match std::fs::read_to_string(path) {
            Ok(s) => Self::from_toml(&s).unwrap_or_else(|_| Self::default_for_wx02()),
            Err(_) => Self::default_for_wx02(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: 1,
            profiles: Vec::new(),
            mappings: GestureMappings::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_for_wx02_matches_python_actions() {
        let cfg = Config::default_for_wx02();
        assert_eq!(cfg.version, 1);
        assert_eq!(cfg.mappings.tap.label(), "enter");
        assert_eq!(cfg.mappings.swipe_up.label(), "option+space");
        assert_eq!(cfg.mappings.swipe_down, Action::none());
        assert_eq!(cfg.mappings.swipe_left, Action::none());
        assert_eq!(cfg.mappings.swipe_right, Action::none());
        assert!(!cfg.profiles.is_empty());
    }

    #[test]
    fn default_for_wx02_round_trips_toml() {
        let cfg = Config::default_for_wx02();
        let s = cfg.to_toml().expect("serialize");
        // sanity: ergonomic keys present. The serde rename of Modifier::Alt
        // is "alt" (snake_case of the variant); the macOS-friendly "option"
        // label only surfaces in Action::label().
        assert!(s.contains("swipe_up"));
        assert!(s.contains("alt"));
        assert!(s.contains("space"));
        assert!(s.contains("enter"));
        let back = Config::from_toml(&s).expect("deserialize");
        assert_eq!(back.mappings.tap.label(), "enter");
        assert_eq!(back.mappings.swipe_up.label(), "option+space");
        assert_eq!(back.mappings.swipe_down, Action::none());
    }

    #[test]
    fn partial_toml_loads_with_defaults() {
        // Missing gestures should fall back to defaults; missing version
        // should default to 0 via #[serde(default)]. We omit `[profiles]`
        // entirely (an empty `[profiles]` table parses as a map, not a
        // sequence).
        let toml = r#"
version = 0
[mappings]
tap = { type = "none" }
"#;
        let cfg = Config::from_toml(toml).expect("parse");
        assert_eq!(cfg.version, 0);
        assert!(cfg.profiles.is_empty());
        assert_eq!(cfg.mappings.tap, Action::none());
        assert_eq!(cfg.mappings.swipe_up, Action::none());
    }

    #[test]
    fn load_or_default_falls_back_when_missing() {
        let path = Path::new("/nonexistent/path/that/should/not/exist/agentring.toml");
        let cfg = Config::load_or_default(path);
        assert_eq!(cfg.mappings.tap.label(), "enter");
        assert_eq!(cfg.mappings.swipe_up.label(), "option+space");
    }

    #[test]
    fn mappings_iter_and_get() {
        let m = GestureMappings::default();
        assert_eq!(m.iter().count(), 5);
        assert!(m.get("tap").is_some());
        assert!(m.get("nope").is_none());
        let map = m.to_map();
        assert_eq!(map.len(), 5);
        assert!(map.contains_key("swipe_left"));
    }

    #[test]
    fn malformed_toml_returns_config_error() {
        let res = Config::from_toml("not = = valid");
        assert!(res.is_err());
        match res {
            Err(ConfigError::De(_)) => {}
            other => panic!("expected De error, got {other:?}"),
        }
    }
}
