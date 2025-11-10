#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! Performance V1服务 - 企业绩效管理v1版本API
//!
//! 提供全面的绩效管理功能：
//! - 绩效周期管理
//! - 绩效活动管理
//! - 绩效结果管理
//! - 评估模板和项目管理
//! - 统计分析功能

use open_lark_core::config::Config;
use serde::{Deserialize, Serialize};

// 声明子模块
pub mod activities;
pub mod cycles;
pub mod results;
pub mod reviews;
pub mod templates;

// 重新导出服务类型
pub use activities::ActivitiesService;
pub use cycles::CyclesService;
pub use results::ResultsService;
pub use reviews::ReviewsService;
pub use templates::TemplatesService;

/// Performance V1服务
#[derive(Debug, Clone)]
pub struct PerformanceV1Service {
    pub config: Config,
    pub cycles: CyclesService,
    pub activities: ActivitiesService,
    pub results: ResultsService,
    pub templates: TemplatesService,
    pub reviews: ReviewsService,
}

impl PerformanceV1Service {
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            cycles: CyclesService::new(config.clone()),
            activities: ActivitiesService::new(config.clone()),
            results: ResultsService::new(config.clone()),
            templates: TemplatesService::new(config.clone()),
            reviews: ReviewsService::new(config),
        }
    }
}

// ==================== 事件模型 ====================

/// 绩效结果开通事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2PerformanceResultOpenedV1 {
    /// 事件ID
    pub event_id: String,
    /// 事件类型
    pub event_type: String,
    /// 事件创建时间戳
    pub created_time: String,
    /// 事件内容
    pub event: PerformanceResultOpenedEvent,
}

/// 绩效结果开通事件内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceResultOpenedEvent {
    /// 周期ID
    pub semester_id: String,
    /// 项目ID
    pub activity_id: String,
    /// 被评估人ID列表
    pub reviewee_ids: Vec<String>,
    /// 开通时间戳
    pub opened_at: i64,
}

/// 绩效详情变更事件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct P2PerformanceDetailChangedV1 {
    /// 事件ID
    pub event_id: String,
    /// 事件类型
    pub event_type: String,
    /// 事件创建时间戳
    pub created_time: String,
    /// 事件内容
    pub event: PerformanceDetailChangedEvent,
}

/// 绩效详情变更事件内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceDetailChangedEvent {
    /// 项目ID
    pub activity_id: String,
    /// 被评估人ID
    pub reviewee_id: String,
    /// 评估人ID
    pub reviewer_id: String,
    /// 变更的评估项ID列表
    pub changed_item_ids: Vec<String>,
    /// 变更类型 (created updated, deleted)
    pub change_type: String,
    /// 变更时间戳
    pub changed_at: i64,
}
