#![feature(exclusive_range_pattern)]

use std::{borrow::Cow, net::SocketAddr};

use mcpe_protocol::prelude::{
    AvailableCommandsPacket, AvailableEntityIdentifiersPacket, ByteArray, ChunkRadiusUpdated,
    CreativeContentPacket, InventoryContentPacket, LevelChunkPacket, LoginPacket, MCPEPacket,
    MCPEPacketData, MCPEPacketDataError, RequestChunkRadiusPacket, ResourcePackStack,
    ResourcePacksInfo, SetTimePacket, StartGamePacket, TickSyncPacket, UnsignedVarInt, VarInt,
    ADVENTURE_SETTINGS, AVAILABLE_ENTITY_IDENTIFIERS_PACKET, BIOME_DEFINITION_LIST, LOGIN_SUCCESS,
    PLAYER_SPAWN,
};
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

                    frame_manager.set_mtu(packet_phoenix.mtu);

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

                                while let Ok(e) = ByteArray::decode(&mut iter) {
                                    let mut iter = e.0.iter();
                                    let uint = UnsignedVarInt::decode(&mut iter).unwrap().0 & 0x3FF;
                                    match uint {
                                        0x01 => {
                                            let packet: LoginPacket =
                                                LoginPacket::decode(&mut iter).unwrap();

                                            send_game_packets(
                                                &mut frame_manager,
                                                &mut buf_write,
                                                &peer,
                                                &socket,
                                                &[LOGIN_SUCCESS],
                                            )
                                            .await
                                            .unwrap();

                                            send_game_packets(
                                                &mut frame_manager,
                                                &mut buf_write,
                                                &peer,
                                                &socket,
                                                &[ResourcePacksInfo::default()],
                                            )
                                            .await
                                            .unwrap();

                                            println!("Login packet ? {:?}", packet);
                                            println!("BALZAC");
                                            //println!("Login packet ?");
                                        }
                                        0x02 => {
                                            println!("CLAVIER FRANCAIS DE MERDE");
                                        }
                                        0x08 => {
                                            let i = *(Iterator::next(&mut iter).unwrap());
                                            println!("Resource pack status {}", i);
                                            if i == 4 {
                                                send_game_packets(
                                                    &mut frame_manager,
                                                    &mut buf_write,
                                                    &peer,
                                                    &socket,
                                                    &[StartGamePacket::new()],
                                                )
                                                .await
                                                .unwrap();

                                                send_game_packets(
                                                    &mut frame_manager,
                                                    &mut buf_write,
                                                    &peer,
                                                    &socket,
                                                    &[CreativeContentPacket::default()],
                                                )
                                                .await
                                                .unwrap();

                                                send_game_packets(
                                                    &mut frame_manager,
                                                    &mut buf_write,
                                                    &peer,
                                                    &socket,
                                                    &[BIOME_DEFINITION_LIST],
                                                )
                                                .await
                                                .unwrap();

                                                send_game_packets(
                                                    &mut frame_manager,
                                                    &mut buf_write,
                                                    &peer,
                                                    &socket,
                                                    &[AVAILABLE_ENTITY_IDENTIFIERS_PACKET],
                                                )
                                                .await
                                                .unwrap();

                                                let paks = (-5..5)
                                                    .map(|x| {
                                                        (-5..5)
                                                            .map(|z| LevelChunkPacket::new(x, z))
                                                            .collect::<Vec<_>>()
                                                    })
                                                    .flatten()
                                                    .collect::<Vec<_>>();

                                                send_game_packets(
                                                    &mut frame_manager,
                                                    &mut buf_write,
                                                    &peer,
                                                    &socket,
                                                    &paks,
                                                )
                                                .await
                                                .unwrap();

                                                send_game_packets(
                                                    &mut frame_manager,
                                                    &mut buf_write,
                                                    &peer,
                                                    &socket,
                                                    &[ADVENTURE_SETTINGS],
                                                )
                                                .await
                                                .unwrap();

                                                send_game_packets(
                                                    &mut frame_manager,
                                                    &mut buf_write,
                                                    &peer,
                                                    &socket,
                                                    &[
                                                        InventoryContentPacket {
                                                            inventory_id: UnsignedVarInt(119),
                                                            slot: UnsignedVarInt(0),
                                                        },
                                                        InventoryContentPacket {
                                                            inventory_id: UnsignedVarInt(120),
                                                            slot: UnsignedVarInt(0),
                                                        },
                                                        InventoryContentPacket {
                                                            inventory_id: UnsignedVarInt(121),
                                                            slot: UnsignedVarInt(0),
                                                        },
                                                        InventoryContentPacket {
                                                            inventory_id: UnsignedVarInt(122),
                                                            slot: UnsignedVarInt(0),
                                                        },
                                                    ],
                                                )
                                                .await
                                                .unwrap();

                                                send_game_packets(
                                                    &mut frame_manager,
                                                    &mut buf_write,
                                                    &peer,
                                                    &socket,
                                                    &[AvailableCommandsPacket::default()],
                                                )
                                                .await
                                                .unwrap();

                                                send_game_packets(
                                                    &mut frame_manager,
                                                    &mut buf_write,
                                                    &peer,
                                                    &socket,
                                                    &[SetTimePacket { time: VarInt(0) }],
                                                )
                                                .await
                                                .unwrap();

                                                send_game_packets(
                                                    &mut frame_manager,
                                                    &mut buf_write,
                                                    &peer,
                                                    &socket,
                                                    &[PLAYER_SPAWN],
                                                )
                                                .await
                                                .unwrap();
                                            } else {
                                                send_game_packets(
                                                    &mut frame_manager,
                                                    &mut buf_write,
                                                    &peer,
                                                    &socket,
                                                    &[ResourcePackStack::default()],
                                                )
                                                .await
                                                .unwrap();
                                            }
                                        }
                                        0x45 => {
                                            let request_chunk_radius_packet =
                                                RequestChunkRadiusPacket::decode(&mut iter)
                                                    .unwrap();
                                            let a =
                                                3.max(request_chunk_radius_packet.radius.0.min(10));
                                            send_game_packets(
                                                &mut frame_manager,
                                                &mut buf_write,
                                                &peer,
                                                &socket,
                                                &[ChunkRadiusUpdated { radius: VarInt(a) }],
                                            )
                                            .await
                                            .unwrap();
                                        }
                                        0x17 => {
                                            let tick = TickSyncPacket::decode(&mut iter).unwrap();
                                            send_game_packets(
                                                &mut frame_manager,
                                                &mut buf_write,
                                                &peer,
                                                &socket,
                                                &[tick],
                                            )
                                            .await
                                            .unwrap();
                                        }
                                        0x9C => {
                                            println!("Violation : {:?}", iter.read_to_end());
                                        }
                                        e => {
                                            println!("Game Packet {}", e);
                                        }
                                    }
                                }

                                // TODO Do things with Game Packet
                            }
                            0x00 => {
                                let ping = ConnectedPing::decode(&mut e).unwrap();
                                send_framed(
                                    &mut frame_manager,
                                    &mut buf_write,
                                    &peer,
                                    &socket,
                                    ConnectedPong::from(ping),
                                )
                                .await
                                .unwrap();
                            }
                            e => {
                                println!("Nous sommes a la HEC! {}", e);
                            }
                        }
                    }
                }
                e => {
                    println!("Où allons nous? A la plage! {}", e);
                }
            }
        }
    }
}

