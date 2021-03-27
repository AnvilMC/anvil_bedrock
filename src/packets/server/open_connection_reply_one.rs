use packet_derive::{packet, Biscuit};

use crate::{
    packets::{
        client::open_connection_request_one::OpenConnectionRequestOne,
        objects::{magic::Magic, server_guid::ServerGUID},
    },
    server::server::Server,
};

#[packet(0x06)]
#[derive(Debug, Biscuit)]
pub struct OpenConnectionReplyOne {
    pub magic: Magic,
    pub id: ServerGUID,
    pub security: bool,
    pub mtu: u16,
}

impl OpenConnectionReplyOne {
    pub fn from(e: OpenConnectionRequestOne, info: &Server) -> Self {
        Self {
            magic: e.magic,
            id: info.guid.clone(),
            security: false,
            mtu: e.mtu.0.len() as u16 + 46,
        }
    }
}
