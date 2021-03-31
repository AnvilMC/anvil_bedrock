use packet_derive::{packet, MCPEPacketDataAuto};

use crate::prelude::UnsignedVarInt;

#[packet(0x4C)]
#[derive(MCPEPacketDataAuto)]
pub struct AvailableCommandsPacket {
    pub unknown1: UnsignedVarInt,
    pub unknown2: UnsignedVarInt,
    pub unknown3: UnsignedVarInt,
    pub unknown4: UnsignedVarInt,
    pub unknown5: UnsignedVarInt,
    pub unknown6: UnsignedVarInt,
}

impl Default for AvailableCommandsPacket {
    fn default() -> Self {
        Self {
            unknown1: UnsignedVarInt(0),
            unknown2: UnsignedVarInt(0),
            unknown3: UnsignedVarInt(0),
            unknown4: UnsignedVarInt(0),
            unknown5: UnsignedVarInt(0),
            unknown6: UnsignedVarInt(0),
        }
    }
}
