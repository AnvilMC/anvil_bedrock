use packet_derive::{packet, MCPEPacketDataAuto};

use crate::prelude::StaticData;

#[packet(0x7A)]
#[derive(MCPEPacketDataAuto)]
pub struct BiomeDefinitionList {
    inventory: StaticData<'static, u8>,
}

pub const BIOME_DEFINITION_LIST: BiomeDefinitionList = BiomeDefinitionList {
    inventory: StaticData(include_bytes!("biome_definitions.dat")),
};
