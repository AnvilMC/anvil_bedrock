use std::ops::Range;

use crate::{objects::RaknetPacket, prelude::{RaknetPacketData, Reader, Writer}};


pub struct UnconnectedPing {
    pub time: i64,
    pub magic: [u8; 16],
    pub client_guid: i64
}

impl RaknetPacketData for UnconnectedPing {
    fn decode(reader: &mut impl Reader) -> Option<Self> {
        Some(Self {
            time: i64::decode(reader)?,
            magic: <[u8; 16]>::decode(reader)?,
            client_guid: i64::decode(reader)?,
        })
    }

    fn encode(&self, writer: &mut impl Writer) -> Option<()> {
        self.time.encode(writer)?;
        self.magic.encode(writer)?;
        self.client_guid.encode(writer)
    }
}

impl RaknetPacket for UnconnectedPing {
    const RANGE: Range<u8> = 1..3;

    fn id(&self) -> u8 {
        1
    }
}