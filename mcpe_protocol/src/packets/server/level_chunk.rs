use packet_derive::{packet, MCPEPacketDataAuto};

use crate::prelude::{ByteArray, Le, UnsignedVarInt, VarInt, VecIndexed};

#[packet(58)]
#[derive(MCPEPacketDataAuto)]
pub struct LevelChunkPacket {
    chunk_x: VarInt,
    chunk_z: VarInt,
    sub_chunk_count: UnsignedVarInt,
    cache: Option<ChunkCache>,
    data: ByteArray,
}

#[derive(MCPEPacketDataAuto)]
pub struct ChunkCache {
    blob_ids: VecIndexed<Le<i64>, UnsignedVarInt>,
}
