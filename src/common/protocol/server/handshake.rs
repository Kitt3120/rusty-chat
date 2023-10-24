use super::super::{client, server};
use std::{
    io::{Error, Read, Write},
    net::TcpStream,
};

pub enum HandshakeError {
    IoError(Error),
    ClientMessageParseError(client::MessageParseError),
    UnexpectedClientMessage(client::Message),
    AuthenticationFailed(String),
}

pub struct ClientHandshakeArguments<'a> {
    taken_usernames: &'a Vec<String>,
}

impl<'a> ClientHandshakeArguments<'a> {
    pub fn new(taken_usernames: &'a Vec<String>) -> ClientHandshakeArguments<'a> {
        ClientHandshakeArguments { taken_usernames }
    }
}

pub struct ClientHandshake {
    username: String,
}

impl ClientHandshake {
    fn new(username: String) -> ClientHandshake {
        ClientHandshake { username }
    }

    pub fn perform(
        mut tcp_stream: &TcpStream,
        arguments: ClientHandshakeArguments,
    ) -> Result<ClientHandshake, HandshakeError> {
        let mut username_request_buffer = Vec::new();

        tcp_stream
            .read_to_end(&mut username_request_buffer)
            .map_err(|err| HandshakeError::IoError(err))?;

        let username_request = client::Message::from_bytes(&username_request_buffer)
            .map_err(|err| HandshakeError::ClientMessageParseError(err))?;

        let username = match username_request {
            client::Message::RequestUsername(username) => username,
            other => return Err(HandshakeError::UnexpectedClientMessage(other)),
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

        Ok(ClientHandshake::new(username))
    }
}
