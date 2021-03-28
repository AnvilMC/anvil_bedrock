use std::collections::{HashMap, HashSet};

use packet_derive::packet;

use crate::packets::traits::IterRead;
use crate::packets::{
    encode,
    objects::uint24le::UInt24Le,
    traits::{Packet, PacketDecoder},
};

/* #[packet(0x80)]
#[derive(Debug)]
pub struct FramePacket {
    pub reliability: Reliability,
    pub split: Option<(i32,u16,i32)>,
    pub message_index: Option<UInt24Le>,
    pub order: Option<(UInt24Le, i8)>,
    pub ack_record: Option<AckRecord>,
    pub payload: Vec<u8>
} */

#[packet(0x84)]
#[derive(Debug)]
pub struct FramePacket {
    pub sequence_number: UInt24Le,
    pub frames: Frames,
}

#[derive(Debug)]
pub struct AckRecord {}

// Il lit les packets
// Si le packet sequenceId < packet précedent
// Réordoner le paquet
// Si le paquet n'a pas split
// Alors
// On merge tous les paquets du buffer

#[derive(Debug, Default)]
pub struct FrameManager {
    last_frames: Option<FrameData>,
    current: u32,
}

#[derive(Debug, Default)]
struct FrameData {
    frames: Vec<FramePacket>,
    number: usize,
    current: usize,
    missing: HashSet<usize>,
    current_data: Vec<u8>
}

impl FrameData {

    fn new(frame: FramePacket) -> Self {
        let n = frame.frames.split.unwrap().0 as usize;
        let current = frame.frames.split.unwrap().2 as usize;
        let (frames, current_data) = if current == 0 {
            (Vec::new(),frame.frames.payload)
        } else {
            (vec![frame],Vec::new())
        };
        println!("{:?}",current_data);
        Self {
            number: n,
            missing: (0..n).filter(|x| x != &current).collect(),
            frames,
            current: if current == 0 {
                1
            } else {
                0
            },
            current_data
        }
    }

    fn flatten(&mut self) {

        let current = self.current;
        
        if let Some(e) = self.frames.iter_mut().find_map(|x| {
            let pm = x.frames.split.as_ref().unwrap().2;
            if pm as usize != current {
                return None;
            }
            Some(&mut x.frames.payload)
        }) {
            // println!("A {:?}",e.len());
            self.current_data.append(e);
            // println!("B {:?}",self.current_data.len());
            self.current += 1;
            self.flatten();
        }
    }

    fn add(&mut self,mut frame: FramePacket) {
        let current = frame.frames.split.unwrap().2 as usize;
        self.missing.remove(&current);
        if self.current == current {
            // println!("C {:?}",frame.frames.payload.len());
            self.current_data.append(&mut frame.frames.payload);
            // println!("D {:?}",self.current_data.len());
            self.current+=1;
            self.flatten();
        } else {
            self.frames.push(frame);
        }

    }
}

impl FrameManager {
    pub fn from_packet(&mut self, packet: impl Packet + PacketDecoder) -> FramePacket {
        let tmp = FramePacket {
            sequence_number: UInt24Le(self.current),
            frames: Frames {
                reliable: UNRELIABLE,
                reliable_index: None,
                sequenced_index: None,
                order: None,
                split: None,
                payload: encode(packet),
            },
        };
        self.current += 1;
        tmp
    }

    pub fn get_range(&self) -> usize {
        self.last_frames.as_ref().map(|x| x.current).unwrap_or(0)
    }

    pub fn append(&mut self, frames: FramePacket) -> (usize,Option<Vec<u8>>) {
        if frames.frames.split.is_none() {
            // println!("NOT SPLITED {:?}",&frames.frames.payload[..4]);
            return (0,Some(frames.frames.payload));
        }
        // println!("SPLITED {:?}",&frames.frames.payload[..4]);
        if let Some(e) = self.last_frames.as_mut() {
            e.add(frames);
            println!("{} {}",e.current,e.number);
            if e.current < e.number {
                return (e.current,None);
            }
        } else {
            let b = FrameData::new(frames);
            let pm = b.current;
            self.last_frames = Some(b);
            return (pm,None);
        };
        let e = self.last_frames.take().unwrap();
        // println!("G {}",e.current_data.len());
        (e.current,Some(e.current_data))
    }
}
#[derive(Debug)]
pub struct Frames {
    pub reliable: Reliability,
    pub reliable_index: Option<UInt24Le>,
    pub sequenced_index: Option<UInt24Le>,
    pub order: Option<(UInt24Le, i8)>,
    pub split: Option<(i32, u16, i32)>,
    pub payload: Vec<u8>,
}

#[derive(Debug)]
pub struct Reliability {
    pub id: u8,
    pub reliable: bool,
    pub ordered: bool,
    pub sequenced: bool,
    pub requires_ack: bool,
}

