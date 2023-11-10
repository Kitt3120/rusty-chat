use std::{fmt::Display, string::FromUtf8Error};

#[derive(Clone, Debug, PartialEq)]
pub enum MessageParseError {
    MessageEmpty,
    UnexcpetedEndOfMessage,
    UnknownKind(u8),
    StringParse(String, FromUtf8Error),
    ByteParse(String),
}

impl Display for MessageParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageParseError::MessageEmpty => write!(f, "Message was empty"),
            MessageParseError::UnexcpetedEndOfMessage => {
                write!(f, "Unexpected end of message")
            }
            MessageParseError::UnknownKind(kind) => {
                write!(f, "Message had unknown kind: {}", kind)
            }
            MessageParseError::StringParse(value, err) => {
                write!(
                    f,
                    "Unable to parse string expected for value {}: {}",
                    value, err
                )
            }
            MessageParseError::ByteParse(value) => {
                write!(f, "Unable to parse bytes expected for value {}", value)
            }
        }
    }
}
