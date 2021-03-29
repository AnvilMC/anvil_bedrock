use crate::prelude::{RaknetPacketData, Reader, Writer};

#[derive(Clone)]
pub struct RaknetString(pub Vec<u8>);

impl RaknetPacketData for RaknetString {
    fn decode(reader: &mut impl Reader) -> Option<Self> {
        Some(RaknetString(
            (0..u16::decode(reader)?)
                .map(|_| reader.next())
                .collect::<Option<_>>()?,
        ))
    }

    fn encode(&self, writer: &mut impl Writer) -> Option<()> {
        (self.0.len() as u16).encode(writer);
        writer.write_slice(&self.0)
    }
}

impl From<&'_ str> for RaknetString {
    fn from(a: &'_ str) -> Self {
        RaknetString(a.as_bytes().to_vec())
    }
}
