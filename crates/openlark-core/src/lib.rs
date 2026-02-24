//! OpenLark Core Infrastructure
//!
//! This crate provides the core infrastructure for the OpenLark SDK including
//! HTTP client configuration, error handling, authentication, and common utilities.

#![warn(missing_docs)]

// 对外稳定导出：尽量保持"少而清晰"的公共 API（KISS）
pub mod api;
pub mod auth;
pub mod config;
pub mod constants;
pub mod error;
pub mod http;
pub mod observability;
pub mod query_params;
pub mod req_option;
pub mod request_builder;
#[cfg(feature = "testing")]
pub mod testing;
pub mod trait_system;
pub mod validation;

// crate 内部实现细节：不对外暴露（避免把 core 变成"全家桶"）
mod app_ticket_manager;
mod content_disposition;
mod improved_response_handler;
mod performance;
mod req_translator;
mod utils;

// Re-export commonly used types from crate root
pub use error::{validation_error, CoreError, SDKResult};

// Re-export validation utilities
pub use validation::validate_required;

// Validatable trait for unified validation semantics
pub trait Validatable {
    fn is_empty_trimmed(&self) -> bool;
}

impl Validatable for &str {
    fn is_empty_trimmed(&self) -> bool { self.trim().is_empty() }
}

impl Validatable for String {
    fn is_empty_trimmed(&self) -> bool { self.trim().is_empty() }
}

impl<T: Validatable> Validatable for &T {
    fn is_empty_trimmed(&self) -> bool { (*self).is_empty_trimmed() }
}

impl<T> Validatable for Vec<T> {
    fn is_empty_trimmed(&self) -> bool { self.is_empty() }
}

impl<T> Validatable for &[T] {
    fn is_empty_trimmed(&self) -> bool { self.is_empty() }
}

// Re-export validate_required macro for docs module
#[macro_export]
macro_rules! validate_required {
    ($field:expr, $error_msg:expr) => {
        if openlark_core::Validatable::is_empty_trimmed(&$field) {
            return Err(openlark_core::error::CoreError::validation_msg($error_msg));
        }
    };
}

/// 验证必填列表字段（检查非空和最大长度）
///
/// # 参数
/// - `$field`: 要验证的列表字段
/// - `$max_len`: 最大长度限制
/// - `$error_msg`: 错误消息
///
/// # 使用示例
/// ```rust,ignore
/// validate_required_list!(self.user_ids, 50, "用户 ID 列表不能为空且不能超过 50 个");
/// ```
#[macro_export]
macro_rules! validate_required_list {
    ($field:expr, $max_len:expr, $error_msg:expr) => {
        if $field.is_empty() {
            return Err(openlark_core::error::CoreError::validation_msg($error_msg));
        }
        if $field.len() > $max_len {
            return Err(openlark_core::error::CoreError::validation_msg($error_msg));
        }
    };
}

/// Prelude module for convenient imports.
pub mod prelude {
    // Re-export new API module（请求/响应基础类型）
    pub use crate::api::prelude::*;

    // Re-export commonly used core modules directly（最小集合）
    pub use crate::config::Config;
    pub use crate::constants::*;
    pub use crate::error::{validation_error, CoreError, LarkAPIError, SDKResult};
    pub use crate::http::Transport;
    pub use crate::req_option::*;
    pub use crate::validate_required;
    pub use crate::validate_required_list;

    // Re-export commonly used dependencies
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::Value;
    pub use std::collections::HashMap;
}
