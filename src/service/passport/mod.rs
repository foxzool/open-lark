#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! Passport服务模块 - 简化实现

use crate::api_resp::{ApiResponseTrait, ResponseFormat};
use crate::config::Config;
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

/// Passport服务
#[derive(Debug, Clone)]
pub struct PassportService {
    pub service: SimpleService,
    /// v1版本的API服务
    pub v1: PassportV1Service,
}

impl PassportService {
    pub fn new(config: Config) -> Self {
        Self {
            service: SimpleService::new(config.clone()),
            v1: PassportV1Service::new(config),
        }
    }
}

// Type alias for compatibility
pub type ServiceType = PassportService;
pub type ResponseType = SimpleResponse;

// 导入v1服务
pub use v1::PassportV1Service;

pub mod models;
pub mod sessions;
pub mod v1;
