//! App_Workflow服务模块 - 简化实现

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use openlark_core::config::Config;
use serde::{Deserialize, Serialize};
/// 简化的服务结构体
pub struct SimpleService {}

impl SimpleService {}
#[derive(Debug, Deserialize, Serialize)]
pub struct SimpleResponse;
impl ApiResponseTrait for SimpleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// App_Workflow服务
pub struct AppWorkflowService {}

impl AppWorkflowService {
    pub fn new(_config: Config) -> Self {
        Self {}
    }
}

// Type alias for compatibility
pub type ServiceType = AppWorkflowService;
pub type ResponseType = SimpleResponse;
