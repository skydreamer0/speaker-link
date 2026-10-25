//! P0 contract preparation only. No protocol codec, hardware support or writes.
#![forbid(unsafe_code)]

pub mod command;
pub mod diagnostics;
pub mod error;
pub mod model;

pub use error::{ErrorCode, Result};
pub const SCHEMA_VERSION: u32 = 1;
