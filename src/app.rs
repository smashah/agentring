//! Agent Ring status & settings window (eframe). Shows ring connection, the two
//! macOS permissions, a live gesture monitor, and the current mappings.
use crate::actions::Action;
use crate::config::Config;
use crate::hid;
use crate::inject::Injector;
use crate::state::SharedState;
use std::sync::atomic::Ordering;
use std::sync::mpsc;
use tray_icon::menu::{
    CheckMenuItem, IsMenuItem, Menu, MenuEvent, MenuId, MenuItem, PredefinedMenuItem, Submenu,
};
use tray_icon::{TrayIcon, TrayIconBuilder};

const GESTURE_QUEUE_CAPACITY: usize = 128;
const MAX_GESTURES_PER_FRAME: usize = 32;

pub fn run(config: Config) -> Result<(), String> {
    let state = SharedState::default();

    // HID reader thread -> gesture channel.
    let (gtx, grx) = mpsc::sync_channel(GESTURE_QUEUE_CAPACITY);
    {
        let st = state.clone();
        std::thread::spawn(move || {
            if let Err(e) = hid::macos::run(gtx, st.clone()) {
                eprintln!("agentring: HID reader stopped: {e}");
                st.input_monitoring_ok.store(false, Ordering::Relaxed);
            }
        });
    }

    let (injector, injector_error) = match Injector::new() {
        Ok(injector) => (Some(injector), None),
        Err(error) => {
            state.enabled.store(false, Ordering::Relaxed);
            (None, Some(error))
        }
    };
    let mappings = config.mappings.to_map();

    // Seed the editable combo fields from the current mappings, in stable order.
    let mut combo_inputs = std::collections::HashMap::new();
    for (name, action) in config.mappings.iter() {
        combo_inputs.insert(name.to_string(), action_to_input(action));
    }

    // Seed connection status immediately so a ring paired before launch shows
    // as connected without waiting for a gesture or the match callback.
    #[cfg(target_os = "macos")]
    state
        .ring_connected
        .store(hid::macos::ring_present(), Ordering::Relaxed);

    let ui = RingApp {
        state,
        mappings,
        injector,
        injector_error,
        grx,
        config,
        combo_inputs,
        combo_errors: std::collections::HashMap::new(),
        last_permission_refresh: std::time::Instant::now() - std::time::Duration::from_secs(1),
        tray: None,
    };

    let opts = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([460.0, 560.0])
            .with_title("Agent Ring"),
        ..Default::default()
    };
    // The tray must be created on the main thread after the event loop is up, so
    // build it inside the app creator closure and hand it to the app to own.
    eframe::run_native(
        "Agent Ring",
        opts,
        Box::new(move |_cc| {
            let mut ui = ui;
            match build_tray(&ui.config) {
                Some(tray) => ui.tray = Some(tray),
                None => eprintln!("agentring: menu bar icon failed to initialize"),
            }
            Ok(Box::new(ui))
        }),
    )
    .map_err(|e| format!("window failed: {e}"))
}

/// Owned handles for the menu-bar tray dropdown. The `TrayIcon` and every item
/// handle are cheap Rc-style clones; keeping them alive for the process
/// lifetime prevents the tray from being dropped (which removes it). Stashing
/// the item handles lets us update their text / checkmark each frame and match
/// click events against their IDs.
struct TrayMenu {
    _icon: TrayIcon,
    ring_connected: CheckMenuItem,
    input_monitoring: CheckMenuItem,
    accessibility: CheckMenuItem,
    remapping: CheckMenuItem,
    last_gesture: MenuItem,
    /// Handles and IDs stay paired in stable config order.
    mapping_items: Vec<MenuItem>,
    mapping_ids: Vec<MenuId>,
    open_id: MenuId,
    quit_id: MenuId,
}

