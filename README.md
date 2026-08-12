# Agent Ring

Remap a Bluetooth finger-ring into any keyboard shortcut. Agent Ring is a lightweight native Rust menu-bar app for **macOS**; Windows 11 support is planned but does not exist yet.

Reverse-engineered from the WX02 page-turner ring, built profile-first so any HID ring can be added. Domain: **agentr.ing**.

- **What it does:** captures the ring's digitizer reports, classifies tap and swipe-up/down/left/right gestures, and injects the keyboard action mapped to each — replacing per-device Karabiner hacks with one native app. Long-press consumer-key capture is not implemented yet.
- **macOS status:** the core classifier, strict WX02 device validation, gesture-to-key injection, settings window, rich menu-bar tray, permission onboarding, editable mappings, and menu-bar-only app bundle are built. An internal build is installed and tested on Mohammed's Mac.
- **Distribution status:** [Agent Ring 0.1.0 for macOS](https://github.com/smashah/agentring/releases/download/v0.1.0/Agent-Ring-0.1.0-macOS.zip) is public, Developer ID signed, notarized by Apple, and Gatekeeper verified. Its [SHA-256 checksum](https://github.com/smashah/agentring/releases/download/v0.1.0/Agent-Ring-0.1.0-macOS.zip.sha256) is published alongside the download.
- **Windows status:** no Windows build exists. The Raw Input and suppression design still needs a physical Windows 11 Bluetooth spike before implementation.
- **Repository status:** public. Hardware recommendations remain separate from the macOS app release — see `docs/MONETIZATION.md` and `docs/AFFILIATE-PLAN.md`.

## Build
```
cargo test    # classifier replay tests against real captured ring streams — no hardware needed
cargo run     # macOS: grant Input Monitoring + Accessibility on first run
```

For Mohammed's internal signed install, `scripts/install-macos.sh` builds the release app, preserves the stable local signing identity used by macOS TCC, and installs it at `/Applications/Agent Ring.app`. It is not a public installer and depends on the dedicated local signing keychain on this Mac.
