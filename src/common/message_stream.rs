pub mod error;

use std::{
    io::{Read, Write},
    net::TcpStream,
    ops::Deref,
};

use self::error::MessageStreamError;

use crate::common::protocol::{message::Message, serializable::Serializable};

#[derive(Debug)]
pub struct MessageStream {
    tcp_stream: TcpStream,
}

impl MessageStream {
    pub fn new(tcp_stream: TcpStream) -> MessageStream {
        MessageStream { tcp_stream }
    }

    pub fn read_message(&mut self) -> Result<Message, MessageStreamError> {
        let mut message_buffer = Vec::<u8>::new();

        self.tcp_stream
            .read_to_end(&mut message_buffer)
            .map_err(MessageStreamError::IoError)?;

        let message =
            Message::from_bytes(&message_buffer).map_err(MessageStreamError::MessageParseError)?;

        Ok(message)
    }

    pub fn send_message(&mut self, message: &Message) -> Result<(), MessageStreamError> {
        let message_bytes = message.as_bytes();

        self.tcp_stream
            .write_all(&message_bytes)
            .map_err(MessageStreamError::IoError)?;

        Ok(())
    }
}

impl Deref for MessageStream {
    type Target = TcpStream;

    fn deref(&self) -> &Self::Target {
        &self.tcp_stream
    }
}

//TODO: Integration tests
