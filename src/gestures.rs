//! Gesture classifier — faithful port of the validated `wx02-remap` Python prototype.
//!
//! Algorithm summary (do not change behaviour without re-validating against the
//! captured fixtures):
//!
//! 1. Each HID element value change arrives as a [`RawEvent`].
//! 2. X / Y events update a running `last_x` / `last_y` cache; while a contact
//!    is active they also update the contact's end position `ex` / `ey`.
//! 3. A `TipSwitch` 0→1 transition begins a contact: the start position
//!    `sx` / `sy` is captured from the `last_x` / `last_y` cache (i.e. the most
//!    recently seen X/Y — this matches the Python prototype which reads
//!    `last["x"]` / `last["y"]` at touch-begin, not a fresh read).
//! 4. A `TipSwitch` 1→0 transition ends the contact and emits exactly one
//!    [`Gesture`] via [`GestureClassifier::feed`].
//! 5. Classification compares `|dx|` and `|dy|` (end − start) against
//!    [`SWIPE_THRESHOLD`]; below threshold ⇒ `Tap`, otherwise the larger axis
//!    wins. Digitizer Y grows downward like screen coordinates, so a physical
//!    upward finger swipe produces decreasing Y and therefore `dy < 0 ⇒
//!    SwipeUp`.
//! 6. If start or end coordinates are missing at contact end (e.g. a
//!    `TipSwitch` pulse with no positional preamble), the contact is a `Tap`.

use serde::{Deserialize, Serialize};

/// Digitizer units; contacts with `|dx|` and `|dy|` both below this are taps.
pub const SWIPE_THRESHOLD: i32 = 150;

/// HID element identity constants used by the WX02 digitizer.
pub mod usage {
    /// Generic Desktop usage page.
    pub const PAGE_GENERIC_DESKTOP: u16 = 0x01;
    /// Digitizer usage page.
    pub const PAGE_DIGITIZER: u16 = 0x0D;
    /// Consumer usage page (long-press consumer keys, future).
    pub const PAGE_CONSUMER: u16 = 0x0C;

    /// Generic Desktop X axis.
    pub const USAGE_X: u32 = 0x30;
    /// Generic Desktop Y axis.
    pub const USAGE_Y: u32 = 0x31;
    /// Digitizer tip switch (contact).
    pub const USAGE_TIP_SWITCH: u32 = 0x42;
}

/// One HID element value change.
///
/// Identity is the `(usage_page, usage)` pair; the value is the raw logical
/// value reported by the device (e.g. digitizer units for X/Y, 0/1 for the tip
/// switch). `timestamp_ms` is optional and unused by the M0 classifier — it
/// exists so M1 can attach HID report timestamps without changing the API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct RawEvent {
    pub usage_page: u16,
    pub usage: u32,
    pub value: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp_ms: Option<u64>,
}

impl RawEvent {
    /// Convenience constructor for an X-axis event (page 1, usage 0x30).
    pub fn x(value: i32) -> Self {
        Self {
            usage_page: usage::PAGE_GENERIC_DESKTOP,
            usage: usage::USAGE_X,
            value,
            timestamp_ms: None,
        }
    }

    /// Convenience constructor for a Y-axis event (page 1, usage 0x31).
    pub fn y(value: i32) -> Self {
        Self {
            usage_page: usage::PAGE_GENERIC_DESKTOP,
            usage: usage::USAGE_Y,
            value,
            timestamp_ms: None,
        }
    }

    /// Convenience constructor for a tip-switch event (page 0x0D, usage 0x42).
    pub fn tip(value: i32) -> Self {
        Self {
            usage_page: usage::PAGE_DIGITIZER,
            usage: usage::USAGE_TIP_SWITCH,
            value,
            timestamp_ms: None,
        }
    }

    fn is_x(&self) -> bool {
        self.usage_page == usage::PAGE_GENERIC_DESKTOP && self.usage == usage::USAGE_X
    }

    fn is_y(&self) -> bool {
        self.usage_page == usage::PAGE_GENERIC_DESKTOP && self.usage == usage::USAGE_Y
    }

    fn is_tip(&self) -> bool {
        self.usage_page == usage::PAGE_DIGITIZER && self.usage == usage::USAGE_TIP_SWITCH
    }
}

/// A classified finger-ring gesture.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Gesture {
    Tap,
    SwipeUp,
    SwipeDown,
    SwipeLeft,
    SwipeRight,
}

impl Gesture {
    /// Stable snake_case identifier matching the config keys and serde form.
    pub fn as_str(&self) -> &'static str {
        match self {
            Gesture::Tap => "tap",
            Gesture::SwipeUp => "swipe_up",
            Gesture::SwipeDown => "swipe_down",
            Gesture::SwipeLeft => "swipe_left",
            Gesture::SwipeRight => "swipe_right",
        }
    }
}

