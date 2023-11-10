use crate::common::{message_stream::error::MessageStreamError, protocol::message::Message};
use std::fmt::Display;

#[derive(Debug)]
pub enum HandshakeError {
    MessageStreamError(MessageStreamError),
    UnexpectedMessage(Message),
    AuthenticationFailed(String),
}

impl Display for HandshakeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HandshakeError::MessageStreamError(err) => {
                write!(f, "Error while streaming message: {}", err)
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
