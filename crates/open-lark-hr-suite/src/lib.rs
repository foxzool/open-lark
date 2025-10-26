#![allow(missing_docs)]

// Include macros first
#[macro_use]
mod macros;

pub mod hire;

pub mod prelude {
    pub use crate::hire::*;
    pub use open_lark_core::*;
}
