use packet_derive::packet;

use crate::packets::objects::guid::GUID;
#[packet(0x09)]
#[derive(Debug, packet_derive::Biscuit)]
pub struct ConnectionRequest {
    pub guid: GUID,
    pub time: i64,
}
