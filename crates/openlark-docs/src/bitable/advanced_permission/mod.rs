//! Bitable Advanced Permission API模块
//!
//! 提供多维表格高级权限管理相关的功能，包括：
//! - V2版本的角色管理
//! - 高级权限控制
//! - 细粒度权限设置
//!
//! # 示例
//!
//! ```rust
//! use openlark_docs::base::bitable::advanced_permission::{AdvancedPermissionService, ListRolesV2Request};
//!
//! let service = AdvancedPermissionService::new(config);
//!
//! // 列出V2角色
//! let response = service
//!     .list_roles_v2_builder()
//!     .app_token("app_token_xxx")
//!     .execute(&service)
//!     .await?;
//!
//! if let Some(roles) = response.roles {
//!     println!("找到V2角色数量: {}", roles.len());
//! }
//! ```

/// 数据模型定义
pub mod models;

/// API服务实现
pub mod services;

// 重新导出主要类型
pub use models::*;
pub use services::{
    AdvancedPermissionService, CreateRoleV2RequestBuilder, ListRolesV2RequestBuilder,
    UpdateRoleV2RequestBuilder,
};
