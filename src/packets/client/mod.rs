use self::{connection_request::ConnectionRequest, frame_packet::FramePacket, open_connection_request_one::OpenConnectionRequestOne, open_connection_request_two::OpenConnectionRequestTwo, ping_packet::PingPacket};

use super::{common::ack::Ack, traits::{IterRead, U8Iter}};

pub mod open_connection_request_one;
pub mod open_connection_request_two;
pub mod connection_request;
pub mod ping_packet;
pub mod frame_packet;

#[derive(Debug)]
pub enum PacketClient {
    PingPacket(PingPacket),
    OpenConnectionRequestOne(OpenConnectionRequestOne),
    OpenConnectionRequestTwo(OpenConnectionRequestTwo),
    FramePacket(FramePacket),
    Ack(Ack)
}

#[derive(Debug)]
pub enum PacketGameClient {
    ConnectionRequest(ConnectionRequest),
}

impl PacketGameClient {
    pub fn parse_packet(iter: &mut U8Iter) -> Option<Self> {
        Some(match iter.next()? {
            0x09 => Self::ConnectionRequest(iter.read()?),
            e => {
                panic!("Packet not yet implemented {}", e);
            }
        })
    }
}

impl PacketClient {
    pub fn parse_packet(iter: &mut U8Iter) -> Option<Self> {
        Some(match iter.next()? {
            1..=2 => Self::PingPacket(iter.read()?),
            0x05 => Self::OpenConnectionRequestOne(iter.read()?),
            0x07 => Self::OpenConnectionRequestTwo(iter.read()?),
            0x80..=0x8D => Self::FramePacket(iter.read()?),
            0xC0 => Self::Ack(iter.read()?),
            e => {
                panic!("Packet not yet implemented {}", e);
            }
        })
    }
}
