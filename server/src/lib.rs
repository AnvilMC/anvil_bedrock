#![feature(exclusive_range_pattern)]
#![feature(min_const_generics)]

use std::{borrow::Cow, collections::HashMap, net::SocketAddr, time::Duration};

use mcpe_protocol::prelude::{
    AvailableCommandsPacket, AvailableEntityIdentifiersPacket, ByteArray, ChunkRadiusUpdated,
    CreativeContentPacket, InventoryContentPacket, LevelChunkPacket, LoginPacket, MCPEPacket,
    MCPEPacketData, MCPEPacketDataError, RequestChunkRadiusPacket, ResourcePackStack,
    ResourcePacksInfo, SetTimePacket, StartGamePacket, TickSyncPacket, UnsignedVarInt, UpdateBlock,
    VarInt, ADVENTURE_SETTINGS, AVAILABLE_ENTITY_IDENTIFIERS_PACKET, BIOME_DEFINITION_LIST,
    LOGIN_SUCCESS, PLAYER_SPAWN,
};
use raknet::prelude::*;
use tokio::net::UdpSocket;

mod player;
pub use player::*;

mod server;
pub use server::*;

pub async fn main() {
    let mut server: Server<{ 1024 }> = Server::new("Anvil test", 10, ([0; 4], 19132)).await;
    loop {
        server.tick_network().await;
        std::thread::sleep(Duration::from_millis(100));
    }
    // let manager = NetworkManager::new().await;
    // manager.run().await.unwrap();
}

// pub struct Server {
//     computed_motd: RaknetString,
//     motd: String,
//     server_uid: i64,
//     worlds: Vec<World>,
//     players: Vec<Player>,
// }

// pub struct World {
//     name: String,
//     chunks: HashMap<i64, ChunkSection>,
// }

// pub fn get_chunk_id_from_coords(x: i32, z: i32) -> i64 {
//     ((x as i64) << 32) | z as i64
// }

// impl World {
//     pub fn get_chunk_section(&self, x: i32, z: i32) -> Option<&ChunkSection> {
//         self.chunks.get(&get_chunk_id_from_coords(x, z))
//     }
//     pub fn get_chunk_section_mut(&mut self, x: i32, z: i32) -> Option<&mut ChunkSection> {
//         self.chunks.get_mut(&get_chunk_id_from_coords(x, z))
//     }
//     pub fn insert_chunk_section(
//         &mut self,
//         x: i32,
//         z: i32,
//         section: ChunkSection,
//     ) -> Option<ChunkSection> {
//         self.chunks.insert(get_chunk_id_from_coords(x, z), section)
//     }
//     pub fn remove_chunk_section(&mut self, x: i32, z: i32) -> Option<ChunkSection> {
//         self.chunks.remove(&get_chunk_id_from_coords(x, z))
//     }

//     pub fn get_block(&mut self, x: i32, y: u8, z: i32) -> Option<&Block> {
//         let chunk = self.get_chunk_section_mut(x >> 4, z >> 4)?;
//         chunk.get_block((x & 0xF) as u8, y, (z & 0xF) as u8)
//     }
//     pub fn get_block_mut(&mut self, x: i32, y: u8, z: i32) -> Option<&mut Block> {
//         let chunk = self.get_chunk_section_mut(x >> 4, z >> 4)?;
//         chunk.get_block_mut((x & 0xF) as u8, y, (z & 0xF) as u8)
//     }
//     pub fn remove_block(&mut self, x: i32, y: u8, z: i32) -> Option<Block> {
//         let chunk = self.get_chunk_section_mut(x >> 4, z >> 4)?;
//         chunk.remove_block((x & 0xF) as u8, y, (z & 0xF) as u8)
//     }
//     pub fn insert_block(&mut self, x: i32, y: u8, z: i32, block: Block) -> Option<Block> {
//         let chunk = self.get_chunk_section_mut(x >> 4, z >> 4)?;
//         chunk.insert_block((x & 0xF) as u8, y, (z & 0xF) as u8, block)
//     }
// }

// pub fn get_block_pos_from_coords_in_chunk(x: u8, y: u8, z: u8) -> u16 {
//     ((x & 0xF) << 4 | (z & 0xF)) as u16 | y as u16
// }

// #[derive(Default)]
// pub struct ChunkSection {
//     blocks: HashMap<u16, Block>,
// }

