use tokio::net::UdpSocket;

use crate::server::server::Server;

use crate::packets::client::PacketClient;

use crate::packets::server::{open_connection_reply_one::OpenConnectionReplyOne,open_connection_reply_two::OpenConnectionReplyTwo};

use crate::packets::encode;

use crate::packets::server::pong_packet::PongPacket;

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

        loop {
            let (size, peer) = socket.recv_from(&mut buf).await?;

            if let 0x80..=0x8D = buf[0] {
                println!("buffer : {:?}", &buf[..size]);
            }
            let mut iter = buf.clone().into_iter().take(size);

            match PacketClient::parse_packet(&mut iter).unwrap() {
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
                    println!("{:?}",packet);
                }
            }
            //println!("Peer : {:?}", peer);
        }
    }
}
