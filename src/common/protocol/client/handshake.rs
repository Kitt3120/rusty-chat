use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::common::protocol::{
    client,
    error::HandshakeError,
    packet::{client::Authenticate, Packet},
    server, Message, Serializable,
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
        let username = arguments.username.clone(); //TODO: Better handling of this
        let authenticate_packet = Authenticate::new(username);
        let message = authenticate_packet.to_message();

        tcp_stream
            .write_all(&message.as_bytes())
            .map_err(HandshakeError::IoError)?;

        let mut handshake_result_buffer = Vec::new();

        tcp_stream
            .read_to_end(&mut handshake_result_buffer)
            .map_err(HandshakeError::IoError)?;

        let message = Message::from_bytes(&handshake_result_buffer)
            .map_err(HandshakeError::MessageParseError)?;

        match message {
            Message::Server(message) => match message {
                server::Message::Authenticated(authenticated) => authenticated,
                server::Message::End(end) => {
                    return Err(HandshakeError::AuthenticationFailed(end.reason))
                }
                _ => return Err(HandshakeError::UnexpectedMessage(Message::Server(message))),
            },
            _ => return Err(HandshakeError::UnexpectedMessage(message)),
        };

        let handshake = Handshake::new(arguments.username);

        Ok(handshake)
    }
}
