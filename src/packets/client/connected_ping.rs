use packet_derive::packet;

use crate::packets::objects::{guid::GUID, time::Time};
#[packet(0x00)]
#[derive(Debug, packet_derive::Biscuit)]
pub struct ConnectedPing {
    pub time: Time,
}
