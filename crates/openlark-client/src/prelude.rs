//! Prelude module for convenient imports
//!
//! This module re-exports commonly used types and traits
//! to make client usage more ergonomic.

// These exports are provided for consumer convenience
#[allow(unused_imports)]
pub use crate::traits::{AsyncLarkClient, ClientBuilder, LarkClientTrait, ServiceRegistry};
#[allow(unused_imports)]
pub use crate::DefaultLarkClient;
#[allow(unused_imports)]
pub use openlark_core::{config::Config, constants::AppType, SDKResult};