/// Build the menu-bar tray icon with a rich Amphetamine-style dropdown: live
/// status indicators, a remapping toggle, the last gesture line, a mappings
/// submenu, and Open / Quit actions. Mapping text is built once from the
/// initial config (editing happens in the settings window, which is what
/// clicking a mapping opens anyway).
fn build_tray(config: &Config) -> Option<TrayMenu> {
    let menu = Menu::new();

    let ring_connected = CheckMenuItem::new("Ring connected", true, false, None);
    let input_monitoring = CheckMenuItem::new("Input Monitoring", true, false, None);
    let accessibility = CheckMenuItem::new("Accessibility", true, false, None);
    let remapping = CheckMenuItem::new("Remapping enabled", true, true, None);
    let last_gesture = MenuItem::new("No gestures yet · 0 total", false, None);

    let mapping_items: Vec<MenuItem> = config
        .mappings
        .iter()
        .map(|(name, action)| {
            let label = match action {
                Action::None => "none".to_string(),
                other => other.label(),
            };
            MenuItem::new(format!("{name} → {label}"), true, None)
        })
        .collect();
    let mapping_ids: Vec<MenuId> = mapping_items.iter().map(|i| i.id().clone()).collect();
    let mapping_refs: Vec<&dyn IsMenuItem> =
        mapping_items.iter().map(|i| i as &dyn IsMenuItem).collect();
    let mappings_submenu = Submenu::with_items("Mappings", true, &mapping_refs).ok()?;

    let open = MenuItem::new("Open Agent Ring", true, None);
    let quit = MenuItem::new("Quit Agent Ring", true, None);
    let open_id = open.id().clone();
    let quit_id = quit.id().clone();

    menu.append(&ring_connected).ok()?;
    menu.append(&input_monitoring).ok()?;
    menu.append(&accessibility).ok()?;
    menu.append(&PredefinedMenuItem::separator()).ok()?;
    menu.append(&remapping).ok()?;
    menu.append(&PredefinedMenuItem::separator()).ok()?;
    menu.append(&last_gesture).ok()?;
    menu.append(&mappings_submenu).ok()?;
    menu.append(&PredefinedMenuItem::separator()).ok()?;
    menu.append(&open).ok()?;
    menu.append(&quit).ok()?;

    let mut builder = TrayIconBuilder::new()
        .with_menu(Box::new(menu))
        .with_tooltip("Agent Ring")
        // Template mode: macOS renders the alpha silhouette in the menu-bar
        // color (white on dark, black on light), adapting to appearance instead
        // of a fixed color that would go invisible in the other mode.
        .with_icon_as_template(true);
    if let Some(icon) = load_tray_icon() {
        builder = builder.with_icon(icon);
    }
    let _icon = builder.build().ok()?;

    Some(TrayMenu {
        _icon,
        ring_connected,
        input_monitoring,
        accessibility,
        remapping,
        last_gesture,
        mapping_items,
        mapping_ids,
        open_id,
        quit_id,
    })
}

/// Load the ring logo as a menu-bar-sized template icon (22×22 RGBA). The image
/// is a black silhouette on transparent; template mode uses its alpha and
/// recolours to match the menu bar.
fn load_tray_icon() -> Option<tray_icon::Icon> {
    let bytes = include_bytes!("../assets/logo_menubar.png");
    let img = image::load_from_memory(bytes)
        .ok()?
        .resize_exact(22, 22, image::imageops::FilterType::Lanczos3)
        .to_rgba8();
    let (w, h) = img.dimensions();
    tray_icon::Icon::from_rgba(img.into_raw(), w, h).ok()
}

/// Sync the tray dropdown's checkmarks and last-gesture text from live state.
/// Called every frame so the open dropdown always reflects the current truth.
fn refresh_tray(tray: &TrayMenu, state: &SharedState) {
    tray.ring_connected
        .set_checked(state.ring_connected.load(Ordering::Relaxed));
    tray.input_monitoring
        .set_checked(state.input_monitoring_ok.load(Ordering::Relaxed));
    tray.accessibility
        .set_checked(state.accessibility_ok.load(Ordering::Relaxed));
    tray.remapping.set_checked(state.is_enabled());

    let count = state.fire_count.lock().map(|c| *c).unwrap_or(0);
    let text = match state.last.lock() {
        Ok(guard) => match guard.as_ref() {
            Some((g, when, action)) => {
                let ago = when.elapsed().as_secs();
                format!(
                    "Last: {} → {} ({}s ago) · {} total",
                    g.as_str(),
                    action,
                    ago,
                    count
                )
            }
            None => format!("No gestures yet · {count} total"),
        },
        Err(_) => format!("No gestures yet · {count} total"),
    };
    tray.last_gesture.set_text(text);
}

