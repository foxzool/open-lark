use openlark_core::config::Config;

// 应用级别API
pub mod copy;
pub mod create;
pub mod delete;
pub mod get;
pub mod update;

// 子模块API
pub mod dashboard;
pub mod role;
pub mod table;
pub mod workflow;

// 导出所有API
pub use copy::*;
pub use create::*;
pub use delete::*;
pub use get::*;
pub use update::*;
pub use dashboard::*;
pub use role::*;
pub use table::*;
pub use workflow::*;

/// 多维表格应用服务
pub struct AppService {
    pub config: Config,
}

impl AppService {
    /// 创建应用服务
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取数据表服务
    pub fn table_service(&self) -> table::TableService {
        table::TableService::new(self.config.clone())
    }

    /// 获取角色服务
    pub fn role_service(&self) -> role::RoleService {
        role::RoleService::new(self.config.clone())
    }

    /// 获取仪表盘服务
    pub fn dashboard_service(&self) -> dashboard::DashboardService {
        dashboard::DashboardService::new(self.config.clone())
    }

    /// 获取工作流服务
    pub fn workflow_service(&self) -> workflow::WorkflowService {
        workflow::WorkflowService::new(self.config.clone())
    }
}