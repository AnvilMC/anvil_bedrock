use packet_derive::{packet, MCPEPacketDataAuto};

use crate::prelude::{Le, VecIndexed};

#[packet(0x08)]
#[derive(Debug, MCPEPacketDataAuto)]
pub struct ResourcePackClientResponsePacket {
    pub status: u8, // i8 in java but only accepted values are 0 1 2 3 4 so nothing will change and this is more correct and performant
    pub pack_ids: VecIndexed<String, Le<u16>>, // in java i16 but u16 should be fine more correct and performant
}
