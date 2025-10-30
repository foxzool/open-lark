//! open-lark-extensions module for OpenLark SDK
//!
//! This crate provides open-lark-extensions functionality for the OpenLark SDK.

#![allow(missing_docs)]

pub mod board;
pub mod event;

/// Re-exports from open-lark-core for convenience.
pub mod prelude {
    pub use open_lark_core::*;
    pub use crate::board::*;
    pub use crate::event::*;
}
