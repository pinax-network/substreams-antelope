use std::error::Error as StdError;
use std::fmt::{self, Display};

/// Custom error type for the Statelog action.
#[derive(Debug)]
pub enum Error {
    JsonDecodeError,
    AbiLoadError,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::JsonDecodeError => write!(f, "failed to decode data json"),
            Error::AbiLoadError => write!(f, "failed to load abi"),
        }
    }
}

impl StdError for Error {}
