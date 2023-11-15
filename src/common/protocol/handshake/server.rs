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

#[derive(Debug)]
pub struct HandshakeArguments<'a> {
    pub taken_usernames: Vec<&'a str>,
}

impl<'a> HandshakeArguments<'a> {
    pub fn new(taken_usernames: Vec<&'a str>) -> HandshakeArguments<'a> {
        HandshakeArguments { taken_usernames }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Handshake {
    pub username: String,
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

        let username = authenticate_packet.username;
        let username_taken = arguments.taken_usernames.contains(&username.as_str());

        send_authentication_result(message_stream, &username, username_taken)?;

        match username_taken {
            true => Err(HandshakeError::AuthenticationFailed(format!(
                "Username {} already taken",
                username
            ))),
            false => Ok(Handshake::new(username)),
        }
    }
}

fn receive_authentication(
    message_stream: &mut MessageStream,
) -> Result<Authenticate, HandshakeError> {
    let message = message_stream
        .read_message()
        .map_err(HandshakeError::MessageStreamError)?;

    let authenticate_packet = match message {
        Message::Client(client::Message::Authenticate(authentication)) => authentication,
        _ => return Err(HandshakeError::UnexpectedMessage(message)),
    };

    Ok(authenticate_packet)
}

fn send_authentication_result(
    message_stream: &mut MessageStream,
    username: &str,
    username_taken: bool,
) -> Result<(), HandshakeError> {
    let message = match username_taken {
        true => {
            let end_packet = End::new(format!("Username {} already taken", username));
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
