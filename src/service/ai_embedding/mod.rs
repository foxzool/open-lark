#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! Ai_Embedding服务模块 - 简化实现

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

/// Ai_Embedding服务
#[derive(Debug, Clone)]
pub struct AiembeddingService {
    pub service: SimpleService,
}

impl AiembeddingService {
    pub fn new(config: Config) -> Self {
        Self {
            service: SimpleService::new(config),
        }
    }
}

// Type alias for compatibility
pub type ServiceType = AiembeddingService;
pub type ResponseType = SimpleResponse;
