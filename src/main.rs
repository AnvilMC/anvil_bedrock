#![feature(arbitrary_enum_discriminant)]
#![allow(clippy::module_inception)]

use server::{network::server::NetworkManager, server::Server};
use std::error::Error;
use tokio::net::UdpSocket;

mod packets;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "0.0.0.0:19132".to_string();

    let socket = UdpSocket::bind(&addr).await?;
    println!("Listening on: {}", socket.local_addr()?);

    let server = NetworkManager {
        socket,
        server_info: Server::default(),
    };

    server.run().await?;

    Ok(())
}
