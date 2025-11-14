//! IM Service Shim
//!
//! Temporary implementation for ImService during migration period.

#[cfg(feature = "im")]
pub use crate::service::shim::ImService;