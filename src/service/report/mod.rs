//! # 飞书汇报服务
//!
//! 飞书汇报 (Report) 服务提供汇报规则、看板和任务管理功能，支持以下核心能力：
//!
//! ## 功能特性
//!
//! - **规则管理**：查询汇报规则
//! - **规则看板管理**：移除规则看板
//! - **任务管理**：查询汇报任务
//!
//! ## 服务模块
//!
//! 该服务包含以下功能模块：
//!
//! - [`models`] - 数据模型和类型定义
//! - [`rule`] - 规则管理模块
//! - [`rule_view`] - 规则看板管理模块
//! - [`task`] - 任务管理模块
//!
//! ## 使用示例
//!
//! ```rust,no_run
//! use open_lark::prelude::*;
//! use open_lark::service::report::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LarkClient::builder("app_id", "app_secret")
//!         .build();
//!
//!     // 查询规则
//!     let rules = client.report.rule.query(
//!         rule::RuleQueryRequest::default(), None
//!     ).await?;
//!     
//!     // 查询任务
//!     let tasks = client.report.task.query(
//!         task::TaskQueryRequest::default(), None
//!     ).await?;
//!     
//!     Ok(())
//! }
//! ```

pub mod models;
pub mod rule;
pub mod rule_view;
pub mod task;

use crate::{
    core::config::Config,
    service::report::{rule::RuleService, rule_view::RuleViewService, task::TaskService},
};

/// 飞书汇报服务
///
/// 提供完整的汇报功能，包括规则、看板和任务管理
pub struct ReportService {
    /// 规则管理服务
    pub rule: RuleService,
    /// 规则看板管理服务
    pub rule_view: RuleViewService,
    /// 任务管理服务
    pub task: TaskService,
}

impl ReportService {
    /// 创建飞书汇报服务实例
    pub fn new(config: Config) -> Self {
        Self {
            rule: RuleService::new(config.clone()),
            rule_view: RuleViewService::new(config.clone()),
            task: TaskService::new(config),
        }
    }
}
