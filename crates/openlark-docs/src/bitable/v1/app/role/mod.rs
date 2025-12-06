//! App_Role服务模块 - 简化实现

use openlark_core::api::{ApiResponseTrait, ResponseFormat};
use serde::{Deserialize, Serialize};

// 导入子模块
pub mod create;
pub mod delete;
pub mod list;
pub mod update;
pub mod member;

#[cfg(test)]
mod tests;

// 导出所有子模块内容，避免命名冲突
pub use create::*;
// delete模块中的类型与create模块冲突，使用重导出避免冲突
pub use delete::{DeleteAppRoleRequest, DeleteAppRoleRequestBuilder, DeleteAppRoleResponse};
// list模块中的类型与create模块冲突，使用重导出避免冲突
pub use list::{ListAppRoleRequest, ListAppRoleRequestBuilder, ListAppRoleResponse};
// update模块中的类型与create模块冲突，使用重导出避免冲突
pub use update::{UpdateAppRoleRequest, UpdateAppRoleRequestBuilder, UpdateAppRoleResponse, UpdateTableRole, UpdateBlockRole, UpdateRole, UpdateAppRoleRequestBody};
pub use member::*;

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
