use crate::prelude::{MCPEPacketData, MCPEPacketDataError};

pub struct TakeAll(pub Vec<u8>);

impl MCPEPacketData for TakeAll {
    fn decode(reader: &mut impl crate::prelude::Reader) -> Result<Self, MCPEPacketDataError> {
        Ok(Self(reader.read_to_end()))
    }

    fn encode(&self, writer: &mut impl crate::prelude::Writer) -> Result<(), MCPEPacketDataError> {
        writer
            .write_slice(&self.0)
            .map_err(|x| x.map("take_all_value"))?;
        Ok(())
    }
}
