//! Core基础设施模块
//!
//! 提供SDK的基础配置、客户端和常量定义

pub mod config;
pub mod client;
pub mod constants;
pub mod version;

// Re-export commonly used types
pub use config::Config;
pub use config::ConfigBuilder;
pub use client::CoreClient;
pub use constants::*;
pub use version::VERSION;