use packet_derive::{packet, MCPEPacketDataAuto};

use crate::prelude::{ByteArray, ByteArrayEncapsulated, MCPEPacketData, PacketReader};

#[packet(0x01)]
#[derive(Debug, MCPEPacketDataAuto)]
pub struct LoginPacket {
    pub protocol_version: i32,
    pub chain_data: ByteArrayEncapsulated<String>,
}
