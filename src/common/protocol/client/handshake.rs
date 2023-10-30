use super::super::Serializable;
use super::{
    super::{client, message, server},
    HandshakeError,
};
use message::Message;
use std::{
    io::{Read, Write},
    net::TcpStream,
};

#[derive(Clone, Debug, PartialEq)]
pub struct HandshakeArguments {
    username: String,
}

impl HandshakeArguments {
    fn new(username: String) -> HandshakeArguments {
        HandshakeArguments { username }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Handshake {
    username: String,
}

impl Handshake {
    fn new(username: String) -> Handshake {
        Handshake { username }
    }

    pub fn perform(
        mut tcp_stream: &TcpStream,
        arguments: HandshakeArguments,
    ) -> Result<Handshake, HandshakeError> {
        let authentication_message =
            Message::Client(client::Message::Authenticate(arguments.username));

        tcp_stream
            .write_all(&authentication_message.as_bytes())
            .map_err(|err| HandshakeError::IoError(err))?;

        let mut handshake_result_buffer = Vec::new();

        tcp_stream
            .read_to_end(&mut handshake_result_buffer)
            .map_err(|err| HandshakeError::IoError(err))?;

        let message = server::Message::from_bytes(&handshake_result_buffer)
            .map_err(|err| HandshakeError::MessageParseError(err))?;

        match message {
            server::Message::Authenticated => Ok(Handshake::new(arguments.username)),
            server::Message::End(reason) => Err(HandshakeError::AuthenticationFailed(reason)),
            other => Err(HandshakeError::UnexpectedMessage(other)),
        }
    }
}
