use mcpe_protocol::{
    prelude::{
        BiomeDefinitionList, ChunkRadiusUpdated, MCPEPacketDataError, PlayStatus, PlayerMovePacket,
        ResourcePackStack, ResourcePacksInfo, StartGamePacket, TickSyncPacket, UpdateBlock,
    },
    traits::{MCPEPacket, MCPEPacketData, Writer},
};

#[derive(Debug)]
pub enum GamePacketSendablePacket {
    PlayStatus(PlayStatus),
    ResourcePacksInfo(ResourcePacksInfo),
    StartGamePacket(StartGamePacket),
    BiomeDefinitionList(BiomeDefinitionList),
    UpdateBlock(UpdateBlock),
    ResourcePackStack(ResourcePackStack),
    ChunkRadiusUpdated(ChunkRadiusUpdated),
    TickSyncPacket(TickSyncPacket),
    PlayerMovePacket(PlayerMovePacket),
}

impl GamePacketSendablePacket {
    pub fn get_id(&self) -> u8 {
        match self {
            GamePacketSendablePacket::PlayStatus(_) => <PlayStatus as MCPEPacket>::PACKET_ID,
            GamePacketSendablePacket::ResourcePacksInfo(_) => {
                <ResourcePacksInfo as MCPEPacket>::PACKET_ID
            }
            GamePacketSendablePacket::StartGamePacket(_) => {
                <StartGamePacket as MCPEPacket>::PACKET_ID
            }
            GamePacketSendablePacket::BiomeDefinitionList(_) => {
                <BiomeDefinitionList as MCPEPacket>::PACKET_ID
            }
            GamePacketSendablePacket::UpdateBlock(_) => <UpdateBlock as MCPEPacket>::PACKET_ID,
            GamePacketSendablePacket::ResourcePackStack(_) => {
                <ResourcePackStack as MCPEPacket>::PACKET_ID
            }
            GamePacketSendablePacket::ChunkRadiusUpdated(_) => {
                <ChunkRadiusUpdated as MCPEPacket>::PACKET_ID
            }
            GamePacketSendablePacket::TickSyncPacket(_) => {
                <TickSyncPacket as MCPEPacket>::PACKET_ID
            }
            GamePacketSendablePacket::PlayerMovePacket(_) => {
                <PlayerMovePacket as MCPEPacket>::PACKET_ID
            }
        }
    }

    pub fn encode(&self, writer: &mut impl Writer) -> Result<(), MCPEPacketDataError> {
        match self {
            GamePacketSendablePacket::PlayStatus(e) => e.encode(writer),
            GamePacketSendablePacket::ResourcePacksInfo(e) => e.encode(writer),
            GamePacketSendablePacket::StartGamePacket(e) => e.encode(writer),
            GamePacketSendablePacket::BiomeDefinitionList(e) => e.encode(writer),
            GamePacketSendablePacket::UpdateBlock(e) => e.encode(writer),
            GamePacketSendablePacket::ResourcePackStack(e) => e.encode(writer),
            GamePacketSendablePacket::ChunkRadiusUpdated(e) => e.encode(writer),
            GamePacketSendablePacket::TickSyncPacket(e) => e.encode(writer),
            GamePacketSendablePacket::PlayerMovePacket(e) => e.encode(writer),
        }
    }
}
