//! OpenLark Client Module
//!
//! This crate provides client interfaces and service aggregation for the OpenLark SDK.
//! It offers a clean separation between client interfaces and service implementations,
//! resolving circular dependencies and enabling better modularity.

#![allow(missing_docs)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]

// Public re-exports
pub use traits::{AsyncLarkClient, ClientBuilder, ServiceRegistry, LarkClientTrait};

// Public modules
pub mod accessors;
pub mod services;

// Re-export the main client implementation
pub use client::LarkClientBuilder;

// Internal modules
mod client;
mod prelude;
mod registry;
mod traits;

// Re-export the client implementation
pub use client::LarkClient;

// Type alias for backward compatibility
/// Default LarkClient type for backward compatibility
pub type DefaultLarkClient = LarkClient;
