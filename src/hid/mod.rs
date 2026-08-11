//! HID backends. Each platform provides `run(tx)` that blocks reading the ring
//! and sends recognised gestures on the channel.
#[cfg(target_os = "macos")]
pub mod macos;
