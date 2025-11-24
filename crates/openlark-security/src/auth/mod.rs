//! 身份认证模块
//!
//! 提供用户登录、登出、令牌管理等身份认证功能。
//!
//! ## 功能特性
//!
//! - 多种登录方式支持（密码、验证码、SSO）
//! - 令牌生命周期管理
//! - 用户认证信息管理
//! - 登录状态跟踪
//! - 安全会话管理

pub mod service;
pub mod v1;

// 重新导出主要类型
pub use service::{AuthService, DefaultAuthService};

// API版本导出
#[cfg(feature = "v1")]
pub use v1::*;

#[cfg(any(feature = "v2", feature = "v3"))]
compile_error!("v2 and v3 API support not yet implemented");
