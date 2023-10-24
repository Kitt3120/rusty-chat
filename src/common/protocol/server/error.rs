use super::super::client;
use super::super::error::MessageParseError;
use std::{
    fmt::{Debug, Display},
    io::Error,
};

pub enum HandshakeError {
    IoError(Error),
    MessageParseError(MessageParseError),
    UnexpectedMessage(client::Message),
    AuthenticationFailed(String),
}

impl Clone for HandshakeError {
    fn clone(&self) -> Self {
        match self {
            HandshakeError::IoError(err) => HandshakeError::IoError(*err.clone()),
            HandshakeError::MessageParseError(err) => {
                HandshakeError::MessageParseError(err.clone())
            }
            HandshakeError::UnexpectedMessage(msg) => {
                HandshakeError::UnexpectedMessage(msg.clone())
            }
            HandshakeError::AuthenticationFailed(msg) => {
                HandshakeError::AuthenticationFailed(msg.clone())
            }
        }
    }
}

impl Debug for HandshakeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandshakeError::IoError(err) => write!(f, "IoError: {}", err),
            HandshakeError::MessageParseError(err) => {
                write!(f, "MessageParseError: {}", err)
            }
            HandshakeError::UnexpectedMessage(msg) => {
                write!(f, "UnexpectedMessage: {:?}", msg)
            }
            HandshakeError::AuthenticationFailed(msg) => {
                write!(f, "AuthenticationFailed: {}", msg)
            }
        }
    }
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
