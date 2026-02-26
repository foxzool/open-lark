//! API验证模块
//!
//! 提供API边界处的类型安全验证和输入安全性检查功能。
//! 包含参数验证、数据清理、边界检查等安全机制。

pub mod core;

// Re-export commonly used types
pub use core::{
    validate_required_list_length, validate_string_length, ValidateBuilder,
    ValidationResult,
};
