use crate::prelude::{BlockVec3, UnsignedVarInt};
use packet_derive::{packet, MCPEPacketDataAuto};

#[packet(121)]
#[derive(Debug, MCPEPacketDataAuto)]
pub struct NetworkChunkPublisherUpdatePacket {
    position: BlockVec3,
    radius: UnsignedVarInt,
}
