//! App_Role服务模块 - 简化实现

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
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

/// AppRole服务
pub struct AppRoleService {}

impl AppRoleService {}

// Type alias for compatibility
pub type ServiceType = AppRoleService;
pub type ResponseType = SimpleResponse;
