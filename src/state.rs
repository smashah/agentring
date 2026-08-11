//! Live state shared between the HID reader thread and the UI.
use crate::gestures::Gesture;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::Instant;

#[derive(Clone)]
pub struct SharedState {
    pub ring_connected: Arc<AtomicBool>,
    pub input_monitoring_ok: Arc<AtomicBool>,
    pub accessibility_ok: Arc<AtomicBool>,
    pub enabled: Arc<AtomicBool>,
    pub last: Arc<Mutex<Option<(Gesture, Instant, String)>>>, // gesture, when, action label
    pub log: Arc<Mutex<VecDeque<(Gesture, String)>>>,         // recent gestures + action
    pub fire_count: Arc<Mutex<u64>>,
}

impl Default for SharedState {
    fn default() -> Self {
        Self {
            ring_connected: Arc::new(AtomicBool::new(false)),
            input_monitoring_ok: Arc::new(AtomicBool::new(false)),
            accessibility_ok: Arc::new(AtomicBool::new(false)),
            enabled: Arc::new(AtomicBool::new(true)),
            last: Arc::new(Mutex::new(None)),
            log: Arc::new(Mutex::new(VecDeque::with_capacity(32))),
            fire_count: Arc::new(Mutex::new(0)),
        }
    }
}

impl SharedState {
    pub fn record(&self, g: Gesture, action_label: String) {
        if let Ok(mut last) = self.last.lock() {
            *last = Some((g, Instant::now(), action_label.clone()));
        }
        if let Ok(mut log) = self.log.lock() {
            log.push_front((g, action_label));
            log.truncate(12);
        }
        if let Ok(mut c) = self.fire_count.lock() {
            *c += 1;
        }
    }
    pub fn is_enabled(&self) -> bool {
        self.enabled.load(Ordering::Relaxed)
    }
}
