#![feature(exclusive_range_pattern)]
#![feature(min_const_generics)]

mod player;
use std::time::Duration;

pub use player::*;

mod server;
pub use server::*;

pub async fn main() {
    let mut server: Server<1024> = Server::new("Anvil test", 10, ([0; 4], 19132)).await;
    loop {
        server.tick_network().await;
        std::thread::sleep(Duration::from_millis(100));
    }
}

// J'eusse déclamé quand nous aillâmes chercher notre pitance que les marauds n'agréent point l'estime qu'on leur adjoint.
