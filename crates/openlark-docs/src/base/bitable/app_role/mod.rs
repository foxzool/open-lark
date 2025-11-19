//! Bitable App Role API模块
//!
//! 提供多维表格角色权限管理相关的功能，包括：
//! - 角色的创建、查询、更新、删除
//! - 权限配置和管理
//! - 角色层级和继承关系
//!
//! # 示例
//!
//! ```rust
//! use openlark_docs::base::bitable::app_role::{AppRoleService, CreateRoleRequest};
//!
//! let service = AppRoleService::new(config);
//!
//! // 创建新角色
//! let response = service
//!     .create_role_builder()
//!     .app_token("app_token_xxx")
//!     .role_name("项目编辑")
//!     .role_type("editor")
//!     .execute(&service)
//!     .await?;
//!
//! if let Some(role) = response.role {
//!     println!("创建成功: role_id={}", role.role_id.unwrap_or_default());
//! }
//! ```

/// 数据模型定义
pub mod models;

/// API服务实现
pub mod services;

// 重新导出主要类型
pub use models::*;
pub use services::{AppRoleService, CreateRoleRequestBuilder, UpdateRoleRequestBuilder};
