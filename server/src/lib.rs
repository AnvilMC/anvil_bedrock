#![feature(exclusive_range_pattern)]
#![feature(min_const_generics)]
#![feature(box_syntax)]
#![feature(once_cell)]

mod player;

pub use player::*;

mod server;
pub use server::*;

pub async fn main() {
    PLAYER_JOIN_EVENT.register_async(|wm, event| {
        println!("JOINEVENT");
        let mut s = event.entity.clone();
        tokio::spawn(async move { s.teleport((15., 25., 15.).into(), 10.0, 20.0) });
    });
    let mut server: Server<{ 1024 * 10 }> = Server::new("Anvil test", 10, ([0; 4], 19132)).await;
    loop {
        server.tick_network().await;
    }
}

// J'eusse déclamé quand nous aillâmes chercher notre pitance que les marauds n'agréent point l'estime qu'on leur adjoint.
