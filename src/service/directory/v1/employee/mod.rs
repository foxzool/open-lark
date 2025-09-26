use crate::core::{config::Config, trait_system::Service};

pub mod create;
pub mod delete;
pub mod filter;
pub mod mget;
pub mod patch;
pub mod regular;
pub mod resurrect;
pub mod search;
pub mod to_be_resigned;

// 重新导出所有请求和响应类型
pub use create::*;
pub use delete::*;
pub use filter::*;
pub use mget::*;
pub use patch::*;
pub use regular::*;
pub use resurrect::*;
pub use search::*;
pub use to_be_resigned::*;

/// 员工管理服务
///
/// 提供员工的创建、更新、删除、查询等管理功能
pub struct EmployeeService {
    pub config: Config,
}

impl EmployeeService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

impl Service for EmployeeService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "employee"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}
