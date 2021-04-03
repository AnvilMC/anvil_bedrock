use mcpe_protocol::{
    prelude::{
        LoginPacket, MCPEPacketDataError, RequestChunkRadiusPacket,
        ResourcePackClientResponsePacket, TickSyncPacket, UnsignedVarInt,
    },
    traits::{MCPEPacketData, Reader},
};

pub enum ReceivablePacket {
    RequestChunkRadiusPacket(RequestChunkRadiusPacket),
    TickSyncPacket(TickSyncPacket),
    LoginPacket(LoginPacket),
    ResourcePackClientResponsePacket(ResourcePackClientResponsePacket),
}

impl ReceivablePacket {
    pub fn try_read(buffer: &[u8]) -> Result<Self, MCPEPacketDataError> {
        let mut iter = buffer.iter();
        let uint = UnsignedVarInt::decode(&mut iter).unwrap().0 & 0x3FF;
        match uint {
            0x01 => Ok(Self::LoginPacket(LoginPacket::decode(&mut iter)?)),
            0x08 => Ok(Self::ResourcePackClientResponsePacket(
                ResourcePackClientResponsePacket::decode(&mut iter)?,
            )),
            0x45 => Ok(Self::RequestChunkRadiusPacket(
                RequestChunkRadiusPacket::decode(&mut iter)?,
            )),
            0x17 => Ok(Self::TickSyncPacket(TickSyncPacket::decode(&mut iter)?)),
            0x9C => {
                panic!("Packet violation : {:?}", iter.read_to_end());
            }
            e => Err(MCPEPacketDataError::new(
                "game_packet_id",
                format!("Unknown game packet id {:?}", e),
            )),
        }
    }
}
