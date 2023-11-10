use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::common::protocol::{
    error::HandshakeError,
    message::{client, Message},
    packet::{
        server::{Authenticated, End},
        Packet,
    },
    Serializable,
};

#[derive(Debug, Clone, PartialEq)]
pub struct HandshakeArguments<'a> {
    taken_usernames: &'a Vec<String>,
}

impl<'a> HandshakeArguments<'a> {
    pub fn new(taken_usernames: &'a Vec<String>) -> HandshakeArguments<'a> {
        HandshakeArguments { taken_usernames }
    }
}

#[derive(Debug, Clone, PartialEq)]
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
        let mut authentication_buffer = Vec::new();

        tcp_stream
            .read_to_end(&mut authentication_buffer)
            .map_err(HandshakeError::IoError)?;

        let message = Message::from_bytes(&authentication_buffer)
            .map_err(HandshakeError::MessageParseError)?;

        let authentication_packet = match message {
            Message::Client(message) => match message {
                client::Message::Authenticate(authentication) => authentication,
                _ => return Err(HandshakeError::UnexpectedMessage(Message::Client(message))),
            },
            _ => return Err(HandshakeError::UnexpectedMessage(message)),
        };

        if arguments
            .taken_usernames
            .contains(&authentication_packet.username)
        {
            let end_packet = End::new(String::from("Username already taken"));
            let message = end_packet.to_message();

            match tcp_stream.write_all(&message.as_bytes()) {
                Ok(_) => {
                    return Err(HandshakeError::AuthenticationFailed(format!(
                        "Username already taken: {}",
                        &authentication_packet.username
                    )))
                }
                Err(err) => return Err(HandshakeError::IoError(err)),
            }
        }

        let authenticated_packet = Authenticated::new();
        let message = authenticated_packet.to_message();

        tcp_stream
            .write_all(&message.as_bytes())
            .map_err(HandshakeError::IoError)?;

        let handshake = Handshake::new(authentication_packet.username);
        Ok(handshake)
    }
}

//TODO: Test
