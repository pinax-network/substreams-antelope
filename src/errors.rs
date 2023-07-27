

use std::error::Error as StdError;
use std::fmt::{self, Display};

const ACTION_DECODE_ERROR: &str = "Failed to decode Action JSON";

/// Custom error type for the Statelog action.
#[derive(Debug)]
pub enum Error {
    ActionDecodeError,
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::ActionDecodeError => write!(f, "{}", ACTION_DECODE_ERROR),
        }
    }
}

impl StdError for Error {}