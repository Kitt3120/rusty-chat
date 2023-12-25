// Packets are structured sender-bound. For example, when the server sends a packet, both server and client will use the server struct.
pub mod client;
pub mod server;

use crate::common::protocol::{message::Message, serializable::Serializable};

pub trait Packet: Serializable {
    fn to_message(self) -> Message;
}
