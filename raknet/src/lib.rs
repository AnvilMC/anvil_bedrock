#![feature(min_const_generics)]

pub mod objects;
pub mod traits;
pub mod protocol;

pub mod prelude {
    pub use crate::traits::*;
    pub use crate::objects::*;
    pub use crate::protocol::*;
}
