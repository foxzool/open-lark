//! # 绩效管理服务
//!
//! 飞书绩效管理 (Performance Management) 服务提供完整的绩效管理功能，支持以下核心能力：
//!
//! ## 功能特性
//!
//! - **后台配置**：周期与项目管理、补充信息管理、人员组管理、评估模板配置、指标配置
//! - **评估任务**：周期任务查询和管理
//! - **指标数据**：关键指标数据的获取和录入
//! - **评估数据**：绩效结果和详情数据的查询
//! - **事件推送**：绩效结果开通和详情变更事件推送
//!
//! ## 服务模块
//!
//! 该服务包含以下功能模块：
//!
//! - [`models`] - 数据模型和类型定义
//! - [`review_config`] - 后台配置管理模块
//! - [`stage_task`] - 评估任务管理模块
//! - [`metric_detail`] - 指标数据管理模块
//! - [`review_data`] - 评估数据管理模块
//! - [`v1`] - 事件定义模块
//!
//! ## 使用示例
//!
//! ```rust,no_run
//! use open_lark::prelude::*;
//! use open_lark::service::performance::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LarkClient::builder("app_id", "app_secret")
//!         .build();
//!
//!     // 获取周期列表
//!     let semesters = client.performance.review_config.list_semesters(
//!         review_config::SemesterListRequest::default(), None
//!     ).await?;
//!     
//!     // 获取项目列表
//!     let activities = client.performance.review_config.query_activities(
//!         review_config::ActivityQueryRequest::default(), None
//!     ).await?;
//!     
//!     // 获取绩效结果
//!     let results = client.performance.review_data.query_results(
//!         review_data::ResultQueryRequest::default(), None
//!     ).await?;
//!     
//!     Ok(())
//! }
//! ```

pub mod metric_detail;
pub mod models;
pub mod review_config;
pub mod review_data;
pub mod stage_task;
pub mod v1;

use crate::{
    core::config::Config,
    service::performance::{
        metric_detail::MetricDetailService, review_config::ReviewConfigService,
        review_data::ReviewDataService, stage_task::StageTaskService,
    },
};

/// 绩效管理服务
///
/// 提供完整的绩效管理功能，包括后台配置、评估任务、指标数据和评估数据管理
pub struct PerformanceService {
    /// 后台配置服务
    pub review_config: ReviewConfigService,
    /// 评估任务服务
    pub stage_task: StageTaskService,
    /// 指标数据服务
    pub metric_detail: MetricDetailService,
    /// 评估数据服务
    pub review_data: ReviewDataService,
}

impl PerformanceService {
    /// 创建绩效管理服务实例
    pub fn new(config: Config) -> Self {
        Self {
            review_config: ReviewConfigService::new(config.clone()),
            stage_task: StageTaskService::new(config.clone()),
            metric_detail: MetricDetailService::new(config.clone()),
            review_data: ReviewDataService::new(config),
        }
    }
}
