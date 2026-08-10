# Agent Ring

Remap a Bluetooth finger-ring into any keyboard shortcut. A maximally-lightweight Rust menu-bar / tray app for **macOS and Windows 11**.

Reverse-engineered from the WX02 page-turner ring, built profile-first so any HID ring can be added. Domain: **agentr.ing**.

- **What it does:** captures the ring's HID reports, classifies gestures (tap, swipe up/down/left/right, long-press), and injects the keystroke or media key you map to each — replacing per-device Karabiner hacks with one native app.
- **Status:** M0 (core engine) in progress. See `docs/PRD.md` for the full spec, validated stack, and milestones.
- **Private** until affiliate monetization is in place — see `docs/MONETIZATION.md`.

## Build
```
cargo test    # classifier replay tests against real captured ring streams — no hardware needed
cargo run     # macOS: grant Input Monitoring + Accessibility on first run
```
