pub mod client;
pub mod server;

use crate::common::protocol::Message;

use super::Serializable;

pub trait Packet: Serializable {
    fn to_message(self) -> Message;
}
