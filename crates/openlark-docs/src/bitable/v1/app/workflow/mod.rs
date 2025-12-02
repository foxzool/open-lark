//! App_Workflow服务模块 - 简化实现

use serde::{Deserialize, Serialize};
use openlark_core::config::Config;
use openlark_core::api::{ApiResponseTrait, ResponseFormat};
/// 简化的服务结构体
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
}

/// App_Workflow服务
pub struct App_WorkflowService {
}

impl App_WorkflowService {
    pub fn new(config: Config) -> Self {
        Self {}
    }
}

// Type alias for compatibility
pub type ServiceType = App_WorkflowService;
pub type ResponseType = SimpleResponse;