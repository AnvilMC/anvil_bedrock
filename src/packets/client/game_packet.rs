use packet_derive::packet;

use crate::packets::objects::{
    compressed_data::Compressed, guid::GUID, take_all::TakeAll, time::Time,
};
#[packet(0xfe)]
#[derive(Debug, packet_derive::Biscuit)]
pub struct GamePacket(pub Compressed<TakeAll<u8>>);
