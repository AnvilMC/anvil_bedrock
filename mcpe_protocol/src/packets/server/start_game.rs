use crate::prelude::{
    Le, MCPEPacketData, MCPEPacketDataError, UnsignedVarInt, UnsignedVarLong, VarInt, VarLong,
    VecIndexed,
};
use packet_derive::{packet, MCPEPacketDataAuto};

#[derive(Debug, MCPEPacketDataAuto)]
pub struct Vec3f(Le<f32>, Le<f32>, Le<f32>);

impl From<(f32, f32, f32)> for Vec3f {
    fn from(e: (f32, f32, f32)) -> Self {
        Vec3f(Le(e.0), Le(e.1), Le(e.2))
    }
}

#[derive(Debug, MCPEPacketDataAuto)]
pub struct BlockVec3(VarInt, UnsignedVarInt, VarInt);

impl From<(i32, u32, i32)> for BlockVec3 {
    fn from(e: (i32, u32, i32)) -> Self {
        BlockVec3(VarInt(e.0), UnsignedVarInt(e.1), VarInt(e.2))
    }
}

#[derive(Debug)]
pub enum GameRule {
    CommandBlockOutput(bool),
    DoDaylightCycle(bool),
    DoEntityDrops(bool),
    DoFireTick(bool),
    DoImmediateRespawn(bool),
    DoMobLoot(bool),
    DoMobSpawning(bool),
    DoTileDrops(bool),
    DoWeatherCycle(bool),
    DrowningDamage(bool),
    FallDamage(bool),
    FireDamage(bool),
    KeepInventory(bool),
    MobGriefing(bool),
    NaturalRegeneration(bool),
    Pvp(bool),
    RandomTickSpeed(u32),
    SendCommandFeedback(bool),
    ShowCoordinates(bool),
    TntExplodes(bool),
    ShowDeathMessage(bool),
}

impl GameRule {
    fn get_message(&self) -> &'static str {
        match self {
            GameRule::CommandBlockOutput(_) => "commandBlockOutput",
            GameRule::DoDaylightCycle(_) => "doDaylightCycle",
            GameRule::DoEntityDrops(_) => "doEntityDrops",
            GameRule::DoFireTick(_) => "doFireTick",
            GameRule::DoImmediateRespawn(_) => "doImmediateRespawn",
            GameRule::DoMobLoot(_) => "doMobLoot",
            GameRule::DoMobSpawning(_) => "doMobSpawning",
            GameRule::DoTileDrops(_) => "doTileDrops",
            GameRule::DoWeatherCycle(_) => "doWeatherCycle",
            GameRule::DrowningDamage(_) => "drowningDamage",
            GameRule::FallDamage(_) => "fallDamage",
            GameRule::FireDamage(_) => "fireDamage",
            GameRule::KeepInventory(_) => "keepInventory",
            GameRule::MobGriefing(_) => "mobGriefing",
            GameRule::NaturalRegeneration(_) => "naturalRegeneration",
            GameRule::Pvp(_) => "pvp",
            GameRule::RandomTickSpeed(_) => "randomTickSpeed",
            GameRule::SendCommandFeedback(_) => "sendCommandFeedback",
            GameRule::ShowCoordinates(_) => "showCoordinates",
            GameRule::TntExplodes(_) => "tntExplodes",
            GameRule::ShowDeathMessage(_) => "showDeathMessage",
        }
    }
    fn from_message_bool(st: &str, b: bool) -> Result<Self, MCPEPacketDataError> {
        Ok(match st.to_lowercase().as_str() {
            "commandblockoutput" => Self::CommandBlockOutput(b),
            "dodaylightcycle" => Self::DoDaylightCycle(b),
            "doentitydrops" => Self::DoEntityDrops(b),
            "dofiretick" => Self::DoFireTick(b),
            "doimmediaterespawn" => Self::DoImmediateRespawn(b),
            "domobloot" => Self::DoMobLoot(b),
            "domobspawning" => Self::DoMobSpawning(b),
            "dotiledrops" => Self::DoTileDrops(b),
            "doweathercycle" => Self::DoWeatherCycle(b),
            "drowningdamage" => Self::DrowningDamage(b),
            "falldamage" => Self::FallDamage(b),
            "firedamage" => Self::FireDamage(b),
            "keepinventory" => Self::KeepInventory(b),
            "mobgriefing" => Self::MobGriefing(b),
            "naturalregeneration" => Self::NaturalRegeneration(b),
            "pvp" => Self::Pvp(b),
            "sendcommandfeedback" => Self::SendCommandFeedback(b),
            "showcoordinates" => Self::ShowCoordinates(b),
            "tntexplodes" => Self::TntExplodes(b),
            "showdeathmessage" => Self::ShowDeathMessage(b),
            e => {
                return Err(MCPEPacketDataError::new(
                    "GameRule:from_message_bool",
                    format!("Invalid gamerule {}", e),
                ))
            }
        })
    }
}

use crate::traits::PacketReader;

impl MCPEPacketData for GameRule {
    // bool => 1
    // int => 2
    fn decode(e: &mut impl crate::prelude::Reader) -> Result<Self, MCPEPacketDataError> {
        let message: String = e.auto_decode().map_err(|x| x.map("gamerule_message"))?;
        e.skip(1);
        if message.to_lowercase().as_str() == "randomtickspeed" {
            let ui: UnsignedVarInt = e.auto_decode().map_err(|x| x.map("uvint_value"))?;
            Ok(Self::RandomTickSpeed(ui.0))
        } else {
            let ui: bool = e.auto_decode().map_err(|x| x.map("bool_value"))?;
            Self::from_message_bool(&message, ui)
        }
    }

