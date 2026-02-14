// SPDX-FileCopyrightText: 2026 RAprogramm <andrey.rozanov.vl@gmail.com>
// SPDX-License-Identifier: MIT

use serde::{Deserialize, Serialize};

/// Phase of the mini app lifecycle emitted by the host container.
///
/// The host delivers these events to let the application respond to visibility
/// and suspend/resume transitions.
///
/// # Examples
///
/// ```
/// use zalo_sdk::lifecycle::AppLifecycleEvent;
///
/// let json = r#""resume""#;
/// let event: AppLifecycleEvent = serde_json::from_str(json).unwrap();
/// assert_eq!(event, AppLifecycleEvent::Resume);
/// ```
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum AppLifecycleEvent {
    /// Application started for the first time in this session.
    Launch,
    /// Application returned to the foreground.
    Resume,
    /// Application moved to the background.
    Pause,
    /// Application is about to be closed.
    Destroy
}

impl AppLifecycleEvent {
    /// Returns `true` when the application is entering or resuming the
    /// foreground.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_sdk::lifecycle::AppLifecycleEvent;
    ///
    /// assert!(AppLifecycleEvent::Launch.is_foreground());
    /// assert!(AppLifecycleEvent::Resume.is_foreground());
    /// assert!(!AppLifecycleEvent::Pause.is_foreground());
    /// assert!(!AppLifecycleEvent::Destroy.is_foreground());
    /// ```
    #[must_use]
    pub fn is_foreground(self) -> bool {
        matches!(self, Self::Launch | Self::Resume)
    }

    /// Returns `true` when the application is leaving the foreground.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_sdk::lifecycle::AppLifecycleEvent;
    ///
    /// assert!(AppLifecycleEvent::Pause.is_background());
    /// assert!(AppLifecycleEvent::Destroy.is_background());
    /// assert!(!AppLifecycleEvent::Launch.is_background());
    /// ```
    #[must_use]
    pub fn is_background(self) -> bool {
        matches!(self, Self::Pause | Self::Destroy)
    }
}

/// Metadata delivered alongside a lifecycle event.
///
/// # Examples
///
/// ```
/// use zalo_sdk::lifecycle::{AppLifecycleEvent, LifecyclePayload};
///
/// let json = r#"{"event":"launch","timestamp_ms":1700000000000}"#;
/// let payload: LifecyclePayload = serde_json::from_str(json).unwrap();
/// assert_eq!(payload.event, AppLifecycleEvent::Launch);
/// assert_eq!(payload.timestamp_ms, 1700000000000);
/// ```
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub struct LifecyclePayload {
    /// The lifecycle transition that occurred.
    pub event:        AppLifecycleEvent,
    /// Unix epoch time in milliseconds when the event was generated.
    pub timestamp_ms: u64
}

impl LifecyclePayload {
    /// Constructs a lifecycle payload.
    ///
    /// # Examples
    ///
    /// ```
    /// use zalo_sdk::lifecycle::{AppLifecycleEvent, LifecyclePayload};
    ///
    /// let payload = LifecyclePayload::new(AppLifecycleEvent::Resume, 1700000000000);
    /// assert!(payload.event.is_foreground());
    /// ```
    #[must_use]
    pub fn new(event: AppLifecycleEvent, timestamp_ms: u64) -> Self {
        Self {
            event,
            timestamp_ms
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn launch_is_foreground() {
        assert!(AppLifecycleEvent::Launch.is_foreground());
        assert!(!AppLifecycleEvent::Launch.is_background());
    }

    #[test]
    fn resume_is_foreground() {
        assert!(AppLifecycleEvent::Resume.is_foreground());
    }

    #[test]
    fn pause_is_background() {
        assert!(AppLifecycleEvent::Pause.is_background());
        assert!(!AppLifecycleEvent::Pause.is_foreground());
    }

    #[test]
    fn destroy_is_background() {
        assert!(AppLifecycleEvent::Destroy.is_background());
    }

    #[test]
    fn lifecycle_event_serialises_to_lowercase() {
        let json = serde_json::to_string(&AppLifecycleEvent::Launch).expect("serialize");
        assert_eq!(json, r#""launch""#);
    }

    #[test]
    fn lifecycle_event_deserialises_all_variants() {
        let variants = ["launch", "resume", "pause", "destroy"];
        let expected = [
            AppLifecycleEvent::Launch,
            AppLifecycleEvent::Resume,
            AppLifecycleEvent::Pause,
            AppLifecycleEvent::Destroy
        ];
        for (raw, expected) in variants.iter().zip(expected.iter()) {
            let json = format!(r#""{raw}""#);
            let event: AppLifecycleEvent = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(&event, expected);
        }
    }

    #[test]
    fn lifecycle_payload_round_trips() {
        let payload = LifecyclePayload::new(AppLifecycleEvent::Resume, 1700000000001);
        let json = serde_json::to_string(&payload).expect("serialize");
        let back: LifecyclePayload = serde_json::from_str(&json).expect("deserialize");
        assert_eq!(back, payload);
    }

    #[test]
    fn lifecycle_payload_accessors() {
        let payload = LifecyclePayload::new(AppLifecycleEvent::Destroy, 999);
        assert_eq!(payload.event, AppLifecycleEvent::Destroy);
        assert_eq!(payload.timestamp_ms, 999);
    }
}
