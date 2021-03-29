use crate::prelude::{RaknetPacket, RaknetPacketData};

#[derive(Debug)]
pub struct OpenConnectionRequestTwo {
    pub magic: [u8; 16],
    pub server_ip_bytes: [u8; 4],
    pub port: u16,
    pub mtu: u16,
    pub client_guid: i64,
}

impl RaknetPacketData for OpenConnectionRequestTwo {
    fn decode(reader: &mut impl crate::prelude::Reader) -> Option<Self> {
        Some(Self {
            magic: <[u8; 16]>::decode(reader)?,
            server_ip_bytes: {
                reader.next()?;
                <[u8; 4]>::decode(reader)?
            },
            port: u16::decode(reader)?,
            mtu: u16::decode(reader)?,
            client_guid: i64::decode(reader)?,
        })
    }

    fn encode(&self, writer: &mut impl crate::prelude::Writer) -> Option<()> {
        self.magic.encode(writer)?;
        writer.write(4)?;
        self.server_ip_bytes.encode(writer)?;
        self.port.encode(writer)?;
        self.mtu.encode(writer)?;
        self.client_guid.encode(writer)
    }
}

impl RaknetPacket for OpenConnectionRequestTwo {
    const RANGE: std::ops::Range<u8> = 0x07..0x08;

    fn id(&self) -> u8 {
        0x07
    }
}
