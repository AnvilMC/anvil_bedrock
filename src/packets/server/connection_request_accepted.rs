use packet_derive::{packet, Biscuit};

use crate::{
    packets::{
        client::{
            connection_request::ConnectionRequest,
            open_connection_request_one::OpenConnectionRequestOne,
        },
        objects::{address::Address, magic::Magic, server_guid::ServerGUID},
    },
    server::server::Server,
};

#[packet(0x10)]
#[derive(Debug, Biscuit)]
pub struct ConnectionRequestAccepted {
    pub client_address: Address,
    pub system_index: i16,
    pub internal_ids: [Address; 10],
    pub request_time: i64,
    pub time: i64,
}

use either::Either;

macro_rules! bd {
    () => {
        Address {
            ip_bytes: Either::Left([255; 4]),
            port: 19132,
        };
    };
}

const fn build() -> [Address; 10] {
    [
        bd!(),
        bd!(),
        bd!(),
        bd!(),
        bd!(),
        bd!(),
        bd!(),
        bd!(),
        bd!(),
        bd!(),
    ]
}

impl ConnectionRequestAccepted {
    pub fn from(e: ConnectionRequest, client_address: Address) -> Self {
        Self {
            client_address,
            system_index: 0,
            internal_ids: build(),
            request_time: e.time,
            time: e.time,
        }
    }
}
