use crate::prelude::{RaknetPacketData, RaknetUInt24Le};

use super::FramePacket;

use super::*;

impl RaknetPacket for FramePacket {
    const RANGE: std::ops::Range<u8> = 0x80..0x8E;

    fn id(&self) -> u8 {
        0x84
    }
}

impl RaknetPacketData for FramePacket {
    fn decode(reader: &mut impl crate::prelude::Reader) -> Option<Self> {
        let sequence_id = RaknetUInt24Le::decode(reader)?.0;
        let flags = reader.next()?;
        let reliability =
            Reliability::lookup((flags & FLAG_RELIABILITY) >> super::FLAG_RELIABILITY_INDEX)?;
        let length = (u16::decode(reader)? >> 3) as usize;
        let reliable_index = if reliability.reliable {
            Some(RaknetUInt24Le::decode(reader)?.0)
        } else {
            None
        };
        let sequenced_index = if reliability.sequenced {
            Some(RaknetUInt24Le::decode(reader)?.0)
        } else {
            None
        };
        let order = if reliability.ordered || reliability.sequenced {
            Some((RaknetUInt24Le::decode(reader)?.0, i8::decode(reader)?))
        } else {
            None
        };
        let split: Option<SplitInfo> = if (flags & FLAG_SPLIT) > 0 {
            Some(SplitInfo {
                size: i32::decode(reader)? as u32,
                id: u16::decode(reader)?,
                index: i32::decode(reader)? as u32,
            })
        } else {
            None
        };
        let payload: Vec<u8> = (0..length).map(|_| reader.next()).collect::<Option<_>>()?;
        Some(Self {
            sequence_id,
            reliability,
            reliable_index,
            sequenced_index,
            order,
            split,
            payload,
        })
    }

    fn encode(&self, writer: &mut impl crate::prelude::Writer) -> Option<()> {
        RaknetUInt24Le(self.sequence_id).encode(writer)?;

        writer.write(self.reliability.compute_flag(self.split.is_some()));

        ((self.payload.len() << 3) as u16).encode(writer);

        if let Some(e) = self.reliable_index {
            RaknetUInt24Le(e).encode(writer)?;
        }

        if let Some(e) = self.sequenced_index {
            RaknetUInt24Le(e).encode(writer)?;
        }

        if let Some((order_index, order_channel)) = self.order {
            RaknetUInt24Le(order_index).encode(writer)?;
            order_channel.encode(writer)?;
        }

        if let Some(split) = &self.split {
            split.size.encode(writer)?;
            split.id.encode(writer)?;
            split.index.encode(writer)?;
        }

        writer.write_slice(&self.payload);

        Some(())
    }
}
