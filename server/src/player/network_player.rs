use std::{net::SocketAddr, sync::Arc};

use crossbeam::channel::{bounded, Receiver, Sender};
use mcpe_protocol::{
    prelude::{ByteArray, MCPEPacketDataError},
    traits::MCPEPacketData,
};
use raknet::prelude::{
    ConnectedPing, ConnectedPong, ConnectionRequest, ConnectionRequestAccepted, FrameManager,
    FramePacket, GamePacket, RaknetPacket, RaknetPacketData,
};
use tokio::net::UdpSocket;

use crate::{
    send, send_framed, send_game_packets, EntityPlayer, GamePacketSendablePacket, ReceivablePacket,
    WorldManager,
};

pub struct PlayerJoinEvent {
    pub cancelled: bool,
    pub entity: EntityPlayer,
    pub packet_sending_queue: Arc<Sender<GamePacketSendablePacket>>,
}

pub struct NetworkPlayer {
    pub packet_sending_queue_s: Arc<Sender<GamePacketSendablePacket>>,
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
            packet_sending_queue_s: Arc::new(s),
            packet_sending_queue_r: r,
            peer,
            socket,
            frame_manager: FrameManager::new(mtu),
            write_buffer: vec![0; 1024 * 1024],
        }
    }

    pub async fn handle_frame_receive(
        &mut self,
        world_manager: &WorldManager,
        packet: FramePacket,
    ) -> Result<Option<ReceivablePacket>, MCPEPacketDataError> {
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
                        match ReceivablePacket::try_read(&e.0) {
                            Ok(e) => return Ok(Some(e)),
                            Err(e) => println!("Can't decode packet: {}", e),
                        }
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

        Ok(None)
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

    pub async fn send_game_packet(
        &mut self,
        packet: GamePacketSendablePacket,
    ) -> Result<(), MCPEPacketDataError> {
        send_game_packets(
            &mut self.frame_manager,
            &mut self.write_buffer,
            &self.peer,
            self.socket.as_ref(),
            &[packet],
        )
        .await
    }

    pub async fn handle_send_packet(&mut self) -> Result<(), MCPEPacketDataError> {
        let mut packets = Vec::new();
        while let Ok(e) = self.packet_sending_queue_r.try_recv() {
            println!("{:?}", e);
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
