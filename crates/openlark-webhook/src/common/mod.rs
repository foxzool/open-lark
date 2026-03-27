//! Webhook 模块的通用能力。

#[cfg(feature = "signature")]
/// 飞书 webhook 签名辅助函数。
pub mod signature;

/// Webhook 错误类型定义。
pub mod error;
/// Webhook 输入参数校验工具。
pub mod validation;
