#![feature(exclusive_range_pattern)]

use std::{borrow::Cow, net::SocketAddr};

use mcpe_protocol::prelude::{ByteArray, LoginPacket, MCPEPacket, MCPEPacketData, UnsignedVarInt};
use raknet::prelude::*;
use tokio::net::UdpSocket;

pub async fn main() {
    let manager = NetworkManager::new().await;
    manager.run().await.unwrap();
}

pub struct Server {
    computed_motd: RaknetString,
    motd: String,
    server_uid: i64,
    worlds: Vec<World>,
    players: Vec<Player>,
}

pub struct World {
    name: String,
}

pub struct Player {}

impl Server {
    fn new(motd: &str) -> Self {
        let mut tmp = Self {
            computed_motd: RaknetString(vec![]),
            server_uid: 66742570745275,
            worlds: vec![World {
                name: "Malou is here!".to_owned(),
            }],
            players: vec![Player {}, Player {}],
            motd: motd.to_owned(),
        };
        tmp.update_motd();
        tmp
    }

    fn update_motd(&mut self) {
        self.computed_motd = format!(
            "MCPE;{};354;1.11;{};{};{};{};Survival",
            self.motd,
            self.players.len(),
            self.players.capacity(),
            self.server_uid,
            self.worlds[0].name
        )
        .as_str()
        .into();
    }
}

pub struct NetworkManager {
    pub server_info: Server,
    pub socket: UdpSocket,
}

impl NetworkManager {
    async fn new() -> Self {
        Self {
            server_info: Server::new("The first Rust bedrock implementation!"),
            socket: UdpSocket::bind::<SocketAddr>(([0; 4], 19132).into())
                .await
                .unwrap(),
        }
    }

    pub async fn run(self) -> Result<(), std::io::Error> {
        let NetworkManager {
            socket,
            server_info,
        } = self;

        let mut buf = vec![0; 1024 * 1024];
        let mut buf_write = vec![0; 1024 * 1024];

        let mut frame_manager = FrameManager::default();

        loop {
            let (size, peer) = socket.recv_from(buf.as_mut_slice()).await?;

            let mut iter = buf.iter().take(size);

            match *Iterator::next(&mut iter).unwrap() {
                1..3 => {
                    let packet_phoenix = UnconnectedPing::decode(&mut iter).unwrap();

                    send(
                        &mut buf_write,
                        &peer,
                        &socket,
                        UnconnectedPong {
                            time: packet_phoenix.time,
                            server_guid: server_info.server_uid,
                            magic: packet_phoenix.magic,
                            server_id_string: Cow::Borrowed(&server_info.computed_motd),
                        },
                    )
                    .await
                    .unwrap();
                }
                0x05 => {
                    let packet_phoenix = OpenConnectionRequestOne::decode(&mut iter).unwrap();

                    send(
                        &mut buf_write,
                        &peer,
                        &socket,
                        OpenConnectionReplyOne::from(&packet_phoenix, server_info.server_uid),
                    )
                    .await
                    .unwrap();
                }
                0x07 => {
                    let packet_phoenix = OpenConnectionRequestTwo::decode(&mut iter).unwrap();

                    send(
                        &mut buf_write,
                        &peer,
                        &socket,
                        OpenConnectionReplyTwo::from(
                            &packet_phoenix,
                            &peer,
                            server_info.server_uid,
                        ),
                    )
                    .await
                    .unwrap();
                }
                0x80..0x8E => {
                    let packet_phoenix = FramePacket::decode(&mut iter).unwrap();
                    let (ack, paket) = frame_manager.process(packet_phoenix);

                    if let Some(e) = ack {
                        send(&mut buf_write, &peer, &socket, e).await.unwrap();
                    }
                    if let Some(e) = paket {
                        let mut e = e.iter();
                        match *Iterator::next(&mut e).unwrap() {
                            0x09 => {
                                let packet_phoenix = ConnectionRequest::decode(&mut e).unwrap();

                                send_framed(
                                    &mut frame_manager,
                                    &mut buf_write,
                                    &peer,
                                    &socket,
                                    ConnectionRequestAccepted::from(packet_phoenix, &peer),
                                )
                                .await
                                .unwrap();
                            }
                            0xFE => {
                                let _packet_phoenix = GamePacket::decode(&mut e).unwrap();

                                std::fs::write("game_packet.bin", &_packet_phoenix.0).unwrap();

                                let mut iter = _packet_phoenix.0.iter();

                                while let Some(e) = ByteArray::decode(&mut iter) {
                                    let mut iter = e.0.iter();
                                    let uint = UnsignedVarInt::decode(&mut iter).unwrap().0 & 0x3FF;
                                    match uint {
                                        0x01 => {
                                            let packet: LoginPacket =
                                                LoginPacket::decode(&mut iter).unwrap();

                                            std::fs::write("result.bin", packet.chain_data.0)
                                                .unwrap();

                                            //println!("Login packet ? {:?}", packet);
                                            println!("BALZAC");
                                            //println!("Login packet ?");
                                        }
                                        0x02 => {
                                            println!("CLAVIER FRANCAIS DE MERDE");
                                        }
                                        e => {
                                            println!("Game Packet {}", e);
                                        }
                                    }
                                }

                                // TODO Do things with Game Packet
                            }
                            e => {
                                println!("Nous sommes a la HEC!");
                            }
                        }
                    }
                }
                e => {
                    println!("Où allons nous? A la plage!");
                }
            }
        }
    }
}

// J'eusse déclamé quand nous aillâmes chercher notre pitance que les marauds n'agréent point l'estime qu'on leur adjoint.

async fn send_framed(
    frame: &mut FrameManager,
    buf: &mut Vec<u8>,
    peer: &SocketAddr,
    socket: &UdpSocket,
    packet: impl RaknetPacket,
) -> Option<()> {
    buf.clear();
    let packet = frame.encode_as_frame(packet);
    buf.push(packet.id());
    packet.encode(buf)?;

    socket.send_to(buf, peer).await.ok()?;
    Some(())
}

async fn send(
    buf: &mut Vec<u8>,
    peer: &SocketAddr,
    socket: &UdpSocket,
    packet: impl RaknetPacket,
) -> Option<()> {
    buf.clear();
    buf.push(packet.id());
    packet.encode(buf)?;
    socket.send_to(buf, peer).await.ok()?;
    Some(())
}
