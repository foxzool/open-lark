//! App_Engine服务模块 - 简化实现

use crate::core::api_resp::{ApiResponseTrait, ResponseFormat};
use crate::core::config::Config;
use serde::{Deserialize, Serialize};

/// 简化的服务结构体
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

/// App_Engine服务
#[derive(Debug, Clone)]
pub struct AppengineService {
    pub service: SimpleService,
}

impl AppengineService {
    pub fn new(config: Config) -> Self {
        Self {
            service: SimpleService::new(config),
        }
    }
}

// Type alias for compatibility
pub type ServiceType = AppengineService;
pub type ResponseType = SimpleResponse;
