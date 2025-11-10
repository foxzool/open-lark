#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! 搜索服务模块
//!
//! 提供企业级的搜索功能，支持多种数据源的搜索和高级查询。
//! 包含用户搜索、文档搜索等多种搜索场景的完整实现。

use crate::api_resp::{ApiResponseTrait, ResponseFormat};
use crate::config::Config;
use serde::{Deserialize, Serialize};

// 导入V1版本API
pub mod v1;

/// 简化的服务结构体（保持向后兼容）
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

/// Search服务
#[derive(Debug, Clone)]
pub struct SearchService {
    pub service: SimpleService,
    /// V1版本搜索服务
    #[cfg(feature = "search")]
    pub v1: v1::SearchV1Service,
}

impl SearchService {
    pub fn new(config: Config) -> Self {
        Self {
            service: SimpleService::new(config.clone()),
            #[cfg(feature = "search")]
            v1: v1::SearchV1Service::new(config),
        }
    }
}

// Type alias for compatibility
pub type ServiceType = SearchService;
pub type ResponseType = SimpleResponse;
