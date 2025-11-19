//! Bitable App Dashboard API模块
//!
//! 提供多维表格仪表板管理相关的功能，包括：
//! - 仪表板的创建、查询、更新、删除
//! - 仪表板组件配置和布局管理
//! - 数据可视化和图表配置
//!
//! # 示例
//!
//! ```rust
//! use openlark_docs::base::bitable::app_dashboard::{AppDashboardService, CreateDashboardRequest};
//!
//! let service = AppDashboardService::new(config);
//!
//! // 创建新仪表板
//! let response = service
//!     .create_dashboard_builder()
//!     .app_token("app_token_xxx")
//!     .dashboard_name("项目数据看板")
//!     .execute(&service)
//!     .await?;
//!
//! if let Some(dashboard) = response.dashboard {
//!     println!("创建成功: dashboard_id={}", dashboard.dashboard_id.unwrap_or_default());
//! }
//! ```

/// 数据模型定义
pub mod models;

/// API服务实现
pub mod services;

// 重新导出主要类型
pub use models::*;
pub use services::{AppDashboardService, CreateDashboardRequestBuilder};
