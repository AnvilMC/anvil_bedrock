use crate::prelude::{ConnectedPing, RaknetPacket, RaknetPacketData};

#[derive(Debug)]
pub struct ConnectedPong {
    pub time: u64,
    pub pong_time: u64,
}

impl RaknetPacketData for ConnectedPong {
    fn decode(reader: &mut impl crate::prelude::Reader) -> Option<Self> {
        Some(Self {
            time: u64::decode(reader)?,
            pong_time: u64::decode(reader)?,
        })
    }

    fn encode(&self, writer: &mut impl crate::prelude::Writer) -> Option<()> {
        self.time.encode(writer)?;
        self.pong_time.encode(writer)
    }
}

impl RaknetPacket for ConnectedPong {
    const RANGE: std::ops::Range<u8> = 0x03..0x04;

    fn id(&self) -> u8 {
        0x03
    }
}

impl From<ConnectedPing> for ConnectedPong {
    fn from(ping: ConnectedPing) -> Self {
        Self {
            time: ping.time,
            pong_time: ping.time,
        }
    }
}
