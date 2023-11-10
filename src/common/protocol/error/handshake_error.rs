use crate::common::protocol::{error::MessageParseError, message::Message};
use std::{fmt::Display, io::Error};

#[derive(Debug)]
pub enum HandshakeError {
    IoError(Error),
    MessageParseError(MessageParseError),
    UnexpectedMessage(Message),
    AuthenticationFailed(String),
}

impl Display for HandshakeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandshakeError::IoError(err) => write!(f, "IoError while reading message: {}", err),
            HandshakeError::MessageParseError(err) => {
                write!(f, "Unable to parse message: {}", err)
            }
            HandshakeError::UnexpectedMessage(message) => {
                write!(f, "Unexpected message: {}", message)
            }
            HandshakeError::AuthenticationFailed(reason) => {
                write!(f, "Authentication failed: {}", reason)
            }
        }
    }
}
