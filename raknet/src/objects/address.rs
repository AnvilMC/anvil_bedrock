use std::{convert::TryInto, net::SocketAddr};

use crate::prelude::RaknetPacketData;

#[derive(Debug, Clone)]
pub struct Address {
    pub ip: [u8; 4],
    pub port: u16,
}

impl RaknetPacketData for Address {
    fn decode(reader: &mut impl crate::prelude::Reader) -> Option<Self> {
        reader.skip(1);
        Some(Self {
            ip: reader.next_array()?,
            port: u16::decode(reader)?,
        })
    }

    fn encode(&self, writer: &mut impl crate::prelude::Writer) -> Option<()> {
        writer.write(4)?;
        self.ip.encode(writer)?;
        self.port.encode(writer)
    }
}

impl<const N: usize> RaknetPacketData for [Address; N] {
    fn decode(reader: &mut impl crate::prelude::Reader) -> Option<Self> {
        (0..N)
            .map(|_| Address::decode(reader))
            .collect::<Option<Vec<_>>>()?
            .try_into()
            .ok()
    }

    fn encode(&self, writer: &mut impl crate::prelude::Writer) -> Option<()> {
        for i in self.iter() {
            i.encode(writer)?;
        }
        Some(())
    }
}

impl From<&SocketAddr> for Address {
    fn from(e: &SocketAddr) -> Self {
        match e {
            SocketAddr::V4(e) => Self {
                ip: e.ip().octets(),
                port: e.port(),
            },
            SocketAddr::V6(e) => {
                panic!("Can't handle V6 IPs {:?}", e);
            }
        }
    }
}
