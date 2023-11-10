use crate::common::protocol::{
    error::MessageParseError, message::client, packet::Packet, Message, Serializable,
};
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub struct Chat {
    pub message: String,
}

impl Chat {
    pub fn new(message: String) -> Chat {
        Chat { message }
    }
}

impl Display for Chat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Serializable for Chat {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(self.message.as_bytes());

        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Result<Chat, MessageParseError> {
        if bytes.is_empty() {
            return Err(MessageParseError::UnexcpetedEndOfMessage);
        }

        let message = match String::from_utf8(bytes.to_vec()) {
            Ok(message) => message,
            Err(err) => return Err(MessageParseError::StringParse(String::from("Message"), err)),
        };

        Ok(Chat::new(message))
    }
}

impl Packet for Chat {
    fn to_message(self) -> Message {
        Message::Client(client::Message::Chat(self))
    }
}
