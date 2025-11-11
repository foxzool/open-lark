//! 事件处理模块
//!
//! 提供飞书开放平台事件的接收、处理和分发功能。
//! 支持WebSocket长连接和HTTP回调两种事件接收方式。
//!
//! 此模块从 `open-lark-core` crate重新导出事件处理功能。

// Re-export event functionality from core crate
pub use openlark_core::event::*;
