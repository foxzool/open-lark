//! VC会议报告API模块
//!
//! 提供视频会议报告相关的功能，包括：
//! - 每日会议使用报告
//! - Top用户统计报告
//!
//! # 示例
//!
//! ```rust
//! use openlark_docs::report::vc_report::{VcReportService, GetDailyReportRequest};
//!
//! let service = VcReportService::new(config);
//!
//! // 获取每日会议报告
//! let response = service
//!     .get_daily_report_builder()
//!     .start_date("2024-01-01")
//!     .end_date("2024-01-31")
//!     .execute(&service)
//!     .await?;
//!
//! if let Some(report_data) = response.data {
//!     println!("报告数据: {:?}", report_data);
//! }
//! ```

/// 数据模型定义
pub mod models;

/// API服务实现
pub mod services;

// 重新导出主要类型
pub use models::*;
pub use services::{GetDailyReportRequestBuilder, GetTopUserReportRequestBuilder, VcReportService};