struct RingApp {
    state: SharedState,
    mappings: std::collections::HashMap<String, crate::actions::Action>,
    injector: Option<Injector>,
    injector_error: Option<String>,
    grx: mpsc::Receiver<crate::gestures::Gesture>,
    config: Config,
    /// Editable combo text per gesture name (e.g. "cmd+enter").
    combo_inputs: std::collections::HashMap<String, String>,
    /// Parse error per gesture name, shown inline; absent when valid.
    combo_errors: std::collections::HashMap<String, String>,
    last_permission_refresh: std::time::Instant,
    /// Menu-bar tray dropdown; kept alive for the process lifetime (drop removes it).
    tray: Option<TrayMenu>,
}

/// Render an action as an editable combo string ("" for none).
fn action_to_input(action: &crate::actions::Action) -> String {
    match action {
        crate::actions::Action::None => String::new(),
        other => other.label(),
    }
}

fn config_file_path() -> Result<std::path::PathBuf, String> {
    let dir = std::env::var("HOME")
        .map(|h| std::path::PathBuf::from(h).join(".config/agentring"))
        .unwrap_or_else(|_| std::path::PathBuf::from(".agentring"));
    std::fs::create_dir_all(&dir)
        .map_err(|e| format!("could not create config directory {}: {e}", dir.display()))?;
    Ok(dir.join("config.toml"))
}

fn gesture_index(gesture: &str) -> Option<usize> {
    match gesture {
        "tap" => Some(0),
        "swipe_up" => Some(1),
        "swipe_down" => Some(2),
        "swipe_left" => Some(3),
        "swipe_right" => Some(4),
        _ => None,
    }
}

impl RingApp {
    /// Apply an edited combo to a gesture: update the live inject map, the
    /// persisted config struct, and write config.toml. Records a parse error
    /// instead when the text is invalid.
    fn apply_combo(&mut self, gesture: &str, text: &str) {
        match crate::actions::parse_combo(text) {
            Ok(action) => {
                let mut next_config = self.config.clone();
                match gesture {
                    "tap" => next_config.mappings.tap = action.clone(),
                    "swipe_up" => next_config.mappings.swipe_up = action.clone(),
                    "swipe_down" => next_config.mappings.swipe_down = action.clone(),
                    "swipe_left" => next_config.mappings.swipe_left = action.clone(),
                    "swipe_right" => next_config.mappings.swipe_right = action.clone(),
                    _ => {
                        self.combo_errors
                            .insert(gesture.to_string(), "unknown gesture".to_string());
                        return;
                    }
                }
                let saved = next_config
                    .to_toml()
                    .map_err(|e| format!("could not serialize config: {e}"))
                    .and_then(|toml| {
                        let path = config_file_path()?;
                        std::fs::write(&path, toml)
                            .map_err(|e| format!("could not save {}: {e}", path.display()))
                    });
                if let Err(error) = saved {
                    self.combo_errors.insert(gesture.to_string(), error);
                    return;
                }

                self.config = next_config;
                self.mappings.insert(gesture.to_string(), action.clone());
                self.combo_errors.remove(gesture);
                if let (Some(tray), Some(index)) = (&self.tray, gesture_index(gesture)) {
                    if let Some(item) = tray.mapping_items.get(index) {
                        let label = match &action {
                            Action::None => "none".to_string(),
                            other => other.label(),
                        };
                        item.set_text(format!("{gesture} → {label}"));
                    }
                }
            }
            Err(e) => {
                self.combo_errors.insert(gesture.to_string(), e);
            }
        }
    }
}

