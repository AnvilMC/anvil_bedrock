use std::collections::HashMap;

mod parsing;
mod reliability;

pub use reliability::*;

use crate::prelude::RaknetPacket;

use super::{Ack, Record};

#[derive(Default)]
pub struct FrameManager {
    current_frame_number: u32,
    frame_parts: Option<FramePart>,
}

impl FrameManager {
    pub fn process(&mut self, packet: FramePacket) -> (Option<Ack>, Option<Vec<u8>>) {
        if packet.split.is_some() {
            if let Some(e) = &mut self.frame_parts {
                let acc = e.current;
                e.add(packet);
                if e.current < e.frame_number {
                    return (
                        if acc != e.current {
                            Some(Ack {
                                record: vec![Record(0, e.current)],
                            })
                        } else {
                            None
                        },
                        None,
                    );
                }
            } else {
                let p = FramePart::new(packet);
                let acc = p.current;
                self.frame_parts = Some(p);
                return (
                    if acc == 1 {
                        Some(Ack {
                            record: vec![Record(0, 1)],
                        })
                    } else {
                        None
                    },
                    None,
                );
            }
        } else {
            return (
                Some(Ack {
                    record: vec![Record(0, 0)],
                }),
                Some(packet.payload),
            );
        };
        let p = self.frame_parts.take().unwrap();
        (
            Some(Ack {
                record: vec![Record(0, p.frame_number + 4)],
            }),
            Some(p.data),
        )
    }

    pub fn encode_as_frame(&mut self, packet: impl RaknetPacket) -> FramePacket {
        let tmp = FramePacket {
            sequence_id: self.current_frame_number,
            reliability: UNRELIABLE,
            reliable_index: None,
            sequenced_index: None,
            order: None,
            split: None,
            payload: {
                let mut buffer = Vec::with_capacity(1024 * 1024);
                buffer.push(packet.id());
                packet.encode(&mut buffer);
                buffer
            },
        };
        self.current_frame_number += 1;
        tmp
    }
}

struct FramePart {
    pub data: Vec<u8>,
    pub frame_number: u32,
    pub current: u32,
    pub waiting_frames: HashMap<u32, Vec<u8>>,
}

impl FramePart {
    fn new(packet: FramePacket) -> Self {
        let p = packet.split.as_ref().unwrap();
        let current = p.index;
        let frame_number = p.size;
        let (waiting_frames, data) = if current == 0 {
            (HashMap::new(), packet.payload)
        } else {
            let mut map = HashMap::with_capacity(1);
            map.insert(current, packet.payload);
            (map, vec![])
        };
        Self {
            data,
            frame_number,
            current: if current == 0 { 1 } else { 0 },
            waiting_frames,
        }
    }

    fn add(&mut self, mut packet: FramePacket) {
        let p = packet.split.as_ref().unwrap();
        if self.current > p.index || self.waiting_frames.contains_key(&p.index) {
            println!("CORONED");
            return;
        }
        println!("EBOLED");
        if self.current == p.index {
            self.data.append(&mut packet.payload);
            self.current += 1;
            println!("PESTED");
            while let Some(mut e) = self.waiting_frames.remove(&self.current) {
                self.data.append(&mut e);
                self.current += 1;
            }
        } else {
            self.waiting_frames.insert(p.index, packet.payload);
        }
    }
}

pub struct FramePacket {
    pub sequence_id: u32,
    reliability: Reliability,
    reliable_index: Option<u32>,
    sequenced_index: Option<u32>,
    order: Option<(u32, i8)>,
    split: Option<SplitInfo>,
    pub payload: Vec<u8>,
}

struct SplitInfo {
    pub size: u32,
    pub id: u16,
    pub index: u32,
}
