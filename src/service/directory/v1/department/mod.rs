use crate::core::{config::Config, trait_system::Service};

pub mod create;
pub mod delete;
pub mod filter;
pub mod mget;
pub mod patch;
pub mod search;

// 重新导出所有请求和响应类型
pub use create::*;
pub use delete::*;
pub use filter::*;
pub use mget::*;
pub use patch::*;
pub use search::*;

/// 部门管理服务
///
/// 提供部门的创建、更新、删除、查询等管理功能
pub struct DepartmentService {
    pub config: Config,
}

impl DepartmentService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl Service for DepartmentService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "department"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}
