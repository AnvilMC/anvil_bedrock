use packet_derive::{packet, MCPEPacketDataAuto};

use crate::prelude::{BlockVec3, VarInt, GLOBAL_BLOCK_PALETTE};

#[packet(21)]
#[derive(Debug, MCPEPacketDataAuto)]
pub struct UpdateBlock {
    block_coord: BlockVec3,
    block_runtime_id: VarInt,
    flags: VarInt,
    data_layer: VarInt,
}

impl UpdateBlock {
    pub fn new(x: i32, y: u32, z: i32) -> Self {
        Self {
            block_coord: (x, y, z).into(),
            block_runtime_id: VarInt(GLOBAL_BLOCK_PALETTE.get_or_create_runtime_id(1, 0)),
            flags: VarInt(0),
            data_layer: VarInt(0),
        }
    }
}
