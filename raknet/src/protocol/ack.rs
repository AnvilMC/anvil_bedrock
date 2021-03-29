use crate::prelude::{RaknetPacket, RaknetPacketData, RaknetUInt24Le};

#[derive(Debug, PartialEq, Clone)]
pub struct Ack {
    pub record: Vec<Record>,
}

impl RaknetPacket for Ack {
    const RANGE: std::ops::Range<u8> = 0xc0..0xc1;

    fn id(&self) -> u8 {
        0xc0
    }
}

impl RaknetPacketData for Ack {
    fn decode(reader: &mut impl crate::prelude::Reader) -> Option<Self> {
        let count = i16::decode(reader)?;

        Some(Self {
            record: (0..count)
                .map(|_| Record::decode(reader))
                .collect::<Option<_>>()?,
        })
    }

    fn encode(&self, writer: &mut impl crate::prelude::Writer) -> Option<()> {
        (self.record.len() as u16).encode(writer);
        for i in &self.record {
            i.encode(writer)?;
        }
        Some(())
    }
}

// impl PacketDecoder for Ack {
//     fn read(iter: &mut crate::packets::traits::U8Iter) -> Option<Self> {
//         let count: i16 = iter.read()?;
//         Some(Self {
//             record: (0..count).map(|_| iter.read()).collect::<Option<_>>()?,
//         })
//     }

//     fn write(self, vec: &mut Vec<u8>) -> Option<()> {
//         (self.record.len() as i16).write(vec);
//         for i in self.record.into_iter() {
//             i.write(vec)?;
//         }
//         Some(())
//     }
// }

#[derive(Debug, PartialEq, Clone)]
pub struct Record(pub u32, pub u32); // START - END

impl RaknetPacketData for Record {
    fn decode(reader: &mut impl crate::prelude::Reader) -> Option<Self> {
        let (start, end) = if bool::decode(reader)? {
            let o = RaknetUInt24Le::decode(reader)?.0;
            (o, o)
        } else {
            (
                RaknetUInt24Le::decode(reader)?.0,
                RaknetUInt24Le::decode(reader)?.0,
            )
        };
        Some(Record(start, end))
    }

    fn encode(&self, writer: &mut impl crate::prelude::Writer) -> Option<()> {
        if self.0 == self.1 {
            writer.write(1)?;
            RaknetUInt24Le(self.0).encode(writer)
        } else {
            writer.write(0)?;
            RaknetUInt24Le(self.0.min(self.1)).encode(writer)?;
            RaknetUInt24Le(self.1.max(self.0)).encode(writer)
        }
    }
}

// impl PacketDecoder for Record {
//     fn read(iter: &mut crate::packets::traits::U8Iter) -> Option<Self> {
//         Some(Record(if iter.read()? {
//             Either::Left(iter.read()?)
//         } else {
//             Either::Right((iter.read()?, iter.read()?))
//         }))
//     }

//     fn write(self, vec: &mut Vec<u8>) -> Option<()> {
//         match self.0 {
//             Either::Left(e) => {
//                 true.write(vec)?;
//                 e.write(vec)?;
//             }
//             Either::Right((x, y)) => {
//                 false.write(vec)?;
//                 x.write(vec)?;
//                 y.write(vec)?;
//             }
//         }
//         Some(())
//     }
// }
