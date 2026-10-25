//! Synthetic local-file tests only; no hardware or user files are accessed.
use speakerlink_core::diagnostics::{DiagnosticPreview, DiagnosticSaveError};
use speakerlink_core::ErrorCode;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Barrier;

struct SyntheticDirectory(PathBuf);

impl SyntheticDirectory {
    fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        loop {
            let path = std::env::temp_dir().join(format!(
                "speakerlink-synthetic-export-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            match fs::create_dir(&path) {
                Ok(()) => return Self(path),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => panic!("cannot create synthetic test directory: {error}"),
            }
        }
    }

    fn path(&self, name: &str) -> PathBuf {
        self.0.join(name)
    }
}

impl Drop for SyntheticDirectory {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.0).expect("remove only this test's synthetic directory");
    }
}

const ERRORS: [ErrorCode; 9] = [
    ErrorCode::InvalidArgument,
    ErrorCode::Unsupported,
    ErrorCode::AccessDenied,
    ErrorCode::Disconnected,
    ErrorCode::Timeout,
    ErrorCode::ProtocolError,
    ErrorCode::Busy,
    ErrorCode::Indeterminate,
    ErrorCode::InternalError,
];

#[test]
fn each_saved_report_is_exactly_the_previewed_allowlist() {
    let directory = SyntheticDirectory::new();
    for error in ERRORS {
        let preview = DiagnosticPreview::for_error(error);
        let path = directory.path(error.as_str());
        assert_eq!(preview.save_new(&path), Ok(()));
        let expected = format!(
            "{{\"schema_version\":1,\"app_version\":\"0.0.0-dev\",\"error_code\":\"{}\"}}\n",
            error.as_str()
        );
        assert_eq!(fs::read(path).unwrap(), expected.as_bytes());
        assert_eq!(preview.as_bytes(), expected.as_bytes());
    }
}

#[test]
fn existing_files_even_empty_ones_are_never_overwritten() {
    let directory = SyntheticDirectory::new();
    let preview = DiagnosticPreview::for_error(ErrorCode::Timeout);
    for (name, original) in [
        ("empty", &b""[..]),
        ("nonempty", &b"synthetic original"[..]),
    ] {
        let path = directory.path(name);
        fs::write(&path, original).unwrap();
        assert_eq!(
            preview.save_new(&path),
            Err(DiagnosticSaveError::AlreadyExists)
        );
        assert_eq!(fs::read(path).unwrap(), original);
    }
}

#[test]
fn a_second_save_cannot_replace_the_first_preview() {
    let directory = SyntheticDirectory::new();
    let path = directory.path("report.json");
    let first = DiagnosticPreview::for_error(ErrorCode::Timeout);
    let second = DiagnosticPreview::for_error(ErrorCode::Disconnected);
    first.save_new(&path).unwrap();
    assert_eq!(
        second.save_new(&path),
        Err(DiagnosticSaveError::AlreadyExists)
    );
    assert_eq!(fs::read(path).unwrap(), first.as_bytes());
}

#[test]
fn existing_directories_are_preserved() {
    let directory = SyntheticDirectory::new();
    let path = directory.path("existing-directory");
    fs::create_dir(&path).unwrap();
    let preview = DiagnosticPreview::for_error(ErrorCode::Timeout);
    assert!(preview.save_new(&path).is_err());
    assert!(path.is_dir());
    assert_eq!(fs::read_dir(path).unwrap().count(), 0);
}

#[test]
fn missing_parent_does_not_create_directories_or_expose_private_path() {
    let directory = SyntheticDirectory::new();
    let parent = directory.path("SYNTHETIC_PRIVATE_USER_私人");
    let preview = DiagnosticPreview::for_error(ErrorCode::Timeout);
    let result = preview.save_new(parent.join("private-report.json"));
    assert_eq!(result, Err(DiagnosticSaveError::CreateFailed));
    assert!(!format!("{result:?}").contains("SYNTHETIC_PRIVATE"));
    assert!(!parent.exists());
}

#[test]
fn competing_saves_have_one_winner_without_mixing_or_truncating_bytes() {
    let directory = SyntheticDirectory::new();
    let path = directory.path("raced-report.json");
    let previews = [
        DiagnosticPreview::for_error(ErrorCode::Timeout),
        DiagnosticPreview::for_error(ErrorCode::Disconnected),
    ];
    let barrier = Barrier::new(2);
    let results = std::thread::scope(|scope| {
        let workers: Vec<_> = previews
            .iter()
            .map(|preview| {
                let path = &path;
                let barrier = &barrier;
                scope.spawn(move || {
                    barrier.wait();
                    preview.save_new(path)
                })
            })
            .collect();
        workers
            .into_iter()
            .map(|worker| worker.join().unwrap())
            .collect::<Vec<_>>()
    });
    assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
    let winner = results.iter().position(|result| result.is_ok()).unwrap();
    assert_eq!(results[1 - winner], Err(DiagnosticSaveError::AlreadyExists));
    assert_eq!(fs::read(path).unwrap(), previews[winner].as_bytes());
}

#[test]
fn chosen_filename_and_unknown_metadata_never_enter_report_bytes() {
    let directory = SyntheticDirectory::new();
    let name = "SYNTHETIC_SERIAL_0001_BTHENUM_私人喇叭.json";
    let preview = DiagnosticPreview::for_error(ErrorCode::Unsupported);
    let path = directory.path(name);
    preview.save_new(&path).unwrap();
    let bytes = fs::read(path).unwrap();
    assert_eq!(bytes, preview.as_bytes());
    let text = std::str::from_utf8(&bytes).unwrap();
    for prohibited in [
        name,
        "device_ref",
        "firmware",
        "topology",
        "raw_error",
        "path",
    ] {
        assert!(!text.contains(prohibited));
    }
}

#[test]
fn save_failures_have_only_normalized_shared_error_codes() {
    assert_eq!(
        DiagnosticSaveError::AlreadyExists.code(),
        ErrorCode::InvalidArgument
    );
    for error in [
        DiagnosticSaveError::CreateFailed,
        DiagnosticSaveError::WriteFailed,
        DiagnosticSaveError::SyncFailed,
    ] {
        assert_eq!(error.code(), ErrorCode::InternalError);
    }
}

#[cfg(unix)]
#[test]
fn existing_and_dangling_final_symlinks_are_not_followed() {
    use std::os::unix::fs::symlink;
    let directory = SyntheticDirectory::new();
    let target = directory.path("synthetic-target");
    fs::write(&target, b"synthetic original").unwrap();
    let missing = directory.path("missing-target");
    let preview = DiagnosticPreview::for_error(ErrorCode::Timeout);
    for (name, destination) in [
        ("existing-link", target.as_path()),
        ("dangling-link", missing.as_path()),
    ] {
        let link = directory.path(name);
        symlink(destination, &link).unwrap();
        assert_eq!(
            preview.save_new(&link),
            Err(DiagnosticSaveError::AlreadyExists)
        );
        assert_eq!(fs::read_link(link).unwrap(), destination);
    }
    assert_eq!(fs::read(target).unwrap(), b"synthetic original");
    assert!(!missing.exists());
}
