use super::super::error::MessageParseError;
use std::fmt::{Debug, Display};

pub enum Message {
    Authenticated,
    Chat(String, String),
    End(String),
}

impl Clone for Message {
    fn clone(&self) -> Self {
        match self {
            Message::Authenticated => Message::Authenticated,
            Message::Chat(username, message) => Message::Chat(username.clone(), message.clone()),
            Message::End(reason) => Message::End(reason.clone()),
        }
    }
}

impl Debug for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::Authenticated => write!(f, "Authenticated"),
            Message::Chat(username, message) => write!(f, "Chat({}, {})", username, message),
            Message::End(reason) => write!(f, "End({})", reason),
        }
    }
}

impl Display for Message {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Message::Authenticated => write!(f, "Authenticated"),
            Message::Chat(username, message) => write!(f, "Chat({}, {})", username, message),
            Message::End(reason) => write!(f, "End({})", reason),
        }
    }
}

impl Message {
    pub fn id(&self) -> u8 {
        match self {
            Message::Authenticated => 0,
            Message::Chat(_, _) => 1,
            Message::End(_) => 2,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        match self {
            Message::Authenticated => vec![self.id()],
            Message::Chat(username, message) => {
                let username_bytes = username.as_bytes();
                let username_length = username_bytes.len().to_le_bytes();

                let message_bytes = message.as_bytes();

                let mut bytes = vec![self.id()];
                bytes.extend_from_slice(&username_length);
                bytes.extend_from_slice(username_bytes);
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
            0 => Ok(Message::Authenticated),
            1 => {
                if bytes.len() < 17 {
                    return Err(MessageParseError::UnexcpetedEndOfMessage);
                }

                let mut read_end = 9;

                let username_length = usize::from_le_bytes(bytes[1..read_end].try_into().unwrap());
                read_end += username_length;

                let username = match String::from_utf8(bytes[9..read_end].to_vec()) {
                    Ok(username) => username,
                    Err(err) => {
                        return Err(MessageParseError::StringParse(
                            String::from("Username"),
                            err,
                        ))
                    }
                };

                let message = match String::from_utf8(bytes[read_end..].to_vec()) {
                    Ok(message) => message,
                    Err(err) => {
                        return Err(MessageParseError::StringParse(String::from("Message"), err))
                    }
                };

                Ok(Message::Chat(username, message))
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
    fn message_authenticated_converts_correctly() {
        let message = Message::Authenticated;
        let bytes = message.as_bytes();
        let parsed_message = match Message::from_bytes(&bytes) {
            Ok(message) => message,
            Err(err) => panic!("Failed to parse message: {}", err),
        };

        assert_eq!(message.id(), parsed_message.id());
    }

    #[test]
    fn message_chat_converts_correctly() {
        let username = String::from("Kitt3120");
        let message = String::from("⚡");

        let username_comparison_clone = username.clone();
        let message_comparison_clone = message.clone();

        let message = Message::Chat(username, message);
        let bytes = message.as_bytes();
        let parsed_message = match Message::from_bytes(&bytes) {
            Ok(message) => message,
            Err(err) => panic!("Failed to parse message: {}", err),
        };

        assert_eq!(message.id(), parsed_message.id());
        if let Message::Chat(username, message) = parsed_message {
            assert_eq!(username, username_comparison_clone);
            assert_eq!(message, message_comparison_clone);
        } else {
            panic!("Parsed message is not of type MessageKind::Chat");
        }
    }

    #[test]
    fn message_end_converts_correctly() {
        let message_string = String::from("❌");
        let message_comparison_clone = message_string.clone();

        let message = Message::End(message_string);
        let bytes = message.as_bytes();
        let parsed_message = match Message::from_bytes(&bytes) {
            Ok(message) => message,
            Err(err) => panic!("Failed to parse message: {}", err),
        };

        assert_eq!(message.id(), parsed_message.id());
        if let Message::End(reason) = parsed_message {
            assert_eq!(reason, message_comparison_clone);
        } else {
            panic!("Parsed message is not of type MessageKind::End");
        }
    }
}