// impl ChunkSection {
//     pub fn get_block(&mut self, x: u8, y: u8, z: u8) -> Option<&Block> {
//         self.blocks
//             .get(&get_block_pos_from_coords_in_chunk(x, y, z))
//     }
//     pub fn get_block_mut(&mut self, x: u8, y: u8, z: u8) -> Option<&mut Block> {
//         self.blocks
//             .get_mut(&get_block_pos_from_coords_in_chunk(x, y, z))
//     }
//     pub fn remove_block(&mut self, x: u8, y: u8, z: u8) -> Option<Block> {
//         self.blocks
//             .remove(&get_block_pos_from_coords_in_chunk(x, y, z))
//     }
//     pub fn insert_block(&mut self, x: u8, y: u8, z: u8, block: Block) -> Option<Block> {
//         self.blocks
//             .insert(get_block_pos_from_coords_in_chunk(x, y, z), block)
//     }
// }

// pub struct Block {
//     material_id: u32,
// }

// pub struct Player {}

// impl Server {
//     fn new(motd: &str) -> Self {
//         let mut tmp = Self {
//             computed_motd: RaknetString(vec![]),
//             server_uid: 66742570745275,
//             worlds: vec![World {
//                 name: "Malou is here!".to_owned(),
//                 chunks: HashMap::new(),
//             }],
//             players: vec![Player {}, Player {}],
//             motd: motd.to_owned(),
//         };
//         tmp.update_motd();
//         tmp
//     }

//     fn update_motd(&mut self) {
//         self.computed_motd = format!(
//             "MCPE;{};354;1.11;{};{};{};{};Survival",
//             self.motd,
//             self.players.len(),
//             self.players.capacity(),
//             self.server_uid,
//             self.worlds[0].name
//         )
//         .as_str()
//         .into();
//     }
// }

// pub struct NetworkManager {
//     pub server_info: Server,
//     pub socket: UdpSocket,
// }

// impl NetworkManager {
//     async fn new() -> Self {
//         println!("TEST LOADED");
//         Self {
//             server_info: Server::new("The first Rust bedrock implementation!"),
//             socket: UdpSocket::bind::<SocketAddr>(([0; 4], 19132).into())
//                 .await
//                 .unwrap(),
//         }
//     }

//     pub async fn run(self) -> Result<(), std::io::Error> {
//         let NetworkManager {
//             socket,
//             server_info,
//         } = self;

//         let mut buf = vec![0; 1024 * 1024];
//         let mut buf_write = vec![0; 1024 * 1024];

//         let mut frame_manager = FrameManager::default();

//         let mut players =

//         loop {
//             let (size, peer) = socket.recv_from(buf.as_mut_slice()).await?;

//             let mut iter = buf.iter().take(size);

//             match *Iterator::next(&mut iter).unwrap() {
//                 1..3 => {
//                     let packet_phoenix = UnconnectedPing::decode(&mut iter).unwrap();

//                     send(
//                         &mut buf_write,
//                         &peer,
//                         &socket,
//                         UnconnectedPong {
//                             time: packet_phoenix.time,
//                             server_guid: server_info.server_uid,
//                             magic: packet_phoenix.magic,
//                             server_id_string: Cow::Borrowed(&server_info.computed_motd),
//                         },
//                     )
//                     .await
//                     .unwrap();
//                 }
//                 0x05 => {
//                     let packet_phoenix = OpenConnectionRequestOne::decode(&mut iter).unwrap();

//                     send(
//                         &mut buf_write,
//                         &peer,
//                         &socket,
//                         OpenConnectionReplyOne::from(&packet_phoenix, server_info.server_uid),
//                     )
//                     .await
//                     .unwrap();
//                 }
//                 0x07 => {
//                     let packet_phoenix = OpenConnectionRequestTwo::decode(&mut iter).unwrap();

//                     frame_manager.set_mtu(packet_phoenix.mtu);

//                     send(
//                         &mut buf_write,
//                         &peer,
//                         &socket,
//                         OpenConnectionReplyTwo::from(
//                             &packet_phoenix,
//                             &peer,
//                             server_info.server_uid,
//                         ),
//                     )
//                     .await
//                     .unwrap();
//                 }
//                 0x80..0x8E => {
//                     let frame = FramePacket::decode(&mut iter).unwrap();
//                 }
//                 e => {
//                     println!("Où allons nous? A la plage! {}", e);
//                 }
//             }
//         }
//     }
// }

// J'eusse déclamé quand nous aillâmes chercher notre pitance que les marauds n'agréent point l'estime qu'on leur adjoint.
