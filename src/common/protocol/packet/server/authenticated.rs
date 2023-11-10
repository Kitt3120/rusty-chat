use crate::common::protocol::{
    error::MessageParseError,
    message::{server, Message},
    packet::Packet,
    Serializable,
};
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub struct Authenticated {}

impl Authenticated {
    pub fn new() -> Authenticated {
        Authenticated {}
    }
}

impl Default for Authenticated {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Authenticated {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Ok(())
    }
}

impl Serializable for Authenticated {
    fn as_bytes(&self) -> Vec<u8> {
        Vec::new()
    }

    fn from_bytes(_bytes: &[u8]) -> Result<Authenticated, MessageParseError> {
        Ok(Authenticated::new())
    }
}

impl Packet for Authenticated {
    fn to_message(self) -> Message {
        Message::Server(server::Message::Authenticated(self))
    }
}