    fn encode(&self, writer: &mut impl crate::prelude::Writer) -> Result<(), MCPEPacketDataError> {
        self.get_message().to_lowercase().encode(writer)?;
        match self {
            GameRule::CommandBlockOutput(e)
            | GameRule::DoDaylightCycle(e)
            | GameRule::DoEntityDrops(e)
            | GameRule::DoFireTick(e)
            | GameRule::DoImmediateRespawn(e)
            | GameRule::DoMobLoot(e)
            | GameRule::DoMobSpawning(e)
            | GameRule::DoTileDrops(e)
            | GameRule::DoWeatherCycle(e)
            | GameRule::DrowningDamage(e)
            | GameRule::FallDamage(e)
            | GameRule::FireDamage(e)
            | GameRule::KeepInventory(e)
            | GameRule::MobGriefing(e)
            | GameRule::NaturalRegeneration(e)
            | GameRule::Pvp(e)
            | GameRule::SendCommandFeedback(e)
            | GameRule::ShowCoordinates(e)
            | GameRule::TntExplodes(e)
            | GameRule::ShowDeathMessage(e) => {
                writer.write(1)?;
                (*e).encode(writer)
            }
            GameRule::RandomTickSpeed(e) => {
                writer.write(2)?;
                UnsignedVarInt(*e).encode(writer)
            }
        }
    }
}

#[packet(0x0B)]
#[derive(Debug, MCPEPacketDataAuto)]
pub struct StartGamePacket {
    entity_unique_id: VarLong,
    entity_runtime_id: UnsignedVarLong,
    player_gamemode: VarInt,
    spawn: Vec3f,
    yaw: Le<f32>,
    pitch: Le<f32>,
    seed: VarInt,
    spawn_biome_type: Le<i16>,
    custom_biome_name: String,
    dimension: VarInt,
    generator: VarInt,
    world_game_mode: VarInt,
    difficulty: VarInt,
    world_spawn: BlockVec3,
    has_achievements_disabled: bool,
    day_cycle_stop_time: VarInt,
    edu_edition_offer: VarInt,
    has_edu_features_enabled: bool,
    edu_production_id: String,
    rain_level: Le<f32>,
    lightning_level: Le<f32>,
    has_confirmed_platform_locked_content: bool,
    multiplayer_game: bool,
    broadcast_to_lan: bool,
    xbl_broadcast_intent: VarInt,
    platform_broadcast_intent: VarInt,
    commands_enabled: bool,
    is_texture_packs_required: bool,
    game_rules: VecIndexed<GameRule, UnsignedVarInt>,
    _unknown1: Le<i32>, // putLInt(0); maybe (NOT IN PROTOCOL)
    _unknown2: bool,    // putBoolean(false); maybe (NOT IN PROTOCOL)
    bonus_chest: bool,
    has_start_with_map_enabled: bool,
    permission_level: VarInt,
    server_chunk_tick_range: Le<i32>,
    has_locked_behavior_pack: bool,
    has_locked_resource_pack: bool,
    is_from_locked_world_template: bool,
    is_using_msa_gamertags_only: bool,
    is_from_world_template: bool,
    is_world_template_option_locked: bool,
    is_only_spawning_v1_villagers: bool,
    vanilla_version: String,
    limited_world_width: i32,  // maybe Le<i32>
    limited_world_height: i32, // maybe Le<i32>
    is_nether_type: bool,
    is_force_experimental_gameplay: bool,
    level_id: String,
    world_name: String,
    premium_world_template_id: String,
    is_trial: bool,
    is_movement_server_authoritative: bool, // VarInt in Nukkit
    _unknown3: VarInt,                      // Maybe change to bool for optimization
    _unknown4: bool,
    current_tick: Le<i64>,
    enchantment_seed: VarInt,
    _unknown5: UnsignedVarInt,
    item_data_palette: VecIndexed<ItemDef, UnsignedVarInt>,
    multiplayer_correlation_id: String,
    is_inventory_server_authoritative: bool,
}

impl StartGamePacket {
    pub fn new() -> Self {
        Self {
            entity_unique_id: VarLong(-1),
            entity_runtime_id: UnsignedVarLong(1),
            player_gamemode: VarInt(1),
            spawn: (0., 0., 0.).into(),
            yaw: Le(0.),
            pitch: Le(0.),
            seed: VarInt(150),
            spawn_biome_type: Le(0),
            custom_biome_name: "plains".to_owned(),
            dimension: VarInt(0),
            generator: VarInt(0),
            world_game_mode: VarInt(1),
            difficulty: VarInt(1),
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
            game_rules: VecIndexed::from(vec![]),
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
            vanilla_version: "1.16.210".to_owned(),
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
}

#[derive(serde::Deserialize, Debug)]
pub struct ItemDef {
    pub name: String,
    pub id: i16,
    pub oldData: Option<i16>,
    pub oldId: Option<i16>,
}

impl MCPEPacketData for ItemDef {
    fn decode(reader: &mut impl crate::traits::Reader) -> Result<Self, MCPEPacketDataError> {
        Ok(Self {
            name: reader.auto_decode().map_err(|x| x.map("name"))?,
            id: reader.auto_decode().map_err(|x| x.map("id"))?,
            oldData: None,
            oldId: None,
        })
    }

    fn encode(&self, writer: &mut impl crate::traits::Writer) -> Result<(), MCPEPacketDataError> {
        self.name.encode(writer).map_err(|x| x.map("name"))?;
        self.id.encode(writer).map_err(|x| x.map("id"))?;
        writer.write(0).map_err(|x| x.map("empty_byte"))
    }
}
