use super::super::{
    client::Message as ClientMessage, server, server::Message as ServerMessage, HandshakeError,
    Message, Serializable,
};
use std::{
    io::{Read, Write},
    net::TcpStream,
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

        let authentication = match message {
            Message::Client(message) => match message {
                ClientMessage::Authenticate(authentication) => authentication,
                _ => return Err(HandshakeError::UnexpectedMessage(Message::Client(message))),
            },
            _ => return Err(HandshakeError::UnexpectedMessage(message)),
        };

        if arguments.taken_usernames.contains(&authentication.username) {
            let end = server::message::End::new(String::from("Username already taken"));
            let end_message = Message::Server(ServerMessage::End(end));

            match tcp_stream.write_all(&end_message.as_bytes()) {
                Ok(_) => {
                    return Err(HandshakeError::AuthenticationFailed(format!(
                        "Username already taken: {}",
                        &authentication.username
                    )))
                }
                Err(err) => return Err(HandshakeError::IoError(err)),
            }
        }

        let authenticated = server::message::Authenticated::new();
        let message = Message::Server(ServerMessage::Authenticated(authenticated));

        tcp_stream
            .write_all(&message.as_bytes())
            .map_err(HandshakeError::IoError)?;

        let handshake = Handshake::new(authentication.username);
        Ok(handshake)
    }
}
