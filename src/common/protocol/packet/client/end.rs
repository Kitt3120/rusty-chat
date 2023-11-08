use crate::common::protocol::{error::MessageParseError, Serializable};
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub struct End {
    pub reason: String,
}

impl End {
    pub fn new(reason: String) -> End {
        End { reason }
    }
}

impl Display for End {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl Serializable for End {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes.extend_from_slice(self.reason.as_bytes());

        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Result<End, MessageParseError> {
        if bytes.is_empty() {
            return Err(MessageParseError::UnexcpetedEndOfMessage);
        }

        let reason = match String::from_utf8(bytes.to_vec()) {
            Ok(reason) => reason,
            Err(err) => return Err(MessageParseError::StringParse(String::from("Reason"), err)),
        };

        Ok(End::new(reason))
    }
}
