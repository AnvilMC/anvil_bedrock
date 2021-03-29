use crate::packets::traits::{IterRead, PacketDecoder};

use super::bedrock_var_int::VarU32;

pub struct VarSizedVec<T: PacketDecoder>(Vec<T>);

impl<T: PacketDecoder> PacketDecoder for VarSizedVec<T> {
    fn read(iter: &mut crate::packets::traits::U8Iter) -> Option<Self> {
        let len: VarU32 = iter.read()?;
        Some(Self(
            (0..len.0).map(|_| iter.read()).collect::<Option<_>>()?,
        ))
    }

    fn write(self, vec: &mut Vec<u8>) -> Option<()> {
        VarU32(self.0.len() as u32).write(vec);
        for i in self.0 {
            i.write(vec);
        }
        Some(())
    }
}