impl Reliability {
    const fn new(
        id: u8,
        reliable: bool,
        ordered: bool,
        sequenced: bool,
        requires_ack: bool,
    ) -> Self {
        Self {
            id,
            reliable,
            ordered,
            sequenced,
            requires_ack,
        }
    }

    fn lookup(id: u8) -> Option<Self> {
        Some(match id {
            0 => UNRELIABLE,
            1 => UNRELIABLE_SEQUENCED,
            2 => RELIABLE,
            3 => RELIABLE_ORDERED,
            4 => RELIABLE_SEQUENCED,
            5 => UNRELIABLE_WITH_ACK_RECEIPT,
            6 => RELIABLE_WITH_ACK_RECEIPT,
            7 => RELIABLE_ORDERED_WITH_ACK_RECEIPT,
            _ => return None,
        })
    }

    fn compute_flag(&self, is_split: bool) -> u8 {
        (self.id << FLAG_RELIABILITY_INDEX) | if is_split { FLAG_SPLIT } else { 0 }
    }
}

pub const UNRELIABLE: Reliability = Reliability::new(0, false, false, false, false);
pub const UNRELIABLE_SEQUENCED: Reliability = Reliability::new(1, false, false, true, false);
pub const RELIABLE: Reliability = Reliability::new(2, true, false, false, false);
pub const RELIABLE_ORDERED: Reliability = Reliability::new(3, true, true, false, false);
pub const RELIABLE_SEQUENCED: Reliability = Reliability::new(4, true, false, true, false);
pub const UNRELIABLE_WITH_ACK_RECEIPT: Reliability = Reliability::new(5, false, false, false, true);
pub const RELIABLE_WITH_ACK_RECEIPT: Reliability = Reliability::new(6, true, false, false, true);
pub const RELIABLE_ORDERED_WITH_ACK_RECEIPT: Reliability =
    Reliability::new(7, true, true, false, true);

const FLAG_RELIABILITY_INDEX: u8 = 5;
const FLAG_RELIABILITY: u8 = 0b11100000;
const FLAG_SPLIT: u8 = 0b00010000;

impl PacketDecoder for FramePacket {
    fn read(iter: &mut crate::packets::traits::U8Iter) -> Option<Self> {
        let sequence_number: UInt24Le = iter.read()?;

        let flags = iter.next()?;
        let reliability =
            Reliability::lookup((flags & FLAG_RELIABILITY) >> FLAG_RELIABILITY_INDEX)?;
        let length: u16 = iter.read()?;
        //let length: usize = (length / 256) as usize;
        let length = (length >> 3) as usize;
        let message_index: Option<UInt24Le> = if reliability.reliable {
            Some(iter.read()?)
        } else {
            None
        };
        let sequenced_number: Option<UInt24Le> = if reliability.sequenced {
            Some(iter.read()?)
        } else {
            None
        };
        let order: Option<(UInt24Le, i8)> = if reliability.ordered || reliability.sequenced {
            Some((iter.read()?, iter.read()?))
        } else {
            None
        };
        // Size i32
        // id u16
        // index i32
        let split: Option<(i32, u16, i32)> = if (flags & FLAG_SPLIT) > 0 {
            Some((iter.read()?, iter.read()?, iter.read()?))
        } else {
            None
        };
        let payload: Vec<u8> = iter.take(length).collect();
        if payload.len() != length {
            return None;
        }

        /* Some(Self {
            reliability,
            split,
            message_index,
            order,
            payload,
            ack_record: None,

        }) */

        Some(Self {
            sequence_number,
            frames: Frames {
                //flags,
                reliable: reliability,
                reliable_index: message_index,
                sequenced_index: sequenced_number,
                order,
                split,
                payload,
            },
        })
    }

    fn write(mut self, vec: &mut Vec<u8>) -> Option<()> {
        self.sequence_number.write(vec);

        vec.push(
            self.frames
                .reliable
                .compute_flag(self.frames.split.is_some()),
        );

        ((self.frames.payload.len() << 3) as u16).write(vec);

        if let Some(e) = self.frames.reliable_index {
            e.write(vec);
        }

        if let Some(e) = self.frames.sequenced_index {
            e.write(vec);
        }

        if let Some((order_index, order_channel)) = self.frames.order {
            order_index.write(vec);
            order_channel.write(vec);
        }

        if let Some((split_count, split_id, split_index)) = self.frames.split {
            split_count.write(vec);
            split_id.write(vec);
            split_index.write(vec);
        }

        vec.append(&mut self.frames.payload);

        Some(())
    }
}

