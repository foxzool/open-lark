/// 通讯录服务 v3 版本
///
/// 提供企业通讯录完整管理功能，包括：
/// - 权限范围管理
/// - 用户生命周期管理  
/// - 部门和组织架构管理
/// - 用户组和角色管理
/// - 职级职务管理等
pub mod models;
pub mod v3;

pub use models::*;
pub use v3::*;

use crate::core::config::Config;

/// 通讯录服务
pub struct ContactService {
    /// v3版本API
    pub v3: v3::V3,
}

impl ContactService {
    pub fn new(config: Config) -> Self {
        Self {
            v3: v3::V3::new(config),
        }
    }
}
