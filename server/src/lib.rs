#![feature(exclusive_range_pattern)]
#![feature(min_const_generics)]
#![feature(box_syntax)]
#![feature(once_cell)]

mod player;

use std::time::Duration;

pub use player::*;

mod server;
pub use server::*;

pub async fn main() {
    PLAYER_JOIN_EVENT.register_async(|_wm, event| {
        println!("JOINEVENT");
        let mut s = event.entity.clone();
        tokio::spawn(async move {
            std::thread::sleep(Duration::from_secs(6));
            s.teleport((10., 15., 10.).into(), 0.0, 0.0)
        });
    });
    let mut server: Server<{ 1024 * 10 }> = Server::new("Anvil test", 10, ([0; 4], 19132)).await;
    loop {
        server.tick_network().await;
    }
}

// J'eusse déclamé quand nous aillâmes chercher notre pitance que les marauds n'agréent point l'estime qu'on leur adjoint.
