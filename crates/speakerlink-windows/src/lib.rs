//! Windows service discovery; all model capabilities remain independently gated.
//! Constructing the WinRT adapter reads device access status. Polling discovery
//! rechecks access and starts service inspection. Neither requests permission.
#![forbid(unsafe_code)]

// Reexports keep the foundation API source-compatible while the contract lives
// in the platform-neutral core. No other OS implementation is implied.
pub use speakerlink_core::discovery::{
    CacheMode, DiscoveredService, DiscoveryOutcome, DiscoveryRequest, NoWriteDiscovery,
};
use speakerlink_core::{ErrorCode, Result};
use std::future::{ready, Future};

#[cfg(any(windows, test))]
mod runtime;
#[cfg(windows)]
pub mod winrt;

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
