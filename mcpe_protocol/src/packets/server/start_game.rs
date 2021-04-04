use crate::prelude::{
    Le, MCPEPacketData, MCPEPacketDataError, UnsignedVarInt, UnsignedVarLong, VarInt, VarLong,
    VecIndexed,
};
use packet_derive::{packet, MCPEPacketDataAuto};

#[derive(Debug, MCPEPacketDataAuto, Clone)]
pub struct Vec3f(pub Le<f32>, pub Le<f32>, pub Le<f32>);

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
    pub entity_unique_id: VarLong,
    pub entity_runtime_id: UnsignedVarLong,
    pub player_gamemode: GameMode,
    pub spawn: Vec3f,
    pub yaw: Le<f32>,
    pub pitch: Le<f32>,
    pub seed: VarInt,
    pub spawn_biome_type: Le<i16>,
    pub custom_biome_name: String,
    pub dimension: VarInt,
    pub generator: VarInt,
    pub world_game_mode: VarInt,
    pub difficulty: Difficulty,
    pub world_spawn: BlockVec3,
    pub has_achievements_disabled: bool,
    pub day_cycle_stop_time: VarInt,
    pub edu_edition_offer: VarInt,
    pub has_edu_features_enabled: bool,
    pub edu_production_id: String,
    pub rain_level: Le<f32>,
    pub lightning_level: Le<f32>,
    pub has_confirmed_platform_locked_content: bool,
    pub multiplayer_game: bool,
    pub broadcast_to_lan: bool,
    pub xbl_broadcast_intent: VarInt,
    pub platform_broadcast_intent: VarInt,
    pub commands_enabled: bool,
    pub is_texture_packs_required: bool,
    pub game_rules: VecIndexed<GameRule, UnsignedVarInt>,
    pub _unknown1: Le<i32>, // putLInt(0); maybe (NOT IN PROTOCOL)
    pub _unknown2: bool,    // putBoolean(false); maybe (NOT IN PROTOCOL)
    pub bonus_chest: bool,
    pub has_start_with_map_enabled: bool,
    pub permission_level: VarInt,
    pub server_chunk_tick_range: Le<i32>,
    pub has_locked_behavior_pack: bool,
    pub has_locked_resource_pack: bool,
    pub is_from_locked_world_template: bool,
    pub is_using_msa_gamertags_only: bool,
    pub is_from_world_template: bool,
    pub is_world_template_option_locked: bool,
    pub is_only_spawning_v1_villagers: bool,
    pub vanilla_version: String,
    pub limited_world_width: i32,  // maybe Le<i32>
    pub limited_world_height: i32, // maybe Le<i32>
    pub is_nether_type: bool,
    pub is_force_experimental_gameplay: bool,
    pub level_id: String,
    pub world_name: String,
    pub premium_world_template_id: String,
    pub is_trial: bool,
    pub is_movement_server_authoritative: bool, // VarInt in Nukkit
    pub _unknown3: VarInt,                      // Maybe change to bool for optimization
    pub _unknown4: bool,
    pub current_tick: Le<i64>,
    pub enchantment_seed: VarInt,
    pub _unknown5: UnsignedVarInt,
    pub item_data_palette: VecIndexed<ItemDef, UnsignedVarInt>,
    pub multiplayer_correlation_id: String,
    pub is_inventory_server_authoritative: bool,
}

#[repr(u8)]
#[packet_derive::mcpe_packet_data_enum(u8 VarInt)]
#[derive(Debug)]
pub enum GameMode {
    Survival = 0,
    Creative = 1,
    Adventure = 2,
    Spectator = 3,
}

#[repr(u8)]
#[packet_derive::mcpe_packet_data_enum(u8 VarInt)]
#[derive(Debug)]
pub enum Difficulty {
    Peacefull = 0,
    Easy = 1,
    Normal = 2,
    Hard = 3,
}

#[allow(non_snake_case)]
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
