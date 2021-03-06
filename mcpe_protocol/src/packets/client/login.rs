use packet_derive::packet;

use crate::prelude::{ByteArrayEncapsulated, MCPEPacketData, MCPEPacketDataError};

#[packet(0x01)]
#[derive(Debug)]
pub struct LoginPacket {
    pub protocol_version: i32,
    pub display_name: String,
    pub identity: String,
}

#[derive(serde::Deserialize)]
struct TokenChain {
    chain: Vec<String>,
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize)]
struct Inside {
    extraData: Identity,
}

#[allow(non_snake_case)]
#[derive(serde::Deserialize)]
struct Identity {
    displayName: String,
    identity: String,
}

impl MCPEPacketData for LoginPacket {
    fn decode(reader: &mut impl crate::prelude::Reader) -> Result<Self, MCPEPacketDataError> {
        let protocol_version = i32::decode(reader)?;
        let chain_data = <ByteArrayEncapsulated<String>>::decode(reader)?;
        let json = serde_json::from_str::<TokenChain>(&chain_data.0)
            .map_err(|_| MCPEPacketDataError::new("json_string", "Invalid json"))?
            .chain;
        let data_inside: Identity = json
            .iter()
            .find_map(|x| {
                if let Some(e) = x.find(".") {
                    let x = &x[e + 1..];
                    if let Some(e) = x.find(".") {
                        let base64 = base64::decode(&x[..e]).ok()?;
                        serde_json::from_slice::<Inside>(&base64).ok()
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .ok_or_else(|| MCPEPacketDataError::new("json_string", "No json"))?
            .extraData;
        Ok(Self {
            protocol_version,
            display_name: data_inside.displayName,
            identity: data_inside.identity,
        })
    }

    fn encode(&self, _writer: &mut impl crate::prelude::Writer) -> Result<(), MCPEPacketDataError> {
        todo!()
    }
}
