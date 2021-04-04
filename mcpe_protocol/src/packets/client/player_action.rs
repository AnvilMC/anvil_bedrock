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

//TODO add enum for action
/*
ACTION_START_BREAK = 0
ACTION_ABORT_BREAK = 1
ACTION_STOP_BREAK = 2
ACTION_GET_UPDATED_BLOCK = 3
ACTION_DROP_ITEM = 4
ACTION_START_SLEEPING = 5
ACTION_STOP_SLEEPING = 6
ACTION_RESPAWN = 7
ACTION_JUMP = 8
ACTION_START_SPRINT = 9
ACTION_STOP_SPRINT = 10
ACTION_START_SNEAK = 11
ACTION_STOP_SNEAK = 12
ACTION_DIMENSION_CHANGE_REQUEST = 13
ACTION_DIMENSION_CHANGE_ACK = 14
ACTION_START_GLIDE = 15
ACTION_STOP_GLIDE = 16
ACTION_BUILD_DENIED = 17
ACTION_CONTINUE_BREAK = 18
ACTION_SET_ENCHANTMENT_SEED = 20
ACTION_START_SWIMMING = 21
ACTION_STOP_SWIMMING = 22
ACTION_START_SPIN_ATTACK = 23
ACTION_STOP_SPIN_ATTACK = 24;
*/
