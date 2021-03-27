use std::{collections::HashMap, convert::TryInto};

use std::hash::Hash;

use encoding::{all::ASCII, EncoderTrap, Encoding};

macro_rules! primitive {
    ($tp:ty,$len:expr) => {
        impl PacketDecoder for $tp {
            fn read(iter: &mut U8Iter) -> Option<Self> {
                Some(Self::from_be_bytes(
                    iter.take($len).collect::<Vec<_>>().try_into().ok()?,
                ))
            }
            fn write(self, vec: &mut Vec<u8>) -> Option<()> {
                vec.extend(self.to_be_bytes().iter());
                Some(())
            }
        }
    };
}
macro_rules! array_impl {
    ($alen:expr) => {
        impl PacketDecoder for [u8; $alen] {
            fn read(iter: &mut U8Iter) -> Option<Self> {
                iter.take($alen).collect::<Vec<_>>().try_into().ok()
            }
            fn write(self, vec: &mut Vec<u8>) -> Option<()> {
                vec.extend(self.iter());
                Some(())
            }
        }
    };
}

array_impl!(2);
array_impl!(4);
array_impl!(8);
array_impl!(16);

primitive!(i64, 8);
primitive!(u64, 8);
primitive!(u128, 16);
primitive!(i8, 1);
primitive!(u8, 1);
primitive!(u16, 2);
primitive!(i16, 2);

pub type U8Iter = dyn Iterator<Item = u8>;

pub trait PacketDecoder: Sized {
    fn read(iter: &mut U8Iter) -> Option<Self>;
    fn write(self, vec: &mut Vec<u8>) -> Option<()>;
}

pub trait IterRead<T: PacketDecoder> {
    fn read(&mut self) -> Option<T>;
}

impl<T: PacketDecoder> IterRead<T> for U8Iter {
    fn read(&mut self) -> Option<T> {
        T::read(self)
    }
}

pub trait Packet {
    fn id() -> u8;
}

impl PacketDecoder for String {
    fn read(iter: &mut U8Iter) -> Option<Self> {
        let length: u16 = iter.read()?;
        let str_bytes = (0..length)
            .map(|_| iter.next())
            .collect::<Option<Vec<u8>>>()?;
        String::from_utf8(str_bytes).ok()
    }

    fn write(self, vec: &mut Vec<u8>) -> Option<()> {
        let mut bytes = ASCII.encode(&self, EncoderTrap::Ignore).ok()?;
        let length = self.len() as u16;
        length.write(vec);
        vec.append(&mut bytes);
        Some(())
    }
}

impl PacketDecoder for bool {
    fn read(iter: &mut U8Iter) -> Option<Self> {
        Some(iter.next()? == 1)
    }

    fn write(self, vec: &mut Vec<u8>) -> Option<()> {
        vec.push(if self { 1 } else { 0 });
        Some(())
    }
}

impl<T: PacketDecoder + Hash + Eq, E: PacketDecoder + Hash + Eq> PacketDecoder for HashMap<T, E> {
    fn read(iter: &mut U8Iter) -> Option<Self> {
        let length: u8 = iter.read()?;
        (0..length)
            .map(|_| Some((T::read(iter)?, E::read(iter)?)))
            .collect::<Option<HashMap<T, E>>>()
    }

    fn write(self, vec: &mut Vec<u8>) -> Option<()> {
        let length = self.len() as u8;
        length.write(vec);
        for (x, y) in self.into_iter() {
            x.write(vec)?;
            y.write(vec)?;
        }
        Some(())
    }
}

#[derive(Debug)]
pub struct Mtu(pub Vec<u8>);

impl PacketDecoder for Mtu {
    fn read(iter: &mut U8Iter) -> Option<Self> {
        Some(Mtu(iter.collect()))
    }

    fn write(mut self, vec: &mut Vec<u8>) -> Option<()> {
        vec.append(&mut self.0);
        Some(())
    }
}
