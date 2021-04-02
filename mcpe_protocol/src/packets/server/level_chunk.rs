use std::convert::TryInto;

use packet_derive::{packet, MCPEPacketDataAuto};

use crate::prelude::{
    BiomeIdArray, ByteArray, ByteArrayEncapsulated, Le, PalettedBlockStorage, ReadToEndVec,
    StaticData, UnsignedVarInt, VarInt, VecIndexed,
};

// #[packet(0x3A)]
// #[derive(MCPEPacketDataAuto)]
// pub struct LevelChunkPacket {
//     chunk_x: VarInt,
//     chunk_z: VarInt,
//     sub_chunk_count: UnsignedVarInt,
//     cache: bool,
//     data: StaticData<'static, u8>,
// }

// impl LevelChunkPacket {
//     pub fn new(chunk_x: i32, chunk_z: i32) -> Self {
//         Self {
//             chunk_x: VarInt(chunk_x),
//             chunk_z: VarInt(chunk_z),
//             sub_chunk_count: UnsignedVarInt(6),
//             cache: false,
//             data: StaticData(include_bytes!("0.2new.bin")),
//         }
//     }
// }

#[packet(0x3A)]
#[derive(MCPEPacketDataAuto)]
pub struct LevelChunkPacket {
    chunk_x: VarInt,
    chunk_z: VarInt,
    sub_chunk_count: UnsignedVarInt,
    cache: bool,
    data: ByteArrayEncapsulated<LevelChunkDataData>,
}

#[derive(MCPEPacketDataAuto, Debug, Clone)]
pub struct LevelChunkSection {
    unknown_byte_1: i8, // value = 8,
    unknown_byte_2: i8, // value = 2,
    //storage: PalettedBlockStorage, // writeTo in cn.nukkit.level.format.anvil.util.BlockStorage
    storage: StaticData<'static, u8>,
    empty_storage: StaticData<'static, u8>,
}

impl LevelChunkSection {
    pub fn new(/* storage: PalettedBlockStorage */) -> Self {
        Self {
            unknown_byte_1: 8,
            unknown_byte_2: 2,
            storage: StaticData(EMPTY_STORAGE),
            empty_storage: StaticData(EMPTY_STORAGE),
        }
    }
}

#[derive(MCPEPacketDataAuto, Debug)]
pub struct LevelChunkDataData {
    sections: [LevelChunkSection; 16],
    biome_id_array: BiomeIdArray,
    unknown_byte_1: i8, // DEFAULT: 0
                        //block_entities for Later
}

/* BinaryStream stream = ((BinaryStream)ThreadCache.binaryStream.get()).reset();
int count = 0;
ChunkSection[] sections = chunk.getSections();
int i;
for (i = sections.length - 1; i >= 0; i--) {
  if (!sections[i].isEmpty()) {
    count = i + 1;
    break;
  }
}
for (i = 0; i < count; i++)
  sections[i].writeTo(stream);
stream.put(chunk.getBiomeIdArray());
stream.putByte((byte)0);
stream.put(blockEntities);
getLevel().chunkRequestCallback(timestamp, x, z, count, stream.getBuffer()); */

impl LevelChunkPacket {
    pub fn new(chunk_x: i32, chunk_z: i32) -> Self {
        let SECTION: LevelChunkSection = LevelChunkSection::new(/* {
            let mut palette = PalettedBlockStorage::new(&crate::prelude::V1);
            palette.set_block(
                3,
                crate::prelude::GLOBAL_BLOCK_PALETTE.get_or_create_runtime_id(3, 0),
            );
            palette.set_block(
                4,
                crate::prelude::GLOBAL_BLOCK_PALETTE.get_or_create_runtime_id(2, 0),
            );
            palette.set_block(
                5,
                crate::prelude::GLOBAL_BLOCK_PALETTE.get_or_create_runtime_id(1, 0),
            );
            palette
        } */);
        println!("A1");
        Self {
            chunk_x: VarInt(chunk_x),
            chunk_z: VarInt(chunk_z),
            sub_chunk_count: UnsignedVarInt(6),
            cache: false,
            data: ByteArrayEncapsulated(LevelChunkDataData {
                sections: (0..16)
                    .map(|_| {
                        println!("A2");
                        SECTION.clone()
                    })
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
                biome_id_array: BiomeIdArray::default(),
                unknown_byte_1: 0,
            }), /*StaticData(include_bytes!("0.2new.bin"))*/
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

const EMPTY_STORAGE: &'static [u8] = &[
    3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 2, 140, 2,
];

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
