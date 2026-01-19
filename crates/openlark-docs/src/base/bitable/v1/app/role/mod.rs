/// 自定义角色服务模块
///
/// 提供多维表格高级权限中自定义角色的创建、更新、删除、查询等能力。
use openlark_core::config::Config;

pub mod create;
pub mod delete;
pub mod list;
pub mod member;
pub mod models;
pub mod update;

#[cfg(test)]
mod tests;

pub use create::{CreateAppRoleRequest, CreateAppRoleResponse};
pub use delete::{DeleteAppRoleRequest, DeleteAppRoleResponse};
pub use list::{ListAppRoleRequest, ListAppRoleResponse};
pub use member::*;
pub use models::{BlockRole, Role, TableRole};
pub use update::{UpdateAppRoleRequest, UpdateAppRoleResponse};

// 兼容历史导出（在 app/mod.rs 中被引用）
pub use create::CreateAppRoleRequestBody;
pub use update::UpdateAppRoleRequestBody;

/// 角色服务
pub struct AppRoleService {
    config: Config,
}

impl AppRoleService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 创建自定义角色请求
    pub fn create(&self) -> CreateAppRoleRequest {
        CreateAppRoleRequest::new(self.config.clone())
    }

    /// 更新自定义角色请求
    pub fn update(&self) -> UpdateAppRoleRequest {
        UpdateAppRoleRequest::new(self.config.clone())
    }

    /// 删除自定义角色请求
    pub fn delete(&self) -> DeleteAppRoleRequest {
        DeleteAppRoleRequest::new(self.config.clone())
    }

    /// 查询自定义角色列表请求
    pub fn list(&self) -> ListAppRoleRequest {
        ListAppRoleRequest::new(self.config.clone())
    }
}

// Type alias for compatibility
pub type ServiceType = AppRoleService;
