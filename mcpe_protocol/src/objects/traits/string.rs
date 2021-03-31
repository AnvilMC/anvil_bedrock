use crate::prelude::{MCPEPacketData, Reader, UnsignedVarInt, Writer};

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
    fn decode(_: &mut impl Reader) -> Option<Self> {
        todo!()
    }

    fn encode(&self, writer: &mut impl Writer) -> Option<()> {
        UnsignedVarInt(self.len() as u32).encode(writer)?;
        writer.write_slice(self.as_bytes())
    }
}
