use crate::common::protocol::{
    error::MessageParseError,
    message::{server, Message},
    packet::Packet,
    Serializable,
};
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub struct Chat {
    pub username: String,
    pub message: String,
}

impl Chat {
    pub fn new(username: String, message: String) -> Chat {
        Chat { username, message }
    }
}

impl Display for Chat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.username, self.message)
    }
}

impl Serializable for Chat {
    fn as_bytes(&self) -> Vec<u8> {
        let username_bytes = self.username.as_bytes();
        let mut bytes = Vec::<u8>::new();

        bytes.extend(username_bytes.len().to_le_bytes());
        bytes.extend_from_slice(username_bytes);
        bytes.extend(self.message.as_bytes());

        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Result<Chat, MessageParseError> {
        let usize_bytes = usize::BITS as usize / 8; //

        if bytes.len() < usize_bytes {
            return Err(MessageParseError::UnexcpetedEndOfMessage);
        }

        let username_length = match bytes[0..usize_bytes].try_into() {
            Ok(bytes) => usize::from_le_bytes(bytes),
            Err(_) => {
                return Err(MessageParseError::ByteParse(String::from(
                    "Username Length",
                )))
            }
        };

        // usize username length
        // + username_length bytes
        // + at least 1 character for message
        if bytes.len() < usize_bytes + username_length + 1 {
            return Err(MessageParseError::UnexcpetedEndOfMessage);
        }

        let username =
            match String::from_utf8(bytes[usize_bytes..usize_bytes + username_length].to_vec()) {
                Ok(username) => username,
                Err(err) => {
                    return Err(MessageParseError::StringParse(
                        String::from("Username"),
                        err,
                    ))
                }
            };

        let message = match String::from_utf8(bytes[usize_bytes + username_length..].to_vec()) {
            Ok(message) => message,
            Err(err) => return Err(MessageParseError::StringParse(String::from("Message"), err)),
        };

        Ok(Chat::new(username, message))
    }
}

impl Packet for Chat {
    fn to_message(self) -> Message {
        Message::Server(server::Message::Chat(self))
    }
}
