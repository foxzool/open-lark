//! open-lark-communication module for OpenLark SDK
//!
//! This crate provides communication functionality for the OpenLark SDK including
//! instant messaging, contacts, and other communication-related services.

#![allow(missing_docs)]

// Include macros first
#[macro_use]
mod macros;

pub mod im;
pub mod contact;

/// Re-exports from open-lark-core for convenience.
pub mod prelude {
    pub use open_lark_core::*;
    // Re-export communication services
    pub use crate::im::*;
    pub use crate::contact::*;
}
