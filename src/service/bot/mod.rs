//! Bot服务模块 - 简化实现

use serde::{Deserialize, Serialize};
use crate::core::config::Config;
use crate::core::api_resp::{ApiResponseTrait, ResponseFormat};

/// 简化的服务结构体
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
    fn format(&self) -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// Bot服务
#[derive(Debug, Clone)]
pub struct BotService {
    pub service: SimpleService,
}

impl BotService {
    pub fn new(config: Config) -> Self {
        Self {
            service: SimpleService::new(config),
        }
    }
}

// Type alias for compatibility
pub type ServiceType = BotService;
pub type ResponseType = SimpleResponse;
