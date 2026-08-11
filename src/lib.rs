//! Agent Ring — cross-platform ring remapper.
//!
//! M0: pure-logic core (profile match, gesture classifier, actions, config).
//! M1: macOS HID capture, key injection, and a menu-bar tray app.
pub mod actions;
pub mod config;
pub mod gestures;
pub mod profile;
pub mod state;

#[cfg(target_os = "macos")]
pub mod permissions;

#[cfg(target_os = "macos")]
pub mod app;
pub mod hid;
#[cfg(target_os = "macos")]
pub mod inject;
