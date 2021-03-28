use either::Either;
use tokio::net::UdpSocket;

use crate::{packets::traits::PacketDecoder, server::server::Server};

use crate::packets::client::PacketClient;

use crate::packets::server::{open_connection_reply_one::OpenConnectionReplyOne,open_connection_reply_two::OpenConnectionReplyTwo, connection_request_accepted::ConnectionRequestAccepted};

use crate::packets::encode;

use crate::packets::server::pong_packet::PongPacket;
use crate::packets::client::frame_packet::{FrameManager, FramePacket, Frames, UNRELIABLE};
use crate::packets::client::PacketGameClient;
use crate::packets::server::connected_pong::ConnectedPong;
use crate::packets::objects::uint24le::UInt24Le;
use crate::packets::common::ack::{Ack, Record};

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

            if let 0x80..=0x8D = buf[0] {
                println!("buffer : {:?}", &buf[..size]);
            }
            let mut iter = buf.clone().into_iter().take(size);

            let pocket = PacketClient::parse_packet(&mut iter);

            if pocket.is_none() {
                continue;
            }

            match pocket.unwrap() {
                PacketClient::PingPacket(packet) => {
                    //println!("{:?}",packet);
                    socket
                        .send_to(&encode(PongPacket::from(packet, &server_info)), peer)
                        .await
                        .expect("Can't send");
                }
                PacketClient::OpenConnectionRequestOne(packet) => {
                    socket
                        .send_to(
                            &encode(OpenConnectionReplyOne::from(packet, &server_info)),
                            peer,
                        )
                        .await
                        .expect("Can't send");
                }
                PacketClient::OpenConnectionRequestTwo(packet) => {
                    socket
                        .send_to(
                            &encode(OpenConnectionReplyTwo::from(packet, (&peer).into(),&server_info)),
                            peer,
                        )
                        .await
                        .expect("Can't send");
                }
                PacketClient::FramePacket(packet) => {

                    if let Some(e) = frame_manager.append(packet) {
                        
                        let mut iter = e.into_iter().take(size);

                        match PacketGameClient::parse_packet(&mut iter).unwrap() {
                            PacketGameClient::ConnectionRequest(packet) => {
                                println!("{:?}",packet);

                                let send_packet = ConnectionRequestAccepted::from(packet, (&peer).into());

                                let payload = encode(send_packet);

                                let enco = &encode(FramePacket {
                                    sequence_number: UInt24Le(0),
                                    frames: Frames {
                                        reliable: UNRELIABLE.set_bas(true),
                                        length: payload.len() as u16,
                                        reliable_index: None,
                                        sequenced_index: None,
                                        order: None,
                                        split: None,
                                        payload,
                                        
                                    },
                                    
                                });

                                println!("{:?}", enco);

                                 socket
                                .send_to(
                                    enco
                                        
                                    ,
                                    peer,
                                )
                                .await
                                .expect("Can't send"); 

                                let ack = Ack {
                                    record: vec![Record(Either::Left(UInt24Le(0)))]
                                };
            
                                socket
                                    .send_to(
                                        &encode(ack),
                                        peer,
                                    )
                                    .await
                                    .expect("Can't send");
            
                                //println!("{:?}", packet);

                                
                            }
                            PacketGameClient::NewIncomingConnection(e) => {
                                println!("{:?}",e);
                            }
                            PacketGameClient::ConnectedPing(packet_ping) => {
                                socket
                                    .send_to(
                                        &encode(ConnectedPong::from(packet_ping)),
                                        peer,
                                    )
                                    .await
                                    .expect("Can't send");
                            }
                        }
                
                    }
                }
                PacketClient::Ack(ack) => {
                    println!("{:?}", ack);
                    /* socket
                        .send_to(
                            &encode(ack),
                            peer,
                        )
                        .await
                        .expect("Can't send"); */
                }
            }
            //println!("Peer : {:?}", peer);
        }
    }
}

/* #[test]
fn ack_valid() {
    // let ack = Ack {
    //     record_count: 1,
    //     record: Record(Either::Left(UInt24Le(51))),
    // };
    let i = encode(ack.clone());
    println!("{:?}",i);
    assert_eq!(Ack::read(&mut i.into_iter().skip(1)), Some(ack));
} */