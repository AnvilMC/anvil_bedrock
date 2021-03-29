#![feature(exclusive_range_pattern)]

use std::{borrow::Cow, net::SocketAddr};

use raknet::prelude::*;
use tokio::net::UdpSocket;

#[tokio::main]
async fn main() {

    let manager = NetworkManager::new().await;
    manager.run().await.unwrap();
    println!("Hello, world!");
}

pub struct Server {
    computed_motd: RaknetString,
    motd: String,
    server_uid: i64,
    worlds: Vec<World>,
    players: Vec<Player>
}

pub struct World {
    name: String,
}

pub struct Player {

}

impl Server {
    fn new(motd: &str) -> Self {
        let mut tmp = Self {
            computed_motd: RaknetString(vec![]),
            server_uid: 66742570745275,
            worlds: vec![World {
                name: "Malou is here!".to_owned()
            }],
            players: vec![Player{},Player{}],
            motd: motd.to_owned()
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
        ).as_str().into();
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
            socket: UdpSocket::bind::<SocketAddr>(([0;4],19132).into()).await.unwrap(),
        }
    }
    pub async fn run(self) -> Result<(), std::io::Error> {
        let NetworkManager {
            socket,
            server_info,
        } = self;
        let mut buf: Vec<u8> = vec![0; 1024 * 1024];

        loop {
            let (size, peer) = socket.recv_from(buf.as_mut_slice()).await?;

            //println!("{:?}",&buf[..size]);
            let mut iter = buf.iter().take(size);

            match *Iterator::next(&mut iter).unwrap() {
                1..3 => {
                    let packet_phoenix = UnconnectedPing::decode(&mut iter).unwrap();
                    
                    send(&mut buf, &peer, &socket, UnconnectedPong {
                        time: packet_phoenix.time,
                        server_guid: server_info.server_uid,
                        magic: packet_phoenix.magic,
                        server_id_string: Cow::Borrowed(&server_info.computed_motd),
                    }).await.unwrap();
                },
                e => {
                    println!("Où allons nous? A la plage! {}",e);
                }
            }
            
            //println!("Peer : {:?}", peer);
        }
    }
}

// J'eusse déclamé quand nous aillâmes chercher notre pitance que les marauds n'agréent point l'estime qu'on leur accorde.

async fn send(buf: &mut Vec<u8>,peer: &SocketAddr, socket: &UdpSocket, packet: impl RaknetPacket) -> Option<()> {
    buf.clear();
    buf.push(packet.id());
    packet.encode(buf)?;
    socket.send_to(buf, peer).await.ok()?;
    Some(())
}