use std::{borrow::Cow, collections::HashMap};
use std::{net::SocketAddr, sync::Arc};

use crate::*;
use crossbeam::channel::{unbounded, Receiver, Sender};
use raknet::prelude::{
    FramePacket, OpenConnectionReplyOne, OpenConnectionReplyTwo, OpenConnectionRequestOne,
    OpenConnectionRequestTwo, RaknetPacketData, RaknetString, UnconnectedPing, UnconnectedPong,
};
use tokio::net::UdpSocket;

use crate::NetworkPlayer;

mod world;
pub use world::*;

pub const ANVIL_VERSION: &'static str = "Anvil MCBE Alpha 0.0.0";

pub struct Server<const MAX_PACKET_SIZE: usize> {
    pub network_players: HashMap<SocketAddr, NetworkPlayer>,
    pub server_uid: i64,
    pub udp_socket: Arc<UdpSocket>,
    writer_buf: Vec<u8>,
    read_buf: [u8; MAX_PACKET_SIZE],
    computed_motd: RaknetString,
    pub worlds: Vec<World>,
    pub motd: String,
    player_handler_r: Receiver<EntityPlayer>,
    player_handler_s: Arc<Sender<EntityPlayer>>,
}

impl<const MAX_PACKET_SIZE: usize> Server<MAX_PACKET_SIZE> {
    pub async fn new(
        motd: &str,
        max_players: usize,
        adress: impl Into<SocketAddr>,
    ) -> server::Server<MAX_PACKET_SIZE> {
        let (s, r) = unbounded();
        Self {
            computed_motd: format!(
                "MCPE;{};354;1.11;{};{};{};{};Survival",
                motd,
                0,
                max_players,
                66742570745275i64,
                "AnvilWorld".to_owned()
            )
            .as_str()
            .into(),
            server_uid: 66742570745275,
            network_players: HashMap::with_capacity(max_players),
            motd: motd.to_owned(),
            udp_socket: Arc::new(UdpSocket::bind(adress.into()).await.unwrap()),
            writer_buf: Vec::with_capacity(MAX_PACKET_SIZE),
            read_buf: [0; MAX_PACKET_SIZE],
            worlds: vec![World {
                name: "AnvilWorld".to_owned(),
                player_entities: Vec::new(),
            }],
            player_handler_r: r,
            player_handler_s: Arc::new(s),
        }
    }

    pub fn update_motd(&mut self) {
        self.computed_motd = format!(
            "MCPE;{};354;1.11;{};{};{};{};Survival",
            self.motd,
            self.network_players.len(),
            self.network_players.capacity(),
            self.server_uid,
            self.worlds[0].name
        )
        .as_str()
        .into();
    }

    pub fn get_player_from_addr(&self, peer: &SocketAddr) -> Option<&NetworkPlayer> {
        self.network_players.get(peer)
    }

    pub fn get_player_from_addr_mut(&mut self, peer: &SocketAddr) -> Option<&mut NetworkPlayer> {
        self.network_players.get_mut(peer)
    }

    pub fn remove_player(&mut self, peer: &SocketAddr) -> Option<NetworkPlayer> {
        self.network_players.remove(peer)
    }

    pub fn add_player(&mut self, peer: SocketAddr, player: NetworkPlayer) {
        self.network_players.insert(peer, player);
    }

    pub async fn tick_network(&mut self) {
        for x in self.network_players.values_mut() {
            if let Err(e) = x.handle_send_packet().await {
                println!("Error while sending packet : {}", e);
            }
        }
        while let Ok((size, peer)) = self.udp_socket.try_recv_from(&mut self.read_buf) {
            let mut iter = self.read_buf.iter().take(size);
            match *Iterator::next(&mut iter).unwrap() {
                1..3 => {
                    let packet_phoenix = UnconnectedPing::decode(&mut iter).unwrap();

                    send(
                        &mut self.writer_buf,
                        &peer,
                        &self.udp_socket,
                        UnconnectedPong {
                            time: packet_phoenix.time,
                            server_guid: self.server_uid,
                            magic: packet_phoenix.magic,
                            server_id_string: Cow::Borrowed(&self.computed_motd),
                        },
                    )
                    .await
                    .unwrap();
                }
                0x05 => {
                    if !self.network_players.contains_key(&peer) {
                        let packet_phoenix = OpenConnectionRequestOne::decode(&mut iter).unwrap();

                        println!("MTU: {}", packet_phoenix.mtu.len());
                        send(
                            &mut self.writer_buf,
                            &peer,
                            &self.udp_socket,
                            OpenConnectionReplyOne::from(&packet_phoenix, self.server_uid),
                        )
                        .await
                        .unwrap();
                    } else {
                        println!("Already logged in!");
                    }
                }
                0x07 => {
                    if !self.network_players.contains_key(&peer) {
                        let packet_phoenix = OpenConnectionRequestTwo::decode(&mut iter).unwrap();
                        println!("MTU1: {}", packet_phoenix.mtu);
                        let player = NetworkPlayer::new(
                            packet_phoenix.mtu,
                            self.udp_socket.clone(),
                            peer.clone(),
                            self.player_handler_s.clone(),
                        );
                        self.network_players.insert(peer.clone(), player);

                        send(
                            &mut self.writer_buf,
                            &peer,
                            &self.udp_socket,
                            OpenConnectionReplyTwo::from(&packet_phoenix, &peer, self.server_uid),
                        )
                        .await
                        .unwrap();
                    } else {
                        println!("Already connected");
                    }
                }
                0x80..0x8E => {
                    if let Some(player) = self.network_players.get_mut(&peer) {
                        let frame = FramePacket::decode(&mut iter).unwrap();
                        if let Err(e) = player.handle_frame_receive(frame).await {
                            println!("Error while receiving frame {}", e);
                        }
                    } else {
                        println!("Peer not correctly connected");
                    }
                }
                e => {
                    println!("Où allons nous? A la plage! {}", e);
                }
            }
        }
        while let Ok(e) = self.player_handler_r.try_recv() {
            self.worlds[0].player_entities.push(e);
        }
    }
}
