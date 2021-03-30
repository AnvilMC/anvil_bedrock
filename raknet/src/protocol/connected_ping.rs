use crate::prelude::{RaknetPacket, RaknetPacketData};

#[derive(Debug)]
pub struct ConnectedPing {
    pub time: u64,
}

impl RaknetPacketData for ConnectedPing {
    fn decode(reader: &mut impl crate::prelude::Reader) -> Option<Self> {
        Some(Self {
            time: u64::decode(reader)?,
        })
    }

    fn encode(&self, writer: &mut impl crate::prelude::Writer) -> Option<()> {
        self.time.encode(writer)
    }
}

impl RaknetPacket for ConnectedPing {
    const RANGE: std::ops::Range<u8> = 0x00..0x01;

    fn id(&self) -> u8 {
        0x00
    }
}
