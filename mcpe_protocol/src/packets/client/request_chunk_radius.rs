use packet_derive::{packet, MCPEPacketDataAuto};

use crate::prelude::VarInt;

#[packet(69)]
#[derive(Debug, MCPEPacketDataAuto)]
pub struct RequestChunkRadiusPacket {
    pub radius: VarInt,
}
