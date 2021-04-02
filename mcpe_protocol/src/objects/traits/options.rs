use crate::prelude::{MCPEPacketData, MCPEPacketDataError};

impl<T: MCPEPacketData> MCPEPacketData for Option<T> {
    fn decode(reader: &mut impl crate::prelude::Reader) -> Result<Self, MCPEPacketDataError> {
        if reader.next().map_err(|x| x.map("option_discriminant"))? == 1 {
            Ok(Some(T::decode(reader).map_err(|x| x.map("option_value"))?))
        } else {
            Ok(None)
        }
    }

    fn encode(&self, writer: &mut impl crate::prelude::Writer) -> Result<(), MCPEPacketDataError> {
        match self {
            Some(e) => {
                writer.write(1).map_err(|x| x.map("option_discriminant"))?;
                e.encode(writer).map_err(|x| x.map("option_value"))
            }
            None => writer.write(0).map_err(|x| x.map("option_discriminant")),
        }
    }
}