// J'eusse déclamé quand nous aillâmes chercher notre pitance que les marauds n'agréent point l'estime qu'on leur adjoint.

async fn send_game_packets<T: MCPEPacket>(
    frame: &mut FrameManager,
    buf: &mut Vec<u8>,
    peer: &SocketAddr,
    socket: &UdpSocket,
    packet: &[T],
) -> Result<(), MCPEPacketDataError> {
    buf.clear();
    let mut buffer = Vec::with_capacity(1024 * 1024);
    for i in packet {
        buffer.push(T::PACKET_ID);
        i.encode(&mut buffer)?;
        ByteArray::from(buffer).encode(buf)?;
        buffer = Vec::with_capacity(1024 * 1024);
    }
    let game_packet = GamePacket(buf.clone());
    send_framed(frame, buf, peer, socket, game_packet).await?;
    Ok(())
}

async fn send_framed(
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

async fn send(
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

// Server client communication (Login process after ClientToServerHandshake)
//  0x02 : PlayStatus (0) - S > C
//  0x81 : ClientCacheStatus (bool: true, UNKNOWN 1 byte (Probably bool: true)) - C > S
//  Group send: S > C
//    0x28 : SetEntityMotion (VarLong (EntityId), MotionX (Vec3), MotionY: float, MotionZ: float)
//    0x27 : SetEntityData (VarLong (EntityId), Metadata Dictionary)
//    0x27 : SetEntityData (VarLong (EntityId), Metadata Dictionary)
//    0x27 : SetEntityData (VarLong (EntityId), Metadata Dictionary)
//    0x27 : SetEntityData (VarLong (EntityId), Metadata Dictionary)
//    0x06 : ResourcePackInfo (bool: accept, bool: script, size1: Le<u16>, size2: Le<u16>)
//  Group send end
//  0x08 : ResourcePackResponse (status: byte, packIds: ResourcePackIds) - C > S
//  0x07 : ResourcePackStack () - S > C
//  0x08 : ResourcePackResponse (status: byte, packIds: ResourcePackIds) - C > S
//  Group send: S > C
//    0x0B : StartGame
//    0x7A : BiomeDefinitionList
//    0x77 : AvailableEntityIdentifiers
//    0x91 : CreativeContent
//    0x37 : AdventureSettings
//    0x1D : UpdateAttributes
//    0x27 : SetEntityData
//    0x0A : SetTime
//    0x1D : UpdateAttributes
//    0x27 : SetEntityData
//    0x27 : SetEntityData
//    0x27 : SetEntityData
//    0x27 : SetEntityData
//    0x27 : SetEntityData
//    0x27 : SetEntityData
//  Group send end

#[test]
fn decoder() {
    fn from_hex(i: u8) -> u8 {
        match i {
            b'0'..=b'9' => i - b'0',
            b'A'..=b'F' => i - b'A' + 10,
            b'a'..=b'f' => i - b'a' + 10,
            _ => panic!("WINDOZE FATALE ERREAURE"),
        }
    }
    let file = std::fs::read_to_string("decode.hex").unwrap();
    for (x, line) in file.lines().enumerate() {
        let bin = line
            .as_bytes()
            .chunks(2)
            .map(|x| from_hex(x[0]) * 16 + from_hex(x[1]))
            .collect::<Vec<u8>>();
        println!("-----------        LINE {}          ---------", x);
        let bin = bin[bin.iter().position(|x| *x == 0xFE).unwrap() + 1..].to_vec();
        let mut paket = GamePacket::decode(&mut bin.iter()).unwrap().0;
        let mut iter = paket.iter();
        let mut p = 0;
        while let Some(e) = ByteArray::decode(&mut iter) {
            p += 1;
            println!("{:02X} {:?}", e.0[0], &e.0[0..e.0.len().min(25)]);
            //std::fs::write(format!("{}.{}.bin", x, p), e.0).unwrap();
        }
        println!("--------------------------------------------");
    }
}
