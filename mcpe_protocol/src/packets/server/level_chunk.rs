use packet_derive::{packet, MCPEPacketDataAuto};

use crate::prelude::{
    ByteArray, ByteArrayEncapsulated, Le, ReadToEndVec, StaticData, UnsignedVarInt, VarInt,
    VecIndexed,
};

#[packet(0x3A)]
#[derive(MCPEPacketDataAuto)]
pub struct LevelChunkPacket {
    chunk_x: VarInt,
    chunk_z: VarInt,
    sub_chunk_count: UnsignedVarInt,
    cache: bool,
    data: StaticData<'static, u8>,
}

impl LevelChunkPacket {
    pub fn new(chunk_x: i32, chunk_z: i32) -> Self {
        Self {
            chunk_x: VarInt(chunk_x),
            chunk_z: VarInt(chunk_z),
            sub_chunk_count: UnsignedVarInt(6),
            cache: false,
            data: StaticData(include_bytes!("0.2new.bin")),
        }
    }
}

// #[packet(0x3A)]
// #[derive(MCPEPacketDataAuto)]
// pub struct LevelChunkPacket {
//     chunk_x: VarInt,
//     chunk_z: VarInt,
//     sub_chunk_count: UnsignedVarInt,
//     cache: Option<ChunkCache>,
//     data: ByteArrayEncapsulated<ChunkData>,
// }

// pub struct ChunkData {
//     sections: ReadToEndVec<EmptyChunkSection>,
//     biome_array:
// }

// // BinaryStream stream = ((BinaryStream)ThreadCache.binaryStream.get()).reset();
// //     int count = 0;
// //     ChunkSection[] sections = chunk.getSections();
// //     int i;
// //     for (i = sections.length - 1; i >= 0; i--) {
// //       if (!sections[i].isEmpty()) {
// //         count = i + 1;
// //         break;
// //       }
// //     }
// //     for (i = 0; i < count; i++)
// //       sections[i].writeTo(stream);
// //     stream.put(chunk.getBiomeIdArray());
// //     stream.putByte((byte)0);
// //     stream.put(blockEntities);
// //     getLevel().chunkRequestCallback(timestamp, x, z, count, stream.getBuffer());

// #[derive(MCPEPacketDataAuto)]
// pub struct ChunkCache {
//     blob_ids: VecIndexed<Le<i64>, UnsignedVarInt>,
// }

// #[derive(MCPEPacketDataAuto)]
// pub struct EmptyChunkSection {
//     first_byte: i8,
//     second_byte: i8,
//     empty_storage1: StaticData<'static, u8>,
//     empty_storage2: StaticData<'static, u8>,
// }

// const EMPTY_STORAGE: &'static [u8] = &[
//     3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
//     0, 2, 140, 2,
// ];

// impl Default for EmptyChunkSection {
//     fn default() -> Self {
//         Self {
//             first_byte: 8,
//             second_byte: 2,
//             empty_storage1: StaticData(EMPTY_STORAGE),
//             empty_storage2: StaticData(EMPTY_STORAGE),
//         }
//     }
// }
// // stream.putByte((byte)8);
// // stream.putByte((byte)2);
// // EMPTY_STORAGE.writeTo(stream);
// // EMPTY_STORAGE.writeTo(stream);
