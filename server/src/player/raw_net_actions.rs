use std::net::SocketAddr;

use mcpe_protocol::{
    prelude::{ByteArray, MCPEPacketDataError},
    traits::MCPEPacketData,
};
use raknet::prelude::{FrameManager, GamePacket, RaknetPacket, RaknetPacketData};
use tokio::net::UdpSocket;

use crate::GamePacketSendablePacket;

pub async fn send_game_packets(
    frame: &mut FrameManager,
    buf: &mut Vec<u8>,
    peer: &SocketAddr,
    socket: &UdpSocket,
    packet: &[GamePacketSendablePacket],
) -> Result<(), MCPEPacketDataError> {
    buf.clear();
    let mut buffer = Vec::with_capacity(1024 * 1024);
    for i in packet {
        buffer.push(i.get_id());
        i.encode(&mut buffer)?;
        ByteArray::from(buffer).encode(buf)?;
        buffer = Vec::with_capacity(1024 * 1024);
    }
    let game_packet = GamePacket(buf.clone());
    send_framed(frame, buf, peer, socket, game_packet).await?;
    Ok(())
}

pub async fn send_framed(
    frame: &mut FrameManager,
    buf: &mut Vec<u8>,
    peer: &SocketAddr,
    socket: &UdpSocket,
    packet: impl RaknetPacket,
) -> Result<(), MCPEPacketDataError> {
    for i in frame.encode_as_frame(packet) {
        buf.clear();
        buf.push(i.id());
        i.encode(buf)
            .ok_or_else(|| MCPEPacketDataError::new("raknet_error", "Unknown raknet error"))?;

        socket
            .send_to(buf, peer)
            .await
            .ok()
            .ok_or_else(|| MCPEPacketDataError::new("raknet_error", "Unknown network error"))?;
    }
    Ok(())
}

pub async fn send(
    buf: &mut Vec<u8>,
    peer: &SocketAddr,
    socket: &UdpSocket,
    packet: impl RaknetPacket,
) -> Result<(), MCPEPacketDataError> {
    buf.clear();
    buf.push(packet.id());
    packet
        .encode(buf)
        .ok_or_else(|| MCPEPacketDataError::new("raknet_error", "Unknown raknet error"))?;
    socket
        .send_to(buf, peer)
        .await
        .ok()
        .ok_or_else(|| MCPEPacketDataError::new("raknet_error", "Unknown network error"))?;
    Ok(())
}
