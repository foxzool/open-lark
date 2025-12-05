//! Report Task 报告任务API模块
//!
//! 提供报告任务管理相关的功能，包括：
//! - 任务查询和过滤
//! - 任务状态跟踪
//! - 任务执行历史
//! - 完整的错误处理和参数验证
//!
//! # 示例
//!
//! ```rust
//! use openlark_docs::report::{ReportService, QueryTaskRequest};
//!
//! let service = ReportService::new(config);
//!
//! // 查询报告任务
//! let response = service
//!     .task()
//!     .query_task_builder()
//!     .task_type("daily_report")
//!     .status("completed")
//!     .page_size(20)
//!     .execute(&service.task())
//!     .await?;
//!
//! if let Some(tasks) = response.data {
//!     println!("查询到任务数量: {}", tasks.total);
//!     for task in tasks.items {
//!         println!("任务ID: {}, 状态: {}", task.task_id, task.status);
//!     }
//! }
//! ```

pub mod models;
pub mod services;

// 重新导出主要类型
pub use services::{QueryTaskRequestBuilder, TaskService};
