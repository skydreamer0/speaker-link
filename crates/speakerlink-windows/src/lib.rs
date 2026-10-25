//! P0 no-write discovery contracts. No WinRT dependency or hardware I/O yet.
//! The real adapter awaits Windows toolchain, access and service experiments.
#![forbid(unsafe_code)]

use speakerlink_core::model::{DeviceRef, Snapshot};
use speakerlink_core::{ErrorCode, Result};
use std::future::{ready, Future};
use std::num::NonZeroUsize;
use std::time::Duration;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CacheMode {
    Cached,
    Uncached,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DiscoveryRequest {
    pub cache_mode: CacheMode,
    /// Caller-selected finite budget; adapter-tested defaults are not set yet.
    timeout: Duration,
    max_services: NonZeroUsize,
}

impl DiscoveryRequest {
    pub fn new(timeout: Duration, max_services: usize) -> Result<Self> {
        if timeout.is_zero() {
            return Err(ErrorCode::InvalidArgument);
        }
        let max_services = NonZeroUsize::new(max_services).ok_or(ErrorCode::InvalidArgument)?;
        Ok(Self {
            cache_mode: CacheMode::Uncached,
            timeout,
            max_services,
        })
    }

    pub const fn timeout(&self) -> Duration {
        self.timeout
    }

    pub const fn max_services(&self) -> usize {
        self.max_services.get()
    }

    pub fn validate_result(&self, services: &[DiscoveredService]) -> Result<()> {
        if services.len() > self.max_services.get() {
            return Err(ErrorCode::ProtocolError);
        }
        Ok(())
    }
}

/// A local endpoint as returned by a discovery provider, never a hardcoded
/// RFCOMM channel. DeviceRef redacts both endpoint fields in Debug output.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiscoveredService {
    pub device_ref: DeviceRef,
    pub service_uuid: [u8; 16],
    pub connection_host: DeviceRef,
    pub connection_service_name: DeviceRef,
    /// Actual source of the result, not merely the requested cache mode.
    pub cache_mode: CacheMode,
}

impl DiscoveredService {
    /// A service, including standard AVRCP, grants no speaker capability.
    pub fn unobserved_snapshot(&self) -> Snapshot {
        Snapshot::unobserved(self.device_ref.clone())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AccessStatus {
    Allowed,
    DeniedByUser,
    DeniedBySystem,
    Unspecified,
}

impl AccessStatus {
    /// No permission request or consent UI is invoked by this conversion.
    pub fn require_allowed(self) -> Result<()> {
        match self {
            Self::Allowed => Ok(()),
            Self::DeniedByUser | Self::DeniedBySystem | Self::Unspecified => {
                Err(ErrorCode::AccessDenied)
            }
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DiscoveryOutcome {
    Services(Vec<DiscoveredService>),
    /// Cancellation is a control outcome, not a fabricated SPEC error code.
    Cancelled,
}

/// Future implementations must enforce the finite timeout and result budget,
/// support cancellation by dropping the future and release all resources.
/// Only service discovery is
/// exposed: no socket, write, pairing, connection-change or raw-frame method.
/// This contract has no hardware implementation in P0 preparation.
pub trait NoWriteDiscovery {
    fn discover(
        &mut self,
        request: DiscoveryRequest,
    ) -> impl Future<Output = Result<DiscoveryOutcome>>;
}

/// Honest sentinel on every OS, including Windows. It cannot scan, acquire
/// device access, open sockets or report simulated devices as real devices.
pub struct UnimplementedWindowsDiscovery;

impl NoWriteDiscovery for UnimplementedWindowsDiscovery {
    fn discover(
        &mut self,
        _request: DiscoveryRequest,
    ) -> impl Future<Output = Result<DiscoveryOutcome>> {
        ready(Err(ErrorCode::Unsupported))
    }
}
