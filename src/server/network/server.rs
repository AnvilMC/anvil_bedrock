use std::{future::Future, net::SocketAddr};

use either::Either;
use tokio::net::UdpSocket;

use crate::{
    packets::traits::{Packet, PacketDecoder},
    server::server::Server,
};

use crate::packets::client::PacketClient;

use crate::packets::server::{
    connection_request_accepted::ConnectionRequestAccepted,
    open_connection_reply_one::OpenConnectionReplyOne,
    open_connection_reply_two::OpenConnectionReplyTwo,
};

use crate::packets::encode;

use crate::packets::client::frame_packet::{FrameManager, FramePacket, Frames, UNRELIABLE};
use crate::packets::client::PacketGameClient;
use crate::packets::common::ack::{Ack, Record};
use crate::packets::objects::uint24le::UInt24Le;
use crate::packets::server::connected_pong::ConnectedPong;
use crate::packets::server::pong_packet::PongPacket;

use tokio::runtime::Runtime;

pub struct NetworkManager {
    pub server_info: Server,
    pub socket: UdpSocket,
}

impl NetworkManager {
    pub async fn run(self) -> Result<(), std::io::Error> {
        let NetworkManager {
            socket,
            server_info,
        } = self;
        let mut buf = vec![0; 1024 * 1024];

        let mut frame_manager = FrameManager::default();

        loop {
            let (size, peer) = socket.recv_from(&mut buf).await?;

            //println!("{:?}",&buf[..size]);
            let mut iter = buf.clone().into_iter().take(size);


            if let Some(packet) = PacketClient::parse_packet(&mut iter) {
                match packet {
                    PacketClient::PingPacket(packet) => {
                        socket
                            .send_packet(&peer, PongPacket::from(packet, &server_info))
                            .await
                            .expect("Can't send");
                    }
                    PacketClient::OpenConnectionRequestOne(packet) => {
                        socket
                            .send_packet(&peer, OpenConnectionReplyOne::from(packet, &server_info))
                            .await
                            .expect("Can't send");
                    }
                    PacketClient::OpenConnectionRequestTwo(packet) => {
                        socket
                            .send_packet(
                                &peer,
                                OpenConnectionReplyTwo::from(packet, (&peer).into(), &server_info),
                            )
                            .await
                            .expect("Can't send");
                    }
                    PacketClient::FramePacket(packet) => {
                        let p0 = frame_manager.get_range();
                        let (p1,p2) = frame_manager.append(packet);
                        //println!("IIII {:?}",p1);
                        if p0 != p1 {
                            socket
                            .send_packet(
                                &peer,
                                Ack {
                                    record: vec![Record(Either::Right((UInt24Le(0),UInt24Le(p1 as u32 + 3))))]
                                }
                            ).await;
                        }
                        if let Some(e) = p2 {
                            //println!("REASSEMBLED {:?}", e);
                            let mut iter = e.into_iter();

                            match PacketGameClient::parse_packet(&mut iter).unwrap() {
                                PacketGameClient::ConnectionRequest(packet) => {
                                    //println!("{:?}", packet);

                                    socket
                                        .send_framed(
                                            &mut frame_manager,
                                            &peer,
                                            ConnectionRequestAccepted::from(packet, (&peer).into()),
                                        )
                                        .await
                                        .expect("Can't send FramePacket ConnectionRequest");

                                    // socket
                                    //     .send_packet(
                                    //         //&mut frame_manager,
                                    //         &peer,
                                    //         Ack {
                                    //             record: vec![Record(Either::Left(UInt24Le(0)))],
                                    //         },
                                    //     )
                                    //     .await
                                    //     .expect("Can't send ACK");
                                }
                                PacketGameClient::NewIncomingConnection(e) => {
                                    //println!("{:?}", e);
                                },
                                PacketGameClient::ConnectedPing(packet_ping) => {
                                    socket
                                            .send_framed(
                                                &mut frame_manager,
                                                &peer,
                                                ConnectedPong::from(packet_ping),
                                            )
                                            .await
                                            .expect("Can't send");
                                   /*  socket
                                            .send_packet(
                                                //&mut frame_manager,
                                                &peer,
                                                Ack {
                                                    record: vec![Record(Either::Left(UInt24Le(0)))],
                                                },
                                            )
                                            .await
                                            .expect("Can't send ACK"); */
                                }
                                PacketGameClient::GamePacket(game_packet) => {
                                    println!("{:?}", game_packet);
                                }
                            }
                        }
                    }
                    PacketClient::Ack(ack) => {
                        //println!("{:?}", ack);
                    },
                    PacketClient::ConnectedPing(packet_ping) => {
                        socket
                                .send_framed(
                                    &mut frame_manager,
                                    &peer,
                                    ConnectedPong::from(packet_ping),
                                )
                                .await
                                .expect("Can't send");
                    }
                }
            }
            //println!("Peer : {:?}", peer);
        }
    }
}

#[async_trait::async_trait]
trait PacketSender {
    async fn send_packet(
        &self,
        peer: &SocketAddr,
        packet: impl Packet + PacketDecoder + Send + 'async_trait,
    ) -> std::io::Result<usize>;

    async fn send_framed(
        &self,
        frame_manager: &mut FrameManager,
        peer: &SocketAddr,
        packet: impl Packet + PacketDecoder + Send + 'async_trait,
    ) -> std::io::Result<usize>;
}

#[async_trait::async_trait]
impl PacketSender for UdpSocket {
    async fn send_packet(
        &self,
        peer: &SocketAddr,
        packet: impl Packet + PacketDecoder + Send + 'async_trait,
    ) -> std::io::Result<usize> {
        let pa = encode(packet);
        self.send_to(&pa, peer).await
    }

    async fn send_framed(
        &self,
        frame_manager: &mut FrameManager,
        peer: &SocketAddr,
        packet: impl Packet + PacketDecoder + Send + 'async_trait,
    ) -> std::io::Result<usize> {
        self.send_to(&encode(frame_manager.from_packet(packet)), peer)
            .await
    }
}
