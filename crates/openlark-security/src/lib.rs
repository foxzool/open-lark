//! # OpenLark 安全认证服务模块
//!
//! 飞书开放平台安全认证服务模块，提供身份认证、权限管理、安全审计等功能。
//!
//! ## 功能模块
//!
//! - **auth**: 身份认证 (44 APIs中的主要部分) - 用户登录、令牌管理
//! - **acs**: 访问控制 - 权限验证、角色管理
//! - **audit**: 安全审计 - 操作日志、安全监控
//! - **token**: 令牌管理 - 访问令牌、刷新令牌
//!
//! ## 快速开始
//!
//! ```rust,ignore
//! use openlark_security::prelude::*;
//!
//! // 创建安全管理器
//! let manager = SecurityServiceManager::new(config);
//!
//! // 检查服务健康状态
//! let health = manager.health_check().await?;
//! println!("Security service health: {:?}", health);
//!
//! // 获取服务统计信息
//! let stats = manager.get_statistics().await?;
//! println!("Service statistics: {:?}", stats);
//! ```
//!
//! ## 特性
//!
//! - ✅ **44 APIs全覆盖** - 飞书安全认证服务完整实现
//! - ✅ **类型安全** - 强类型请求/响应结构
//! - ✅ **异步支持** - 基于tokio的异步API
//! - ✅ **版本化API** - 支持v1/v2/v3多版本API
//! - ✅ **构建器模式** - 流畅的API调用体验
//! - ✅ **安全增强** - 支持多重认证、加密传输

// #![deny(missing_docs)] // 暂时禁用，在开发阶段
#![warn(clippy::all)]

// Core modules
pub mod error;
pub mod models;

// 主要服务模块
pub mod service;

// 功能模块按安全域组织
#[cfg(feature = "auth")]
pub mod auth;

#[cfg(feature = "acs")]
pub mod acs;

#[cfg(feature = "audit")]
pub mod audit;

#[cfg(feature = "token")]
pub mod token;

// API版本模块
#[cfg(any(feature = "v1", feature = "v2", feature = "v3"))]
pub mod versions;

// Prelude模块 - 常用导入
pub mod prelude;

// 重新导出主要类型
pub use error::{SecurityError, SecurityResult};
pub use service::SecurityService;

// 重新导出各域服务
#[cfg(feature = "auth")]
pub use auth::AuthService;

#[cfg(feature = "acs")]
pub use acs::AccessControlService;

#[cfg(feature = "audit")]
pub use audit::AuditService;

#[cfg(feature = "token")]
pub use token::TokenService;
