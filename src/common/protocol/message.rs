use super::{client, error::MessageParseError, server, Serializable};
use std::fmt::{Debug, Display};

#[derive(Clone, Debug, PartialEq)]
pub enum Message {
    Client(client::Message),
    Server(server::Message),
}

impl Message {
    fn id(&self) -> u8 {
        match self {
            Message::Client(_) => 0,
            Message::Server(_) => 1,
        }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::Client(message) => write!(f, "Client({})", message),
            Message::Server(message) => write!(f, "Server({})", message),
        }
    }
}

impl Serializable for Message {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        match self {
            Message::Client(message) => {
                bytes.push(self.id());
                bytes.extend(message.as_bytes());
            }
            Message::Server(message) => {
                bytes.push(self.id());
                bytes.extend(message.as_bytes());
            }
        }

        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Result<Message, MessageParseError> {
        if bytes.is_empty() {
            return Err(MessageParseError::MessageEmpty);
        }

        let message_kind = bytes[0];
        match message_kind {
            0 => Ok(Message::Client(client::Message::from_bytes(&bytes[1..])?)),
            1 => Ok(Message::Server(server::Message::from_bytes(&bytes[1..])?)),
            _ => Err(MessageParseError::UnknownKind(message_kind)),
        }
    }
}

//TODO: Tests
