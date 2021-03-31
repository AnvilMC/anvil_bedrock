use crate::prelude::MCPEPacketData;

impl<T: MCPEPacketData> MCPEPacketData for Option<T> {
    fn decode(reader: &mut impl crate::prelude::Reader) -> Option<Self> {
        if reader.next()? == 1 {
            Some(Some(T::decode(reader)?))
        } else {
            Some(None)
        }
    }

    fn encode(&self, writer: &mut impl crate::prelude::Writer) -> Option<()> {
        match self {
            Some(e) => {
                writer.write(1)?;
                e.encode(writer)
            }
            None => writer.write(0),
        }
    }
}
