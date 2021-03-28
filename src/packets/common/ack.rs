use either::Either;
use packet_derive::packet;

use crate::packets::traits::IterRead;
use crate::packets::{
    objects::{guid::GUID, uint24le::UInt24Le},
    traits::PacketDecoder,
};

#[packet(0xc0)]
#[derive(Debug, PartialEq, Clone)]
pub struct Ack {
    pub record: Vec<Record>,
}

impl PacketDecoder for Ack {
    fn read(iter: &mut crate::packets::traits::U8Iter) -> Option<Self> {
        let count: i16 = iter.read()?;
        Some(Self {
            record: (0..count).map(|_| iter.read()).collect::<Option<_>>()?,
        })
    }

    fn write(self, vec: &mut Vec<u8>) -> Option<()> {
        (self.record.len() as i16).write(vec);
        for i in self.record.into_iter() {
            i.write(vec)?;
        }
        Some(())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Record(pub Either<UInt24Le, (UInt24Le, UInt24Le)>);

impl PacketDecoder for Record {
    fn read(iter: &mut crate::packets::traits::U8Iter) -> Option<Self> {
        Some(Record(if iter.read()? {
            Either::Left(iter.read()?)
        } else {
            Either::Right((iter.read()?, iter.read()?))
        }))
    }

    fn write(self, vec: &mut Vec<u8>) -> Option<()> {
        match self.0 {
            Either::Left(e) => {
                true.write(vec)?;
                e.write(vec)?;
            }
            Either::Right((x, y)) => {
                false.write(vec)?;
                x.write(vec)?;
                y.write(vec)?;
            }
        }
        Some(())
    }
}
