//! Command input shapes only. This module cannot send frames or execute F3.
use crate::model::{DeviceRef, SpeakerTargetRef};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DesiredConnectionState {
    Connected,
    Disconnected,
}

/// Both the speaker and its known source target must be explicitly supplied.
/// A future engine must validate target membership, exact reviewed capability
/// evidence, fresh state, exclusive ownership, and readback before success.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SetConnectionRequest {
    pub device_ref: DeviceRef,
    pub target_ref: SpeakerTargetRef,
    pub desired_state: DesiredConnectionState,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Command {
    Discover,
    Snapshot { device_ref: DeviceRef },
    SetConnection(SetConnectionRequest),
    DiagnosticReport { device_ref: DeviceRef },
}
