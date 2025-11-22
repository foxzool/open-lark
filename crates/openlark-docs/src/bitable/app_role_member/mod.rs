//! Bitable App Role Member API模块
//!
//! 提供多维表格角色成员管理相关的功能，包括：
//! - 角色成员的添加、查询、删除
//! - 批量成员操作支持
//! - 成员权限继承和管理
//!
//! # 示例
//!
//! ```rust
//! use openlark_docs::base::bitable::app_role_member::{AppRoleMemberService, CreateRoleMemberRequest};
//!
//! let service = AppRoleMemberService::new(config);
//!
//! // 添加角色成员
//! let response = service
//!     .create_role_member_builder()
//!     .app_token("app_token_xxx")
//!     .role_id("role_id_xxx")
//!     .user_id("user_id_xxx")
//!     .execute(&service)
//!     .await?;
//!
//! if let Some(member) = response.member {
//!     println!("添加成功: member_id={}", member.member_id.unwrap_or_default());
//! }
//! ```

/// 数据模型定义
pub mod models;

/// API服务实现
pub mod services;

// 重新导出主要类型
pub use models::*;
pub use services::{AppRoleMemberService, CreateRoleMemberRequestBuilder};
