use std::{fmt, io};

#[derive(Debug)]
pub enum RawModeError {
    AlreadyCaptured,
    EnableDisableFailed(std::io::Error),
    NotEnabled,
}

impl From<io::Error> for RawModeError {
    fn from(error: io::Error) -> Self {
        Self::EnableDisableFailed(error)
    }
}

impl fmt::Display for RawModeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            // Self::Io(e) => write!(f, "I/O Error: {e}."),
            Self::EnableDisableFailed(e) => write!(f, "Raw mode enable/disable failed: {e}"),
            Self::NotEnabled => write!(f, "Raw mode was not enabled"),
            Self::AlreadyCaptured => write!(f, "Raw mode already captured"),
        }
    }
}

impl std::error::Error for RawModeError {}
