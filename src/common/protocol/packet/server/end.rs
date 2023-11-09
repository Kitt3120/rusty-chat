use crate::common::protocol::{
    error::MessageParseError, packet::Packet, server, Message, Serializable,
};
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub struct End {
    pub reason: String,
}

impl End {
    pub fn new(reason: String) -> End {
        End { reason }
    }
}

impl Display for End {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl Serializable for End {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(self.reason.as_bytes());

        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Result<End, MessageParseError> {
        if bytes.is_empty() {
            return Err(MessageParseError::UnexcpetedEndOfMessage);
        }

        let reason = String::from_utf8_lossy(bytes).to_string();

        Ok(End::new(reason))
    }
}

impl Packet for End {
    fn to_message(self) -> Message {
        Message::Server(server::Message::End(self))
    }
}
