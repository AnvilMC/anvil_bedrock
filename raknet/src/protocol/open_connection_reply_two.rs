use std::net::SocketAddr;

use crate::prelude::{RaknetPacket, RaknetPacketData};

use super::OpenConnectionRequestTwo;

#[derive(Debug)]
pub struct OpenConnectionReplyTwo {
    pub magic: [u8; 16],
    pub server_guid: i64,
    pub ip_bytes: [u8; 4],
    pub port: u16,
    pub mtu: u16,
    pub encryption: bool,
}

impl RaknetPacketData for OpenConnectionReplyTwo {
    fn decode(reader: &mut impl crate::prelude::Reader) -> Option<Self> {
        Some(Self {
            magic: <[u8; 16]>::decode(reader)?,
            server_guid: i64::decode(reader)?,
            ip_bytes: {
                reader.next()?;
                <[u8; 4]>::decode(reader)?
            },
            port: u16::decode(reader)?,
            mtu: u16::decode(reader)?,
            encryption: bool::decode(reader)?,
        })
    }

    fn encode(&self, writer: &mut impl crate::prelude::Writer) -> Option<()> {
        self.magic.encode(writer)?;
        self.server_guid.encode(writer)?;
        writer.write(4);
        self.ip_bytes.encode(writer)?;
        self.port.encode(writer)?;
        self.mtu.encode(writer)?;
        self.encryption.encode(writer)
    }
}

impl RaknetPacket for OpenConnectionReplyTwo {
    const RANGE: std::ops::Range<u8> = 0x08..0x09;

    fn id(&self) -> u8 {
        0x08
    }
}

impl OpenConnectionReplyTwo {
    pub fn from(e: &OpenConnectionRequestTwo, peer: &SocketAddr, server_guid: i64) -> Self {
        Self {
            magic: e.magic,
            server_guid,
            ip_bytes: if let SocketAddr::V4(e) = peer {
                e.ip().octets()
            } else {
                panic!("IPV6 isn't supported");
            },
            port: if let SocketAddr::V4(e) = peer {
                e.port()
            } else {
                panic!("IPV6 isn't supported");
            },
            mtu: e.mtu,
            encryption: false,
        }
    }
}
