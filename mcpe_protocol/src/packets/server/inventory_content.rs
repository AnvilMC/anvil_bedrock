use packet_derive::{packet, MCPEPacketDataAuto};

use crate::prelude::UnsignedVarInt;

#[packet(0x31)]
#[derive(MCPEPacketDataAuto)]
pub struct InventoryContentPacket {
    pub inventory_id: UnsignedVarInt,
    pub slot: UnsignedVarInt,
}
