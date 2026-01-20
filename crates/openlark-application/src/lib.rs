//! # OpenLark 应用管理模块
//!
//! OpenLark SDK 的应用管理模块，提供飞书应用 API 的完整访问。

#![allow(missing_docs)]

mod service;

pub mod common;

#[cfg(feature = "v1")]
pub mod v1;

pub mod prelude;

pub use service::ApplicationService;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
