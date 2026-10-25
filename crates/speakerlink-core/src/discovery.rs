//! Platform-neutral service-discovery contracts. A service is not model support.
use crate::model::{DeviceRef, Snapshot};
use crate::{ErrorCode, Result};
use std::future::Future;
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
    /// Explicit caller budget, capped at 60 seconds for resource safety.
    /// This is not a measured Bluetooth latency or a release default.
    timeout: Duration,
    max_services: NonZeroUsize,
}

impl DiscoveryRequest {
    pub fn new(timeout: Duration, max_services: usize) -> Result<Self> {
        if timeout.is_zero() || timeout > Duration::from_secs(60) || max_services > 256 {
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
    /// API query policy, not independent evidence of speaker-state freshness.
    pub cache_mode: CacheMode,
}

impl DiscoveredService {
    /// A service, including standard AVRCP, grants no speaker capability.
    pub fn unobserved_snapshot(&self) -> Snapshot {
        Snapshot::unobserved(self.device_ref.clone())
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
/// Only service discovery is exposed: no socket, write, pairing, connection-change or raw-frame method.
/// No protocol commands or capability enabling belong to this interface.
pub trait NoWriteDiscovery {
    fn discover(
        &mut self,
        request: DiscoveryRequest,
    ) -> impl Future<Output = Result<DiscoveryOutcome>>;
}
