use packet_derive::{packet, MCPEPacketDataAuto};
#[packet(0x3B)]
#[derive(MCPEPacketDataAuto)]
pub struct SetCommandsEnabledPacket {
    enabled: bool,
}
