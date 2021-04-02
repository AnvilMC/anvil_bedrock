#![feature(min_const_generics)] // Only here because `rust-analyzer` complains if it is not present

pub mod errors;
pub mod objects;
pub mod packets;
pub mod traits;

pub mod prelude {
    pub use crate::errors::*;
    pub use crate::objects::*;
    pub use crate::packets::*;
    pub use crate::traits::*;
}
