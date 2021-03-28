use packet_derive::packet;

use crate::packets::objects::{address::Address};

#[packet(0x05)]
#[derive(Debug, packet_derive::Biscuit)]
pub struct NewIncomingConnection {
    pub server_address: Address,
    pub internal_address: Address,
}
