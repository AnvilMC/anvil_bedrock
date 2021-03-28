use crate::packets::traits::PacketDecoder;

pub struct TakeAll<T: PacketDecoder>(pub Vec<T>);

impl <T: PacketDecoder> PacketDecoder for TakeAll<T> {
    fn read(iter: &mut crate::packets::traits::U8Iter) -> Option<Self> {
        let mut vec = Vec::new();
        while let Some(e) = T::read(iter) {
            vec.push(e);
        }
        Some(TakeAll(vec))
    }

    fn write(self, vec: &mut Vec<u8>) -> Option<()> {
        for i in self.0 {
            i.write(vec);
        }
        Some(())
    }
}

impl <T: PacketDecoder + std::fmt::Debug> std::fmt::Debug for TakeAll<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl <T: PacketDecoder + Clone> Clone for TakeAll<T> {
    fn clone(&self) -> Self {
        TakeAll(self.0.clone())
    }
}