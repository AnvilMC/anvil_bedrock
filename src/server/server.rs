use crate::packets::objects::server_guid::ServerGUID;

use super::{player::Player, world::World};

pub struct Server {
    pub guid: ServerGUID,
    pub serveruid: i64,
    pub motd: String,
    pub players: Vec<Player>,
    pub worlds: Vec<World>,
}

impl Server {
    pub fn new(
        guid: ServerGUID,
        serveruid: i64,
        motd: String,
        max_players: u16,
        worlds: Vec<World>,
    ) -> Self {
        Self {
            worlds,
            players: Vec::with_capacity(max_players as usize),
            guid,
            serveruid,
            motd,
        }
    }

    pub fn build_server_info(&self) -> String {
        format!(
            "MCPE;{};354;1.11;{};{};{};{};Survival",
            self.motd,
            self.players.len(),
            self.players.capacity(),
            self.serveruid,
            self.worlds[0].name
        )
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new(
            ServerGUID(-7147596012246568791),
            8406419495588612579,
            "Welcome on Anvil!".to_owned(),
            u16::MAX,
            vec![World {
                name: "Anvil spawn".to_owned(),
            }],
        )
    }
}
