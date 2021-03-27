use packet_derive::{packet, Biscuit};

use crate::{packets::{client::{open_connection_request_two::OpenConnectionRequestTwo}, objects::{address::Address, magic::Magic, server_guid::ServerGUID}}, server::server::Server};

#[packet(0x08)]
#[derive(Debug, Biscuit)]
pub struct OpenConnectionReplyTwo {
    pub magic: Magic,
    pub id: ServerGUID,
    pub client_address: Address,
    pub mtu: i16,
    pub encryption_enabled: bool,
}

impl OpenConnectionReplyTwo {
    pub fn from(e: OpenConnectionRequestTwo, client_address: Address, info: &Server) -> Self {
        Self {
            magic: e.magic,
            id: info.guid.clone(),
            client_address,
            mtu: e.mtu,
            encryption_enabled: false,
        }
    }
}
