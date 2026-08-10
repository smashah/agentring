 Agent Ring (agentr.ing) — cross-platform ring remapper app

 Context

 We reverse-engineered the WX02 Bluetooth finger-ring page turner this session: quick presses emit synthetic digitizer touch swipes (usage page 0x0D/0x05 — invisible to Karabiner, discarded by
 macOS), long presses emit consumer keys (volume up/down, power). Two working Python prototypes exist (~/bin/wx02-events monitor, ~/bin/wx02-remap gesture→keystroke daemon) that prove the full
 pipeline: HID capture → swipe/tap classification → synthetic keypress injection.

 Goal: turn this into Agent Ring — a releasable, maximally lightweight Rust menu-bar/tray app for macOS + Windows 11 (both in v1), with a native settings window (no webview), WX02-family support
 first on an extensible device-profile core. Branding: domain agentr.ing, repo ~/projects/agentring, binary agentring.

 Validated stack (researched 2026-08-10)

 ┌─────────────────────┬────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┐
 │       Concern       │                                                                                 Choice                                                                                 │
 ├─────────────────────┼────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
 │ HID read (macOS)    │ hidapi 2.6.6 with macos-shared-device feature (IOHIDManager, non-seized)                                                                                               │
 ├─────────────────────┼────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
 │ HID read (Windows)  │ Raw Input API (windows crate, WM_INPUT + RIDEV_INPUTSINK) — Windows opens digitizer TLCs exclusively, hidapi cannot stream them; consumer TLC (0x0C/0x01) is shared    │
 │                     │ and hidapi-readable                                                                                                                                                    │
 ├─────────────────────┼────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
 │ Report parsing      │ hidreport 0.6.0 against get_report_descriptor() (works both platforms; reconstructed from preparsed data on Windows)                                                   │
 ├─────────────────────┼────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
 │ Gesture→key         │ enigo 0.6.1 (Option+Space, media keys, cross-platform)                                                                                                                 │
 │ injection           │                                                                                                                                                                        │
 ├─────────────────────┼────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
 │ Tray + settings UI  │ tray-icon 0.24.2 + eframe/egui 0.36.1                                                                                                                                  │
 ├─────────────────────┼────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
 │ Config              │ TOML via serde + directories, hot-reloadable                                                                                                                           │
 ├─────────────────────┼────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┤
 │ Packaging           │ cargo-packager 0.11.8 (.app/.dmg + .exe/MSI), notarization via xcrun notarytool (local — GitHub Actions is billing-blocked on this account, all release builds run     │
 │                     │ locally)                                                                                                                                                               │
 └─────────────────────┴────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────────┘

 Device matching rule: the ring spoofs Apple VID 0x05AC / PID 0x0220 (real Apple keyboard IDs) — never match VID/PID alone; match product string WX02 + transport Bluetooth + usage page, encoded in
 the device profile.

 Repo layout (~/projects/agentring, single crate)
     Device matching rule: the ring spoofs Apple VID 0x05AC / PID 0x0220 (real Apple keyboard IDs) — never match VID/PID alone; match product string WX02 + transport Bluetooth + usage page, encoded
     in the device profile.

     Repo layout (~/projects/agentring, single crate)

     src/
       main.rs        — startup, single-instance guard, permissions preflight
       hid/mod.rs     — HidBackend trait → channel of timestamped raw reports
       hid/macos.rs   — hidapi shared-open reader; Input Monitoring preflight;
                        optionally attempt device seize so long-press volume keys
                        don't also change system volume (fallback: CGEventTap swallow)
       hid/windows.rs — Raw Input registration (0x0D/0x05 + 0x0C/0x01) on a hidden
                        message window; WH_MOUSE_LL/WH_KEYBOARD_LL suppression of
                        double-handled events (dwExtraInfo touch marker 0xFF515700)
       profile.rs     — device profiles: match rules + hidreport-driven field maps
       gestures.rs    — port of the Python classifier: tip up/down cycle, dx/dy vs
                        150-unit threshold → tap / swipe_{up,down,left,right};
                        plus long-press consumer-key events as first-class inputs
       actions.rs     — action model (key combo, media key, none) + enigo dispatch
       config.rs      — TOML config: per-gesture action, per-device profile
       app.rs         — eframe app: tray created inside run_native closure (macOS
                        main-thread + post-init rules), ActivationPolicy::Accessory
                        (no Dock icon), settings viewport, "press a button to map"
                        learn mode fed live from the gesture channel
     tests/fixtures/  — real captured WX02 event streams from this session (replay
                        tests for the classifier — deterministic, no hardware)

     Milestones

     1. M0 — core engine (no OS deps): scaffold, config model, gesture classifier with replay tests built from this session's real captures (swipe-up Y 818→316, swipe-down Y 409→847, tap at
     1364,682, bare-tap variants).
     2. M1 — macOS backend + tray: hidapi reader, enigo dispatch, tray menu (enable/disable, open settings, quit). Parity with the Python prototype = daily-drivable.
     3. M2 — settings window: egui mapping UI with learn mode (click ring button → gesture highlights → pick action from list or record a shortcut).
     4. M3 — Windows spike then backend (top risk, do spike early): plug the ring into a Windows 11 machine and test in an hour whether the fake descriptor binds as a live touchpad (cursor chaos)
     or fails PTP init and is ignored. Then implement Raw Input backend + the suppression ladder. Needs a physical Windows box with Bluetooth.
     5. M4 — release: cargo-packager bundles, codesign + notarize (needs a Developer ID — ad-hoc/unsigned builds lose TCC grants on every rebuild, so a stable signing identity matters even during
     dev), README with the reverse-engineering story + demo GIF, publish under agentr.ing.

     Known risks

     - Windows touch binding unknown until the M3 spike — mitigation ladder researched (likely PTP init failure = benign; else LL-hook suppression; worst case Windows v1 ships long-press-only).
     - macOS TCC dev loop: grants are keyed to code signature; sign dev builds with a stable identity from day one.
     - Two permissions on macOS (Input Monitoring + Accessibility) — first-run onboarding must walk the user through both or the app looks broken.

     Verification

     - cargo test — classifier replay tests against real captured streams (no hardware needed).
     - Manual macOS: run app, grant both permissions, confirm quick-click up = Option+Space, middle = Enter, long-presses fire remapped keys with no system volume change; confirm tray toggle +
     settings learn mode; confirm Karabiri rules from today can be deleted afterwards (app replaces them).
     - Manual Windows: spike findings first; then same button matrix on the Windows box.
     - Packaging: install .dmg on a clean user account, verify TCC prompts and persistence across relaunch.
