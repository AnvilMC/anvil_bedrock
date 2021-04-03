use crate::prelude::{BitInformation, Indexable, MCPEPacketData, MCPEPacketDataError, ZigZag};

#[derive(Debug)]
pub struct UnsignedVarInt(pub u32);

impl MCPEPacketData for UnsignedVarInt {
    fn decode(reader: &mut impl crate::prelude::Reader) -> Result<Self, MCPEPacketDataError> {
        let mut shift_amount: u32 = 0;
        let mut decoded_value: u32 = 0;
        loop {
            let next_byte = reader.next().map_err(|x| x.map("unsigned_var_int"))?;
            decoded_value |= ((next_byte & 0b01111111) as u32) << shift_amount;
            if next_byte.has_most_signifigant_bit() {
                shift_amount += 7;
            } else {
                return Ok(Self(decoded_value));
            }
        }
    }

    fn encode(&self, writer: &mut impl crate::prelude::Writer) -> Result<(), MCPEPacketDataError> {
        let mut value: u32 = self.0;
        if value == 0 {
            writer.write(0).map_err(|x| x.map("unsigned_var_int"))?;
        } else {
            while value >= 0b10000000 {
                writer.write(((value & 0b01111111) as u8) | 0b10000000)?;
                value = value >> 7;
            }
            writer.write((value & 0b01111111) as u8)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct UnsignedVarLong(pub u64);

impl MCPEPacketData for UnsignedVarLong {
    fn decode(reader: &mut impl crate::prelude::Reader) -> Result<Self, MCPEPacketDataError> {
        let mut shift_amount: u64 = 0;
        let mut decoded_value: u64 = 0;
        loop {
            let next_byte = reader.next().map_err(|x| x.map("unsigned_var_long"))?;
            decoded_value |= ((next_byte & 0b01111111) as u64) << shift_amount;
            if next_byte.has_most_signifigant_bit() {
                shift_amount += 7;
            } else {
                return Ok(Self(decoded_value));
            }
        }
    }

    fn encode(&self, writer: &mut impl crate::prelude::Writer) -> Result<(), MCPEPacketDataError> {
        let mut value: u64 = self.0;
        if value == 0 {
            writer.write(0).map_err(|x| x.map("unsigned_var_long"))?;
        } else {
            while value >= 0b10000000 {
                writer.write(((value & 0b01111111) as u8) | 0b10000000)?;
                value = value >> 7;
            }
            writer.write((value & 0b01111111) as u8)?;
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct VarLong(pub i64);

impl MCPEPacketData for VarLong {
    fn decode(reader: &mut impl crate::prelude::Reader) -> Result<Self, MCPEPacketDataError> {
        Ok(Self(
            UnsignedVarLong::decode(reader)
                .map_err(|x| x.map("var_long"))?
                .0
                .zigzag(),
        ))
    }

    fn encode(&self, writer: &mut impl crate::prelude::Writer) -> Result<(), MCPEPacketDataError> {
        UnsignedVarLong(self.0.zigzag())
            .encode(writer)
            .map_err(|x| x.map("var_long"))
    }
}

#[derive(Debug)]
pub struct VarInt(pub i32);

impl MCPEPacketData for VarInt {
    fn decode(reader: &mut impl crate::prelude::Reader) -> Result<Self, MCPEPacketDataError> {
        Ok(Self(
            UnsignedVarInt::decode(reader)
                .map_err(|x| x.map("var_int"))?
                .0
                .zigzag(),
        ))
    }

    fn encode(&self, writer: &mut impl crate::prelude::Writer) -> Result<(), MCPEPacketDataError> {
        UnsignedVarInt(self.0.zigzag())
            .encode(writer)
            .map_err(|x| x.map("var_int"))
    }
}

impl Indexable for VarInt {
    fn as_usize(&self) -> usize {
        self.0 as _
    }

    fn from_usize(u: usize) -> Self {
        Self(u as _)
    }
}

impl Indexable for UnsignedVarInt {
    fn as_usize(&self) -> usize {
        self.0 as _
    }

    fn from_usize(u: usize) -> Self {
        Self(u as _)
    }
}
