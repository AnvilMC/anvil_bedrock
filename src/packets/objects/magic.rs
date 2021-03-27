use std::fmt::Formatter;

use packet_derive::Biscuit;

#[derive(Biscuit, Clone, PartialEq)]
pub struct Magic([u8; 16]);

impl std::fmt::Debug for Magic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|x| format!("{:02X?}", x))
                .collect::<Vec<_>>()
                .join("")
        )
    }
}
