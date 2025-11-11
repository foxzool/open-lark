//! open-lark-ai-platform module for OpenLark SDK
//!
//! This crate provides open-lark-ai-platform functionality for the OpenLark SDK.

#![deny(missing_docs)]

/// Re-exports from open-lark-core for convenience.
pub mod prelude {
    pub use openlark_core::{client::LarkClient, SDKResult};
}
