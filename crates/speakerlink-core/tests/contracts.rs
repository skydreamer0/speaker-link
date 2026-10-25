//! Entirely synthetic contract inputs. No device access or captured data.
use speakerlink_core::command::{Command, DesiredConnectionState, SetConnectionRequest};
use speakerlink_core::diagnostics::DiagnosticPreview;
use speakerlink_core::model::{
    BatteryPercent, Capabilities, CapabilityState, DeviceRef, Freshness, Observation, Snapshot,
    SpeakerDevice, SpeakerTargetRef, UnknownReason,
};
use speakerlink_core::{ErrorCode, SCHEMA_VERSION};

const ERRORS: [(ErrorCode, &str, u8); 9] = [
    (ErrorCode::InvalidArgument, "InvalidArgument", 2),
    (ErrorCode::Unsupported, "Unsupported", 3),
    (ErrorCode::AccessDenied, "AccessDenied", 4),
    (ErrorCode::Disconnected, "Disconnected", 5),
    (ErrorCode::Timeout, "Timeout", 6),
    (ErrorCode::ProtocolError, "ProtocolError", 7),
    (ErrorCode::Busy, "Busy", 8),
    (ErrorCode::Indeterminate, "Indeterminate", 9),
    (ErrorCode::InternalError, "InternalError", 10),
];

#[test]
fn errors_match_the_spec_codes_and_cli_exit_contract() {
    for (code, name, exit) in ERRORS {
        assert_eq!(code.as_str(), name);
        assert_eq!(code.cli_exit_code(), exit);
    }
}

#[test]
fn battery_percent_accepts_only_the_full_zero_to_hundred_range() {
    for value in 0..=u8::MAX {
        match BatteryPercent::new(value) {
            Ok(percent) => {
                assert!(value <= 100);
                assert_eq!(percent.get(), value);
            }
            Err(error) => {
                assert!(value > 100);
                assert_eq!(error, ErrorCode::InvalidArgument);
            }
        }
    }
}

#[test]
fn empty_control_and_oversized_local_references_are_rejected() {
    for invalid in ["", " ", "\n", "synthetic\0device", "synthetic\rdevice"] {
        assert_eq!(DeviceRef::new(invalid), Err(ErrorCode::InvalidArgument));
        assert_eq!(
            SpeakerTargetRef::new(invalid),
            Err(ErrorCode::InvalidArgument)
        );
    }
    assert!(DeviceRef::new("a".repeat(4096)).is_ok());
    assert_eq!(
        DeviceRef::new("a".repeat(4097)),
        Err(ErrorCode::InvalidArgument)
    );
}

#[test]
fn unicode_local_references_are_preserved_locally_but_not_in_debug() {
    let raw = "synthetic:測試喇叭🎵";
    let device = DeviceRef::new(raw).unwrap();
    let target = SpeakerTargetRef::new(raw).unwrap();
    assert_eq!(device.as_local_str(), raw);
    assert_eq!(target.as_local_str(), raw);
    assert!(!format!("{device:?} {target:?}").contains(raw));
}

#[test]
fn snapshot_starts_unknown_instead_of_inventing_battery_or_an_empty_list() {
    let snapshot = Snapshot::unobserved(DeviceRef::new("synthetic-speaker").unwrap());
    assert_eq!(snapshot.schema_version, SCHEMA_VERSION);
    assert_eq!(snapshot.observed_at, None);
    assert_eq!(snapshot.freshness, Freshness::Unknown);
    assert_eq!(snapshot.capabilities, Capabilities::default());
    assert_eq!(
        snapshot.battery.percent,
        Observation::Unknown(UnknownReason::NotObserved)
    );
    assert_eq!(
        snapshot.battery.charging,
        Observation::Unknown(UnknownReason::NotObserved)
    );
    assert_eq!(
        snapshot.battery.source,
        Observation::Unknown(UnknownReason::NotObserved)
    );
    assert_eq!(
        snapshot.speaker_devices,
        Observation::Unknown(UnknownReason::NotObserved)
    );
    assert_ne!(snapshot.speaker_devices, Observation::Known(Vec::new()));
}

#[test]
fn paired_connected_and_active_audio_remain_independent() {
    let device = SpeakerDevice {
        target_ref: SpeakerTargetRef::new("synthetic-source").unwrap(),
        paired: Observation::Known(true),
        connected: Observation::Unknown(UnknownReason::NotObserved),
        active_audio: Observation::Known(false),
    };
    assert_eq!(device.paired, Observation::Known(true));
    assert_eq!(
        device.connected,
        Observation::Unknown(UnknownReason::NotObserved)
    );
    assert_eq!(device.active_audio, Observation::Known(false));
}

#[test]
fn capability_states_do_not_collapse_unknown_into_unsupported() {
    assert_eq!(CapabilityState::Unknown.as_str(), "unknown");
    assert_eq!(CapabilityState::Unsupported.as_str(), "unsupported");
    assert_eq!(CapabilityState::Verified.as_str(), "verified");
    assert_ne!(CapabilityState::Unknown, CapabilityState::Unsupported);
}

#[test]
fn command_shapes_require_explicit_device_target_and_desired_state() {
    let command = Command::SetConnection(SetConnectionRequest {
        device_ref: DeviceRef::new("synthetic-speaker").unwrap(),
        target_ref: SpeakerTargetRef::new("synthetic-source").unwrap(),
        desired_state: DesiredConnectionState::Disconnected,
    });
    if let Command::SetConnection(request) = command {
        assert_eq!(request.desired_state, DesiredConnectionState::Disconnected);
        assert_eq!(request.device_ref.as_local_str(), "synthetic-speaker");
        assert_eq!(request.target_ref.as_local_str(), "synthetic-source");
    } else {
        panic!("unexpected command shape");
    }
}

#[test]
fn preview_has_exact_allowlisted_bytes_for_every_error() {
    for (code, name, _) in ERRORS {
        let preview = DiagnosticPreview::for_error(code);
        let expected = format!(
            "{{\"schema_version\":1,\"app_version\":\"0.0.0-dev\",\"error_code\":\"{name}\"}}\n"
        );
        assert_eq!(preview.as_bytes(), expected.as_bytes());
        assert_eq!(preview.as_bytes(), preview.clone().as_bytes());
    }
}

#[test]
fn synthetic_local_identifiers_and_raw_errors_cannot_enter_preview() {
    let seeds = [
        "02:00:00:00:00:01",
        "SYNTHETIC-SERIAL-0001",
        "BTHENUM\\SYNTHETIC_INSTANCE",
        "C:\\Users\\SYNTHETIC_USER\\private.txt",
        "/home/SYNTHETIC_USER/private.txt",
        "synthetic:私人喇叭🎵",
        "raw synthetic exception with identifiers",
    ];
    for seed in seeds {
        let snapshot = Snapshot::unobserved(DeviceRef::new(seed).unwrap());
        assert!(!format!("{snapshot:?}").contains(seed));
        // Only the enum, not the snapshot or raw exception, enters the preview.
        let preview = DiagnosticPreview::for_error(ErrorCode::ProtocolError);
        assert!(!std::str::from_utf8(preview.as_bytes())
            .unwrap()
            .contains(seed));
    }
}
