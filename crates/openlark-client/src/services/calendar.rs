//! 📅 OpenLark 日历会议服务
//!
//! 提供飞书日历功能，包括会议安排、日程管理等
//! 支持重复提醒和资源预订

use crate::Config;

/// 📅 日历会议服务
///
/// 提供飞书日历管理功能
#[derive(Debug, Clone)]
pub struct CalendarService {
    /// 🔧 客户端配置
    config: Config,
}

impl CalendarService {
    /// 🆕 创建新的日历服务实例
    pub fn new() -> Self {
        Self {
            config: Config::default(),
        }
    }

    /// 🆕 基于配置创建日历服务实例
    pub fn with_config(config: Config) -> Self {
        Self { config }
    }

    /// 🔍 获取当前配置
    pub fn config(&self) -> &Config {
        &self.config
    }
}
