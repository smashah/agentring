//! Device profile + device-matching rule.
//!
//! The WX02 finger-ring spoofs Apple VID 0x05AC / PID 0x0220 (a real Apple
//! keyboard), so VID/PID alone is never a safe match. The matching rule, quoted
//! from `docs/PRD.md` lines 35–36:
//!
//! > Device matching rule: the ring spoofs Apple VID 0x05AC / PID 0x0220 (real
//! > Apple keyboard IDs) — never match VID/PID alone; match product string WX02
//! > + transport Bluetooth + usage page, encoded in the device profile.
//!
//! `DeviceProfile::matches` encodes this rule and rejects descriptors where
//! VID/PID is the only thing that matches.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::gestures::usage;

/// Transport over which a HID device advertises itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Transport {
    Bluetooth,
    Usb,
    /// Wildcard — matches any transport.
    Any,
}

/// A logical field exposed by a device profile, used to map abstract field
/// names (X, Y, tip switch) to concrete HID `(usage_page, usage)` identities.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Field {
    X,
    Y,
    TipSwitch,
}

/// Mapping from logical field to HID `(usage_page, usage)`.
///
/// Stored as a sorted map keyed by field name so the TOML representation is
/// stable and ergonomic.
pub type FieldMap = BTreeMap<Field, (u16, u32)>;

/// A device profile: how to recognise a supported device and how to read its
/// HID elements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceProfile {
    pub id: String,
    pub product_string: String,
    pub transport: Transport,
    pub usage_pages: Vec<u16>,
    pub fields: FieldMap,
}

impl DeviceProfile {
    /// Canonical profile for the WX02 Bluetooth finger-ring.
    ///
    /// The `usage_pages` list includes the digitizer (0x0D) and generic desktop
    /// (0x01) pages used by the swipe/tap classifier, plus the consumer page
    /// (0x0C) used by the long-press consumer keys.
    pub fn wx02() -> Self {
        let mut fields = FieldMap::new();
        fields.insert(Field::X, (usage::PAGE_GENERIC_DESKTOP, usage::USAGE_X));
        fields.insert(Field::Y, (usage::PAGE_GENERIC_DESKTOP, usage::USAGE_Y));
        fields.insert(
            Field::TipSwitch,
            (usage::PAGE_DIGITIZER, usage::USAGE_TIP_SWITCH),
        );
        Self {
            id: "wx02".to_string(),
            product_string: "WX02".to_string(),
            transport: Transport::Bluetooth,
            usage_pages: vec![
                usage::PAGE_GENERIC_DESKTOP,
                usage::PAGE_DIGITIZER,
                usage::PAGE_CONSUMER,
            ],
            fields,
        }
    }

    /// Required usage pages — the sorted, deduplicated union of the pages
    /// declared in `usage_pages` and the pages referenced by the field map.
    /// For the WX02 profile this yields generic desktop (0x01), consumer
    /// (0x0C), and digitizer (0x0D).
    pub fn required_usage_pages(&self) -> Vec<u16> {
        let mut pages: Vec<u16> = self
            .usage_pages
            .iter()
            .copied()
            .chain(self.fields.values().map(|(p, _)| *p))
            .collect();
        pages.sort_unstable();
        pages.dedup();
        pages
    }
}

/// A discovered device's identifying attributes, as reported by the OS HID
/// layer. All fields are optional because any given OS may report a subset.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DeviceId {
    pub vendor_id: Option<u16>,
    pub product_id: Option<u16>,
    pub product_string: Option<String>,
    pub transport: Option<Transport>,
    pub usage_pages: Vec<u16>,
}

impl DeviceId {
    /// The spoofed Apple keyboard descriptor: VID 0x05AC, PID 0x0220, no
    /// product string, no transport, no usage pages. Used to prove that
    /// `DeviceProfile::matches` rejects VID/PID-only matches.
    pub fn apple_keyboard_spoof() -> Self {
        Self {
            vendor_id: Some(0x05AC),
            product_id: Some(0x0220),
            product_string: None,
            transport: None,
            usage_pages: Vec::new(),
        }
    }

    /// A complete, honest WX02 descriptor — product string, Bluetooth
    /// transport, and the generic desktop, consumer, and digitizer usage
    /// pages. VID/PID spoofing (0x05AC / 0x0220) is tolerated because the
    /// structural signals are all present.
    pub fn wx02_bluetooth() -> Self {
        Self {
            vendor_id: Some(0x05AC),
            product_id: Some(0x0220),
            product_string: Some("WX02".to_string()),
            transport: Some(Transport::Bluetooth),
            usage_pages: vec![
                usage::PAGE_GENERIC_DESKTOP,
                usage::PAGE_CONSUMER,
                usage::PAGE_DIGITIZER,
            ],
        }
    }
}

/// Why a profile rejected a candidate device.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RejectReason {
    /// Only VID/PID matched — unsafe because the WX02 spoofs Apple keyboard IDs.
    /// This is the safety gate against the spoofed-Apple trap.
    VendorPidOnly,
    /// Product string missing or did not equal the profile's product string.
    ProductStringMismatch,
    /// Transport missing or did not satisfy the profile's transport.
    TransportMismatch,
    /// One or more required usage pages were absent from the candidate.
    MissingUsagePages(Vec<u16>),
}

/// The outcome of matching a [`DeviceProfile`] against a [`DeviceId`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchResult {
    Match,
    Reject(RejectReason),
}

