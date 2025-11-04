//! Contact Service Shim
//!
//! Temporary implementation for ContactService during migration period.

#[cfg(feature = "contact")]
pub use crate::service::shim::ContactService;