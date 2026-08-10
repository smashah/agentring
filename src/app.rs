//! macOS tray application. A menu-bar icon (no Dock presence) with Enable/Disable
//! and Quit, driven by a tao event loop. The HID reader runs on its own thread
//! and streams gestures back; the loop dispatches them through the injector using
//! the loaded config.
use crate::config::Config;
use crate::hid;
use crate::inject::Injector;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::sync::Arc;
use tao::event_loop::{ControlFlow, EventLoopBuilder};
use tao::platform::macos::{ActivationPolicy, EventLoopExtMacOS};
use tray_icon::menu::{Menu, MenuEvent, MenuItem};
use tray_icon::{TrayIconBuilder, TrayIconEvent};

pub fn run(config: Config) -> Result<(), String> {
    let enabled = Arc::new(AtomicBool::new(true));

    // HID reader thread → gesture channel.
    let (gtx, grx) = mpsc::channel();
    std::thread::spawn(move || {
        if let Err(e) = hid::macos::run(gtx) {
            eprintln!("agentring: HID reader stopped: {e}");
        }
    });

    let mut injector = Injector::new()?;
    let mappings = config.mappings.to_map();

    let mut event_loop = EventLoopBuilder::new().build();
    event_loop.set_activation_policy(ActivationPolicy::Accessory); // menu-bar only, no Dock icon

    let tray_menu = Menu::new();
    let toggle = MenuItem::new("Enabled", true, None);
    let quit = MenuItem::new("Quit Agent Ring", true, None);
    tray_menu.append(&toggle).ok();
    tray_menu.append(&quit).ok();

    let _tray = TrayIconBuilder::new()
        .with_menu(Box::new(tray_menu))
        .with_tooltip("Agent Ring")
        .with_title("◉") // visible menu-bar glyph until we ship a real icon
        .build()
        .map_err(|e| format!("tray build failed: {e}"))?;

    let menu_channel = MenuEvent::receiver();
    let _tray_channel = TrayIconEvent::receiver();
    let toggle_id = toggle.id().clone();
    let quit_id = quit.id().clone();

    event_loop.run(move |_event, _target, control_flow| {
        *control_flow = ControlFlow::WaitUntil(
            std::time::Instant::now() + std::time::Duration::from_millis(30),
        );

        // Drain any gestures the ring produced and fire their mapped action.
        while let Ok(gesture) = grx.try_recv() {
            if !enabled.load(Ordering::Relaxed) {
                continue;
            }
            if let Some(action) = mappings.get(gesture.as_str()) {
                injector.dispatch(action);
            }
        }

        if let Ok(ev) = menu_channel.try_recv() {
            if ev.id == quit_id {
                *control_flow = ControlFlow::Exit;
            } else if ev.id == toggle_id {
                let now = !enabled.load(Ordering::Relaxed);
                enabled.store(now, Ordering::Relaxed);
                toggle.set_text(if now { "Enabled" } else { "Disabled" });
            }
        }
    });
}