impl DeviceProfile {
    /// Match this profile against a candidate device descriptor.
    ///
    /// Rule: match iff (a) `product_string` equals the profile's, (b)
    /// `transport` is satisfied, and (c) all required usage pages (derived from
    /// the field map) are present in the candidate. VID/PID is allowed but
    /// never sufficient on its own — a descriptor that only matches on
    /// VID/PID is rejected with [`RejectReason::VendorPidOnly`].
    pub fn matches(&self, candidate: &DeviceId) -> MatchResult {
        let product_ok = candidate
            .product_string
            .as_deref()
            .map(|s| s == self.product_string)
            .unwrap_or(false);

        let transport_ok = match self.transport {
            Transport::Any => true,
            _ => candidate.transport == Some(self.transport),
        };

        let required = self.required_usage_pages();
        let missing: Vec<u16> = required
            .iter()
            .copied()
            .filter(|p| !candidate.usage_pages.contains(p))
            .collect();
        let usage_ok = missing.is_empty();

        // Compute the "VID/PID only" condition: nothing structural matches.
        // This is the explicit safety gate against the spoofed-Apple trap.
        let vid_pid_only = !product_ok && !transport_ok && !usage_ok;

        if product_ok && transport_ok && usage_ok {
            MatchResult::Match
        } else if vid_pid_only {
            MatchResult::Reject(RejectReason::VendorPidOnly)
        } else if !product_ok {
            MatchResult::Reject(RejectReason::ProductStringMismatch)
        } else if !transport_ok {
            MatchResult::Reject(RejectReason::TransportMismatch)
        } else {
            MatchResult::Reject(RejectReason::MissingUsagePages(missing))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wx02_profile_fields_are_well_formed() {
        let p = DeviceProfile::wx02();
        assert_eq!(p.product_string, "WX02");
        assert_eq!(p.transport, Transport::Bluetooth);
        assert_eq!(
            p.required_usage_pages(),
            vec![
                usage::PAGE_GENERIC_DESKTOP,
                usage::PAGE_CONSUMER,
                usage::PAGE_DIGITIZER,
            ]
        );
        assert_eq!(
            p.fields.get(&Field::X).copied(),
            Some((usage::PAGE_GENERIC_DESKTOP, usage::USAGE_X))
        );
        assert_eq!(
            p.fields.get(&Field::TipSwitch).copied(),
            Some((usage::PAGE_DIGITIZER, usage::USAGE_TIP_SWITCH))
        );
    }

    #[test]
    fn honest_wx02_descriptor_matches() {
        let profile = DeviceProfile::wx02();
        let result = profile.matches(&DeviceId::wx02_bluetooth());
        assert_eq!(result, MatchResult::Match);
    }

    #[test]
    fn spoofed_apple_descriptor_rejected_as_vid_pid_only() {
        let profile = DeviceProfile::wx02();
        let result = profile.matches(&DeviceId::apple_keyboard_spoof());
        assert_eq!(result, MatchResult::Reject(RejectReason::VendorPidOnly));
    }

    #[test]
    fn wrong_product_string_rejected() {
        let profile = DeviceProfile::wx02();
        let mut dev = DeviceId::wx02_bluetooth();
        dev.product_string = Some("Magic Keyboard".to_string());
        // VID/PID still match, but transport + usage pages also match — so the
        // structural signals are satisfied except for product string, hence
        // this is NOT VendorPidOnly.
        let result = profile.matches(&dev);
        assert_eq!(
            result,
            MatchResult::Reject(RejectReason::ProductStringMismatch)
        );
    }

    #[test]
    fn wrong_transport_rejected() {
        let profile = DeviceProfile::wx02();
        let mut dev = DeviceId::wx02_bluetooth();
        dev.transport = Some(Transport::Usb);
        assert_eq!(
            profile.matches(&dev),
            MatchResult::Reject(RejectReason::TransportMismatch)
        );
    }

    #[test]
    fn missing_usage_page_rejected() {
        let profile = DeviceProfile::wx02();
        let mut dev = DeviceId::wx02_bluetooth();
        // Drop the digitizer page (keep generic desktop + consumer).
        dev.usage_pages = vec![usage::PAGE_GENERIC_DESKTOP, usage::PAGE_CONSUMER];
        match profile.matches(&dev) {
            MatchResult::Reject(RejectReason::MissingUsagePages(missing)) => {
                assert_eq!(missing, vec![usage::PAGE_DIGITIZER]);
            }
            other => panic!("expected MissingUsagePages, got {other:?}"),
        }
    }

    #[test]
    fn missing_consumer_usage_page_rejected() {
        // A candidate that is otherwise a valid WX02 (product "WX02",
        // Bluetooth) but is missing the consumer usage page must be rejected
        // with MissingUsagePages naming page 0x0C. This guards the union of
        // declared `usage_pages` and field-map pages: previously the consumer
        // page was not enforced and a candidate lacking it still matched.
        let profile = DeviceProfile::wx02();
        let mut dev = DeviceId::wx02_bluetooth();
        dev.usage_pages = vec![usage::PAGE_GENERIC_DESKTOP, usage::PAGE_DIGITIZER];
        match profile.matches(&dev) {
            MatchResult::Reject(RejectReason::MissingUsagePages(missing)) => {
                assert_eq!(missing, vec![usage::PAGE_CONSUMER]);
            }
            other => panic!("expected MissingUsagePages, got {other:?}"),
        }
    }

    #[test]
    fn profile_round_trips_toml() {
        let p = DeviceProfile::wx02();
        let s = toml::to_string(&p).expect("serialize");
        let back: DeviceProfile = toml::from_str(&s).expect("deserialize");
        assert_eq!(back.id, p.id);
        assert_eq!(back.product_string, p.product_string);
        assert_eq!(back.transport, p.transport);
        assert_eq!(back.usage_pages, p.usage_pages);
        assert_eq!(back.fields, p.fields);
    }
}
