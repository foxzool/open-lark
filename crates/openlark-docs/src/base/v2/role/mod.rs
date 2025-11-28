//! Base V2 角色管理模块
//!
//! 提供基础应用（base）V2版本的角色管理功能，包括：
//! - 创建自定义角色
//! - 更新角色信息
//! - 列出角色列表

use openlark_core::config::Config;

pub mod create;
pub mod list;
pub mod update;

// 测试模块
#[cfg(test)]
pub mod tests;

pub use create::{
    CreateRoleRequest, CreateRoleRequestBuilder, CreateRoleResponse,
    CreateRoleResponseData
};
pub use list::{
    ListRolesRequest, ListRolesRequestBuilder, ListRolesResponse,
    RoleInfo
};
pub use update::{
    UpdateRoleRequest, UpdateRoleRequestBuilder, UpdateRoleResponse,
    UpdateRoleResponseData
};

/// 角色管理服务
pub struct RoleService {
    pub config: Config,
}

impl RoleService {
    /// 创建角色管理服务
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置
    pub fn config(&self) -> &Config {
        &self.config
    }
}