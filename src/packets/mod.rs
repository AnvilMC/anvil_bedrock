use self::traits::{Packet, PacketDecoder};

pub mod client;
pub mod objects;
pub mod server;
pub mod traits;
pub mod common;

pub fn encode<T: PacketDecoder + Packet>(t: T) -> Vec<u8> {
    let mut vec = vec![T::id()];
    t.write(&mut vec);
    //println!("{:?}", vec);
    vec
}
