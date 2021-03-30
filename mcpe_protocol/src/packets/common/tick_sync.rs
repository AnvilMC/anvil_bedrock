use packet_derive::{packet, MCPEPacketDataAuto};

#[packet(0x17)]
#[derive(Debug, MCPEPacketDataAuto)]
pub struct TickSyncPacket {
    request_time: i64,
    response_time: i64,
}
