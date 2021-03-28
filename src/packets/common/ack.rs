use either::Either;
use packet_derive::packet;

use crate::packets::{objects::{guid::GUID, uint24le::UInt24Le}, traits::PacketDecoder};
use crate::packets::traits::IterRead;

#[packet(0xc0)]
#[derive(Debug, packet_derive::Biscuit)]
pub struct Ack {
    pub record_count: i16,
    pub record: Record,
}

#[derive(Debug)]
pub struct Record (pub Either<UInt24Le, (UInt24Le, UInt24Le)>);

impl PacketDecoder for Record {
    fn read(iter: &mut crate::packets::traits::U8Iter) -> Option<Self> {
        Some(Record(if iter.read()? {
            Either::Left(iter.read()?)
        }else {
           Either::Right((iter.read()?, iter.read()?))
        }))
    }

    fn write(self, vec: &mut Vec<u8>) -> Option<()> {
        match self.0 {
            Either::Left(e) => {
                true.write(vec)?;
                e.write(vec)?;
            }
            Either::Right((x ,y)) => {
                false.write(vec)?;
                x.write(vec)?;
                y.write(vec)?;
            }
        }
        Some(())
    }
}