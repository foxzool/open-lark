use crate::core::config::Config;

pub mod department;
pub mod employee;
pub mod models;

pub use models::*;

/// 组织架构 v1 API
pub struct V1 {
    /// 员工管理
    pub employee: employee::EmployeeService,
    /// 部门管理
    pub department: department::DepartmentService,
}

impl V1 {
    pub fn new(config: Config) -> Self {
        Self {
            employee: employee::EmployeeService::new(config.clone()),
            department: department::DepartmentService::new(config),
        }
    }
}
