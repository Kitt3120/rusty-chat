use super::super::client;
use super::super::error::MessageParseError;
use std::{
    fmt::{Debug, Display},
    io::Error,
};

#[derive(Debug)]
pub enum HandshakeError {
    IoError(Error),
    MessageParseError(MessageParseError),
    UnexpectedMessage(client::Message),
    AuthenticationFailed(String),
}

impl Display for HandshakeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandshakeError::IoError(err) => write!(f, "IoError while reading message: {}", err),
            HandshakeError::MessageParseError(err) => {
                write!(f, "Unable to parse Message: {}", err)
            }
            HandshakeError::UnexpectedMessage(msg) => {
                write!(f, "Unexpected Message: {}", msg)
            }
            HandshakeError::AuthenticationFailed(msg) => {
                write!(f, "Authentication failed: {}", msg)
            }
        }
    }
}
