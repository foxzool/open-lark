//! # OpenLark 帮助台模块
//!
//! OpenLark SDK 的帮助台模块，提供飞书帮助台 API 的完整访问。

#![allow(missing_docs)]

mod service;

pub mod common;

#[cfg(feature = "v1")]
pub mod v1;

pub mod prelude;

pub use service::HelpdeskService;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
