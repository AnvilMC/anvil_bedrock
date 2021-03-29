use std::convert::TryInto;

pub trait RaknetPacketData: Sized {
    fn decode(reader: &mut impl Reader) -> Option<Self>;
    fn encode(&self, writer: &mut impl Writer) -> Option<()>;
}

pub trait Reader {
    fn skip(&mut self, n: usize);
    fn next(&mut self) -> Option<u8>;
    fn next_array<const N: usize>(&mut self) -> Option<[u8; N]>;
    fn read_to_end(&mut self) -> Vec<u8>;
}

pub trait Writer {
    fn write(&mut self, data: u8) -> Option<()>;
    fn write_slice(&mut self, slice: &[u8]) -> Option<()>;
}

impl<'a, T: Iterator<Item = &'a u8>> Reader for T {
    fn next(&mut self) -> Option<u8> {
        self.next().copied()
    }

    fn next_array<const N: usize>(&mut self) -> Option<[u8; N]> {
        Some(self.take(N).copied().collect::<Vec<u8>>().try_into().ok()?)
    }

    fn read_to_end(&mut self) -> Vec<u8> {
        self.copied().collect()
    }

    fn skip(&mut self, n: usize) {
        self.take(n).for_each(drop);
    }
}

impl Writer for Vec<u8> {
    fn write(&mut self, data: u8) -> Option<()> {
        self.push(data);
        Some(())
    }

    fn write_slice(&mut self, slice: &[u8]) -> Option<()> {
        self.extend_from_slice(slice);
        Some(())
    }
}
