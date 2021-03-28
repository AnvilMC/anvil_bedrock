use packet_derive::packet;

use crate::packets::{objects::uint24le::UInt24Le, traits::PacketDecoder};
use crate::packets::traits::IterRead;

#[packet(0x80)]
#[derive(Debug)]
pub struct FramePacket {
    pub reliability: Reliability,
    pub split: Option<(i32,u16,i32)>,
    pub message_index: Option<UInt24Le>,
    pub order: Option<(UInt24Le, i8)>,
    pub ack_record: Option<AckRecord>,
    pub payload: Vec<u8>
}

#[derive(Debug)]
pub struct AckRecord {

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
    const fn new(id: u8, reliable: bool, ordered: bool, sequenced: bool, requires_ack: bool) -> Self {
        Self {
            id,
            reliable,
            ordered,
            sequenced,
            requires_ack
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
            _ => return None
        })
    }
}

const UNRELIABLE: Reliability = Reliability::new(0,false,false,false,false);
const UNRELIABLE_SEQUENCED: Reliability = Reliability::new(1, false, false, true, false);
const RELIABLE: Reliability = Reliability::new(2, true, false, false, false);
const RELIABLE_ORDERED: Reliability = Reliability::new(3, true, true, false, false);
const RELIABLE_SEQUENCED: Reliability = Reliability::new(4, true, false, true, false);
const UNRELIABLE_WITH_ACK_RECEIPT: Reliability = Reliability::new(5, false, false, false, true);
const RELIABLE_WITH_ACK_RECEIPT: Reliability = Reliability::new(6, true, false, false, true);
const RELIABLE_ORDERED_WITH_ACK_RECEIPT: Reliability = Reliability::new(7, true, true, false, true);

const FLAG_RELIABILITY_INDEX: u8 = 5;
const FLAG_RELIABILITY: u8 = 0b11100000;
const FLAG_SPLIT: u8  = 0b00010000;

impl PacketDecoder for FramePacket {
    fn read(iter: &mut crate::packets::traits::U8Iter) -> Option<Self> {
        let flags = iter.next()?;
        let reliability = Reliability::lookup((flags & FLAG_RELIABILITY) >> FLAG_RELIABILITY_INDEX)?;
        let length: u16 = iter.read()?;
        let length: usize = (length / 256) as usize;
        let message_index: Option<UInt24Le> = if reliability.reliable {
            Some(iter.read()?)
        } else {
            None
        };
        let order: Option<(UInt24Le,i8)> = if reliability.ordered || reliability.sequenced {
            Some((iter.read()?,iter.read()?))
        } else {
            None
        };
        let split: Option<(i32,u16,i32)> = if (flags & FLAG_SPLIT) > 0 {
            Some((iter.read()?,iter.read()?,iter.read()?))
        } else {
            None
        };
        let payload: Vec<u8> = iter.take(length).collect();
        if payload.len() != length {
            return None;
        }


        Some(Self {
            reliability,
            split,
            message_index,
            order,
            payload,
            ack_record: None,

        })
    }

    fn write(mut self, vec: &mut Vec<u8>) -> Option<()> {
        let mut flags: u8 = 0;
        flags |= self.reliability.id << FLAG_RELIABILITY_INDEX;
        flags |= if self.split.is_some() {
            FLAG_SPLIT
        } else {
            0
        };
        vec.push(flags);
        ((self.payload.len() * 256) as u16).write(vec);
        if let Some(_) = self.ack_record {
            // if e.is_ranged() {
            //     return None;
            // }
        } else if self.reliability.requires_ack {
            return None;
        }
        if self.reliability.reliable {
            if let Some(e) = self.message_index {
                e.write(vec);
            } else {
                return None;
            }
        }
        if self.reliability.ordered || self.reliability.sequenced {
            if let Some((order_index,order_channel)) = self.order {
                order_index.write(vec);
                order_channel.write(vec);
            } else {
                return None;
            }
        }
        if let Some((split_count,split_id,split_index)) = self.split {
            split_count.write(vec);
            split_id.write(vec);
            split_index.write(vec);
        }

        vec.append(&mut self.payload);

        Some(())
    }  
}