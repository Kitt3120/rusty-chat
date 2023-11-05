use std::{
    fmt::{Debug, Display},
    io::Error,
    string::FromUtf8Error,
};

use super::Message;

#[derive(Clone, Debug, PartialEq)]
pub enum MessageParseError {
    MessageEmpty,
    UnexcpetedEndOfMessage,
    UnknownKind(u8),
    StringParse(String, FromUtf8Error),
    ByteParse(String),
}

impl Display for MessageParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageParseError::MessageEmpty => write!(f, "Message was empty"),
            MessageParseError::UnexcpetedEndOfMessage => {
                write!(f, "Unexpected end of message")
            }
            MessageParseError::UnknownKind(kind) => {
                write!(f, "Message had unknown kind: {}", kind)
            }
            MessageParseError::StringParse(value, err) => {
                write!(
                    f,
                    "Unable to parse string expected for value {}: {}",
                    value, err
                )
            }
            MessageParseError::ByteParse(value) => {
                write!(f, "Unable to parse bytes expected for value {}", value)
            }
        }
    }
}

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
