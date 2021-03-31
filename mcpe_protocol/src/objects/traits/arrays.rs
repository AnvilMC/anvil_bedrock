use std::convert::TryInto;

use crate::prelude::{MCPEPacketData, Reader, UnsignedVarInt, Writer};

#[derive(Debug)]
pub struct ByteArray(pub Vec<u8>);

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

pub struct ReadToEndVec<T: MCPEPacketData>(pub Vec<T>);

pub struct StaticData<'a, T: MCPEPacketData>(pub &'a [T]);

impl<T: MCPEPacketData> MCPEPacketData for StaticData<'_, T> {
    fn decode(_: &mut impl Reader) -> Option<Self> {
        todo!()
    }

    fn encode(&self, writer: &mut impl Writer) -> Option<()> {
        for i in self.0 {
            i.encode(writer)?;
        }
        Some(())
    }
}

impl<T: MCPEPacketData> MCPEPacketData for ReadToEndVec<T> {
    fn decode(reader: &mut impl Reader) -> Option<Self> {
        let mut out = Vec::new();
        while let Some(e) = T::decode(reader) {
            out.push(e);
        }
        Some(Self(out))
    }

    fn encode(&self, writer: &mut impl Writer) -> Option<()> {
        for i in &self.0 {
            i.encode(writer)?;
        }
        Some(())
    }
}
