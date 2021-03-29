use std::{convert::TryInto, net::SocketAddr};

use crate::prelude::{Address, RaknetPacket, RaknetPacketData};

use super::ConnectionRequest;

pub struct ConnectionRequestAccepted {
    pub client_adress: Address,
    pub system_index: i16,
    pub internal_ids: [Address; 10],
    pub request_time: i64,
    pub time: i64,
}

impl RaknetPacket for ConnectionRequestAccepted {
    const RANGE: std::ops::Range<u8> = 0x10..0x11;

    fn id(&self) -> u8 {
        0x10
    }
}

impl RaknetPacketData for ConnectionRequestAccepted {
    fn decode(reader: &mut impl crate::prelude::Reader) -> Option<Self> {
        Some(Self {
            client_adress: Address::decode(reader)?,
            system_index: i16::decode(reader)?,
            internal_ids: <[Address; 10]>::decode(reader)?,
            request_time: i64::decode(reader)?,
            time: i64::decode(reader)?,
        })
    }

    fn encode(&self, writer: &mut impl crate::prelude::Writer) -> Option<()> {
        self.client_adress.encode(writer)?;
        self.system_index.encode(writer)?;
        self.internal_ids.encode(writer)?;
        self.request_time.encode(writer)?;
        self.time.encode(writer)
    }
}

impl ConnectionRequestAccepted {
    pub fn from(req: ConnectionRequest, peer: &SocketAddr) -> Self {
        Self {
            client_adress: peer.into(),
            system_index: 0,
            internal_ids: {
                let address = Address {
                    ip: [255; 4],
                    port: 19132,
                };
                (0..10)
                    .map(|_| address.clone())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap()
            },
            request_time: req.time,
            time: req.time,
        }
    }
}
