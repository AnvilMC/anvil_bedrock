use crate::prelude::MCPEPacketData;

pub struct TakeAll(pub Vec<u8>);

impl MCPEPacketData for TakeAll {
    fn decode(reader: &mut impl crate::prelude::Reader) -> Option<Self> {
        Some(Self(reader.read_to_end()))
    }

    fn encode(&self, writer: &mut impl crate::prelude::Writer) -> Option<()> {
        writer.write_slice(&self.0);
        Some(())
    }
}
