use crate::core::config::Config;

pub mod create;
pub mod delete;
pub mod filter;
pub mod mget;
pub mod patch;
pub mod search;

// 重新导出所有请求和响应类型
pub use create::*;
// TODO: 以下功能尚未实现，暂时注释
// pub use patch::*;
// pub use delete::*;
// pub use mget::*;
// pub use filter::*;
// pub use search::*;

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