/// Stateful gesture classifier.
///
/// Feed raw HID element events in device order via [`feed`](Self::feed); a
/// gesture is returned exactly when a contact ends (tip-switch 1→0).
#[derive(Debug, Clone, Default)]
pub struct GestureClassifier {
    touching: bool,
    last_x: Option<i32>,
    last_y: Option<i32>,
    sx: Option<i32>,
    sy: Option<i32>,
    ex: Option<i32>,
    ey: Option<i32>,
}

impl GestureClassifier {
    /// Construct a fresh classifier with no contact in progress.
    pub fn new() -> Self {
        Self::default()
    }

    /// Consume one raw event; return `Some(gesture)` exactly on contact end.
    pub fn feed(&mut self, event: RawEvent) -> Option<Gesture> {
        if event.is_x() {
            self.last_x = Some(event.value);
            if self.touching {
                self.ex = Some(event.value);
            }
            None
        } else if event.is_y() {
            self.last_y = Some(event.value);
            if self.touching {
                self.ey = Some(event.value);
            }
            None
        } else if event.is_tip() {
            if event.value != 0 && !self.touching {
                // Contact begin: capture start from the running last_x/last_y cache,
                // and seed end with the same coordinates so a motionless contact
                // classifies as a tap.
                self.touching = true;
                self.sx = self.last_x;
                self.sy = self.last_y;
                self.ex = self.last_x;
                self.ey = self.last_y;
                None
            } else if event.value == 0 && self.touching {
                // Contact end: classify, then reset inter-contact position cache
                // exactly like the Python prototype.
                self.touching = false;
                let gesture = classify_contact(self.sx, self.sy, self.ex, self.ey);
                self.last_x = None;
                self.last_y = None;
                Some(gesture)
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Start/end coordinates of the most recently classified contact.
    ///
    /// Returns `(sx, sy, ex, ey)`. Useful for tests and debug UI; not part of
    /// the classification contract.
    #[allow(clippy::type_complexity)]
    pub fn last_contact_span(
        &self,
    ) -> Option<(Option<i32>, Option<i32>, Option<i32>, Option<i32>)> {
        Some((self.sx, self.sy, self.ex, self.ey))
    }

    /// Whether a contact is currently in progress.
    pub fn is_touching(&self) -> bool {
        self.touching
    }
}

/// Classify a completed contact from its start/end coordinates.
///
/// Matches the Python prototype: any `None` coordinate ⇒ tap; below threshold
/// ⇒ tap; otherwise the dominant axis wins, with digitizer Y growing downward
/// so `dy < 0 ⇒ SwipeUp`.
fn classify_contact(sx: Option<i32>, sy: Option<i32>, ex: Option<i32>, ey: Option<i32>) -> Gesture {
    let (Some(sx), Some(sy), Some(ex), Some(ey)) = (sx, sy, ex, ey) else {
        return Gesture::Tap;
    };
    let dx = ex - sx;
    let dy = ey - sy;
    if dx.abs() < SWIPE_THRESHOLD && dy.abs() < SWIPE_THRESHOLD {
        Gesture::Tap
    } else if dx.abs() >= dy.abs() {
        if dx < 0 {
            Gesture::SwipeLeft
        } else {
            Gesture::SwipeRight
        }
    } else if dy < 0 {
        Gesture::SwipeUp
    } else {
        Gesture::SwipeDown
    }
}

/// Drain an event stream and return all emitted gestures, in order.
///
/// Convenience helper for tests and deterministic replay.
pub fn classify_events(events: impl IntoIterator<Item = RawEvent>) -> Vec<Gesture> {
    let mut clf = GestureClassifier::new();
    let mut out = Vec::new();
    for ev in events {
        if let Some(g) = clf.feed(ev) {
            out.push(g);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Motionless contact at a fixed position classifies as a tap.
    #[test]
    fn motionless_contact_is_tap() {
        let events = vec![
            RawEvent::x(1364),
            RawEvent::y(682),
            RawEvent::tip(1),
            RawEvent::tip(0),
        ];
        assert_eq!(classify_events(events), vec![Gesture::Tap]);
    }

    /// Bare tip-switch pulse with no positional preamble classifies as a tap
    /// (start/end are `None`).
    #[test]
    fn bare_tip_pulse_is_tap() {
        let events = vec![RawEvent::tip(1), RawEvent::tip(0)];
        assert_eq!(classify_events(events), vec![Gesture::Tap]);
    }

    /// Below-threshold movement is a tap, not a swipe.
    #[test]
    fn subthreshold_move_is_tap() {
        let events = vec![
            RawEvent::x(100),
            RawEvent::y(100),
            RawEvent::tip(1),
            RawEvent::x(110),
            RawEvent::y(105),
            RawEvent::tip(0),
        ];
        assert_eq!(classify_events(events), vec![Gesture::Tap]);
    }

    /// Swipe up: Y decreasing (digitizer Y grows downward).
    #[test]
    fn swipe_up_y_decreasing() {
        let events = vec![
            RawEvent::x(1364),
            RawEvent::y(818),
            RawEvent::tip(1),
            RawEvent::y(316),
            RawEvent::tip(0),
        ];
        assert_eq!(classify_events(events), vec![Gesture::SwipeUp]);
    }

    /// Swipe down: Y increasing.
    #[test]
    fn swipe_down_y_increasing() {
        let events = vec![
            RawEvent::x(1364),
            RawEvent::y(409),
            RawEvent::tip(1),
            RawEvent::y(847),
            RawEvent::tip(0),
        ];
        assert_eq!(classify_events(events), vec![Gesture::SwipeDown]);
    }

    /// Horizontal moves classify on the X axis in both directions.
    #[test]
    fn horizontal_swipes() {
        let right = vec![
            RawEvent::x(100),
            RawEvent::y(500),
            RawEvent::tip(1),
            RawEvent::x(300),
            RawEvent::tip(0),
        ];
        let left = vec![
            RawEvent::x(300),
            RawEvent::y(500),
            RawEvent::tip(1),
            RawEvent::x(100),
            RawEvent::tip(0),
        ];
        assert_eq!(classify_events(right), vec![Gesture::SwipeRight]);
        assert_eq!(classify_events(left), vec![Gesture::SwipeLeft]);
    }

    /// Diagonal resolves to the dominant axis (|dx| > |dy| here).
    #[test]
    fn diagonal_dominant_axis() {
        let events = vec![
            RawEvent::x(100),
            RawEvent::y(100),
            RawEvent::tip(1),
            RawEvent::x(400),
            RawEvent::y(200),
            RawEvent::tip(0),
        ];
        assert_eq!(classify_events(events), vec![Gesture::SwipeRight]);
    }

    /// State resets cleanly between contacts: three contacts emit three gestures.
    #[test]
    fn multiple_contacts_reset_state() {
        let events = vec![
            // swipe up
            RawEvent::x(1364),
            RawEvent::y(818),
            RawEvent::tip(1),
            RawEvent::y(316),
            RawEvent::tip(0),
            // tap
            RawEvent::x(1400),
            RawEvent::y(600),
            RawEvent::tip(1),
            RawEvent::tip(0),
            // swipe down
            RawEvent::x(1364),
            RawEvent::y(409),
            RawEvent::tip(1),
            RawEvent::y(847),
            RawEvent::tip(0),
        ];
        assert_eq!(
            classify_events(events),
            vec![Gesture::SwipeUp, Gesture::Tap, Gesture::SwipeDown]
        );
    }

    /// A mid-contact move that does not end the contact emits nothing, and the
    /// eventual gesture reflects the full start→end span.
    #[test]
    fn mid_contact_move_emits_nothing() {
        let mut clf = GestureClassifier::new();
        assert_eq!(clf.feed(RawEvent::x(100)), None);
        assert_eq!(clf.feed(RawEvent::y(100)), None);
        assert_eq!(clf.feed(RawEvent::tip(1)), None);
        assert_eq!(clf.feed(RawEvent::x(300)), None); // dx=200 ≥ threshold
        assert!(clf.is_touching());
        assert_eq!(clf.feed(RawEvent::tip(0)), Some(Gesture::SwipeRight));
        assert!(!clf.is_touching());
    }

    /// `last_contact_span` reflects the most recent start/end coordinates.
    #[test]
    fn last_contact_span_reports_state() {
        let mut clf = GestureClassifier::new();
        for ev in [
            RawEvent::x(100),
            RawEvent::y(100),
            RawEvent::tip(1),
            RawEvent::x(250),
            RawEvent::tip(0),
        ] {
            clf.feed(ev);
        }
        assert_eq!(
            clf.last_contact_span(),
            Some((Some(100), Some(100), Some(250), Some(100)))
        );
    }

    /// Gesture serde round-trips to snake_case identifiers. TOML requires a
    /// table at the root, so we wrap the gesture in a one-field struct.
    #[test]
    fn gesture_serde_roundtrip() {
        #[derive(serde::Serialize, serde::Deserialize)]
        struct Wrapper {
            g: Gesture,
        }
        for g in [
            Gesture::Tap,
            Gesture::SwipeUp,
            Gesture::SwipeDown,
            Gesture::SwipeLeft,
            Gesture::SwipeRight,
        ] {
            let s = toml::to_string(&Wrapper { g }).expect("serialize gesture");
            // The serialized form should contain `g = "swipe_up"` etc.
            assert!(
                s.contains(&format!("g = \"{}\"", g.as_str())),
                "serialized form {s} does not mention {}",
                g.as_str()
            );
            let back: Wrapper = toml::from_str(&s).expect("deserialize gesture");
            assert_eq!(g, back.g);
        }
    }
}
