use packet_derive::{packet, MCPEPacketDataAuto};

use crate::prelude::VarInt;

#[packet(0x46)]
#[derive(MCPEPacketDataAuto)]
pub struct ChunkRadiusUpdated {
    pub radius: VarInt,
}
