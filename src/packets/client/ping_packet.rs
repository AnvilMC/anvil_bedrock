use packet_derive::packet;

use crate::packets::objects::{client_guid::ClientGUID, magic::Magic, time::Time};

#[packet(0x01)]
#[derive(Debug, packet_derive::Biscuit)]
pub struct PingPacket {
    pub time: Time,
    pub magic: Magic,
    pub id: ClientGUID,
}
