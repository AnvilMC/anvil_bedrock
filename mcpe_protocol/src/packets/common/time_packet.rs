use packet_derive::{packet, MCPEPacketDataAuto};

use crate::prelude::VarInt;

#[packet(10)]
#[derive(Debug, MCPEPacketDataAuto)]
pub struct SetTimePacket {
    pub time: VarInt,
}
