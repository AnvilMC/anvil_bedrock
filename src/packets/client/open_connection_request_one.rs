use packet_derive::packet;

use crate::packets::{objects::magic::Magic, traits::Mtu};

#[packet(0x05)]
#[derive(Debug, packet_derive::Biscuit)]
pub struct OpenConnectionRequestOne {
    pub magic: Magic,
    pub protocol_version: u8,
    pub mtu: Mtu,
}
