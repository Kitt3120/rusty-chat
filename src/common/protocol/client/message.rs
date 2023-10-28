use super::super::error::MessageParseError;
use std::fmt::{Debug, Display};

pub enum Message {
    Authenticate(String),
    Chat(String),
    End(String),
}

impl Clone for Message {
    fn clone(&self) -> Self {
        match self {
            Message::Authenticate(username) => Message::Authenticate(username.clone()),
            Message::Chat(message) => Message::Chat(message.clone()),
            Message::End(reason) => Message::End(reason.clone()),
        }
    }
}

impl Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::Authenticate(username) => write!(f, "Authenticate ({:?})", username),
            Message::Chat(message) => write!(f, "Chat({:?})", message),
            Message::End(reason) => write!(f, "End({:?})", reason),
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

impl Message {
    pub fn id(&self) -> u8 {
        match self {
            Message::Authenticate(_) => 0,
            Message::Chat(_) => 1,
            Message::End(_) => 2,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            Message::Authenticate(username) => {
                let username_bytes = username.as_bytes();

                let mut bytes = vec![self.id()];
                bytes.extend_from_slice(username_bytes);
                bytes
            }
            Message::Chat(message) => {
                let message_bytes = message.as_bytes();

                let mut bytes = vec![self.id()];
                bytes.extend_from_slice(message_bytes);
                bytes
            }
            Message::End(reason) => {
                let mut bytes = vec![self.id()];
                bytes.extend_from_slice(reason.as_bytes());
                bytes
            }
        }
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Message, MessageParseError> {
        if bytes.is_empty() {
            return Err(MessageParseError::MessageEmpty);
        }

        match bytes[0] {
            0 => {
                if bytes.len() < 2 {
                    return Err(MessageParseError::UnexcpetedEndOfMessage);
                }

                let username = match String::from_utf8(bytes[1..].to_vec()) {
                    Ok(username) => username,
                    Err(err) => {
                        return Err(MessageParseError::StringParse(
                            String::from("Username"),
                            err,
                        ))
                    }
                };

                Ok(Message::Authenticate(username))
            }
            1 => {
                if bytes.len() < 2 {
                    return Err(MessageParseError::UnexcpetedEndOfMessage);
                }

                let message = match String::from_utf8(bytes[1..].to_vec()) {
                    Ok(message) => message,
                    Err(err) => {
                        return Err(MessageParseError::StringParse(String::from("Message"), err))
                    }
                };

                Ok(Message::Chat(message))
            }
            2 => {
                if bytes.len() < 2 {
                    return Err(MessageParseError::UnexcpetedEndOfMessage);
                }

                let reason = match String::from_utf8(bytes[1..].to_vec()) {
                    Ok(reason) => reason,
                    Err(err) => {
                        return Err(MessageParseError::StringParse(String::from("Reason"), err))
                    }
                };

                Ok(Message::End(reason))
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
        let username_comparison_clone = username.clone();

        let message = Message::Authenticate(username);
        let bytes = message.as_bytes();
        let parsed_message = match Message::from_bytes(&bytes) {
            Ok(message) => message,
            Err(err) => panic!("Failed to parse message: {}", err),
        };

        assert_eq!(message.id(), parsed_message.id());
        if let Message::Authenticate(username) = parsed_message {
            assert_eq!(username, username_comparison_clone);
        } else {
            panic!("Parsed message is not of type MessageKind::Authenticate");
        }
    }

    #[test]
    fn message_chat_converts_correctly() {
        let message_content = String::from("⚡");
        let message_content_comparison_clone = message_content.clone();

        let message = Message::Chat(message_content);
        let bytes = message.as_bytes();
        let parsed_message = match Message::from_bytes(&bytes) {
            Ok(message) => message,
            Err(err) => panic!("Failed to parse message: {}", err),
        };

        assert_eq!(message.id(), parsed_message.id());
        if let Message::Chat(message) = parsed_message {
            assert_eq!(message, message_content_comparison_clone);
        } else {
            panic!("Parsed message is not of type MessageKind::Chat");
        }
    }

    #[test]
    fn message_end_converts_correctly() {
        let reason = String::from("❌");
        let reason_comparison_clone = reason.clone();

        let message = Message::Chat(reason);
        let bytes = message.as_bytes();
        let parsed_message = match Message::from_bytes(&bytes) {
            Ok(message) => message,
            Err(err) => panic!("Failed to parse message: {}", err),
        };

        assert_eq!(message.id(), parsed_message.id());
        if let Message::End(reason) = parsed_message {
            assert_eq!(reason, reason_comparison_clone);
        } else {
            panic!("Parsed message is not of type MessageKind::End");
        }
    }
}
