//! Bitable App Workflow API模块
//!
//! 提供多维表格工作流管理相关的功能，包括：
//! - 工作流的查询、更新
//! - 工作流状态管理
//! - 审批流程处理
//!
//! # 示例
//!
//! ```rust
//! use openlark_docs::base::bitable::app_workflow::{AppWorkflowService, ListWorkflowRequest};
//!
//! let service = AppWorkflowService::new(config);
//!
//! // 列出工作流
//! let response = service
//!     .list_workflow_builder()
//!     .app_token("app_token_xxx")
//!     .execute(&service)
//!     .await?;
//!
//! if let Some(workflows) = response.workflows {
//!     println!("找到工作流数量: {}", workflows.len());
//! }
//! ```

/// 数据模型定义
pub mod models;

/// API服务实现
pub mod services;

// 重新导出主要类型
pub use models::*;
pub use services::{AppWorkflowService, ListWorkflowRequestBuilder, UpdateWorkflowRequestBuilder};
