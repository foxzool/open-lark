//! Message服务模块 - 简化实现
use serde::{Deserialize, Serialize};
use crate::core::config::Config;
use crate::core::api_resp::{ApiResponseTrait, ResponseFormat};
/// 简化的服务结构体
#[derive(Debug, Clone)]
pub struct SimpleService {
}

impl SimpleService {
}
#[derive(Debug, Deserialize, Serialize)]
pub struct SimpleResponse;
impl ApiResponseTrait for SimpleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
}
/// Message服务
#[derive(Debug, Clone)]
pub struct MessageService {
}

impl MessageService {
}
// Type alias for compatibility
pub type ServiceType = MessageService;
pub type ResponseType = SimpleResponse;
}