//! open-lark-enterprise module for OpenLark SDK
//!
//! This crate provides open-lark-enterprise functionality for the OpenLark SDK.

#![allow(missing_docs)]

/// Enterprise services module
pub mod auth;

/// Re-exports from open-lark-core for convenience.
pub mod prelude {
    pub use openlark_core::{client::LarkClient, SDKResult};
}
