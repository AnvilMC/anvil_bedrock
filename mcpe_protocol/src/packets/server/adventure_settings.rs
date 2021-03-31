use crate::prelude::StaticData;

use packet_derive::{packet, MCPEPacketDataAuto};

#[packet(0x37)]
#[derive(MCPEPacketDataAuto)]
pub struct AdventureSettingsPacket(StaticData<'static, u8>);

pub const ADVENTURE_SETTINGS: AdventureSettingsPacket =
    AdventureSettingsPacket(StaticData(&[224, 2, 1, 31, 2, 0, 1, 0, 0, 0, 0, 0, 0, 0]));
