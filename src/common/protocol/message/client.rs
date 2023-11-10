use std::fmt::{Debug, Display};

use crate::common::protocol::{
    error::MessageParseError,
    packet::client::{Authenticate, Chat, End},
    Serializable,
};

#[derive(Clone, Debug, PartialEq)]
pub enum Message {
    Authenticate(Authenticate),
    Chat(Chat),
    End(End),
}

impl Message {
    fn id(&self) -> u8 {
        match self {
            Message::Authenticate(_) => 0,
            Message::Chat(_) => 1,
            Message::End(_) => 2,
        }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::Authenticate(username) => write!(f, "Authenticate ({})", username),
            Message::Chat(message) => write!(f, "Chat({})", message),
            Message::End(reason) => write!(f, "End({})", reason),
        }
    }
}

impl Serializable for Message {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![self.id()];
        bytes.extend(match self {
            Message::Authenticate(username) => username.as_bytes(),
            Message::Chat(message) => message.as_bytes(),
            Message::End(reason) => reason.as_bytes(),
        });
        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Result<Message, MessageParseError> {
        if bytes.is_empty() {
            return Err(MessageParseError::MessageEmpty);
        }

        let message_kind = bytes[0];
        match message_kind {
            0 => {
                let authenticate = Authenticate::from_bytes(&bytes[1..])?;
                Ok(Message::Authenticate(authenticate))
            }
            1 => {
                let chat = Chat::from_bytes(&bytes[1..])?;
                Ok(Message::Chat(chat))
            }
            2 => {
                let end = End::from_bytes(&bytes[1..])?;
                Ok(Message::End(end))
            }
            kind => Err(MessageParseError::UnknownKind(kind)),
        }
    }
}

impl From<Message> for Vec<u8> {
    fn from(value: Message) -> Self {
        value.as_bytes()
    }
}

impl From<&Message> for Vec<u8> {
    fn from(value: &Message) -> Self {
        value.as_bytes()
    }
}

impl From<&mut Message> for Vec<u8> {
    fn from(value: &mut Message) -> Self {
        value.as_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_authenticate_converts_correctly() {
        let username = String::from("Kitt3120");

        let authenticate = Authenticate::new(username);
        let authenticate_comparison_clone = authenticate.clone();

        let message = Message::Authenticate(authenticate);
        let bytes = message.as_bytes();

        let parsed_message = match Message::from_bytes(&bytes) {
            Ok(message) => message,
            Err(err) => panic!("Failed to parse message: {}", err),
        };

        assert_eq!(message.id(), parsed_message.id());
        if let Message::Authenticate(authenticate) = parsed_message {
            assert_eq!(authenticate, authenticate_comparison_clone);
        } else {
            panic!("Parsed message is not of type Message::Authenticate");
        }
    }

    #[test]
    fn message_chat_converts_correctly() {
        let message_content = String::from("⚡");

        let chat = Chat::new(message_content);
        let chat_comparison_clone = chat.clone();

        let message = Message::Chat(chat);
        let bytes = message.as_bytes();

        let parsed_message = match Message::from_bytes(&bytes) {
            Ok(message) => message,
            Err(err) => panic!("Failed to parse message: {}", err),
        };

        assert_eq!(message.id(), parsed_message.id());
        if let Message::Chat(chat) = parsed_message {
            assert_eq!(chat, chat_comparison_clone);
        } else {
            panic!("Parsed message is not of type Message::Chat");
        }
    }

    #[test]
    fn message_end_converts_correctly() {
        let reason = String::from("❌");

        let end = End::new(reason);
        let end_comparison_clone = end.clone();

        let message = Message::End(end);
        let bytes = message.as_bytes();

        let parsed_message = match Message::from_bytes(&bytes) {
            Ok(message) => message,
            Err(err) => panic!("Failed to parse message: {}", err),
        };

        assert_eq!(message.id(), parsed_message.id());
        if let Message::End(end) = parsed_message {
            assert_eq!(end, end_comparison_clone);
        } else {
            panic!("Parsed message is not of type Message::End");
        }
    }
}
