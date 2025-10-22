//! open-lark-collaboration module for OpenLark SDK
//!
//! This crate provides collaboration functionality for the OpenLark SDK including
//! cloud documents, sheets, drive, and other collaboration-related services.

#![allow(missing_docs)]

// Include macros first
#[macro_use]
mod macros;

pub mod cloud_docs;

/// Re-exports from open-lark-core for convenience.
pub mod prelude {
    pub use open_lark_core::*;
    // Re-export collaboration services
    pub use cloud_docs::*;
}
