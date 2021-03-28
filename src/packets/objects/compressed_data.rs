use miniz_oxide::{deflate::compress_to_vec, inflate::decompress_to_vec_zlib};

use crate::packets::traits::PacketDecoder;

pub struct Compressed<T: PacketDecoder>(pub T);

impl <T: PacketDecoder> PacketDecoder for Compressed<T> {
    fn read(iter: &mut crate::packets::traits::U8Iter) -> Option<Self> {
        iter.next()?;
        Some(Compressed(T::read(&mut decompress_to_vec_zlib(&iter.collect::<Vec<u8>>()).unwrap().into_iter())?))
    }

    fn write(self, vec: &mut Vec<u8>) -> Option<()> {
        let mut buffer = Vec::new();
        self.0.write(&mut buffer);
        vec.append(&mut compress_to_vec(&buffer, 6));
        Some(())
    }
}

impl <T: PacketDecoder + std::fmt::Debug> std::fmt::Debug for Compressed<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl <T: PacketDecoder + Clone> Clone for Compressed<T> {
    fn clone(&self) -> Self {
        Compressed(self.0.clone())
    }
}