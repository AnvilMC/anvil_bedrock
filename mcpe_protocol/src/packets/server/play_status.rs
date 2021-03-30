use packet_derive::{packet, MCPEPacketDataAuto};

pub const LOGIN_SUCCESS: PlayStatus = PlayStatus(0);
pub const LOGIN_FAILED_CLIENT: PlayStatus = PlayStatus(1);
pub const LOGIN_FAILED_SERVER: PlayStatus = PlayStatus(2);
pub const PLAYER_SPAWN: PlayStatus = PlayStatus(3);
pub const LOGIN_FAILED_INVALID_TENANT: PlayStatus = PlayStatus(4);
pub const LOGIN_FAILED_VANILLA_EDU: PlayStatus = PlayStatus(5);
pub const LOGIN_FAILED_EDU_VANILLA: PlayStatus = PlayStatus(6);
pub const LOGIN_FAILED_SERVER_FULL: PlayStatus = PlayStatus(7);

#[packet(0x02)]
#[derive(Debug, MCPEPacketDataAuto)]
pub struct PlayStatus(u32);
