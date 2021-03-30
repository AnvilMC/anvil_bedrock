use std::{borrow::Cow, convert::TryInto};

use crate::{
    prelude::UnsignedVarInt,
    traits::{MCPEPacketData, Reader, Writer},
};

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
    };
}

#[derive(Debug)]
pub struct ByteArrayEncapsulated<T>(pub T);

impl<T: MCPEPacketData> MCPEPacketData for ByteArrayEncapsulated<T> {
    fn decode(reader: &mut impl Reader) -> Option<Self> {
        Some(ByteArrayEncapsulated(T::decode(
            &mut ByteArray::decode(reader)?.0.iter(),
        )?))
    }

    fn encode(&self, writer: &mut impl Writer) -> Option<()> {
        let mut buffer = Vec::new();
        self.0.encode(&mut buffer)?;
        ByteArray(buffer).encode(writer)
    }
}

pub enum MaybeOwned<'a, T> {
    Owned(T),
    Borrowed(&'a T),
}

impl<T> AsRef<T> for MaybeOwned<'_, T> {
    fn as_ref(&self) -> &T {
        match self {
            MaybeOwned::Owned(e) => &e,
            MaybeOwned::Borrowed(e) => e,
        }
    }
}

#[derive(Debug)]
pub struct ByteArray(pub Vec<u8>);

pub struct Lifetimed<'a, T>(pub MaybeOwned<'a, T>);

impl<'a, T: MCPEPacketData> MCPEPacketData for Lifetimed<'a, T> {
    fn decode(reader: &mut impl Reader) -> Option<Self> {
        Some(Self(MaybeOwned::Owned(T::decode(reader)?)))
    }

    fn encode(&self, writer: &mut impl Writer) -> Option<()> {
        self.0.as_ref().encode(writer)
    }
}

impl MCPEPacketData for ByteArray {
    fn decode(reader: &mut impl Reader) -> Option<Self> {
        let length = UnsignedVarInt::decode(reader)?.0 as usize;

        let binary = reader.read(length)?;
        // std::fs::write("login_packet.bin", &binary).unwrap();

        Some(ByteArray(binary))
    }

    fn encode(&self, writer: &mut impl Writer) -> Option<()> {
        UnsignedVarInt(self.0.len() as u32).encode(writer)?;
        writer.write_slice(&self.0)
    }
}

// TODO: Check for encoding / decoding issue
impl MCPEPacketData for String {
    fn decode(reader: &mut impl Reader) -> Option<Self> {
        //let length = UnsignedVarInt::decode(reader)?.0 as usize;
        let length = u32::from_le_bytes(<[u8; 4]>::decode(reader)?) as usize;

        let binary = reader.read(length)?;

        String::from_utf8(binary).ok()
    }

    fn encode(&self, writer: &mut impl Writer) -> Option<()> {
        UnsignedVarInt(self.len() as u32).encode(writer)?;
        writer.write_slice(self.as_bytes())
    }
}

impl MCPEPacketData for &'_ str {
    fn decode(reader: &mut impl Reader) -> Option<Self> {
        todo!()
    }

    fn encode(&self, writer: &mut impl Writer) -> Option<()> {
        UnsignedVarInt(self.len() as u32).encode(writer)?;
        writer.write_slice(self.as_bytes())
    }
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
#[derive(Debug)]
pub struct Le<T>(pub T);

primitive!(i8, 1); // Named byte in protocol
primitive!(u16, 2); // Named unsigned byte in protocol
primitive!(i16, 2); // Named byte in protocol
primitive!(u32, 4); // Named unsigned int in protocol
primitive!(i32, 4); // Named int in protocol
primitive!(u64, 8); // Named unsigned long in protocol
primitive!(i64, 8); // Named long in protocol
primitive!(f32, 4); // Named float in protocol
