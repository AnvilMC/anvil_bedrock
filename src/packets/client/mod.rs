use self::{connected_ping::ConnectedPing, connection_request::ConnectionRequest, frame_packet::FramePacket, game_packet::GamePacket, new_incoming_connection::NewIncomingConnection, open_connection_request_one::OpenConnectionRequestOne, open_connection_request_two::OpenConnectionRequestTwo, ping_packet::PingPacket};

use super::{
    common::ack::Ack,
    traits::{IterRead, U8Iter},
};

pub mod connected_ping;
pub mod connection_request;
pub mod frame_packet;
pub mod new_incoming_connection;
pub mod open_connection_request_one;
pub mod open_connection_request_two;
pub mod ping_packet;
pub mod game_packet;

#[derive(Debug)]
pub enum PacketClient {
    PingPacket(PingPacket),
    OpenConnectionRequestOne(OpenConnectionRequestOne),
    OpenConnectionRequestTwo(OpenConnectionRequestTwo),
    ConnectedPing(ConnectedPing),
    FramePacket(FramePacket),
    Ack(Ack),
}

#[derive(Debug)]
pub enum PacketGameClient {
    ConnectionRequest(ConnectionRequest),
    NewIncomingConnection(NewIncomingConnection),
    ConnectedPing(ConnectedPing),
    GamePacket(GamePacket)
}

impl PacketGameClient {
    pub fn parse_packet(iter: &mut U8Iter) -> Option<Self> {
        Some(match iter.next()? {
            0x09 => Self::ConnectionRequest(iter.read()?),
            0x13 => Self::NewIncomingConnection(iter.read()?),
            0x00 => Self::ConnectedPing(iter.read()?),
            0xfe => Self::GamePacket(iter.read()?),
            e => {
                panic!("Packet not yet implemented {}", e);
            }
        })
    }
}

impl PacketClient {
    pub fn parse_packet(iter: &mut U8Iter) -> Option<Self> {
        let id = iter.next()?;
        println!("READING: {}",id);
        Some(match id {
            1..=2 => Self::PingPacket(iter.read()?),
            0x05 => Self::OpenConnectionRequestOne(iter.read()?),
            0x07 => Self::OpenConnectionRequestTwo(iter.read()?),
            0x80..=0x8D => Self::FramePacket(iter.read()?),
            0xC0 => Self::Ack(iter.read()?),
            0x00 => Self::ConnectedPing(iter.read()?),
            e => {
                panic!("Packet not yet implemented {}", e);
            }
        })
    }
}
