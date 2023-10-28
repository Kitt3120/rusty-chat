use super::MessageParseError;

pub trait Serializable: Sized {
    fn id(&self) -> u8;

    fn as_bytes(&self) -> Vec<u8>;

    fn from_bytes(bytes: &[u8]) -> Result<Self, MessageParseError>
    where
        Self: Sized;
}
