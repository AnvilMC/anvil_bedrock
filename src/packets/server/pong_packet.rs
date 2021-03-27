use packet_derive::{packet, Biscuit};

use crate::{
    packets::{
        client::ping_packet::PingPacket,
        objects::{magic::Magic, server_guid::ServerGUID, time::Time},
    },
    server::server::Server,
};

#[packet(0x1c)]
#[derive(Debug, Biscuit, Clone, PartialEq)]
pub struct PongPacket {
    pub time: Time,
    pub id: ServerGUID,
    pub magic: Magic,
    pub server_data: String,
    //connection_type: ConnectionType
}

impl PongPacket {
    pub fn from(e: PingPacket, data: &Server) -> Self {
        Self {
            time: e.time,
            id: data.guid.clone(),
            magic: e.magic,
            server_data: data.build_server_info(),
        }
    }
}
