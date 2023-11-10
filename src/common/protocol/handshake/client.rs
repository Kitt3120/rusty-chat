use crate::common::{
    message_stream::MessageStream,
    protocol::{
        error::HandshakeError,
        message::{server, Message},
        packet::{client::Authenticate, server::Authenticated, Packet},
    },
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
        message_stream: &mut MessageStream,
        arguments: HandshakeArguments,
    ) -> Result<Handshake, HandshakeError> {
        send_authentication(message_stream, arguments.username.clone())?;
        let _authenticated = receive_authentication_result(message_stream)?;
        // Currently, the server's authenticated packet does not contain any data, so we don't need anything from it.
        // However, it's already implemented here so we don't have to adapt here again when finally adding data to the packet.

        let handshake = Handshake::new(arguments.username);
        Ok(handshake)
    }
}

fn send_authentication(
    message_stream: &mut MessageStream,
    username: String,
) -> Result<(), HandshakeError> {
    let authenticate_packet = Authenticate::new(username);
    let message = authenticate_packet.to_message();

    message_stream
        .send_message(&message)
        .map_err(HandshakeError::MessageStreamError)?;

    Ok(())
}

fn receive_authentication_result(
    message_stream: &mut MessageStream,
) -> Result<Authenticated, HandshakeError> {
    let message = message_stream
        .read_message()
        .map_err(HandshakeError::MessageStreamError)?;

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
