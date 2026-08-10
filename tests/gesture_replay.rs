//! Replay fixtures for the gesture classifier.
//!
//! These replay synthetic but PRD-accurate event streams through the
//! classifier and assert the resulting gestures. The salient coordinates come
//! from `docs/PRD.md` line 68 and the captured WX02 session:
//!
//! - swipe-up:    Y 818 → 316  (finger up = decreasing Y ⇒ SwipeUp)
//! - swipe-down:  Y 409 → 847  (finger down = increasing Y ⇒ SwipeDown)
//! - tap:         motionless contact at (1364, 682) ⇒ Tap

use agentring::gestures::{classify_events, Gesture, RawEvent};

/// Helper: a complete single-contact stream.
///
/// `pre_x`/`pre_y` establish position before touch-begin (they seed the
/// `last_x`/`last_y` cache that the start position is captured from). During
/// the contact, optional `mid_x`/`mid_y` events update the running end
/// position; the final end coordinate is whatever the last such event set.
fn contact(
    pre_x: Option<i32>,
    pre_y: Option<i32>,
    mid: Vec<RawEvent>,
    final_x: Option<i32>,
    final_y: Option<i32>,
) -> Vec<RawEvent> {
    let mut evs = Vec::new();
    if let Some(x) = pre_x {
        evs.push(RawEvent::x(x));
    }
    if let Some(y) = pre_y {
        evs.push(RawEvent::y(y));
    }
    evs.push(RawEvent::tip(1));
    evs.extend(mid);
    if let Some(x) = final_x {
        evs.push(RawEvent::x(x));
    }
    if let Some(y) = final_y {
        evs.push(RawEvent::y(y));
    }
    evs.push(RawEvent::tip(0));
    evs
}

#[test]
fn fixture_swipe_up_y_818_to_316() {
    // Y 818 → 316: dy = -502, below the |dx|=0 threshold axis ⇒ SwipeUp.
    let events = contact(Some(1364), Some(818), vec![], None, Some(316));
    assert_eq!(classify_events(events), vec![Gesture::SwipeUp]);
}

#[test]
fn fixture_swipe_down_y_409_to_847() {
    // Y 409 → 847: dy = +438 ⇒ SwipeDown.
    let events = contact(Some(1364), Some(409), vec![], None, Some(847));
    assert_eq!(classify_events(events), vec![Gesture::SwipeDown]);
}

#[test]
fn fixture_tap_at_1364_682_motionless() {
    // A motionless contact at (1364, 682): below threshold ⇒ Tap.
    let events = contact(Some(1364), Some(682), vec![], None, None);
    assert_eq!(classify_events(events), vec![Gesture::Tap]);
}

#[test]
fn fixture_bare_tap_no_positional_preamble() {
    // Tip-switch pulse with no X/Y preamble at all: sx/sy/ex/ey all None ⇒ Tap
    // (matches the Python prototype's None-handling).
    let events = vec![RawEvent::tip(1), RawEvent::tip(0)];
    assert_eq!(classify_events(events), vec![Gesture::Tap]);
}

#[test]
fn fixture_multi_contact_resets_state() {
    // Three back-to-back contacts must produce three gestures in order — this
    // proves the start/end caches reset cleanly between contacts.
    let mut events = Vec::new();
    // 1) swipe up (Y 818 → 316)
    events.extend(contact(Some(1364), Some(818), vec![], None, Some(316)));
    // 2) tap at (1400, 600)
    events.extend(contact(Some(1400), Some(600), vec![], None, None));
    // 3) swipe down (Y 409 → 847)
    events.extend(contact(Some(1364), Some(409), vec![], None, Some(847)));
    assert_eq!(
        classify_events(events),
        vec![Gesture::SwipeUp, Gesture::Tap, Gesture::SwipeDown]
    );
}

#[test]
fn fixture_horizontal_right_dominant() {
    // dx = 400 (≥150), dy = 5 (<150) ⇒ SwipeRight.
    let events = contact(Some(100), Some(500), vec![], Some(500), Some(505));
    assert_eq!(classify_events(events), vec![Gesture::SwipeRight]);
}

#[test]
fn fixture_horizontal_left_dominant() {
    // dx = -400 (|dx|≥150), dy small ⇒ SwipeLeft.
    let events = contact(Some(500), Some(500), vec![], Some(100), Some(505));
    assert_eq!(classify_events(events), vec![Gesture::SwipeLeft]);
}

#[test]
fn fixture_subthreshold_move_is_tap() {
    // dx = 10, dy = 5: both below SWIPE_THRESHOLD (150) ⇒ Tap, not a swipe.
    let events = contact(Some(100), Some(100), vec![], Some(110), Some(105));
    assert_eq!(classify_events(events), vec![Gesture::Tap]);
}

#[test]
fn fixture_mid_contact_progression_is_swipe_up() {
    // Multi-step mid-contact Y descent (818 → 600 → 316) must still classify
    // as SwipeUp — proves end position tracks the running cache.
    let mid = vec![RawEvent::y(600), RawEvent::y(450)];
    let events = contact(Some(1364), Some(818), mid, None, Some(316));
    assert_eq!(classify_events(events), vec![Gesture::SwipeUp]);
}

#[test]
fn fixture_start_captured_from_last_xy_before_touch() {
    // The Python prototype captures sx/sy from the last-seen X/Y before
    // tip=1, NOT from a fresh X/Y read after touch-begin. So emitting X=100,
    // Y=200 *before* tip=1 then a far Y *after* tip=1 must treat 200 as sy.
    // Y 200 → 50 ⇒ dy = -150, exactly at threshold ⇒ SwipeUp (>= threshold).
    let events = contact(Some(100), Some(200), vec![], None, Some(50));
    assert_eq!(classify_events(events), vec![Gesture::SwipeUp]);
}
