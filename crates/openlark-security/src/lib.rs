//! # OpenLark 安全认证服务模块
//!
//! 飞书开放平台安全认证服务模块，提供身份认证、权限管理、安全审计等功能。
//!
//! ## 功能模块
//!
//! - **auth**: 身份认证 (11 APIs) - 用户登录、令牌管理
//! - **acs**: 访问控制 (14 APIs) - 权限验证、角色管理
//! - **compliance**: 合规审计 (8 APIs) - 审计日志、合规检查、风险评估
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
//! - ✅ **33 APIs全覆盖** - 飞书安全认证服务完整实现 (auth: 11, acs: 14, compliance: 8)
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

#[cfg(feature = "compliance")]
pub mod compliance;

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

#[cfg(feature = "compliance")]
pub use compliance::ComplianceService;
