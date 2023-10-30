use super::super::Serializable;
use super::{
    super::{client, server},
    HandshakeError,
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
            .map_err(|err| HandshakeError::IoError(err))?;

        let authentication = client::Message::from_bytes(&authentication_buffer)
            .map_err(|err| HandshakeError::MessageParseError(err))?;

        let username = match authentication {
            client::Message::Authenticate(username) => username,
            other => return Err(HandshakeError::UnexpectedMessage(other)),
        };

        if arguments.taken_usernames.contains(&username) {
            let end_message =
                server::Message::End(format!("Username already taken: {}", &username));

            match tcp_stream.write_all(&end_message.as_bytes()) {
                Ok(_) => {
                    return Err(HandshakeError::AuthenticationFailed(format!(
                        "Username already taken: {}",
                        &username
                    )))
                }
                Err(err) => return Err(HandshakeError::IoError(err)),
            }
        }

        let authenticated_message = server::Message::Authenticated;

        tcp_stream
            .write_all(&authenticated_message.as_bytes())
            .map_err(|err| HandshakeError::IoError(err))?;

        Ok(Handshake::new(username))
    }
}
