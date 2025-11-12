//! openlark-docs module for OpenLark SDK
//!
//! This crate provides cloud document functionality for the OpenLark SDK,
//! including documents, spreadsheets, bitables, wikis, drive storage, and
//! content collaboration management.

#![allow(missing_docs)]

// Include macros first
#[macro_use]
mod macros;

// Re-exports from open-lark-core for convenience.
pub mod prelude {
    pub use openlark_core::{client::LarkClient, SDKResult};
}

// Cloud document service modules
#[cfg(feature = "docs")]
pub mod docs;

#[cfg(feature = "sheet")]
pub mod sheet;

#[cfg(feature = "bitable")]
pub mod bitable;

#[cfg(feature = "wiki")]
pub mod wiki;

#[cfg(feature = "drive")]
pub mod drive;

#[cfg(feature = "ccm")]
pub mod ccm;

// Re-export service types for convenience
#[cfg(feature = "docs")]
pub use docs::*;

#[cfg(feature = "sheet")]
pub use sheet::*;

#[cfg(feature = "bitable")]
pub use bitable::*;

#[cfg(feature = "wiki")]
pub use wiki::*;

#[cfg(feature = "drive")]
pub use drive::*;

#[cfg(feature = "ccm")]
pub use ccm::*;
