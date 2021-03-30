use crate::prelude::{BitInformation, MCPEPacketData, ZigZag};

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

pub struct VarInt(pub i32);

impl MCPEPacketData for VarInt {
    fn decode(reader: &mut impl crate::prelude::Reader) -> Option<Self> {
        Some(Self(UnsignedVarInt::decode(reader)?.0.zigzag()))
    }

    fn encode(&self, writer: &mut impl crate::prelude::Writer) -> Option<()> {
        let a: u8 = UnsignedVarInt(self.0.zigzag()).0 as u8;
        writer.write(a)
    }
}
