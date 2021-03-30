#![feature(min_const_generics)]

pub mod objects;
pub mod traits;

pub mod prelude {
    pub use crate::objects::*;
    pub use crate::traits::*;
}
