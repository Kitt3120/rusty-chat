use std::{
    io::{Read, Write},
    net::TcpStream,
};

use crate::common::protocol::{
    error::HandshakeError,
    message::{client, Message},
    packet::{
        client::Authenticate,
        server::{Authenticated, End},
        Packet,
    },
    Serializable,
};

#[derive(Debug, Clone, PartialEq)]
pub struct HandshakeArguments<'a> {
    taken_usernames: &'a [String],
}

impl<'a> HandshakeArguments<'a> {
    pub fn new(taken_usernames: &'a [String]) -> HandshakeArguments<'a> {
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
        tcp_stream: &mut TcpStream,
        arguments: HandshakeArguments,
    ) -> Result<Handshake, HandshakeError> {
        let authenticate_packet = receive_authentication(tcp_stream)?;
        send_authentication_result(
            tcp_stream,
            arguments.taken_usernames,
            authenticate_packet.username.clone(),
        )?;

        let handshake = Handshake::new(authenticate_packet.username);
        Ok(handshake)
    }
}

fn receive_authentication(tcp_stream: &mut TcpStream) -> Result<Authenticate, HandshakeError> {
    let mut authenticate_buffer = Vec::new();

    tcp_stream
        .read_to_end(&mut authenticate_buffer)
        .map_err(HandshakeError::IoError)?;

    let message =
        Message::from_bytes(&authenticate_buffer).map_err(HandshakeError::MessageParseError)?;

    let authenticate_packet = match message {
        Message::Client(message) => match message {
            client::Message::Authenticate(authentication) => authentication,
            _ => return Err(HandshakeError::UnexpectedMessage(Message::Client(message))),
        },
        _ => return Err(HandshakeError::UnexpectedMessage(message)),
    };

    Ok(authenticate_packet)
}

fn send_authentication_result(
    tcp_stream: &mut TcpStream,
    taken_usernames: &[String],
    username: String,
) -> Result<(), HandshakeError> {
    let message = match taken_usernames.contains(&username) {
        true => {
            let end_packet = End::new(String::from("Username already taken"));
            end_packet.to_message()
        }
        false => {
            let authenticated_packet = Authenticated::new();
            authenticated_packet.to_message()
        }
    };

    tcp_stream
        .write_all(&message.as_bytes())
        .map_err(HandshakeError::IoError)?;

    Ok(())
}
//TODO: Test