impl eframe::App for RingApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // Menu-bar tray: handle dropdown clicks.
        while let Ok(ev) = MenuEvent::receiver().try_recv() {
            let Some(tray) = self.tray.as_ref() else {
                continue;
            };
            let id = &ev.id;
            if id == tray.ring_connected.id() {
                #[cfg(target_os = "macos")]
                {
                    let present = crate::hid::macos::ring_present();
                    self.state.ring_connected.store(present, Ordering::Relaxed);
                }
            } else if id == tray.input_monitoring.id() {
                #[cfg(target_os = "macos")]
                {
                    if !crate::permissions::input_monitoring_granted() {
                        crate::permissions::request_input_monitoring();
                    }
                }
            } else if id == tray.accessibility.id() {
                #[cfg(target_os = "macos")]
                {
                    if !crate::permissions::accessibility_granted() {
                        crate::permissions::request_accessibility();
                    }
                }
            } else if id == tray.remapping.id() {
                // CheckMenuItem auto-toggled visually; flip the real state and
                // let refresh_tray sync the checkmark next frame.
                let current = self.state.is_enabled();
                self.state.enabled.store(!current, Ordering::Relaxed);
            } else if id == &tray.open_id || tray.mapping_ids.contains(id) {
                ctx.send_viewport_cmd(eframe::egui::ViewportCommand::Visible(true));
                ctx.send_viewport_cmd(eframe::egui::ViewportCommand::Focus);
            } else if id == &tray.quit_id {
                std::process::exit(0);
            }
        }
        // Closing the window hides to the menu bar instead of quitting, so the
        // ring keeps working and the tray icon stays. Quit is from the tray menu.
        if ctx.input(|i| i.viewport().close_requested()) {
            ctx.send_viewport_cmd(eframe::egui::ViewportCommand::CancelClose);
            ctx.send_viewport_cmd(eframe::egui::ViewportCommand::Visible(false));
        }

        // TCC checks cross a process boundary and generate system log traffic,
        // so refresh status once per second rather than every 60 ms frame.
        #[cfg(target_os = "macos")]
        if self.last_permission_refresh.elapsed() >= std::time::Duration::from_secs(1) {
            self.state.accessibility_ok.store(
                crate::permissions::accessibility_granted(),
                Ordering::Relaxed,
            );
            self.state.input_monitoring_ok.store(
                crate::permissions::input_monitoring_granted(),
                Ordering::Relaxed,
            );
            // NOTE: no background HID polling here. Re-scanning by opening an
            // IOHIDManager on a timer churned the input system and made global
            // hotkeys (e.g. CleanShot) unresponsive. Presence updates come from
            // the startup seed, the value-callback liveness flip, and the manual
            // Refresh button only.
            self.last_permission_refresh = std::time::Instant::now();
        }

        // Sync the tray dropdown's checkmarks and last-gesture text each frame.
        if let Some(tray) = &self.tray {
            refresh_tray(tray, &self.state);
        }

        // Process a bounded batch so a noisy device cannot starve the UI.
        for _ in 0..MAX_GESTURES_PER_FRAME {
            let Ok(g) = self.grx.try_recv() else {
                break;
            };
            let label = self
                .mappings
                .get(g.as_str())
                .map(|a| a.label())
                .unwrap_or_else(|| "(unmapped)".into());
            let outcome = if !self.state.is_enabled() {
                format!("remapping disabled · {label}")
            } else if let Some(action) = self.mappings.get(g.as_str()) {
                match self.injector.as_mut() {
                    Some(injector) => match injector.dispatch(action) {
                        Ok(()) => label,
                        Err(error) => format!("injection failed: {error}"),
                    },
                    None => format!(
                        "injection unavailable: {}",
                        self.injector_error.as_deref().unwrap_or("unknown error")
                    ),
                }
            } else {
                label
            };
            self.state.record(g, outcome);
        }
        ctx.request_repaint_after(std::time::Duration::from_millis(60));

        use eframe::egui::{self, Color32, RichText};
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add_space(4.0);
            ui.heading("Agent Ring");
            ui.label(RichText::new("Turn your ring into keyboard shortcuts").weak());
            ui.separator();

            let ring = self.state.ring_connected.load(Ordering::Relaxed);
            let im = self.state.input_monitoring_ok.load(Ordering::Relaxed);
            let ax = self.state.accessibility_ok.load(Ordering::Relaxed);

            // status rows
            ui.add_space(6.0);
            ui.horizontal(|ui| {
                status_row(ui, "Ring connected", ring, "No ring detected — pair the WX02 over Bluetooth, then Refresh");
                if ui.button("↻ Refresh").on_hover_text("Re-scan for the ring now").clicked() {
                    #[cfg(target_os = "macos")]
                    {
                        let present = crate::hid::macos::ring_present();
                        self.state.ring_connected.store(present, Ordering::Relaxed);
                    }
                }
            });
            status_row(ui, "Input Monitoring", im, "Grant in System Settings > Privacy & Security > Input Monitoring");
            status_row(ui, "Accessibility", ax, "Grant in System Settings > Privacy & Security > Accessibility");

            // onboarding banner if not ready
            if !(im && ax) {
                ui.add_space(8.0);
                egui::Frame::none()
                    .fill(Color32::from_rgb(40, 33, 20))
                    .inner_margin(10.0)
                    .rounding(6.0)
                    .show(ui, |ui| {
                        ui.label(RichText::new("Setup needed").strong().color(Color32::from_rgb(240, 200, 90)));
                        ui.label("Agent Ring needs both permissions to read the ring and send keystrokes. Click Grant — macOS will prompt you and add Agent Ring to the list automatically.");
                        ui.add_space(6.0);
                        ui.horizontal(|ui| {
                            if !im && ui.button("Grant Input Monitoring").clicked() {
                                #[cfg(target_os = "macos")]
                                { crate::permissions::request_input_monitoring(); }
                            }
                            if !ax && ui.button("Grant Accessibility").clicked() {
                                #[cfg(target_os = "macos")]
                                { crate::permissions::request_accessibility(); }
                            }
                        });
                    });
            }

            if let Some(error) = &self.injector_error {
                ui.add_space(8.0);
                ui.colored_label(
                    Color32::from_rgb(220, 120, 90),
                    format!("Remapping unavailable: {error}"),
                );
            }

            ui.add_space(10.0);
            let mut enabled = self.state.is_enabled();
            if ui.checkbox(&mut enabled, "Remapping enabled").changed() {
                self.state.enabled.store(enabled, Ordering::Relaxed);
            }

            ui.add_space(10.0);
            ui.separator();
            ui.label(RichText::new("Live gesture monitor").strong());
            let count = self.state.fire_count.lock().map(|c| *c).unwrap_or(0);
            if let Ok(last) = self.state.last.lock() {
                if let Some((g, when, action)) = last.as_ref() {
                    let ago = when.elapsed().as_secs_f32();
                    let hot = ago < 0.6;
                    ui.label(
                        RichText::new(format!("▶ {}  →  {}", g.as_str(), action))
                            .size(18.0)
                            .color(if hot { Color32::LIGHT_GREEN } else { Color32::GRAY }),
                    );
                    ui.label(RichText::new(format!("{count} gestures this session · last {ago:.1}s ago")).weak());
                } else {
                    ui.label(RichText::new("Click your ring — gestures appear here in real time").weak());
                }
            }

            ui.add_space(6.0);
            egui::ScrollArea::vertical().max_height(120.0).show(ui, |ui| {
                if let Ok(log) = self.state.log.lock() {
                    for (g, action) in log.iter() {
                        ui.label(format!("{:<11} → {}", g.as_str(), action));
                    }
                }
            });

            ui.add_space(10.0);
            ui.separator();
            ui.label(RichText::new("Mappings").strong());
            ui.label(RichText::new("Type a combo — e.g. cmd+enter, option+space, ctrl+shift+4, or leave blank for none").weak().size(11.0));
            ui.add_space(4.0);
            let gestures = ["tap", "swipe_up", "swipe_down", "swipe_left", "swipe_right"];
            let mut pending: Option<(String, String)> = None;
            egui::Grid::new("map").num_columns(2).spacing([12.0, 6.0]).show(ui, |ui| {
                for gesture in gestures {
                    ui.label(gesture);
                    let entry = self.combo_inputs.entry(gesture.to_string()).or_default();
                    let resp = ui.add(
                        egui::TextEdit::singleline(entry)
                            .desired_width(180.0)
                            .hint_text("none"),
                    );
                    if resp.changed() {
                        pending = Some((gesture.to_string(), entry.clone()));
                    }
                    if let Some(err) = self.combo_errors.get(gesture) {
                        ui.label(RichText::new(format!("⚠ {err}")).color(Color32::from_rgb(220, 120, 90)).size(11.0));
                    }
                    ui.end_row();
                }
            });
            if let Some((gesture, text)) = pending {
                self.apply_combo(&gesture, &text);
            }
        });
    }
}

fn status_row(ui: &mut eframe::egui::Ui, label: &str, ok: bool, hint: &str) {
    use eframe::egui::{Color32, RichText};
    ui.horizontal(|ui| {
        let (dot, col) = if ok {
            ("●", Color32::LIGHT_GREEN)
        } else {
            ("●", Color32::from_rgb(220, 90, 90))
        };
        ui.label(RichText::new(dot).color(col).size(16.0));
        ui.label(RichText::new(label).strong());
        if !ok {
            ui.label(RichText::new(hint).weak().size(11.0));
        }
    });
}
