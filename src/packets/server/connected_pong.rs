use packet_derive::packet;

use crate::{packets::{client::connected_ping::ConnectedPing, objects::{guid::GUID, time::Time}}, server::server::Server};
#[packet(0x03)]
#[derive(Debug, packet_derive::Biscuit)]
pub struct ConnectedPong {
    pub ping_time: Time,
    pub pong_time: Time,
}

impl ConnectedPong {
    pub fn from(e: ConnectedPing) -> Self {
        Self {
            ping_time: e.time.clone(),
            pong_time: e.time,
            
        }
    }
}