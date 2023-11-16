use std::fmt::Display;

use crate::common::protocol::{
    error::MessageParseError,
    message::{client, Message},
    packet::Packet,
    serializable::Serializable,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Authenticate {
    pub username: String,
}

impl Authenticate {
    pub fn new(username: String) -> Authenticate {
        Authenticate { username }
    }
}

impl Display for Authenticate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.username)
    }
}

impl Serializable for Authenticate {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(self.username.as_bytes());

        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Result<Authenticate, MessageParseError> {
        if bytes.is_empty() {
            return Err(MessageParseError::UnexcpetedEndOfMessage);
        }

        let username = match String::from_utf8(bytes.to_vec()) {
            Ok(username) => username,
            Err(err) => {
                return Err(MessageParseError::StringParse(
                    String::from("Username"),
                    err,
                ))
            }
        };

        Ok(Authenticate::new(username))
    }
}

impl Packet for Authenticate {
    fn to_message(self) -> Message {
        Message::Client(client::Message::Authenticate(self))
    }
}

//TODO: Tests
