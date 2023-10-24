use super::super::error::MessageParseError;
use super::super::server;
use std::{
    fmt::{Debug, Display},
    io::Error,
};

pub enum HandshakeError {
    IoError(Error),
    MessageParseError(MessageParseError),
    UnexpectedMessage(server::Message),
    AuthenticationFailed(String),
}

impl Clone for HandshakeError {
    fn clone(&self) -> Self {
        match self {
            HandshakeError::IoError(err) => HandshakeError::IoError(*err.clone()),
            HandshakeError::MessageParseError(err) => {
                HandshakeError::MessageParseError(err.clone())
            }
            HandshakeError::UnexpectedMessage(message) => {
                HandshakeError::UnexpectedMessage(message.clone())
            }
            HandshakeError::AuthenticationFailed(reason) => {
                HandshakeError::AuthenticationFailed(reason.clone())
            }
        }
    }
}

impl Debug for HandshakeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandshakeError::IoError(err) => write!(f, "IoError({:?})", err),
            HandshakeError::MessageParseError(err) => {
                write!(f, "MessageParseError({:?})", err)
            }
            HandshakeError::UnexpectedMessage(message) => {
                write!(f, "UnexpectedMessage({:?})", message)
            }
            HandshakeError::AuthenticationFailed(reason) => {
                write!(f, "AuthenticationFailed({:?})", reason)
            }
        }
    }
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
