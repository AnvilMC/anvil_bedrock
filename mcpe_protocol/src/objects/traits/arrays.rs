use std::convert::TryInto;

use crate::prelude::{
    MCPEPacketData, MCPEPacketDataError, Reader, UnsignedVarInt, VecIndexed, Writer,
};

// #[derive(Debug)]
// pub struct ByteArray(pub Vec<u8>);

pub type ByteArray = VecIndexed<u8, UnsignedVarInt>;

impl<T: MCPEPacketData, const N: usize> MCPEPacketData for [T; N] {
    fn decode(reader: &mut impl Reader) -> Result<Self, MCPEPacketDataError> {
        (0..N)
            .map(|_| T::decode(reader))
            .collect::<Result<Vec<_>, _>>()
            .map_err(|x| x.map("const_array_value"))?
            .try_into()
            .map_err(|_| {
                MCPEPacketDataError::new("const_array_value", "Invalid const_array_value length")
            })
    }

    fn encode(&self, writer: &mut impl Writer) -> Result<(), MCPEPacketDataError> {
        for i in self {
            i.encode(writer).map_err(|x| x.map("const_array_value"))?;
        }
        Ok(())
    }
}

// impl MCPEPacketData for ByteArray {
//     fn decode(reader: &mut impl Reader) -> Result<Self, MCPEPacketDataError> {
//         let length = UnsignedVarInt::decode(reader)?.0 as usize;

//         let binary = reader.read(length)?;
//         // std::fs::write("login_packet.bin", &binary).unwrap();

//         Some(ByteArray(binary))
//     }

//     fn encode(&self, writer: &mut impl Writer) -> Result<(), MCPEPacketDataError> {
//         UnsignedVarInt(self.0.len() as u32).encode(writer)?;
//         writer.write_slice(&self.0)
//     }
// }

#[derive(Debug)]
pub struct ByteArrayEncapsulated<T>(pub T);

impl<T: MCPEPacketData> MCPEPacketData for ByteArrayEncapsulated<T> {
    fn decode(reader: &mut impl Reader) -> Result<Self, MCPEPacketDataError> {
        Ok(ByteArrayEncapsulated(
            T::decode(&mut ByteArray::decode(reader)?.0.iter())
                .map_err(|x| x.map("byte_array_encapsulated_value"))?,
        ))
    }

    fn encode(&self, writer: &mut impl Writer) -> Result<(), MCPEPacketDataError> {
        let mut buffer = Vec::new();
        self.0
            .encode(&mut buffer)
            .map_err(|x| x.map("byte_array_encapsulated_buffer"))?;
        ByteArray::from(buffer)
            .encode(writer)
            .map_err(|x| x.map("byte_array_encapsulated_value"))
    }
}

#[derive(Debug)]
pub struct ReadToEndVec<T: MCPEPacketData>(pub Vec<T>);

#[derive(Debug, Clone)]
pub struct StaticData<'a, T: MCPEPacketData>(pub &'a [T]);

impl<T: MCPEPacketData> MCPEPacketData for StaticData<'_, T> {
    fn decode(_: &mut impl Reader) -> Result<Self, MCPEPacketDataError> {
        todo!()
    }

    fn encode(&self, writer: &mut impl Writer) -> Result<(), MCPEPacketDataError> {
        for i in self.0 {
            i.encode(writer).map_err(|x| x.map("static_data"))?;
        }
        Ok(())
    }
}

impl<T: MCPEPacketData> MCPEPacketData for ReadToEndVec<T> {
    fn decode(reader: &mut impl Reader) -> Result<Self, MCPEPacketDataError> {
        let mut out = Vec::new();
        while let Ok(e) = T::decode(reader) {
            out.push(e);
        }
        Ok(Self(out))
    }

    fn encode(&self, writer: &mut impl Writer) -> Result<(), MCPEPacketDataError> {
        for i in &self.0 {
            i.encode(writer).map_err(|x| x.map("read_to_end_value"))?;
        }
        Ok(())
    }
}
