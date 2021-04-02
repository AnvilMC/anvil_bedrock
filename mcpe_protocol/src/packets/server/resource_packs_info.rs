use packet_derive::{packet, MCPEPacketDataAuto};

use crate::prelude::Le;

#[packet(0x06)]
#[derive(Debug, MCPEPacketDataAuto)]
pub struct ResourcePacksInfo {
    pub accept: bool,
    pub scripting: bool,
    pub behaviour_pack_size: Le<u16>,
    pub resource_pack_size: Le<u16>,
}

impl Default for ResourcePacksInfo {
    fn default() -> Self {
        Self {
            accept: false,
            scripting: false,
            behaviour_pack_size: Le(0),
            resource_pack_size: Le(0),
        }
    }
}

// impl MCPEPacketData for ResourcePacksInfo {
//     fn decode(reader: &mut impl crate::prelude::Reader) -> Result<Self, MCPEPacketDataError> {
//         Some(Self {
//             accept: bool::decode(reader)?,
//             scripting: bool::decode(reader)?,
//             behaviour_pack_size: <Le<u16>>::decode(reader)?,
//             resource_pack_size: <Le<u16>>::decode(reader)?,
//         })
//     }

//     fn encode(&self, writer: &mut impl crate::prelude::Writer) -> Result<(), MCPEPacketDataError> {
//         self.accept.encode(writer)?;
//         self.scripting.encode(writer)?;
//         self.behaviour_pack_size.encode(writer)?;
//         self.resource_pack_size.encode(writer)
//     }
// }
