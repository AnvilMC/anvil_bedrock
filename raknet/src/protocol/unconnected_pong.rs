use std::{borrow::Cow, ops::Range};

use crate::{
    objects::{RaknetPacket, RaknetString},
    prelude::{RaknetPacketData, Reader, Writer},
};

pub struct UnconnectedPong<'a> {
    pub time: i64,
    pub server_guid: i64,
    pub magic: [u8; 16],
    pub server_id_string: Cow<'a, RaknetString>,
}

impl RaknetPacketData for UnconnectedPong<'_> {
    fn decode(reader: &mut impl Reader) -> Option<Self> {
        Some(Self {
            time: i64::decode(reader)?,
            server_guid: i64::decode(reader)?,
            magic: <[u8; 16]>::decode(reader)?,
            server_id_string: Cow::Owned(RaknetString::decode(reader)?),
        })
    }

    fn encode(&self, writer: &mut impl Writer) -> Option<()> {
        self.time.encode(writer)?;
        self.server_guid.encode(writer)?;
        self.magic.encode(writer)?;
        self.server_id_string.encode(writer)
    }
}

impl RaknetPacket for UnconnectedPong<'_> {
    const RANGE: Range<u8> = 0x1c..0x1d;

    fn id(&self) -> u8 {
        0x1c
    }
}
