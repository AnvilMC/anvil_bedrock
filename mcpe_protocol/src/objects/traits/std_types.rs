use crate::prelude::{Indexable, Le, MCPEPacketData, Reader, Writer};

macro_rules! primitive {
    ($tp:ty,$len:expr) => {
        impl MCPEPacketData for $tp {
            fn decode(reader: &mut impl Reader) -> Option<Self> {
                Some(Self::from_be_bytes(reader.next_array()?))
            }
            fn encode(&self, writer: &mut impl Writer) -> Option<()> {
                writer.write_slice(&self.to_be_bytes())
            }
        }
        impl MCPEPacketData for Le<$tp> {
            fn decode(reader: &mut impl Reader) -> Option<Self> {
                Some(Le(<$tp>::from_le_bytes(reader.next_array()?)))
            }
            fn encode(&self, writer: &mut impl Writer) -> Option<()> {
                writer.write_slice(&self.0.to_le_bytes())
            }
        }
        impl Indexable for $tp {
            fn as_usize(&self) -> usize {
                *self as usize
            }
            fn from_usize(u: usize) -> Self {
                u as Self
            }
        }
    };
}

primitive!(u8, 1); // Named unsigned byte in protocol
primitive!(i8, 1); // Named byte in protocol
primitive!(u16, 2); // Named unsigned byte in protocol
primitive!(i16, 2); // Named byte in protocol
primitive!(u32, 4); // Named unsigned int in protocol
primitive!(i32, 4); // Named int in protocol
primitive!(u64, 8); // Named unsigned long in protocol
primitive!(u128, 16); // UUID in protocol (In Nukkit encoded as two longs)
primitive!(i64, 8); // Named long in protocol
primitive!(f32, 4); // Named float in protocol

impl MCPEPacketData for bool {
    fn decode(reader: &mut impl Reader) -> Option<Self> {
        Some(reader.next()? == 1)
    }

    fn encode(&self, writer: &mut impl Writer) -> Option<()> {
        writer.write(if *self { 1 } else { 0 })
    }
}
