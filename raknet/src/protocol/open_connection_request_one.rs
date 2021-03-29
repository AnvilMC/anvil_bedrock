use crate::prelude::{RaknetPacket, RaknetPacketData};

#[derive(Debug)]
pub struct OpenConnectionRequestOne {
    pub magic: [u8; 16],
    pub protocol_version: u8,
    pub mtu: Vec<u8>,
}

impl RaknetPacketData for OpenConnectionRequestOne {
    fn decode(reader: &mut impl crate::prelude::Reader) -> Option<Self> {
        Some(Self {
            magic: <[u8; 16]>::decode(reader)?,
            protocol_version: reader.next()?,
            mtu: reader.read_to_end(),
        })
    }

    fn encode(&self, writer: &mut impl crate::prelude::Writer) -> Option<()> {
        self.magic.encode(writer)?;
        writer.write(self.protocol_version)?;
        self.mtu.encode(writer)
    }
}

impl RaknetPacket for OpenConnectionRequestOne {
    const RANGE: std::ops::Range<u8> = 0x05..0x06;

    fn id(&self) -> u8 {
        0x05
    }
}
