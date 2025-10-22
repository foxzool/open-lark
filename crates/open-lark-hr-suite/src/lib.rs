#![allow(missing_docs)]

// Include macros first
#[macro_use]
mod macros;

pub mod hire;

pub mod prelude {
    pub use open_lark_core::*;
    pub use crate::hire::*;
}
