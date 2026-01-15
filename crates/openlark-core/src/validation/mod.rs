//! API验证模块
//!
//! 提供API边界处的类型安全验证和输入安全性检查功能。
//! 包含参数验证、数据清理、边界检查等安全机制。

pub mod core;
pub mod file;
pub mod pagination;
pub mod password;
pub mod uuid;

// Re-export commonly used types
pub use core::{validate_required, validate_string_length, ValidateBuilder, ValidationResult};
pub use file::{
    validate_file_extension, validate_file_name, validate_file_size, validate_upload_file,
};
pub use pagination::{validate_page_size, validate_page_token, validate_pagination_params};
pub use password::{validate_and_sanitize_password, validate_password_strength};
