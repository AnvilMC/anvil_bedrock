use miniz_oxide::{deflate, inflate};

use crate::prelude::{RaknetPacket, RaknetPacketData};

pub struct GamePacket(pub Vec<u8>);

impl RaknetPacket for GamePacket {
    const RANGE: std::ops::Range<u8> = 0xFE..0xFF;

    fn id(&self) -> u8 {
        0xFE
    }
}

impl RaknetPacketData for GamePacket {
    fn decode(reader: &mut impl crate::prelude::Reader) -> Option<Self> {
        let m = reader.read_to_end();
        //println!("LEN: {}", m.len());
        Some(GamePacket(inflate::decompress_to_vec(&m).ok()?))
    }

    fn encode(&self, writer: &mut impl crate::prelude::Writer) -> Option<()> {
        writer.write_slice(&deflate::compress_to_vec(&self.0, 6))
    }
}
