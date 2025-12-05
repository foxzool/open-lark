//! App_Workflow服务模块 - 简化实现

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use openlark_core::config::Config;
use serde::{Deserialize, Serialize};

// 导入子模块
pub mod list;
pub mod update;

// 导出所有子模块内容，避免命名冲突
// list模块中的Workflow与update模块冲突，使用重导出避免冲突
pub use list::{ListWorkflowRequest, ListWorkflowResponse};
pub use update::*;

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
