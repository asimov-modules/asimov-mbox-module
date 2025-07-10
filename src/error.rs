// This is free and unencumbered software released into the public domain.

use core::fmt;

#[derive(Clone, Debug)]
pub enum MboxError {
    InvalidEntry,
    InvalidMessage,
    InvalidHeaders,
}

impl core::error::Error for MboxError {}

impl fmt::Display for MboxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use MboxError::*;
        match self {
            InvalidEntry => write!(f, "invalid mbox entry"),
            InvalidMessage => write!(f, "invalid mbox message"),
            InvalidHeaders => write!(f, "invalid mbox headers"),
        }
    }
}
