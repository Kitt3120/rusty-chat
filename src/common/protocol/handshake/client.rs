use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::common::protocol::{
    error::HandshakeError,
    message::{server, Message},
    packet::{client::Authenticate, server::Authenticated, Packet},
    Serializable,
};

#[derive(Clone, Debug, PartialEq)]
pub struct HandshakeArguments {
    username: String,
}

impl HandshakeArguments {
    pub fn new(username: String) -> HandshakeArguments {
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
        tcp_stream: &mut TcpStream,
        arguments: HandshakeArguments,
    ) -> Result<Handshake, HandshakeError> {
        send_authentication(tcp_stream, arguments.username.clone())?;
        let _authenticated = receive_authentication_result(tcp_stream)?;
        // Currently, the server's authenticated packet does not contain any data, so we don't need anything from it.
        // However, it's already implemented here so we don't have to adapt here again when finally adding data to the packet.

        let handshake = Handshake::new(arguments.username);
        Ok(handshake)
    }
}

fn send_authentication(tcp_stream: &mut TcpStream, username: String) -> Result<(), HandshakeError> {
    let authenticate_packet = Authenticate::new(username);
    let message = authenticate_packet.to_message();

    tcp_stream
        .write_all(&message.as_bytes())
        .map_err(HandshakeError::IoError)?;

    Ok(())
}

fn receive_authentication_result(
    tcp_stream: &mut TcpStream,
) -> Result<Authenticated, HandshakeError> {
    let mut handshake_result_buffer = Vec::new();

    tcp_stream
        .read_to_end(&mut handshake_result_buffer)
        .map_err(HandshakeError::IoError)?;

    let message =
        Message::from_bytes(&handshake_result_buffer).map_err(HandshakeError::MessageParseError)?;

    let authenticated_packet = match message {
        Message::Server(message) => match message {
            server::Message::Authenticated(authenticated) => authenticated,
            server::Message::End(end) => {
                return Err(HandshakeError::AuthenticationFailed(end.reason))
            }
            _ => return Err(HandshakeError::UnexpectedMessage(Message::Server(message))),
        },
        _ => return Err(HandshakeError::UnexpectedMessage(message)),
    };

    Ok(authenticated_packet)
}

//TODO: Test
