use crate::packets::bedrock_objects::utils::bit_utils::BitInformation;
use crate::packets::traits::PacketDecoder;

use super::utils::zig_zag::ZigZag;

pub struct VarU32(pub u32);

impl PacketDecoder for VarU32 {
    fn read(iter: &mut crate::packets::traits::U8Iter) -> Option<Self> {
        let mut shift_amount: u32 = 0;
        let mut decoded_value: u32 = 0;
        loop {
            let next_byte = iter.next()?;
            decoded_value |= ((next_byte & 0b01111111) as u32) << shift_amount;
            if next_byte.has_most_signifigant_bit() {
                shift_amount += 7;
            } else {
                return Some(Self(decoded_value));
            }
        }
    }

    fn write(self, vec: &mut Vec<u8>) -> Option<()> {
        let mut value: u32 = self.0;
        if value == 0 {
            vec.push(0);
        } else {
            while value >= 0b10000000 {
                vec.push(((value & 0b01111111) as u8) | 0b10000000);
                value = value >> 7;
            }
            vec.push((value & 0b01111111) as u8);
        }
        Some(())
    }
}

pub struct VarI32(pub i32);

impl PacketDecoder for VarI32 {
    fn read(iter: &mut crate::packets::traits::U8Iter) -> Option<Self> {
        Some(Self(VarU32::read(iter)?.0.zigzag()))
    }

    fn write(self, vec: &mut Vec<u8>) -> Option<()> {
        VarU32(self.0.zigzag()).write(vec)
    }
}
