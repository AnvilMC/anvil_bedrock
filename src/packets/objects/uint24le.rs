use crate::packets::traits::{PacketDecoder, U8Iter};

#[derive(Debug)]
pub struct UInt24Le(pub u32);

impl PacketDecoder for UInt24Le {
    fn read(iter: &mut U8Iter) -> Option<Self> {
        Some(UInt24Le(u32::from_le_bytes([0,iter.next()?,iter.next()?,iter.next()?])))
    }

    fn write(self, vec: &mut Vec<u8>) -> Option<()> {
        let bytes = self.0.to_le_bytes();
        vec.push(bytes[1]);
        vec.push(bytes[2]);
        vec.push(bytes[3]);
        Some(())
    }
}