use crate::packets::traits::{PacketDecoder, U8Iter};

#[derive(Debug, Clone, PartialEq)]
pub struct UInt24Le(pub u32);

impl PacketDecoder for UInt24Le {
    fn read(iter: &mut U8Iter) -> Option<Self> {
        Some(UInt24Le(u32::from_le_bytes([iter.next()?,iter.next()?,iter.next()?,0])))
    }

    fn write(self, vec: &mut Vec<u8>) -> Option<()> {
        let bytes = self.0.to_le_bytes();
        vec.push(bytes[0]);
        vec.push(bytes[1]);
        vec.push(bytes[2]);
        Some(())
    }
}