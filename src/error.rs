use std::{fmt, io};

#[derive(Debug)]
pub enum JotterError {
    File(FileError),
    IO(std::io::Error),
    RM(RawModeError),
}

#[derive(Debug)]
pub enum RawModeError {
    AlreadyCaptured,
    EnableDisableFailed(std::io::Error),
    NotEnabled,
}

#[derive(Debug)]
pub enum FileError {
    CouldNotOpen(std::io::Error),
}

impl std::error::Error for JotterError {}

impl From<io::Error> for JotterError {
    fn from(error: io::Error) -> Self {
        Self::IO(error)
    }
}

impl From<RawModeError> for JotterError {
    fn from(error: RawModeError) -> Self {
        Self::RM(error)
    }
}

impl From<FileError> for JotterError {
    fn from(error: FileError) -> Self {
        Self::File(error)
    }
}

impl fmt::Display for JotterError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IO(e) => write!(f, "I/O error: {e}"),
            Self::RM(e) => write!(f, "Raw Mode error: {e}"),
            Self::File(e) => write!(f, "File error: {e}"),
        }
    }
}

impl std::error::Error for FileError {}

impl From<io::Error> for FileError {
    fn from(error: io::Error) -> Self {
        Self::CouldNotOpen(error)
    }
}

impl fmt::Display for FileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CouldNotOpen(e) => write!(f, "could not open file: {e}"),
        }
    }
}

impl std::error::Error for RawModeError {}

impl From<io::Error> for RawModeError {
    fn from(error: io::Error) -> Self {
        Self::EnableDisableFailed(error)
    }
}

impl fmt::Display for RawModeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EnableDisableFailed(e) => write!(f, "enable/disable failed: {e}"),
            Self::NotEnabled => write!(f, "was not enabled"),
            Self::AlreadyCaptured => write!(f, "already captured"),
        }
    }
}
