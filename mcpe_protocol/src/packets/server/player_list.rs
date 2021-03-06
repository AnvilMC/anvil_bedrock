use crate::prelude::{
    ByteArray, Le, MCPEPacket, MCPEPacketData, MCPEPacketDataError, UnsignedVarInt, VarLong,
    VecIndexed,
};
use packet_derive::MCPEPacketDataAuto;

#[derive(MCPEPacketDataAuto)]
pub struct PlayerListPlayer {
    uuid: Le<u128>,
    entity_id: VarLong,
    name: String,
    xbox_user_id: String,
    platform_chat_id: String,
    build_platform: Le<i32>,
    skin: Skin,
    is_teacher: bool,
    is_host: bool,
}

#[derive(MCPEPacketDataAuto)]
pub struct Skin {
    skin_id: String,
    play_fab_id: String,
    skin_resource_patch: String,
    skin_data: Image,
    skin_animations: VecIndexed<SkinAnimation, Le<i32>>,
    cape_data: Image,
    geometry_data: String,
    animation_data: String,
    premium: bool,
    persona: bool,
    is_cape_on_classic: bool,
    cape_id: String,
    full_skin_id: String,
    arm_size: String,
    skin_color: String,
    persona_pieces: VecIndexed<PersonaPiece, Le<i32>>,
    persona_piece_tints: VecIndexed<PersonaPieceTint, Le<i32>>,
}

#[derive(MCPEPacketDataAuto)]
struct PersonaPiece {
    id: String,
    piece_type: String,
    pack_id: String,
    is_default: bool,
    product_id: String,
}

#[derive(MCPEPacketDataAuto)]
struct PersonaPieceTint {
    piece_type: String,
    colors: VecIndexed<String, Le<i32>>,
}

#[derive(MCPEPacketDataAuto)]
struct SkinAnimation {
    image: Image,
    anim_type: Le<i32>,
    frames: Le<f32>,
    expression: Le<i32>,
}
#[derive(MCPEPacketDataAuto)]
struct Image {
    width: Le<i32>,
    height: Le<i32>,
    data: ByteArray,
}

pub enum PlayerListPacket {
    Add(VecIndexed<PlayerListPlayer, UnsignedVarInt>),
    Remove(VecIndexed<Le<u128>, UnsignedVarInt>),
}

impl MCPEPacket for PlayerListPacket {
    const PACKET_ID: u8 = 63;
}

impl MCPEPacketData for PlayerListPacket {
    fn decode(
        reader: &mut impl crate::prelude::Reader,
    ) -> Result<Self, crate::prelude::MCPEPacketDataError> {
        let id = reader.next()?;
        match id {
            0 => Ok({
                let i: VecIndexed<PlayerListPlayer, UnsignedVarInt> = <_>::decode(reader)?;
                reader.skip(i.0.len());
                Self::Add(i)
            }),
            1 => Ok(<_>::decode(reader)?),
            _ => Err(MCPEPacketDataError::new(
                "id(PlayerListPacket)",
                "Invalid PlayerList id",
            )),
        }
    }
    // Result<Self, crate::prelude::MCPEPacketDataError>

    fn encode(
        &self,
        writer: &mut impl crate::prelude::Writer,
    ) -> Result<(), crate::prelude::MCPEPacketDataError> {
        match self {
            PlayerListPacket::Add(a) => {
                writer.write(0)?;
                a.encode(writer)?;
                writer.write_slice(&(0..a.len()).map(|_| 1).collect::<Vec<u8>>())?;
                Ok(())
            }
            PlayerListPacket::Remove(a) => {
                writer.write(1)?;
                a.encode(writer)
            }
        }
    }
}
