//! Passport服务模块 - 简化实现

use serde::{Deserialize, Serialize};
use crate::core::config::Config;
use crate::core::api_resp::{ApiResponseTrait, ResponseFormat};

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

/// Passport服务
#[derive(Debug, Clone)]
pub struct PassportService {
    pub service: SimpleService,
}

impl PassportService {
    pub fn new(config: Config) -> Self {
        Self {
            service: SimpleService::new(config),
        }
    }
}

// Type alias for compatibility
pub type ServiceType = PassportService;
pub type ResponseType = SimpleResponse;
