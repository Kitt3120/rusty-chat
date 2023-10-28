use super::{client, error::MessageParseError, server, Serializable};
use std::fmt::{Debug, Display};

pub enum Message {
    Client(client::Message),
    Server(server::Message),
}

impl Clone for Message {
    fn clone(&self) -> Self {
        match self {
            Message::Client(message) => Message::Client(message.clone()),
            Message::Server(message) => Message::Server(message.clone()),
        }
    }
}

impl Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::Client(message) => write!(f, "Client({:?})", message),
            Message::Server(message) => write!(f, "Server({:?})", message),
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
    fn id(&self) -> u8 {
        match self {
            Message::Client(message) => 0,
            Message::Server(message) => 1,
        }
    }

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
