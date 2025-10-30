//! Basic Data v1 - 基础数据管理v1版本API
//!
//! 提供基础数据管理相关的所有功能，包括：
//! - 人员类型管理
//! - 工时制度管理
//! - 地点管理
//! - 国家证件类型管理
//! - 自定义字段管理

use crate::core::config::Config;

/// 基础数据管理v1版本服务
#[derive(Debug, Clone)]
pub struct BasicDataV1Service {
    config: Config,
}

impl BasicDataV1Service {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}