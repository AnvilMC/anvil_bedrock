use std::net::SocketAddr;

use either::Either;

use crate::packets::traits::{IterRead, PacketDecoder};

#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Address {
    //pub ip_type: u8,
    pub ip_bytes: Either<[u8; 4],[u8; 28]>,
    pub port: u16,
}

impl PacketDecoder for Address {
    fn read(iter: &mut crate::packets::traits::U8Iter) -> Option<Self> {
        let ty = iter.next()?;
        Some(Self {
            ip_bytes: if ty == 4 {
                Either::Left(iter.read()?)
            } else {
                Either::Right(iter.read()?)
            },
            port: iter.read()?
        })
    }

    fn write(self, vec: &mut Vec<u8>) -> Option<()> {
        match self.ip_bytes {
            Either::Left(a) => {
                vec.push(4);
                a.write(vec)?;
            }
            Either::Right(a) => {
                vec.push(6);
                a.write(vec)?;
            }
        }
        self.port.write(vec);
        Some(())
    }
}

impl From<&SocketAddr> for Address {
    fn from(e: &SocketAddr) -> Self {
        match e {
            SocketAddr::V4(a) => Self {
                ip_bytes: Either::Left(a.ip().octets()),
                port: a.port(),
            },
            SocketAddr::V6(a) => unreachable!(),
        }
    }
}
