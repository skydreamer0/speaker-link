//! Stable normalized codes from SPEC 0.2.0, not raw OS/transport exceptions.

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorCode {
    InvalidArgument,
    Unsupported,
    AccessDenied,
    Disconnected,
    Timeout,
    ProtocolError,
    Busy,
    Indeterminate,
    InternalError,
}

impl ErrorCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidArgument => "InvalidArgument",
            Self::Unsupported => "Unsupported",
            Self::AccessDenied => "AccessDenied",
            Self::Disconnected => "Disconnected",
            Self::Timeout => "Timeout",
            Self::ProtocolError => "ProtocolError",
            Self::Busy => "Busy",
            Self::Indeterminate => "Indeterminate",
            Self::InternalError => "InternalError",
        }
    }

    pub const fn cli_exit_code(self) -> u8 {
        match self {
            Self::InvalidArgument => 2,
            Self::Unsupported => 3,
            Self::AccessDenied => 4,
            Self::Disconnected => 5,
            Self::Timeout => 6,
            Self::ProtocolError => 7,
            Self::Busy => 8,
            Self::Indeterminate => 9,
            Self::InternalError => 10,
        }
    }
}

pub type Result<T> = std::result::Result<T, ErrorCode>;
