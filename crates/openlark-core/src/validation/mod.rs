//! API验证模块
//!
//! 提供API边界处的类型安全验证和输入安全性检查功能。
//! 包含参数验证、数据清理、边界检查等安全机制。

pub mod calendar;
pub mod core;
pub mod drive;
pub mod employee;
pub mod file;
pub mod hire;
pub mod im;
pub mod message;
pub mod pagination;
pub mod password;
pub mod sheets;
pub mod uuid;

// Re-export commonly used types
pub use core::{validate_required, validate_string_length, ValidateBuilder, ValidationResult};
pub use employee::{validate_email, validate_name, validate_phone};
pub use file::{
    validate_file_extension, validate_file_name, validate_file_size, validate_upload_file,
};
pub use pagination::{validate_page_size, validate_page_token, validate_pagination_params};
pub use password::{validate_and_sanitize_password, validate_password_strength};
