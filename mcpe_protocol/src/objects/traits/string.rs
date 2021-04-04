use crate::prelude::{MCPEPacketData, MCPEPacketDataError, Reader, UnsignedVarInt, Writer};

// TODO: Check for encoding / decoding issue
impl MCPEPacketData for String {
    fn decode(reader: &mut impl Reader) -> Result<Self, MCPEPacketDataError> {
        //let length = UnsignedVarInt::decode(reader)?.0 as usize;
        let length =
            u32::from_le_bytes(<[u8; 4]>::decode(reader).map_err(|x| x.map("string_index"))?)
                as usize;

        let binary = reader.read(length).map_err(|x| x.map("string_value"))?;

        String::from_utf8(binary)
            .map_err(|_| MCPEPacketDataError::new("string_value", "Invalid UTF8 sequence"))
    }

    fn encode(&self, writer: &mut impl Writer) -> Result<(), MCPEPacketDataError> {
        UnsignedVarInt(self.len() as u32)
            .encode(writer)
            .map_err(|x| x.map("string_index"))?;
        writer
            .write_slice(self.as_bytes())
            .map_err(|x| x.map("string_value"))
    }
}

impl MCPEPacketData for &'_ str {
    fn decode(_: &mut impl Reader) -> Result<Self, MCPEPacketDataError> {
        todo!()
    }

    fn encode(&self, writer: &mut impl Writer) -> Result<(), MCPEPacketDataError> {
        UnsignedVarInt(self.len() as u32).encode(writer)?;
        writer.write_slice(self.as_bytes())
    }
}
