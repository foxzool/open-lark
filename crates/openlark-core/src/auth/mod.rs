//! 认证管理模块
//!
//! 提供令牌管理抽象和 App Ticket 支持

pub mod app_ticket;
pub mod token_provider;

// Re-export commonly used types
pub use token_provider::{NoOpTokenProvider, TokenProvider, TokenRequest};
