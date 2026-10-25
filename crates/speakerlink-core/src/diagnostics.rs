//! Partial, device-free diagnostic preview and create-only local save.
//! Full F4 export, random per-export aliases, metadata review, and overwrite
//! confirmation remain gated. This preview deliberately omits all identifiers,
//! topology, timestamps, versions from devices, and raw error text. No upload
//! or replacement operation exists.
use crate::{ErrorCode, SCHEMA_VERSION};
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

/// Sanitized save failures. Neither the chosen path nor an OS error is retained.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DiagnosticSaveError {
    /// An existing destination must be preserved, not replaced or retried.
    AlreadyExists,
    /// The destination could not be opened as a new file.
    CreateFailed,
    /// A new file was created, but may contain only part of the preview.
    WriteFailed,
    /// All bytes were written, but synchronizing the file failed.
    SyncFailed,
}

impl DiagnosticSaveError {
    /// Shared error contract; the variant supplies safe local-save guidance.
    pub const fn code(self) -> ErrorCode {
        match self {
            Self::AlreadyExists => ErrorCode::InvalidArgument,
            Self::CreateFailed | Self::WriteFailed | Self::SyncFailed => ErrorCode::InternalError,
        }
    }
}

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

    /// The preview bytes are immutable; `save_new` writes this exact view,
    /// without independently regenerating a second report.
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Save this preview to an explicitly chosen new file in a trusted directory.
    /// The caller must show these same bytes before calling this method.
    ///
    /// Atomic create-only opening refuses an existing file or final symlink,
    /// including when another saver wins a race. It does not harden parent
    /// directories or make the entire write atomic. A write/sync failure can
    /// leave a partial or complete file: inspect it before retrying. This method
    /// never removes, replaces, or automatically retries a destination.
    /// Success means the bytes were written and file sync succeeded, not a
    /// guarantee against power loss or later changes by another process.
    pub fn save_new(&self, path: impl AsRef<Path>) -> Result<(), DiagnosticSaveError> {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(path)
            .map_err(|error| match error.kind() {
                io::ErrorKind::AlreadyExists => DiagnosticSaveError::AlreadyExists,
                _ => DiagnosticSaveError::CreateFailed,
            })?;
        write_and_sync(&mut file, self.as_bytes(), File::sync_all)
    }
}

fn write_and_sync<W: Write>(
    writer: &mut W,
    bytes: &[u8],
    sync: impl FnOnce(&W) -> io::Result<()>,
) -> Result<(), DiagnosticSaveError> {
    writer
        .write_all(bytes)
        .map_err(|_| DiagnosticSaveError::WriteFailed)?;
    sync(writer).map_err(|_| DiagnosticSaveError::SyncFailed)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct SyntheticWriter {
        bytes: Vec<u8>,
        limit: usize,
        interrupted_once: bool,
        failure: Option<io::ErrorKind>,
    }

    impl Write for SyntheticWriter {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            if self.interrupted_once {
                self.interrupted_once = false;
                return Err(io::ErrorKind::Interrupted.into());
            }
            if self.bytes.len() >= self.limit {
                return match self.failure {
                    Some(kind) => Err(io::Error::new(kind, "synthetic private raw error")),
                    None => Ok(0),
                };
            }
            let count = bytes.len().min(3).min(self.limit - self.bytes.len());
            self.bytes.extend_from_slice(&bytes[..count]);
            Ok(count)
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    fn writer(limit: usize, failure: Option<io::ErrorKind>) -> SyntheticWriter {
        SyntheticWriter {
            bytes: Vec::new(),
            limit,
            interrupted_once: false,
            failure,
        }
    }

    #[test]
    fn short_and_interrupted_writes_continue_on_the_same_open_writer() {
        let preview = DiagnosticPreview::for_error(ErrorCode::Timeout);
        let mut writer = writer(usize::MAX, None);
        writer.interrupted_once = true;
        let result = write_and_sync(&mut writer, preview.as_bytes(), |writer| {
            assert_eq!(writer.bytes, preview.as_bytes());
            Ok(())
        });
        assert_eq!(result, Ok(()));
        assert_eq!(writer.bytes, preview.as_bytes());
    }

    #[test]
    fn partial_or_zero_progress_write_fails_without_sync_or_replay() {
        let preview = DiagnosticPreview::for_error(ErrorCode::Timeout);
        for failure in [None, Some(io::ErrorKind::Other)] {
            let mut writer = writer(7, failure);
            let result = write_and_sync(&mut writer, preview.as_bytes(), |_| {
                panic!("sync must not run after a failed write")
            });
            assert_eq!(result, Err(DiagnosticSaveError::WriteFailed));
            assert_eq!(writer.bytes, &preview.as_bytes()[..7]);
            assert!(!format!("{result:?}").contains("private raw error"));
        }
    }

    #[test]
    fn sync_failure_does_not_report_success_or_rewrite() {
        let preview = DiagnosticPreview::for_error(ErrorCode::Timeout);
        let mut writer = writer(usize::MAX, None);
        let result = write_and_sync(&mut writer, preview.as_bytes(), |_| {
            Err(io::Error::other("synthetic private raw sync error"))
        });
        assert_eq!(result, Err(DiagnosticSaveError::SyncFailed));
        assert_eq!(writer.bytes, preview.as_bytes());
        assert!(!format!("{result:?}").contains("private raw sync error"));
    }
}
