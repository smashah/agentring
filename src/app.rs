//! Agent Ring status & settings window (eframe). Shows ring connection, the two
//! macOS permissions, a live gesture monitor, and the current mappings.
use crate::config::Config;
use crate::hid;
use crate::inject::Injector;
use crate::state::SharedState;
use std::sync::atomic::Ordering;
use std::sync::mpsc;

pub fn run(config: Config) -> Result<(), String> {
    let state = SharedState::default();

    // HID reader thread -> gesture channel.
    let (gtx, grx) = mpsc::channel();
    {
        let st = state.clone();
        std::thread::spawn(move || {
            if let Err(e) = hid::macos::run(gtx, st.clone()) {
                eprintln!("agentring: HID reader stopped: {e}");
                st.input_monitoring_ok.store(false, Ordering::Relaxed);
            }
        });
    }

    let injector = Injector::new().ok();
    let mappings = config.mappings.to_map();

    let ui = RingApp {
        state,
        mappings,
        injector,
        grx,
        config,
    };

    let opts = eframe::NativeOptions {
        viewport: eframe::egui::ViewportBuilder::default()
            .with_inner_size([460.0, 560.0])
            .with_title("Agent Ring"),
        ..Default::default()
    };
    eframe::run_native("Agent Ring", opts, Box::new(|_cc| Ok(Box::new(ui))))
        .map_err(|e| format!("window failed: {e}"))
}

struct RingApp {
    state: SharedState,
    mappings: std::collections::HashMap<String, crate::actions::Action>,
    injector: Option<Injector>,
    grx: mpsc::Receiver<crate::gestures::Gesture>,
    config: Config,
}

impl eframe::App for RingApp {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        // refresh live permission status each frame
        #[cfg(target_os = "macos")]
        {
            self.state.accessibility_ok.store(crate::permissions::accessibility_granted(), Ordering::Relaxed);
            self.state.input_monitoring_ok.store(crate::permissions::input_monitoring_granted(), Ordering::Relaxed);
        }

        // drain gestures, fire mapped actions, record for the monitor
        while let Ok(g) = self.grx.try_recv() {
            let label = self
                .mappings
                .get(g.as_str())
                .map(|a| a.label())
                .unwrap_or_else(|| "(unmapped)".into());
            if self.state.is_enabled() {
                if let (Some(inj), Some(action)) =
                    (self.injector.as_mut(), self.mappings.get(g.as_str()))
                {
                    inj.dispatch(action);
                }
            }
            self.state.record(g, label);
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
            status_row(ui, "Ring connected", ring, "No ring detected — pair the WX02 over Bluetooth");
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
            egui::Grid::new("map").num_columns(2).spacing([16.0, 4.0]).show(ui, |ui| {
                for (gesture, action) in self.config.mappings.iter() {
                    ui.label(gesture);
                    ui.label(action.label());
                    ui.end_row();
                }
            });
        });
    }
}

fn status_row(ui: &mut eframe::egui::Ui, label: &str, ok: bool, hint: &str) {
    use eframe::egui::{Color32, RichText};
    ui.horizontal(|ui| {
        let (dot, col) = if ok { ("●", Color32::LIGHT_GREEN) } else { ("●", Color32::from_rgb(220, 90, 90)) };
        ui.label(RichText::new(dot).color(col).size(16.0));
        ui.label(RichText::new(label).strong());
        if !ok {
            ui.label(RichText::new(hint).weak().size(11.0));
        }
    });
}
