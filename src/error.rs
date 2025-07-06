#[derive(Debug)]
pub(crate) enum CmdError {
    InvalidColour(char),
    QuoteIndex(u8),
}

use std::fmt;

impl fmt::Display for CmdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CmdError::InvalidColour(c) => write!(f, "Invalid colour char '{c}'."),
            CmdError::QuoteIndex(i) => write!(f, "Quote index {i} is out of bounds. [max 32]"),
        }
    }
}

impl std::error::Error for CmdError {}
