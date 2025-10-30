//! Task服务模块
//!
//! 提供任务管理相关的API功能，包括：
//! - 任务创建和管理
//! - 任务状态跟踪
//! - 任务分配和协作

use crate::core::config::Config;

/// 任务服务
#[derive(Debug, Clone)]
pub struct TaskService {
    pub config: Config,
    pub v1: v1::TaskServiceV1,
}

impl TaskService {
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            v1: v1::TaskServiceV1::new(config),
        }
    }
}

pub mod v1;
