use std::convert::TryInto;

use crate::prelude::MCPEPacketDataError;

pub trait MCPEPacketData: Sized {
    fn decode(reader: &mut impl Reader) -> Result<Self, MCPEPacketDataError>;
    fn encode(&self, writer: &mut impl Writer) -> Result<(), MCPEPacketDataError>;
}

pub trait MCPEPacket: MCPEPacketData + Sized {
    const PACKET_ID: u8;
}

pub trait Reader {
    fn skip(&mut self, n: usize);
    fn next(&mut self) -> Result<u8, MCPEPacketDataError>;
    fn next_array<const N: usize>(&mut self) -> Result<[u8; N], MCPEPacketDataError>;
    fn read_to_end(&mut self) -> Vec<u8>;
    fn read(&mut self, length: usize) -> Result<Vec<u8>, MCPEPacketDataError>;
}

pub trait Writer {
    fn write(&mut self, data: u8) -> Result<(), MCPEPacketDataError>;
    fn write_slice(&mut self, slice: &[u8]) -> Result<(), MCPEPacketDataError>;
}

impl<'a, T: Iterator<Item = &'a u8>> Reader for T {
    fn next(&mut self) -> Result<u8, MCPEPacketDataError> {
        self.next()
            .copied()
            .ok_or_else(|| MCPEPacketDataError::new("$iterator_next", "Can't take next value"))
    }

    fn next_array<const N: usize>(&mut self) -> Result<[u8; N], MCPEPacketDataError> {
        self.take(N)
            .copied()
            .collect::<Vec<u8>>()
            .try_into()
            .ok()
            .ok_or_else(|| {
                MCPEPacketDataError::new("$iterator_next_array", "Can't take next array")
            })
    }

    fn read(&mut self, length: usize) -> Result<Vec<u8>, MCPEPacketDataError> {
        let i: Vec<u8> = self.take(length).copied().collect();
        if i.len() != length {
            Err(MCPEPacketDataError::new(
                "$iterator_read",
                "Can't read value (Invalid length)",
            ))
        } else {
            Ok(i)
        }
    }

    fn read_to_end(&mut self) -> Vec<u8> {
        self.copied().collect()
    }

    fn skip(&mut self, n: usize) {
        self.take(n).for_each(drop);
    }
}

impl Writer for Vec<u8> {
    fn write(&mut self, data: u8) -> Result<(), MCPEPacketDataError> {
        self.push(data);
        Ok(())
    }

    fn write_slice(&mut self, slice: &[u8]) -> Result<(), MCPEPacketDataError> {
        self.extend_from_slice(slice);
        Ok(())
    }
}

pub trait PacketReader<T> {
    fn auto_decode(&mut self) -> Result<T, MCPEPacketDataError>;
}

impl<T: MCPEPacketData, E: Reader> PacketReader<T> for E {
    fn auto_decode(&mut self) -> Result<T, MCPEPacketDataError> {
        T::decode(self)
    }
}
