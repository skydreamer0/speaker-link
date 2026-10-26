//! Pure view-model: observations remain distinct from capability and freshness.
use speakerlink_core::model::{
    BatterySource, CapabilityState, Freshness, Observation, Snapshot, UnknownReason,
};
use speakerlink_core::SCHEMA_VERSION;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum SessionState {
    #[default]
    Idle,
    Busy,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DeviceRow {
    pub label: String,
    pub paired: String,
    pub connected: String,
    pub active_audio: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SnapshotView {
    pub freshness: &'static str,
    pub observed_at: String,
    pub battery: String,
    pub charging: String,
    pub battery_source: String,
    pub devices_summary: String,
    pub devices: Vec<DeviceRow>,
    pub capabilities: [CapabilityState; 4],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ViewState {
    Empty,
    UnsupportedSchema { received: u32 },
    Snapshot(SnapshotView),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AppModel {
    pub view: ViewState,
    pub session: SessionState,
}

impl Default for AppModel {
    fn default() -> Self {
        Self {
            view: ViewState::Empty,
            session: SessionState::Idle,
        }
    }
}

impl AppModel {
    pub fn from_snapshot(snapshot: &Snapshot, session: SessionState) -> Self {
        if snapshot.schema_version != SCHEMA_VERSION {
            return Self {
                view: ViewState::UnsupportedSchema {
                    received: snapshot.schema_version,
                },
                session,
            };
        }
        let (devices_summary, devices) = match &snapshot.speaker_devices {
            Observation::Unknown(reason) => {
                (format!("Unknown — {}", unknown_reason(*reason)), Vec::new())
            }
            Observation::Known(devices) => (
                if devices.is_empty() {
                    "Observed empty speaker-side list".into()
                } else {
                    format!("{} observed speaker-side sources", devices.len())
                },
                devices
                    .iter()
                    .enumerate()
                    .map(|(index, device)| DeviceRow {
                        // Never render local IDs or infer one connection flag from another.
                        label: format!("Source {}", index + 1),
                        paired: boolean(&device.paired),
                        connected: boolean(&device.connected),
                        active_audio: boolean(&device.active_audio),
                    })
                    .collect(),
            ),
        };
        Self {
            session,
            view: ViewState::Snapshot(SnapshotView {
                freshness: match snapshot.freshness {
                    Freshness::Fresh => "Fresh",
                    Freshness::Stale => "Stale — cached observation, not current state",
                    Freshness::Unknown => "Unknown freshness",
                },
                observed_at: snapshot
                    .observed_at
                    .map(|value| format!("{value} ms since Unix epoch"))
                    .unwrap_or_else(|| "Not observed".into()),
                battery: match &snapshot.battery.percent {
                    Observation::Known(value) => format!("{}%", value.get()),
                    Observation::Unknown(reason) => {
                        format!("Unknown — {}", unknown_reason(*reason))
                    }
                },
                charging: boolean(&snapshot.battery.charging),
                battery_source: match &snapshot.battery.source {
                    Observation::Known(BatterySource::SpeakerProtocol) => "Speaker protocol".into(),
                    Observation::Known(BatterySource::OperatingSystem) => "Operating system".into(),
                    Observation::Unknown(reason) => {
                        format!("Unknown — {}", unknown_reason(*reason))
                    }
                },
                devices_summary,
                devices,
                capabilities: [
                    snapshot.capabilities.battery,
                    snapshot.capabilities.speaker_devices,
                    snapshot.capabilities.set_connection,
                    snapshot.capabilities.diagnostics,
                ],
            }),
        }
    }

    /// A display-only preview never authorizes any adapter operation, including
    /// when a synthetic snapshot claims a verified capability.
    pub const fn controls_enabled(&self) -> bool {
        false
    }
}

fn boolean(value: &Observation<bool>) -> String {
    match value {
        Observation::Known(true) => "Yes".into(),
        Observation::Known(false) => "No".into(),
        Observation::Unknown(reason) => format!("Unknown — {}", unknown_reason(*reason)),
    }
}

fn unknown_reason(reason: UnknownReason) -> &'static str {
    match reason {
        UnknownReason::NotObserved => "not observed",
        UnknownReason::Disconnected => "disconnected",
        UnknownReason::Unsupported => "unsupported",
        UnknownReason::NotImplemented => "not implemented",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use speakerlink_core::model::{BatteryPercent, DeviceRef, SpeakerDevice, SpeakerTargetRef};
    fn sample() -> Snapshot {
        Snapshot::unobserved(DeviceRef::new("private-local-id").unwrap())
    }
    fn view(model: &AppModel) -> &SnapshotView {
        match &model.view {
            ViewState::Snapshot(view) => view,
            _ => panic!("snapshot expected"),
        }
    }
    #[test]
    fn starts_empty_without_inventing_device() {
        assert_eq!(AppModel::default().view, ViewState::Empty);
        assert!(!AppModel::default().controls_enabled());
    }
    #[test]
    fn unknown_is_not_zero_or_empty() {
        let model = AppModel::from_snapshot(&sample(), SessionState::Idle);
        assert!(view(&model).battery.starts_with("Unknown"));
        assert!(view(&model).devices_summary.starts_with("Unknown"));
        assert_eq!(view(&model).observed_at, "Not observed");
    }
    #[test]
    fn known_zero_and_known_empty_are_preserved() {
        let mut snapshot = sample();
        snapshot.battery.percent = Observation::Known(BatteryPercent::new(0).unwrap());
        snapshot.speaker_devices = Observation::Known(Vec::new());
        let model = AppModel::from_snapshot(&snapshot, SessionState::Idle);
        assert_eq!(view(&model).battery, "0%");
        assert_eq!(
            view(&model).devices_summary,
            "Observed empty speaker-side list"
        );
    }
    #[test]
    fn stale_value_and_timestamp_remain_explicit() {
        let mut snapshot = sample();
        snapshot.freshness = Freshness::Stale;
        snapshot.observed_at = Some(1000);
        snapshot.battery.percent = Observation::Known(BatteryPercent::new(67).unwrap());
        let model = AppModel::from_snapshot(&snapshot, SessionState::Idle);
        assert!(view(&model).freshness.starts_with("Stale"));
        assert_eq!(view(&model).battery, "67%");
        assert_eq!(view(&model).observed_at, "1000 ms since Unix epoch");
    }
    #[test]
    fn source_states_are_independent_and_ids_not_exposed() {
        let mut snapshot = sample();
        snapshot.speaker_devices = Observation::Known(vec![SpeakerDevice {
            target_ref: SpeakerTargetRef::new("secret-source-id").unwrap(),
            paired: Observation::Known(true),
            connected: Observation::Known(false),
            active_audio: Observation::Unknown(UnknownReason::NotObserved),
        }]);
        let model = AppModel::from_snapshot(&snapshot, SessionState::Idle);
        let row = &view(&model).devices[0];
        assert_eq!(row.paired, "Yes");
        assert_eq!(row.connected, "No");
        assert!(row.active_audio.starts_with("Unknown"));
        let display = format!("{model:?}");
        assert!(!display.contains("private-local-id"));
        assert!(!display.contains("secret-source-id"));
    }
    #[test]
    fn verified_or_busy_can_never_enable_hardware_controls() {
        let mut snapshot = sample();
        snapshot.capabilities.set_connection = CapabilityState::Verified;
        for session in [SessionState::Idle, SessionState::Busy] {
            let model = AppModel::from_snapshot(&snapshot, session);
            assert!(!model.controls_enabled());
            assert_eq!(model.session, session);
        }
    }
    #[test]
    fn unsupported_and_unknown_capabilities_are_distinct() {
        let mut snapshot = sample();
        snapshot.capabilities.battery = CapabilityState::Unsupported;
        let model = AppModel::from_snapshot(&snapshot, SessionState::Idle);
        assert_eq!(view(&model).capabilities[0], CapabilityState::Unsupported);
        assert_eq!(view(&model).capabilities[1], CapabilityState::Unknown);
    }
    #[test]
    fn unknown_schema_fails_closed() {
        let mut snapshot = sample();
        snapshot.schema_version += 1;
        let model = AppModel::from_snapshot(&snapshot, SessionState::Idle);
        assert_eq!(
            model.view,
            ViewState::UnsupportedSchema {
                received: snapshot.schema_version
            }
        );
        assert!(!model.controls_enabled());
    }
}
