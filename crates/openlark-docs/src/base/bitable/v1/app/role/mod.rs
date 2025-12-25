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

pub use create::{CreateAppRoleRequest, CreateAppRoleRequestBuilder, CreateAppRoleResponse};
pub use delete::{DeleteAppRoleRequest, DeleteAppRoleRequestBuilder, DeleteAppRoleResponse};
pub use list::{ListAppRoleRequest, ListAppRoleRequestBuilder, ListAppRoleResponse};
pub use member::*;
pub use models::{BlockRole, Role, TableRole};
pub use update::{UpdateAppRoleRequest, UpdateAppRoleRequestBuilder, UpdateAppRoleResponse};

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

    pub fn create(&self) -> CreateAppRoleRequestBuilder {
        CreateAppRoleRequestBuilder::new(self.config.clone())
    }

    pub fn update(&self) -> UpdateAppRoleRequestBuilder {
        UpdateAppRoleRequestBuilder::new(self.config.clone())
    }

    pub fn delete(&self) -> DeleteAppRoleRequestBuilder {
        DeleteAppRoleRequestBuilder::new(self.config.clone())
    }

    pub fn list(&self) -> ListAppRoleRequestBuilder {
        ListAppRoleRequestBuilder::new(self.config.clone())
    }
}

// Type alias for compatibility
pub type ServiceType = AppRoleService;
