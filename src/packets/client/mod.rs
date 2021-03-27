use self::{open_connection_request_one::OpenConnectionRequestOne, open_connection_request_two::OpenConnectionRequestTwo, ping_packet::PingPacket};

use super::traits::{IterRead, U8Iter};

pub mod open_connection_request_one;
pub mod open_connection_request_two;
pub mod ping_packet;

#[derive(Debug)]
pub enum PacketClient {
    PingPacket(PingPacket),
    OpenConnectionRequestOne(OpenConnectionRequestOne),
    OpenConnectionRequestTwo(OpenConnectionRequestTwo),
}

impl PacketClient {
    pub fn parse_packet(iter: &mut U8Iter) -> Option<Self> {
        Some(match iter.next()? {
            1..=2 => Self::PingPacket(iter.read()?),
            0x05 => Self::OpenConnectionRequestOne(iter.read()?),
            0x07 => Self::OpenConnectionRequestTwo(iter.read()?),
            e => {
                panic!("Packet not yet implemented {}", e);
            }
        })
    }
}
