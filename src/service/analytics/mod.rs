#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! Analytics数据分析服务
//!
//! 提供企业级数据分析的完整功能：
//! - 数据概览和统计分析
//! - 用户行为分析和洞察
//! - 应用使用统计和趋势分析
//! - 自定义报表和数据可视化
//! - 智能业务洞察和预测分析
//! - 实时监控和智能告警

use crate::api_resp::{ApiResponseTrait, ResponseFormat};
use crate::config::Config;
use serde::{Deserialize, Serialize};

// 导入V1版本的服务
pub mod v1;
pub use v1::*;

/// Analytics服务主入口
#[derive(Debug, Clone)]
pub struct AnalyticsService {
    pub config: Config,
    pub v1: AnalyticsServiceV1,
}

impl AnalyticsService {
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            v1: AnalyticsServiceV1::new(config),
        }
    }
}

// 兼容性类型别名
pub type ServiceType = AnalyticsService;

// 保持向后兼容的简化实现
#[derive(Debug, Clone)]
pub struct SimpleService {
    pub config: Config,
}

impl SimpleService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SimpleResponse;

impl ApiResponseTrait for SimpleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// Type alias for compatibility
pub type ResponseType = SimpleResponse;
