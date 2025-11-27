//! 📋 OpenLark 任务管理服务
//!
//! 提供飞书任务功能，包括待办事项、项目协作等
//! 支持进度跟踪和团队协作

use crate::Config;

/// 📋 任务管理服务
///
/// 提供飞书任务管理功能
#[derive(Debug, Clone)]
pub struct TaskService {
    /// 🔧 客户端配置
    config: Config,
}

impl TaskService {
    /// 🆕 创建新的任务管理服务实例
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

    /// 🆕 基于配置创建任务管理服务实例
    pub fn with_config(config: Config) -> Self {
        Self { config }
    }

    /// 🔍 获取当前配置
    pub fn config(&self) -> &Config {
        &self.config
    }
}
