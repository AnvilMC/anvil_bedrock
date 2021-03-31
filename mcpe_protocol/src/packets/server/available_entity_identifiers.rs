use crate::prelude::StaticData;

use packet_derive::{packet, MCPEPacketDataAuto};

#[packet(0x77)]
#[derive(MCPEPacketDataAuto)]
pub struct AvailableEntityIdentifiersPacket(StaticData<'static, u8>);

pub const AVAILABLE_ENTITY_IDENTIFIERS_PACKET: AvailableEntityIdentifiersPacket =
    AvailableEntityIdentifiersPacket(StaticData(include_bytes!("entity_identifiers.dat")));
