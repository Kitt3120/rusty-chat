use super::{
    super::{client, server},
    HandshakeError,
};
use std::{
    io::{Read, Write},
    net::TcpStream,
};

pub struct HandshakeArguments {
    username: String,
}

impl HandshakeArguments {
    fn new(username: String) -> HandshakeArguments {
        HandshakeArguments { username }
    }
}

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
        let username_request_message = client::Message::RequestUsername(arguments.username);

        tcp_stream
            .write_all(&username_request_message.as_bytes())
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
