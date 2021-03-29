use crate::prelude::{RaknetPacket, RaknetPacketData};

use super::open_connection_request_one::OpenConnectionRequestOne;

#[derive(Debug)]
pub struct OpenConnectionReplyOne {
    pub magic: [u8; 16],
    pub id: i64,
    pub security: bool,
    pub mtu: u16,
}

impl RaknetPacketData for OpenConnectionReplyOne {
    fn decode(reader: &mut impl crate::prelude::Reader) -> Option<Self> {
        Some(Self {
            magic: <[u8; 16]>::decode(reader)?,
            id: i64::decode(reader)?,
            security: bool::decode(reader)?,
            mtu: u16::decode(reader)?,
        })
    }

    fn encode(&self, writer: &mut impl crate::prelude::Writer) -> Option<()> {
        self.magic.encode(writer)?;
        self.id.encode(writer)?;
        self.security.encode(writer)?;
        self.mtu.encode(writer)
    }
}

impl RaknetPacket for OpenConnectionReplyOne {
    const RANGE: std::ops::Range<u8> = 0x06..0x07;

    fn id(&self) -> u8 {
        0x06
    }
}

impl OpenConnectionReplyOne {
    pub fn from(e: &OpenConnectionRequestOne, server_guid: i64) -> Self {
        Self {
            magic: e.magic,
            id: server_guid,
            security: false,
            mtu: e.mtu.len() as u16 + 46,
        }
    }
}
