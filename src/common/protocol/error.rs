use std::{
    fmt::{Debug, Display},
    string::FromUtf8Error,
};

pub enum MessageParseError {
    MessageEmpty,
    UnexcpetedEndOfMessage,
    UnknownKind(u8),
    StringParse(String, FromUtf8Error),
}

impl Clone for MessageParseError {
    fn clone(&self) -> Self {
        match self {
            MessageParseError::MessageEmpty => MessageParseError::MessageEmpty,
            MessageParseError::UnexcpetedEndOfMessage => MessageParseError::UnexcpetedEndOfMessage,
            MessageParseError::UnknownKind(kind) => MessageParseError::UnknownKind(kind.clone()),
            MessageParseError::StringParse(value, err) => {
                MessageParseError::StringParse(value.clone(), err.clone())
            }
        }
    }
}

impl Debug for MessageParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageParseError::MessageEmpty => write!(f, "MessageEmpty"),
            MessageParseError::UnexcpetedEndOfMessage => {
                write!(f, "UnexcpetedEndOfMessage")
            }
            MessageParseError::UnknownKind(kind) => {
                write!(f, "UnknownKind({:?})", kind)
            }
            MessageParseError::StringParse(value, err) => {
                write!(f, "StringParse({:?}, {:?})", value, err.to_string())
            }
        }
    }
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
        }
    }
}
