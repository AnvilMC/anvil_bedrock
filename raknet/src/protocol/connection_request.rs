use crate::prelude::{RaknetPacket, RaknetPacketData};

pub struct ConnectionRequest {
    pub guid: i64,
    pub time: i64,
}

impl RaknetPacket for ConnectionRequest {
    const RANGE: std::ops::Range<u8> = 0x9..0x10;

    fn id(&self) -> u8 {
        0x9
    }
}

impl RaknetPacketData for ConnectionRequest {
    fn decode(reader: &mut impl crate::prelude::Reader) -> Option<Self> {
        Some(Self {
            guid: i64::decode(reader)?,
            time: i64::decode(reader)?,
        })
    }

    fn encode(&self, writer: &mut impl crate::prelude::Writer) -> Option<()> {
        self.guid.encode(writer)?;
        self.time.encode(writer)
    }
}
