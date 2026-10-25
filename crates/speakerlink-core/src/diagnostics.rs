//! Partial, device-free diagnostic preview. No file I/O or upload exists.
//! Full F4 export, random per-export aliases, metadata review, and overwrite
//! confirmation remain gated. This preview deliberately omits all identifiers,
//! topology, timestamps, versions from devices, and raw error text.
use crate::{ErrorCode, SCHEMA_VERSION};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiagnosticPreview {
    bytes: Vec<u8>,
}

impl DiagnosticPreview {
    /// Only a normalized enum crosses this boundary: no arbitrary strings.
    /// Version and error fields are the entire current allowlist.
    pub fn for_error(error: ErrorCode) -> Self {
        let json = format!(
            "{{\"schema_version\":{},\"app_version\":\"{}\",\"error_code\":\"{}\"}}\n",
            SCHEMA_VERSION,
            env!("CARGO_PKG_VERSION"),
            error.as_str(),
        );
        Self {
            bytes: json.into_bytes(),
        }
    }

    /// The preview bytes are immutable; future file export must write these
    /// exact bytes instead of independently regenerating a second report.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }
}
