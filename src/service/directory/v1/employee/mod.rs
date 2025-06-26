use crate::core::config::Config;

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
pub use filter::*;
// TODO: 以下功能尚未实现，暂时注释
// pub use patch::*;
// pub use delete::*;
// pub use resurrect::*;
// pub use to_be_resigned::*;
// pub use regular::*;
// pub use mget::*;
// pub use search::*;

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
