//! APAAS应用开放平台服务
//!
//! 提供飞书应用开放平台的完整管理功能，包括：
//! - 应用信息查询和管理
//! - 应用权限和配置管理
//! - 应用数据分析和监控
//! - 多租户应用部署
//!
//! # 服务架构
//!
//! ## v1版本
//! - [`v1::apps`] - 应用管理服务，提供应用基本信息查询功能
//!
//! # 使用示例
//!
//! ```rust
//! use open_lark::prelude::*;
//!
//! let client = LarkClient::builder("app_id", "app_secret")
//!     .with_app_type(AppType::SelfBuild)
//!     .build();
//!
//! // 查询应用列表
//! let apps = client.apaas.apps.get_apps_builder()
//!     .page_size(20)
//!     .status("active")
//!     .execute(&client.apaas.apps)
//!     .await?;
//! ```

pub mod v1;

// 重新导出所有服务类型
pub use v1::*;
