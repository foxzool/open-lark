//! 考勤 v1 API

use crate::core::config::Config;

pub mod models;
pub mod shift;
pub mod user_flow;
pub mod user_task;

#[cfg(test)]
mod tests;

pub use models::*;

/// 考勤 v1 版本API集合
pub struct V1 {
    /// 用户考勤任务查询
    pub user_task: user_task::UserTaskService,
    /// 用户打卡流水查询
    pub user_flow: user_flow::UserFlowService,
    /// 排班信息查询
    pub shift: shift::ShiftService,
}

impl V1 {
    pub fn new(config: Config) -> Self {
        Self {
            user_task: user_task::UserTaskService::new(config.clone()),
            user_flow: user_flow::UserFlowService::new(config.clone()),
            shift: shift::ShiftService::new(config),
        }
    }
}
