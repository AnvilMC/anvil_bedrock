use packet_derive::{packet, MCPEPacketDataAuto};

use crate::prelude::VarInt;

#[packet(70)]
#[derive(MCPEPacketDataAuto)]
pub struct ChunkRadiusUpdated {
    radius: VarInt,
}
