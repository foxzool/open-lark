//! 事件服务
//!
//! 提供飞书事件相关的API功能，包括：
//! - 获取事件出口IP地址

use openlark_core::config::Config;

/// 事件服务
#[derive(Debug, Clone)]
pub struct EventService {
    pub config: Config,
    pub v1: v1::EventServiceV1,
}

impl EventService {
    /// 创建事件服务实例
    pub fn new(config: Config) -> Self {
        let v1 = v1::EventServiceV1::new(config.clone());
        Self { config, v1 }
    }
}

/// v1版本API
pub mod v1;
