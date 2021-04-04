use packet_derive::{packet, MCPEPacketDataAuto};

use crate::prelude::{BlockVec3, UnsignedVarLong, VarInt};

#[packet(0x24)]
#[derive(Debug, MCPEPacketDataAuto)]
pub struct PlayerActionPacket {
    pub entity_runtime_id: UnsignedVarLong,
    pub action: VarInt,
    pub coordinates: BlockVec3,
    pub face: VarInt,
}

impl From<VarInt> for u8 {
    fn from(e: VarInt) -> Self {
        e.0 as u8
    }
}

impl From<u8> for VarInt {
    fn from(e: u8) -> Self {
        VarInt(e as i32)
    }
}

#[repr(u8)]
#[packet_derive::mcpe_packet_data_enum(u8 VarInt)]
pub enum PlayerActionType {
    StartBreak = 0,
    AbortBreak = 1,
    StopBreak = 2,
    GetUpdatedBlock = 3,
    DropItem = 4,
    StartSleeping = 5,
    StopSleeping = 6,
    Respawn = 7,
    Jump = 8,
    StartSprint = 9,
    StopSprint = 10,
    StartSneak = 11,
    StopSneak = 12,
    DimensionChangeRequest = 13,
    DimensionChangeAck = 14,
    StartGlide = 15,
    StopGlide = 16,
    BuildDenied = 17,
    ContinueBreak = 18,
    SetEnchantmentSeed = 20,
    StartSwimming = 21,
    StopSwimming = 22,
    StartSpinAttack = 23,
    StopSpinAttack = 24,
}

//TODO add enum for
/*

*/
