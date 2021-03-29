use crate::prelude::{RaknetPacketData, Reader, Writer};

#[derive(Clone)]
pub struct RaknetUInt24Le(pub u32);

impl RaknetPacketData for RaknetUInt24Le {
    fn decode(iter: &mut impl Reader) -> Option<Self> {
        Some(Self(u32::from_le_bytes([
            iter.next()?,
            iter.next()?,
            iter.next()?,
            0,
        ])))
    }

    fn encode(&self, vec: &mut impl Writer) -> Option<()> {
        vec.write_slice(&self.0.to_le_bytes()[0..3]);
        Some(())
    }
}