impl FramePacket {
    pub fn from_packet(
        sequence_number: u32,
        reliable: Reliability,
        packet: impl PacketDecoder + Packet,
    ) -> Self {
        let packet = encode(packet);

        Self {
            sequence_number: UInt24Le(sequence_number),
            frames: Frames {
                reliable,
                reliable_index: None,
                sequenced_index: None,
                order: None,
                split: None,
                payload: packet,
            },
        }
    }
}

use crate::packets::client::PacketGameClient;

#[test]
fn test() {
    let bytes: Vec<u8> = vec![132, 1, 0, 0, 96, 6, 128, 1, 0, 0, 0, 0, 0, 0, 19, 4, 164, 169, 147, 117, 74, 188, 6, 23, 0, 228, 222, 0, 0, 0, 0, 254, 128, 0, 0, 0, 0, 0, 0, 4, 227, 14, 150, 164, 169, 147, 117, 16, 0, 0, 0, 6, 23, 0, 228, 222, 0, 0, 0, 0, 32, 1, 0, 0, 40, 74, 3, 100, 4, 227, 14, 150, 164, 169, 147, 117, 0, 0, 0, 0, 4, 83, 235, 255, 254, 228, 222, 4, 83, 238, 127, 254, 228, 222, 4, 83, 232, 127, 254, 228, 222, 4, 63, 87, 255, 243, 228, 222, 4, 63, 87, 229, 254, 228, 222, 4, 86, 1, 227, 65, 228, 222, 4, 255, 255, 255, 255, 0, 0, 4, 255, 255, 255, 255, 0, 0, 4, 255, 255, 255, 255, 0, 0, 4, 255, 255, 255, 255, 0, 0, 4, 255, 255, 255, 255, 0, 0, 4, 255, 255, 255, 255, 0, 0, 4, 255, 255, 255, 255, 0, 0, 4, 255, 255, 255, 255, 0, 0, 4, 255, 255, 255, 255, 0, 0, 4, 255, 255, 255, 255, 0, 0, 4, 255, 255, 255, 255, 0, 0, 4, 255, 255, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 205, 148, 142, 0, 0, 72, 0, 0, 0, 0, 0, 0, 205, 148, 142];
    let frame: FramePacket = FramePacket::read(&mut bytes.into_iter().skip(1)).unwrap();
    let i = &mut frame.frames.payload.into_iter();
    println!("{:?}", PacketGameClient::parse_packet(i));
    println!("{:?}", PacketGameClient::parse_packet(i));
    println!("{:?}", PacketGameClient::parse_packet(i));
    println!("{:?} left", i.collect::<Vec<u8>>());
    //println!("{}",frame.frames.payload.iter().map(|x| format!("{:02x?}",x)).collect::<Vec<_>>().join(" "));
}
// 13 04 a4 a9 93 75 4a bc 06 17 00 e4 de 00 00 00 00 fe 80 00 00 00 00 00 00 04 e3 0e 96 a4 a9 93 75 10 00 00 00 06 17 00 e4 de 00 00 00 00 20 01 00 00 28 4a 03 64 04 e3 0e 96 a4 a9 93 75 00 00 00 00 04 53 eb ff fe e4 de 04 53 ee 7f fe e4 de 04 53 e8 7f fe e4 de 04 3f 57 ff f3 e4 de 04 3f 57 e5 fe e4 de 04 56 01 e3 41 e4 de 04 ff ff ff ff 00 00 04 ff ff ff ff 00 00 04 ff ff ff ff 00 00 04 ff ff ff ff 00 00 04 ff ff ff ff 00 00 04 ff ff ff ff 00 00 04 ff ff ff ff 00 00 04 ff ff ff ff 00 00 04 ff ff ff ff 00 00 04 ff ff ff ff 00 00 04 ff ff ff ff 00 00 04 ff ff ff ff 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 cd 94 8e 00 00 48 00 00 00 00 00 00 cd 94 8e
// 13 04 a4 a9 93 75 4a bc 06 17 00 e4 de 00 00 00 00 fe 80 00 00 00 00 00 00 04 e3 0e 96 a4 a9 93 75 10 00 00 00 06 17 00 e4 de 00 00 00 00 20 01 00 00 28 4a 03 64 04 e3 0e 96 a4 a9 93 75 00 00 00 00 04 53 eb ff fe e4 de 04 53 ee 7f fe e4 de 04 53 e8 7f fe e4 de 04 3f 57 ff f3 e4 de 04 3f 57 e5 fe e4 de 04 56 01 e3 41 e4 de 04 ff ff ff ff 00 00 04 ff ff ff ff 00 00 04 ff ff ff ff 00 00 04 ff ff ff ff 00 00 04 ff ff ff ff 00 00 04 ff ff ff ff 00 00 04 ff ff ff ff 00 00 04 ff ff ff ff 00 00 04 ff ff ff ff 00 00 04 ff ff ff ff 00 00 04 ff ff ff ff 00 00 04 ff ff ff ff 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 cd 94 8e
