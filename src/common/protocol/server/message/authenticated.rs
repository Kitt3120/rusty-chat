use super::{MessageParseError, Serializable};
use std::fmt::Display;

#[derive(Clone, Debug, PartialEq)]
pub struct Authenticated {}

impl Authenticated {
    pub fn new() -> Authenticated {
        Authenticated {}
    }
}

impl Display for Authenticated {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Serializable for Authenticated {
    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();

        bytes
    }

    fn from_bytes(bytes: &[u8]) -> Result<Authenticated, MessageParseError> {
        Ok(Authenticated::new())
    }
}
