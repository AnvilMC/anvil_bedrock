use std::ops::Range;

use crate::prelude::{RaknetPacketData, Reader, Writer};

macro_rules! primitive {
    ($tp:ty,$len:expr) => {
        impl RaknetPacketData for $tp {
            fn decode(reader: &mut impl Reader) -> Option<Self> {
                Some(Self::from_be_bytes(reader.next_array()?))
            }
            fn encode(&self, writer: &mut impl Writer) -> Option<()> {
                writer.write_slice(&self.to_be_bytes())
            }
        }
    };
}

impl RaknetPacketData for bool {
    fn decode(reader: &mut impl Reader) -> Option<Self> {
        Some(reader.next()? == 1)
    }

    fn encode(&self, writer: &mut impl Writer) -> Option<()> {
        writer.write(if *self { 1 } else { 0 })
    }
}

impl <const N: usize> RaknetPacketData for [u8; N] {
    fn decode(reader: &mut impl Reader) -> Option<Self> {
        reader.next_array()
    }

    fn encode(&self, writer: &mut impl Writer) -> Option<()> {
        writer.write_slice(self)
    }
}

pub trait RaknetPacket: RaknetPacketData {

    const RANGE: Range<u8>;
    fn id(&self) -> u8;
} 

primitive!(i8, 1);
primitive!(u16, 2);
primitive!(i16, 2);
primitive!(u32, 4);
primitive!(u64, 8);
primitive!(i32, 4);
primitive!(i64, 8);
