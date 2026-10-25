//! Device-free presentation of shared core snapshots. No Windows adapter is linked.
#![forbid(unsafe_code)]

pub mod presentation;
#[cfg(feature = "native-ui")]
pub mod ui;
