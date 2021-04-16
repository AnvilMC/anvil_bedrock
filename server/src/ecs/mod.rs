use std::{net::SocketAddr, sync::Arc};

use crossbeam::{
    channel::{Receiver, Sender},
    queue::{ArrayQueue, SegQueue},
};
use mcpe_protocol::{
    prelude::{
        BlockVec3, ChunkRadiusUpdated, Difficulty, GameMode, GameRule, Le, ResourcePackStack,
        ResourcePacksInfo, StartGamePacket, UnsignedVarInt, UnsignedVarLong, VarInt, VarLong,
        Vec3f, VecIndexed, BIOME_DEFINITION_LIST, LOGIN_FAILED_CLIENT, LOGIN_SUCCESS, PLAYER_SPAWN,
    },
    GAME_VERSION, PROTOCOL_VERSION,
};
use specs::{prelude::*, shred::Fetch, storage::MaskedStorage};

use std::collections::HashMap;

use crate::{GamePacketSendablePacket, ReceivablePacket};

// mod uid;
// pub use uid::*;

#[derive(Debug)]
pub struct Position(Vec3f);

#[derive(Debug)]
pub struct Rotation(f32, f32);

#[derive(Debug)]
pub struct IpAdress(SocketAddr);

#[derive(Debug)]
pub struct NetworkSender(Arc<Sender<GamePacketSendablePacket>>);

impl Component for NetworkSender {
    type Storage = VecStorage<Self>;
}

#[derive(Default)]
pub struct EntityPlayer;

impl Component for EntityPlayer {
    type Storage = NullStorage<Self>;
}

#[derive(Debug)]
pub struct PlayerInfo {
    pub username: String,
    pub uuid: String,
}

#[derive(Debug)]
pub struct PlayerState {
    gamemode: GameMode,
}

impl Component for PlayerState {
    type Storage = VecStorage<Self>;
}

impl Component for Position {
    type Storage = VecStorage<Self>;
}

impl Component for IpAdress {
    type Storage = VecStorage<Self>;
}

impl Component for PlayerInfo {
    type Storage = VecStorage<Self>;
}

impl Component for Rotation {
    type Storage = VecStorage<Self>;
}

#[derive(Clone, Debug)]
pub struct WorldSpawn(BlockVec3);

pub fn create_start_game_packet(ecs: &EcsWorld, entity: Entity) -> StartGamePacket {
    let dif = ecs.world.read_resource::<Difficulty>();
    let spawn = ecs.world.read_resource::<WorldSpawn>();

    let (yaw, pitch) = {
        let rot = ecs.world.read_component::<Rotation>();
        let rot = rot.get(entity).unwrap();
        (rot.0, rot.1)
    };

    StartGamePacket {
        entity_unique_id: VarLong(entity.id() as i64),
        entity_runtime_id: UnsignedVarLong(entity.id() as u64),
        player_gamemode: ecs
            .world
            .read_component::<PlayerState>()
            .get(entity)
            .unwrap()
            .gamemode
            .clone(),
        spawn: ecs
            .world
            .read_component::<Position>()
            .get(entity)
            .unwrap()
            .0
            .clone()
            .into(),
        yaw: Le(yaw),
        pitch: Le(pitch),
        seed: VarInt(150),
        spawn_biome_type: Le(0),
        custom_biome_name: "plains".to_owned(),
        dimension: VarInt(0),
        generator: VarInt(0),
        world_game_mode: VarInt(1),
        difficulty: (*dif).clone(),
        world_spawn: spawn.0.clone().into(),
        has_achievements_disabled: true,
        day_cycle_stop_time: VarInt(0),
        edu_edition_offer: VarInt(0),
        has_edu_features_enabled: false,
        edu_production_id: "".to_owned(),
        rain_level: Le(0.0),
        lightning_level: Le(0.0),
        has_confirmed_platform_locked_content: false,
        multiplayer_game: true,
        broadcast_to_lan: false,
        xbl_broadcast_intent: VarInt(0),
        platform_broadcast_intent: VarInt(0),
        commands_enabled: true,
        is_texture_packs_required: false,
        game_rules: VecIndexed::from(vec![GameRule::ShowCoordinates(true)]),
        _unknown1: Le(0),
        _unknown2: false,
        bonus_chest: false,
        has_start_with_map_enabled: false,
        permission_level: VarInt(1),
        server_chunk_tick_range: Le(8),
        has_locked_behavior_pack: false,
        has_locked_resource_pack: false,
        is_from_locked_world_template: false,
        is_using_msa_gamertags_only: false,
        is_from_world_template: false,
        is_world_template_option_locked: false,
        is_only_spawning_v1_villagers: false,
        vanilla_version: GAME_VERSION.to_owned(),
        limited_world_width: 10000,
        limited_world_height: 255,
        is_nether_type: false,
        is_force_experimental_gameplay: false,
        level_id: "1".to_string(),
        world_name: "Anvil_world".to_owned(),
        premium_world_template_id: "".to_owned(),
        is_trial: false,
        is_movement_server_authoritative: false,
        _unknown3: VarInt(0),
        _unknown4: false,
        current_tick: Le(120),
        enchantment_seed: VarInt(548541185),
        _unknown5: UnsignedVarInt(0),
        item_data_palette: VecIndexed::from(vec![]),
        multiplayer_correlation_id: "".to_owned(),
        is_inventory_server_authoritative: false,
    }
}

