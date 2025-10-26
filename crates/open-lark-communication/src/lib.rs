//! open-lark-communication module for OpenLark SDK
//!
//! This crate provides communication functionality for OpenLark SDK including
//! instant messaging, contacts, and other communication-related services.
//!
#![allow(missing_docs)]
// Include macros first
#[macro_use]
mod macros;

pub mod contact;
pub mod im;

/// Re-exports from open-lark-core for convenience.
pub mod prelude {
    pub use open_lark_core::*;
    // Re-export communication services
    pub use crate::contact::models::*;
    // pub use crate::im::*; // 暂时注释掉未使用的导入
}