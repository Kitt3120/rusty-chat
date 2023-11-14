use std::net::SocketAddr;

use crate::common::{message_stream::MessageStream, protocol::handshake::server::Handshake};

#[derive(Debug)]
pub struct ClientHandle {
    pub socket_addr: SocketAddr,
    pub message_stream: MessageStream,
    pub handshake: Handshake,
    pub messages: Vec<String>,
}

impl ClientHandle {
    pub fn new(
        socket_addr: SocketAddr,
        message_stream: MessageStream,
        handshake: Handshake,
    ) -> ClientHandle {
        ClientHandle {
            socket_addr,
            message_stream,
            handshake,
            messages: Vec::new(),
        }
    }
}
