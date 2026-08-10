//! Agent Ring — cross-platform ring remapper core engine.
//!
//! M0 scope: pure-logic core with no OS dependencies. This crate implements the
//! device profile matching rule, the gesture classifier (a faithful port of the
//! validated Python `wx02-remap` prototype), the action model, and the TOML
//! config layer. HID capture, key injection, tray, and UI land in M1+.

pub mod actions;
pub mod config;
pub mod gestures;
pub mod profile;
