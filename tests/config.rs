//! TOML config round-trip tests.

use agentring::actions::{Action, Key, MediaKey, Modifier};
use agentring::config::{Config, ConfigError};

#[test]
fn default_for_wx02_matches_python_defaults() {
    let cfg = Config::default_for_wx02();
    assert_eq!(cfg.mappings.tap.label(), "enter");
    assert_eq!(cfg.mappings.swipe_up.label(), "option+space");
    assert_eq!(cfg.mappings.swipe_down, Action::none());
    assert_eq!(cfg.mappings.swipe_left, Action::none());
    assert_eq!(cfg.mappings.swipe_right, Action::none());
}

#[test]
fn default_round_trips_through_toml() {
    let cfg = Config::default_for_wx02();
    let s = cfg.to_toml().expect("serialize");
    assert!(s.contains("swipe_up"));
    // The serde rename of Modifier::Alt is "alt" (snake_case of the variant);
    // the macOS-friendly "option" label only surfaces in Action::label().
    assert!(s.contains("alt"));
    assert!(s.contains("space"));
    assert!(s.contains("enter"));
    let back = Config::from_toml(&s).expect("deserialize");
    assert_eq!(back.mappings.tap.label(), "enter");
    assert_eq!(back.mappings.swipe_up.label(), "option+space");
    assert_eq!(back.mappings.swipe_down, Action::none());
    assert_eq!(back.mappings.swipe_left, Action::none());
    assert_eq!(back.mappings.swipe_right, Action::none());
}

#[test]
fn partial_toml_loads_with_defaults() {
    // Action uses adjacently-tagged serde (tag = "type", content = "value").
    let toml = r#"
version = 3
[mappings]
tap = { type = "media_key", value = "play_pause" }
"#;
    let cfg = Config::from_toml(toml).expect("parse");
    assert_eq!(cfg.version, 3);
    assert_eq!(cfg.mappings.tap, Action::MediaKey(MediaKey::PlayPause));
    // Unspecified gestures default to None.
    assert_eq!(cfg.mappings.swipe_up, Action::none());
    assert_eq!(cfg.mappings.swipe_down, Action::none());
    assert_eq!(cfg.mappings.swipe_left, Action::none());
    assert_eq!(cfg.mappings.swipe_right, Action::none());
}

#[test]
fn fully_specified_config_round_trips() {
    let cfg = Config {
        version: 2,
        profiles: vec![agentring::profile::DeviceProfile::wx02()],
        mappings: agentring::config::GestureMappings {
            tap: Action::key_combo(&[], Key::Enter),
            swipe_up: Action::key_combo(&[Modifier::Alt], Key::Space),
            swipe_down: Action::key_combo(&[Modifier::Cmd], Key::Space),
            swipe_left: Action::MediaKey(MediaKey::PrevTrack),
            swipe_right: Action::MediaKey(MediaKey::NextTrack),
        },
    };
    let s = cfg.to_toml().expect("serialize");
    let back = Config::from_toml(&s).expect("deserialize");
    assert_eq!(back.version, 2);
    assert_eq!(back.mappings, cfg.mappings);
}

#[test]
fn malformed_toml_errors() {
    let res = Config::from_toml("this is not = = valid toml");
    assert!(matches!(res, Err(ConfigError::De(_))));
}

#[test]
fn load_or_default_falls_back_when_missing() {
    let cfg = Config::load_or_default(std::path::Path::new(
        "/definitely/not/here/agentring-m0-test.toml",
    ))
    .unwrap();
    assert_eq!(cfg.mappings.tap.label(), "enter");
    assert_eq!(cfg.mappings.swipe_up.label(), "option+space");
}
