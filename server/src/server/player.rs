use std::{cell::RefCell, net::SocketAddr, rc::Rc, sync::Arc};

use crossbeam::channel::Sender;
use mcpe_protocol::{
    prelude::{
        Difficulty, GameMode, GameRule, Le, PlayerMoveMode, PlayerMovePacket, StartGamePacket,
        UnsignedVarInt, UnsignedVarLong, VarInt, VarLong, Vec3f, VecIndexed,
    },
    GAME_VERSION,
};

use crate::{get_uuid, GamePacketSendablePacket};

#[derive(Clone)]
pub struct EntityPlayer {
    pub uuid: String,
    pub username: String,
    pub position: Vec3f,
    pub gamemode: GameMode,
    pub yaw: f32,
    pub pitch: f32,
    pub eid: u64,
    pub sender: Arc<Sender<GamePacketSendablePacket>>,
    pub socket_adress: SocketAddr,
    pub world_difficulty: Rc<RefCell<Difficulty>>,
}

impl EntityPlayer {
    pub fn new(
        uuid: String,
        username: String,
        sender: Arc<Sender<GamePacketSendablePacket>>,
        socket_adress: SocketAddr,
        world_difficulty: Rc<RefCell<Difficulty>>,
    ) -> Self {
        Self {
            uuid,
            username,
            gamemode: GameMode::Creative,
            position: Vec3f(Le(0.0), Le(0.0), Le(0.0)),
            yaw: 0.0,
            pitch: 0.0,
            eid: get_uuid(),
            sender,
            socket_adress,
            world_difficulty,
        }
    }

    pub fn build_start_game(&self) -> StartGamePacket {
        StartGamePacket {
            entity_unique_id: VarLong(self.eid as i64),
            entity_runtime_id: UnsignedVarLong(self.eid),
            player_gamemode: self.gamemode,
            spawn: (0., 0., 0.).into(),
            yaw: Le(0.),
            pitch: Le(0.),
            seed: VarInt(150),
            spawn_biome_type: Le(0),
            custom_biome_name: "plains".to_owned(),
            dimension: VarInt(0),
            generator: VarInt(0),
            world_game_mode: VarInt(1),
            difficulty: self.world_difficulty.as_ref().clone(),
            world_spawn: (0, 1, 0).into(),
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

    pub fn set_pos(&mut self, position: Vec3f, yaw: f32, pitch: f32) {
        self.position = position;
        self.yaw = yaw;
        self.pitch = pitch;
        self.send_pos(PlayerMoveMode::Normal);
    }

    pub fn teleport(&mut self, position: Vec3f, yaw: f32, pitch: f32) {
        self.position = position;
        self.yaw = yaw;
        self.pitch = pitch;
        self.send_pos(PlayerMoveMode::Teleport);
    }

    pub fn send_pos(&self, mode: PlayerMoveMode) {
        self.sender
            .send(GamePacketSendablePacket::PlayerMovePacket(
                PlayerMovePacket::new(mode, self.position.clone(), self.pitch, self.yaw, self.eid),
            ))
            .unwrap();
    }
}
