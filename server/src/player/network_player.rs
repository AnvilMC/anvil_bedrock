use std::{net::SocketAddr, sync::Arc};

use crossbeam::channel::{bounded, Receiver, Sender};
use mcpe_protocol::{
    prelude::{
        ByteArray, ChunkRadiusUpdated, MCPEPacketDataError, ResourcePackStack, ResourcePacksInfo,
        StartGamePacket, UpdateBlock, VarInt, BIOME_DEFINITION_LIST, LOGIN_SUCCESS, PLAYER_SPAWN,
    },
    traits::MCPEPacketData,
};
use raknet::prelude::{
    ConnectedPing, ConnectedPong, ConnectionRequest, ConnectionRequestAccepted, FrameManager,
    FramePacket, GamePacket, RaknetPacket, RaknetPacketData,
};
use tokio::net::UdpSocket;

use crate::{send, send_framed, send_game_packets, GamePacketSendablePacket, ReceivablePacket};

pub struct NetworkPlayer {
    pub packet_sending_queue_s: Sender<GamePacketSendablePacket>,
    packet_sending_queue_r: Receiver<GamePacketSendablePacket>,
    pub peer: SocketAddr,
    pub socket: Arc<UdpSocket>,
    pub frame_manager: FrameManager,
    write_buffer: Vec<u8>,
}

impl NetworkPlayer {
    pub fn new(mtu: u16, socket: Arc<UdpSocket>, peer: SocketAddr) -> Self {
        let (s, r) = bounded(100);
        Self {
            packet_sending_queue_s: s,
            packet_sending_queue_r: r,
            peer,
            socket,
            frame_manager: FrameManager::new(mtu),
            write_buffer: vec![0; 1024 * 1024],
        }
    }

    pub async fn handle_frame_receive(
        &mut self,
        packet: FramePacket,
    ) -> Result<(), MCPEPacketDataError> {
        let (ack, paket) = self.frame_manager.process(packet);

        if let Some(e) = ack {
            self.send_raknet_immidietely(e).await?;
        }

        if let Some(e) = paket {
            let mut e = e.iter();
            match *Iterator::next(&mut e).unwrap() {
                0x09 => {
                    let packet_phoenix = ConnectionRequest::decode(&mut e).unwrap();

                    send_framed(
                        &mut self.frame_manager,
                        &mut self.write_buffer,
                        &self.peer,
                        self.socket.as_ref(),
                        ConnectionRequestAccepted::from(packet_phoenix, &self.peer),
                    )
                    .await?;
                }
                0xFE => {
                    let _packet_phoenix = GamePacket::decode(&mut e).unwrap();

                    let mut iter = _packet_phoenix.0.iter();

                    while let Ok(e) = ByteArray::decode(&mut iter) {
                        self.handle_game_packet(ReceivablePacket::try_read(&e.0).unwrap())?;
                    }
                }
                0x00 => {
                    let ping = ConnectedPing::decode(&mut e).unwrap();
                    send_framed(
                        &mut self.frame_manager,
                        &mut self.write_buffer,
                        &self.peer,
                        self.socket.as_ref(),
                        ConnectedPong::from(ping),
                    )
                    .await?;
                }
                e => {
                    println!("Invalid frame packet_id: {:?}", e)
                }
            }
        }

        Ok(())
    }

    pub fn handle_game_packet(
        &mut self,
        game_packet: ReceivablePacket,
    ) -> Result<(), MCPEPacketDataError> {
        match game_packet {
            ReceivablePacket::RequestChunkRadiusPacket(request_chunk_radius_packet) => {
                let a = 3.max(request_chunk_radius_packet.radius.0.min(10));
                self.send_packet(GamePacketSendablePacket::ChunkRadiusUpdated(
                    ChunkRadiusUpdated { radius: VarInt(a) },
                ))?;
            }
            ReceivablePacket::TickSyncPacket(e) => {
                self.send_packet(GamePacketSendablePacket::TickSyncPacket(e))?;
            }
            ReceivablePacket::LoginPacket(_) => {
                self.send_packet(GamePacketSendablePacket::PlayStatus(LOGIN_SUCCESS))?;
                self.send_packet(GamePacketSendablePacket::ResourcePacksInfo(
                    ResourcePacksInfo::default(),
                ))?;
            }
            ReceivablePacket::ResourcePackClientResponsePacket(e) => {
                if e.status == 4 {
                    self.send_packet(GamePacketSendablePacket::StartGamePacket(
                        StartGamePacket::new(),
                    ))?;
                    self.send_packet(GamePacketSendablePacket::BiomeDefinitionList(
                        BIOME_DEFINITION_LIST,
                    ))?;
                    self.send_packet(GamePacketSendablePacket::PlayStatus(PLAYER_SPAWN))?;
                    self.send_packet(GamePacketSendablePacket::UpdateBlock(UpdateBlock::new(
                        0, 1, 0,
                    )))?;
                } else {
                    self.send_packet(GamePacketSendablePacket::ResourcePackStack(
                        ResourcePackStack::default(),
                    ))?;
                }
            }
        }
        Ok(())
    }

    pub async fn send_raknet_immidietely(
        &mut self,
        packet: impl RaknetPacket,
    ) -> Result<(), MCPEPacketDataError> {
        send(
            &mut self.write_buffer,
            &self.peer,
            self.socket.as_ref(),
            packet,
        )
        .await
    }

    pub fn send_packet(&self, packet: GamePacketSendablePacket) -> Result<(), MCPEPacketDataError> {
        self.packet_sending_queue_s
            .try_send(packet)
            .map_err(|e| MCPEPacketDataError::new("sender error", format!("crossbeam error {}", e)))
    }

    pub async fn handle_send_packet(&mut self) -> Result<(), MCPEPacketDataError> {
        let mut packets = Vec::new();
        while let Ok(e) = self.packet_sending_queue_r.try_recv() {
            packets.push(e);
        }
        if packets.is_empty() {
            return Ok(());
        }
        send_game_packets(
            &mut self.frame_manager,
            &mut self.write_buffer,
            &self.peer,
            self.socket.as_ref(),
            &packets,
        )
        .await
    }
}
