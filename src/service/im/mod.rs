#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! Im服务模块 - 简化实现

use api_resp::{ApiResponseTrait, ResponseFormat};
use config::Config;
use serde::{Deserialize, Serialize};

// 声明v1模块
pub mod v1;

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

/// Im服务
#[derive(Debug, Clone)]
pub struct ImService {
    pub service: SimpleService,
    pub v1: crate::service::im::v1::MessageService,
}

impl ImService {
    pub fn new(config: Config) -> Self {
        Self {
            service: SimpleService::new(config),
            v1: crate::service::im::v1::MessageService::new(),
        }
    }
}

// Type alias for compatibility
pub type ServiceType = ImService;
pub type ResponseType = SimpleResponse;