pub type PacketRec = (
    SocketAddr,
    (ReceivablePacket, Arc<Sender<GamePacketSendablePacket>>),
);

pub struct EcsWorld {
    pub world: World,
    pub player_packets_rc: Receiver<PacketRec>,
    pub entity_map: HashMap<SocketAddr, Entity>,
    pub entities: HashMap<u32, Entity>,
}

impl EcsWorld {
    pub fn new(player_packets_rc: Receiver<PacketRec>) -> Self {
        let mut world = World::new();
        world.register::<Position>();
        world.register::<PlayerInfo>();
        world.register::<Rotation>();
        world.register::<EntityPlayer>();
        world.register::<NetworkSender>();
        world.register::<IpAdress>();
        world.register::<PlayerState>();
        world.insert("KAYAK".to_owned());
        world.insert(GameMode::Creative);
        world.insert(Difficulty::Peacefull);
        world.insert(WorldSpawn((0, 1, 0).into()));
        Self {
            world,
            player_packets_rc,
            entity_map: HashMap::new(),
            entities: HashMap::new(),
        }
    }

    pub fn get_name<'a>(&'a self) -> Fetch<String> {
        self.world.read_resource::<String>()
    }

    pub fn get_entity_from_adress(&self, adress: &SocketAddr) -> Entity {
        *self.entity_map.get(adress).unwrap()
    }

    pub fn send_packet(&self, address: &SocketAddr, packet: GamePacketSendablePacket) {
        let entity = self.get_entity_from_adress(address);
        let comp = self.world.read_component::<NetworkSender>();
        comp.get(entity).unwrap().0.send(packet).unwrap();
    }

    pub fn handle_packets(&mut self) {
        while let Ok((adress, (receivable, network_sender))) = self.player_packets_rc.try_recv() {
            match receivable {
                ReceivablePacket::RequestChunkRadiusPacket(e) => {
                    let radius = 3.max(e.radius.0.min(10));
                    network_sender.send(GamePacketSendablePacket::ChunkRadiusUpdated(
                        ChunkRadiusUpdated {
                            radius: VarInt(radius),
                        },
                    ));
                }
                ReceivablePacket::TickSyncPacket(e) => {
                    network_sender.send(GamePacketSendablePacket::TickSyncPacket(e));
                }
                ReceivablePacket::LoginPacket(e) => {
                    if e.protocol_version == PROTOCOL_VERSION {
                        println!("Test1");
                        let e = self
                            .world
                            .create_entity()
                            .with(Position((0.0, 0.0, 0.0).into()))
                            .with(PlayerInfo {
                                username: e.display_name,
                                uuid: e.identity,
                            })
                            .with(NetworkSender(network_sender.clone()))
                            .with(IpAdress(adress.clone()))
                            .with(Rotation(0.0, 0.0))
                            .with(PlayerState {
                                gamemode: GameMode::Creative,
                            })
                            .with(EntityPlayer)
                            .build();
                        self.entities.insert(e.id(), e.clone());
                        self.entity_map.insert(adress, e);

                        network_sender.send(GamePacketSendablePacket::PlayStatus(LOGIN_SUCCESS));
                        network_sender.send(GamePacketSendablePacket::ResourcePacksInfo(
                            ResourcePacksInfo::default(),
                        ));
                    } else {
                        network_sender
                            .send(GamePacketSendablePacket::PlayStatus(LOGIN_FAILED_CLIENT));
                    }
                }
                ReceivablePacket::ResourcePackClientResponsePacket(e) => {
                    if e.status == 4 {
                        println!("Test2");
                        if let Some(entity) = self.entity_map.get(&adress) {
                            network_sender.send(GamePacketSendablePacket::StartGamePacket(
                                create_start_game_packet(self, *entity),
                            ));
                        }
                        network_sender.send(GamePacketSendablePacket::BiomeDefinitionList(
                            BIOME_DEFINITION_LIST,
                        ));
                        network_sender.send(GamePacketSendablePacket::PlayStatus(PLAYER_SPAWN));
                    } else {
                        network_sender.send(GamePacketSendablePacket::ResourcePackStack(
                            ResourcePackStack::default(),
                        ));
                    }
                }
                ReceivablePacket::PlayerMovePacket(e) => {
                    println!("{:?}", e);
                }
                ReceivablePacket::PlayerActionPacket(e) => {
                    println!("{:?}", e);
                }
            }
        }
    }
}
