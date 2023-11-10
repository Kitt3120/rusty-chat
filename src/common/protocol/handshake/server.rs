use crate::common::{
    message_stream::MessageStream,
    protocol::{
        error::HandshakeError,
        message::{client, Message},
        packet::{
            client::Authenticate,
            server::{Authenticated, End},
            Packet,
        },
    },
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
        message_stream: &mut MessageStream,
        arguments: HandshakeArguments,
    ) -> Result<Handshake, HandshakeError> {
        let authenticate_packet = receive_authentication(message_stream)?;
        send_authentication_result(
            message_stream,
            arguments.taken_usernames,
            authenticate_packet.username.clone(),
        )?;

        let handshake = Handshake::new(authenticate_packet.username);
        Ok(handshake)
    }
}

fn receive_authentication(
    message_stream: &mut MessageStream,
) -> Result<Authenticate, HandshakeError> {
    let message = message_stream
        .read_message()
        .map_err(HandshakeError::MessageStreamError)?;

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
    message_stream: &mut MessageStream,
    taken_usernames: &[String],
    username: String,
) -> Result<(), HandshakeError> {
    let username_taken = taken_usernames.contains(&username);
    let message = match username_taken {
        true => {
            let end_packet = End::new(String::from("Username already taken"));
            end_packet.to_message()
        }
        false => {
            let authenticated_packet = Authenticated::new();
            authenticated_packet.to_message()
        }
    };

    message_stream
        .send_message(&message)
        .map_err(HandshakeError::MessageStreamError)?;

    Ok(())
}
//TODO: Test
