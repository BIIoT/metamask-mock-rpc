use std::fmt::{Debug, Display, Formatter};

pub enum SigningError {
    InvalidMessage,
}

impl Debug for SigningError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Message has to be a non-zero 32-bytes slice.")
    }
}

impl Display for SigningError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Message has to be a non-zero 32-bytes slice.")
    }
}

impl std::error::Error for SigningError {}

#[derive(Debug)]
pub enum RecoveryError {
    InvalidMessage,
    InvalidSignature,
}

impl Display for RecoveryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            RecoveryError::InvalidMessage => write!(f, "Message has to be a non-zero 32-bytes slice."),
            RecoveryError::InvalidSignature => write!(f, "Signature is invalid (check recovery id)."),
        }
    }
}

impl std::error::Error for RecoveryError {}