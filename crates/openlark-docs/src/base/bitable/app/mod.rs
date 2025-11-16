//! Bitable App API模块
//!
//! 提供多维表格应用管理相关的功能，包括：
//! - 应用创建和复制
//! - 应用元数据获取和更新
//! - 应用删除操作
//!
//! # 示例
//!
//! ```rust
//! use openlark_docs::base::bitable::app::{AppService, CreateAppRequest};
//!
//! let service = AppService::new(config);
//!
//! // 创建新的多维表格
//! let response = service
//!     .create_app_builder()
//!     .name("项目跟踪表")
//!     .folder_token("folder_token_xxx")
//!     .execute(&service)
//!     .await?;
//!
//! if let Some(app) = response.app {
//!     println!("创建成功: token={}", app.app_token.unwrap_or_default());
//! }
//! ```

/// 数据模型定义
pub mod models;

/// API服务实现
pub mod services;

// 重新导出主要类型
pub use models::*;
pub use services::{AppService, CreateAppRequestBuilder, UpdateAppRequestBuilder};
