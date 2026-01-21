//! 目录服务 V1 API
//!
//! 提供目录服务 V1 版本的 API 访问

use crate::PlatformConfig;
use std::sync::Arc;

/// 目录服务 V1 API
#[derive(Debug, Clone)]
pub struct DirectoryV1 {
    /// 客户端配置
    config: Arc<PlatformConfig>,
}

impl DirectoryV1 {
    /// 创建新的目录服务 V1 实例
    pub fn new(config: Arc<PlatformConfig>) -> Self {
        Self { config }
    }

    /// 用户搜索
    pub fn users(&self) -> super::v1::users::UsersApi {
        super::v1::users::UsersApi::new(self.config.clone(), self.client.clone())
    }

    /// 部门搜索
    pub fn departments(&self) -> super::v1::departments::DepartmentsApi {
        super::v1::departments::DepartmentsApi::new(self.config.clone(), self.client.clone())
    }

    /// 同步服务
    pub fn sync(&self) -> super::v1::sync::SyncApi {
        super::v1::sync::SyncApi::new(self.config.clone(), self.client.clone())
    }
}

pub mod users;
pub mod departments;
pub mod sync;
