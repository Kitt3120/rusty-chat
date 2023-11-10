use std::{fmt::Display, io::Error};

use crate::common::protocol::error::MessageParseError;

#[derive(Debug)]
pub enum MessageStreamError {
    IoError(Error),
    MessageParseError(MessageParseError),
}

impl Display for MessageStreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MessageStreamError::IoError(e) => write!(f, "IoError while streaming message: {}", e),
            MessageStreamError::MessageParseError(e) => {
                write!(f, "Error while parsing message: {}", e)
            }
        }
    }
}
