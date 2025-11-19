//! Drive Permission API模块
//!
//! 提供云空间文件权限管理相关的功能，包括：
//! - 协作者权限验证
//! - 所有者转移
//! - 公共权限设置获取
//!
//! # 示例
//!
//! ```rust
//! use openlark_docs::ccm::drive::permission::{PermissionService, CheckMemberPermissionRequest};
//!
//! let service = PermissionService::new(config);
//!
//! // 检查用户权限
//! let response = service
//!     .check_member_permission_builder()
//!     .file_token("token_xxx")
//!     .permission("view")
//!     .user_id("user_xxx")
//!     .user_id_type("open_id")
//!     .execute(&service)
//!     .await?;
//!
//! if response.permitted.unwrap_or(false) {
//!     println!("用户有权限");
//! }
//! ```

/// 数据模型定义
pub mod models;

/// API服务实现
pub mod services;

// 重新导出主要类型
pub use models::*;
pub use services::{
    CheckMemberPermissionRequestBuilder, GetPublicPermissionRequestBuilder, PermissionService,
    TransferOwnerRequestBuilder,
};
