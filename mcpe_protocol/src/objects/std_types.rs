use std::convert::TryInto;

use crate::traits::{MCPEPacketData, Reader, Writer};

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
    };
}

impl MCPEPacketData for bool {
    fn decode(reader: &mut impl Reader) -> Option<Self> {
        Some(reader.next()? == 1)
    }

    fn encode(&self, writer: &mut impl Writer) -> Option<()> {
        writer.write(if *self { 1 } else { 0 })
    }
}

impl<const N: usize> MCPEPacketData for [u8; N] {
    fn decode(reader: &mut impl Reader) -> Option<Self> {
        reader.next_array()
    }

    fn encode(&self, writer: &mut impl Writer) -> Option<()> {
        writer.write_slice(self)
    }
}

impl<T: MCPEPacketData, const N: usize> MCPEPacketData for [T; N] {
    fn decode(reader: &mut impl Reader) -> Option<Self> {
        (0..N)
            .map(|_| T::decode(reader))
            .collect::<Option<Vec<_>>>()?
            .try_into()
            .ok()
    }

    fn encode(&self, writer: &mut impl Writer) -> Option<()> {
        for i in self {
            i.encode(writer)?;
        }
        Some(())
    }
}

primitive!(i8, 1); // Named byte in protocol
primitive!(u16, 2); // Named unsigned byte in protocol
primitive!(i16, 2); // Named byte in protocol
primitive!(u32, 4); // Named unsigned int in protocol
primitive!(i32, 4); // Named int in protocol
primitive!(u64, 8); // Named unsigned long in protocol
primitive!(i64, 8); // Named long in protocol
