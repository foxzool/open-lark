//! OpenLark Client Module
//!
//! This crate provides client interfaces and service aggregation for the OpenLark SDK.
//! It offers a clean separation between client interfaces and service implementations,
//! resolving circular dependencies and enabling better modularity.

#![allow(missing_docs)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]

// Public re-exports
pub use traits::{AsyncLarkClient, ServiceRegistry, ClientBuilder};

// Public modules
pub mod services;
pub mod accessors;

// Internal modules
mod traits;
mod client;
mod registry;
mod prelude;

// Re-export the default client implementation
pub use client::DefaultLarkClient;

// Type alias for backward compatibility
/// Default LarkClient type for backward compatibility
pub type LarkClient = DefaultLarkClient;