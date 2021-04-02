use crate::prelude::{ByteArray, Le, UnsignedVarInt};
use packet_derive::{packet, MCPEPacketDataAuto};

#[packet(0x07)]
#[derive(Debug, MCPEPacketDataAuto)]
pub struct ResourcePackStack {
    pub accept: bool,
    pub behaviour_pack_size: UnsignedVarInt,
    pub resource_pack_size: UnsignedVarInt,
    pub game_version: ByteArray,
    pub _exp: Le<i32>,
    pub _unknown: bool,
}

impl Default for ResourcePackStack {
    fn default() -> Self {
        Self {
            accept: false,
            behaviour_pack_size: UnsignedVarInt(0),
            resource_pack_size: UnsignedVarInt(0),
            game_version: ByteArray::from("1.16.210".as_bytes().to_vec()),
            _exp: Le(0),
            _unknown: false,
        }
    }
}
