#![feature(min_const_generics)]

pub mod objects;
pub mod protocol;
pub mod traits;

pub mod prelude {
    pub use crate::objects::*;
    pub use crate::protocol::*;
    pub use crate::traits::*;
}

fn from_hex(i: u8) -> u8 {
    match i {
        b'0'..=b'9' => i - b'0',
        b'A'..=b'F' => i - b'A' + 10,
        b'a'..=b'f' => i - b'a' + 10,
        _ => panic!("WINDOZE FATALE ERREAURE"),
    }
}

#[test]
fn decoder() {
    let file = std::fs::read_to_string("decode.hex").unwrap();
    let bin = file
        .as_bytes()
        .chunks(2)
        .map(|x| from_hex(x[0]) * 16 + from_hex(x[1]))
        .collect::<Vec<u8>>();
    let bin = miniz_oxide::inflate::decompress_to_vec(&bin).unwrap();
    std::fs::write("decoded.bin", &bin);
    std::fs::write(
        "decoded.hex",
        bin.iter()
            .map(|x| format!("{:02x}", x))
            .collect::<Vec<String>>()
            .join(" "),
    );
}
