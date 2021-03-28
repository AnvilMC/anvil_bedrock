use std::collections::HashMap;

use packet_derive::packet;

use crate::packets::{objects::uint24le::UInt24Le, traits::PacketDecoder};
use crate::packets::traits::IterRead;

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
pub struct AckRecord {

}

// Il lit les packets
// Si le packet sequenceId < packet précedent
// Réordoner le paquet
// Si le paquet n'a pas split 
// Alors 
// On merge tous les paquets du buffer

#[derive(Debug,Default)]
pub struct FrameManager {
    last_frames: HashMap<u32,Vec<FramePacket>>,
}

impl FrameManager {

    pub fn append(&mut self, frames: FramePacket) -> Option<Vec<u8>> {
        if frames.frames.split.is_none() {
            return Some(frames.frames.payload);
        }
        let seqn = if let Some(e) = self.last_frames.get_mut(&frames.sequence_number.0) {
            let total_len = frames.frames.split.unwrap().2;
            let seqn = frames.sequence_number.0;
            e.push(frames);
            if e.iter().map(|x| x.frames.split.unwrap().0).sum::<i32>() == total_len {
                seqn
            } else {
                return None;
            }
        } else {
            self.last_frames.insert(frames.sequence_number.0, vec![frames]);
            return None;
        };
        let mut p = self.last_frames.remove(&seqn).unwrap();
        p.sort_by(|x,y| x.frames.split.unwrap().2.cmp(&y.frames.split.unwrap().2));
        Some(p.into_iter().map(|x| x.frames.payload).flatten().collect())
    }
}
#[derive(Debug)]
pub struct Frames {
    pub reliable: Reliability,
    pub length: u16,
    pub reliable_index: Option<UInt24Le>,
    pub sequenced_index: Option<UInt24Le>,
    pub order: Option<(UInt24Le, i8)>,
    pub split: Option<(i32,u16,i32)>,
    pub payload: Vec<u8>
}


#[derive(Debug)]
pub struct Reliability {
    pub id: u8,
    pub reliable: bool,
    pub ordered: bool,
    pub sequenced: bool,
    pub requires_ack: bool,
    pub need_bas: bool,
}

impl Reliability {
    const fn new(id: u8, reliable: bool, ordered: bool, sequenced: bool, requires_ack: bool) -> Self {
        Self {
            id,
            reliable,
            ordered,
            sequenced,
            requires_ack,
            need_bas: false
        }
    }

    pub fn set_bas(mut self, bas:bool) -> Self {
        self.need_bas = bas;
        self
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
            _ => return None
        })
    }

    fn compute_flag(&self, is_split: bool) -> u8 {
        (self.id << FLAG_RELIABILITY_INDEX) | if is_split {
            FLAG_SPLIT
        }else {
            0
        } | if self.need_bas {
            FLAG_BAS
        } else {
            0
        }
    }
}

pub const UNRELIABLE: Reliability = Reliability::new(0,false,false,false,false);
pub const UNRELIABLE_SEQUENCED: Reliability = Reliability::new(1, false, false, true, false);
pub const RELIABLE: Reliability = Reliability::new(2, true, false, false, false);
pub const RELIABLE_ORDERED: Reliability = Reliability::new(3, true, true, false, false);
pub const RELIABLE_SEQUENCED: Reliability = Reliability::new(4, true, false, true, false);
pub const UNRELIABLE_WITH_ACK_RECEIPT: Reliability = Reliability::new(5, false, false, false, true);
pub const RELIABLE_WITH_ACK_RECEIPT: Reliability = Reliability::new(6, true, false, false, true);
pub const RELIABLE_ORDERED_WITH_ACK_RECEIPT: Reliability = Reliability::new(7, true, true, false, true);

const FLAG_RELIABILITY_INDEX: u8 = 5;
const FLAG_RELIABILITY: u8 = 0b11100000;
const FLAG_SPLIT: u8  = 0b00010000;
const FLAG_BAS: u8  = 0b00001000;

impl PacketDecoder for FramePacket {
    fn read(iter: &mut crate::packets::traits::U8Iter) -> Option<Self> {

        let sequence_number: UInt24Le = iter.read()?;

        let flags = iter.next()?;
        let reliability = Reliability::lookup((flags & FLAG_RELIABILITY) >> FLAG_RELIABILITY_INDEX)?;
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
        let order: Option<(UInt24Le,i8)> = if reliability.ordered || reliability.sequenced {
            Some((iter.read()?,iter.read()?))
        } else {
            None
        };
        // Size i32
        // id u16
        // index i32
        let split: Option<(i32,u16,i32)> = if (flags & FLAG_SPLIT) > 0 {
            Some((iter.read()?,iter.read()?,iter.read()?))
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
                length: length as u16/* : (length / 256) as u16 */,
                reliable_index: message_index,
                sequenced_index: sequenced_number,
                order,
                split,
                payload,
            },
        }

        )
    }

    fn write(mut self, vec: &mut Vec<u8>) -> Option<()> {
        self.sequence_number.write(vec);

        vec.push(self.frames.reliable.compute_flag(self.frames.split.is_some()));

        ((self.frames.payload.len() << 3) as u16).write(vec);
        
        if let Some(e) = self.frames.reliable_index {
            e.write(vec);
        }

        if let Some(e) = self.frames.sequenced_index {
            e.write(vec);
        }

        if let Some((order_index,order_channel)) = self.frames.order {
            order_index.write(vec);
            order_channel.write(vec);
        }

        if let Some((split_count,split_id,split_index)) = self.frames.split {
            split_count.write(vec);
            split_id.write(vec);
            split_index.write(vec);
        }

        vec.append(&mut self.frames.payload);

        Some(())
    }  
}