#![feature(min_const_generics)]

use crate::prelude::{FrameManager, FramePacket, GamePacket, RaknetPacketData, RELIABLE_ORDERED};

pub mod objects;
pub mod protocol;
pub mod traits;

pub mod prelude {
    pub use crate::objects::*;
    pub use crate::protocol::*;
    pub use crate::traits::*;
}

fn from_hex(i: u8) -> u8 {
    match i {
        b'0'..=b'9' => i - b'0',
        b'A'..=b'F' => i - b'A' + 10,
        b'a'..=b'f' => i - b'a' + 10,
        _ => panic!("WINDOZE FATALE ERREAURE"),
    }
}
