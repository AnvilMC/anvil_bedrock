use crate::prelude::{BitInformation, Indexable, MCPEPacketData, ZigZag};

#[derive(Debug)]
pub struct UnsignedVarInt(pub u32);

impl MCPEPacketData for UnsignedVarInt {
    fn decode(reader: &mut impl crate::prelude::Reader) -> Option<Self> {
        let mut shift_amount: u32 = 0;
        let mut decoded_value: u32 = 0;
        loop {
            let next_byte = reader.next()?;
            decoded_value |= ((next_byte & 0b01111111) as u32) << shift_amount;
            if next_byte.has_most_signifigant_bit() {
                shift_amount += 7;
            } else {
                return Some(Self(decoded_value));
            }
        }
    }

    fn encode(&self, writer: &mut impl crate::prelude::Writer) -> Option<()> {
        let mut value: u32 = self.0;
        if value == 0 {
            writer.write(0);
        } else {
            while value >= 0b10000000 {
                writer.write(((value & 0b01111111) as u8) | 0b10000000);
                value = value >> 7;
            }
            writer.write((value & 0b01111111) as u8);
        }
        Some(())
    }
}

#[derive(Debug)]
pub struct UnsignedVarLong(pub u64);

impl MCPEPacketData for UnsignedVarLong {
    fn decode(reader: &mut impl crate::prelude::Reader) -> Option<Self> {
        let mut shift_amount: u64 = 0;
        let mut decoded_value: u64 = 0;
        loop {
            let next_byte = reader.next()?;
            decoded_value |= ((next_byte & 0b01111111) as u64) << shift_amount;
            if next_byte.has_most_signifigant_bit() {
                shift_amount += 7;
            } else {
                return Some(Self(decoded_value));
            }
        }
    }

    fn encode(&self, writer: &mut impl crate::prelude::Writer) -> Option<()> {
        let mut value: u64 = self.0;
        if value == 0 {
            writer.write(0);
        } else {
            while value >= 0b10000000 {
                writer.write(((value & 0b01111111) as u8) | 0b10000000);
                value = value >> 7;
            }
            writer.write((value & 0b01111111) as u8);
        }
        Some(())
    }
}

#[derive(Debug)]
pub struct VarLong(pub i64);

impl MCPEPacketData for VarLong {
    fn decode(reader: &mut impl crate::prelude::Reader) -> Option<Self> {
        Some(Self(UnsignedVarLong::decode(reader)?.0.zigzag()))
    }

    fn encode(&self, writer: &mut impl crate::prelude::Writer) -> Option<()> {
        UnsignedVarLong(self.0.zigzag()).encode(writer)
    }
}

#[derive(Debug)]
pub struct VarInt(pub i32);

impl MCPEPacketData for VarInt {
    fn decode(reader: &mut impl crate::prelude::Reader) -> Option<Self> {
        Some(Self(UnsignedVarInt::decode(reader)?.0.zigzag()))
    }

    fn encode(&self, writer: &mut impl crate::prelude::Writer) -> Option<()> {
        UnsignedVarInt(self.0.zigzag()).encode(writer)
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
