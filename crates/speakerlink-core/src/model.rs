//! Local domain types. These are not diagnostic-export or CLI wire formats.
use crate::{ErrorCode, Result, SCHEMA_VERSION};
use std::fmt;

/// Local-only identifiers are deliberately absent from diagnostic projections.
/// This bound limits contract input size, not a claim about Windows ID limits.
const MAX_LOCAL_REFERENCE_BYTES: usize = 4096;

#[derive(Clone, Eq, PartialEq)]
pub struct DeviceRef(String);

impl DeviceRef {
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        if value.trim().is_empty()
            || value.len() > MAX_LOCAL_REFERENCE_BYTES
            || value.chars().any(char::is_control)
        {
            return Err(ErrorCode::InvalidArgument);
        }
        Ok(Self(value))
    }

    /// Use only with local adapters, never diagnostic serialization.
    pub fn as_local_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for DeviceRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DeviceRef(<local-only>)")
    }
}

/// A speaker-reported source target, not a Windows paired-device entry.
#[derive(Clone, Eq, PartialEq)]
pub struct SpeakerTargetRef(DeviceRef);

impl SpeakerTargetRef {
    pub fn new(value: impl Into<String>) -> Result<Self> {
        DeviceRef::new(value).map(Self)
    }

    pub fn as_local_str(&self) -> &str {
        self.0.as_local_str()
    }
}

impl fmt::Debug for SpeakerTargetRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SpeakerTargetRef(<local-only>)")
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnknownReason {
    NotObserved,
    Disconnected,
    Unsupported,
    NotImplemented,
}

/// Unknown is distinct from an observed empty list or an observed false value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Observation<T> {
    Known(T),
    Unknown(UnknownReason),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Freshness {
    Fresh,
    Stale,
    Unknown,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CapabilityState {
    Unknown,
    Unsupported,
    Verified,
}

impl CapabilityState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Unsupported => "unsupported",
            Self::Verified => "verified",
        }
    }
}

/// Display data only, never a write-authorization token. P0 has no verified
/// profiles or command executor. Evidence validation is gated by P1.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Capabilities {
    pub battery: CapabilityState,
    pub speaker_devices: CapabilityState,
    pub set_connection: CapabilityState,
    pub diagnostics: CapabilityState,
}

impl Default for Capabilities {
    fn default() -> Self {
        Self {
            battery: CapabilityState::Unknown,
            speaker_devices: CapabilityState::Unknown,
            set_connection: CapabilityState::Unknown,
            diagnostics: CapabilityState::Unknown,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BatteryPercent(u8);

impl BatteryPercent {
    pub fn new(value: u8) -> Result<Self> {
        if value > 100 {
            return Err(ErrorCode::InvalidArgument);
        }
        Ok(Self(value))
    }

    pub const fn get(self) -> u8 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BatterySource {
    SpeakerProtocol,
    OperatingSystem,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Battery {
    pub percent: Observation<BatteryPercent>,
    pub charging: Observation<bool>,
    pub source: Observation<BatterySource>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SpeakerDevice {
    pub target_ref: SpeakerTargetRef,
    pub paired: Observation<bool>,
    pub connected: Observation<bool>,
    pub active_audio: Observation<bool>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Snapshot {
    pub schema_version: u32,
    pub device_ref: DeviceRef,
    pub capabilities: Capabilities,
    /// Unix milliseconds of the speaker-state observation, not discovery time.
    pub observed_at: Option<u64>,
    pub freshness: Freshness,
    pub battery: Battery,
    pub speaker_devices: Observation<Vec<SpeakerDevice>>,
}

impl Snapshot {
    /// Discovery is not an observation of battery or speaker-side topology.
    pub fn unobserved(device_ref: DeviceRef) -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            device_ref,
            capabilities: Capabilities::default(),
            observed_at: None,
            freshness: Freshness::Unknown,
            battery: Battery {
                percent: Observation::Unknown(UnknownReason::NotObserved),
                charging: Observation::Unknown(UnknownReason::NotObserved),
                source: Observation::Unknown(UnknownReason::NotObserved),
            },
            speaker_devices: Observation::Unknown(UnknownReason::NotObserved),
        }
    }
}
