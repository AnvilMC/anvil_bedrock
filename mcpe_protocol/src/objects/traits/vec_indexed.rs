use std::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::prelude::{MCPEPacketData, MCPEPacketDataError, Reader, Writer};

pub trait Indexable {
    fn as_usize(&self) -> usize;
    fn from_usize(u: usize) -> Self;
}

#[derive(Debug)]
pub struct VecIndexed<T: MCPEPacketData, E: Indexable + MCPEPacketData>(pub Vec<T>, PhantomData<E>);

impl<T: MCPEPacketData, E: Indexable + MCPEPacketData> From<Vec<T>> for VecIndexed<T, E> {
    fn from(e: Vec<T>) -> Self {
        VecIndexed(e, PhantomData)
    }
}
impl<T: MCPEPacketData, E: Indexable + MCPEPacketData> Into<Vec<T>> for VecIndexed<T, E> {
    fn into(self) -> Vec<T> {
        self.0
    }
}

impl<T: MCPEPacketData, E: Indexable + MCPEPacketData> AsRef<Vec<T>> for VecIndexed<T, E> {
    fn as_ref(&self) -> &Vec<T> {
        &self.0
    }
}

impl<T: MCPEPacketData, E: Indexable + MCPEPacketData> AsMut<Vec<T>> for VecIndexed<T, E> {
    fn as_mut(&mut self) -> &mut Vec<T> {
        &mut self.0
    }
}

impl<T: MCPEPacketData, E: Indexable + MCPEPacketData> Deref for VecIndexed<T, E> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T: MCPEPacketData, E: Indexable + MCPEPacketData> DerefMut for VecIndexed<T, E> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<T: MCPEPacketData, E: Indexable + MCPEPacketData> MCPEPacketData for VecIndexed<T, E> {
    fn decode(reader: &mut impl Reader) -> Result<Self, MCPEPacketDataError> {
        let size: usize = E::decode(reader)
            .map_err(|x| x.map("vec_indexed_index"))?
            .as_usize();
        Ok(Self(
            (0..size)
                .map(|_| T::decode(reader))
                .collect::<Result<_, _>>()
                .map_err(|x| x.map("vec_indexed_value"))?,
            PhantomData,
        ))
    }

    fn encode(&self, writer: &mut impl Writer) -> Result<(), MCPEPacketDataError> {
        (E::from_usize(self.0.len()))
            .encode(writer)
            .map_err(|x| x.map("vec_indexed_index"))?;
        for i in &self.0 {
            i.encode(writer).map_err(|x| x.map("vec_indexed_value"))?;
        }
        Ok(())
    }
}
