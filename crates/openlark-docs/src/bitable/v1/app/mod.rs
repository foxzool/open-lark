use openlark_core::config::Config;

// 导入Base模块的RoleService
#[cfg(feature = "base")]
use crate::base::v2::app::role::RoleService;

// 应用级别API
pub mod copy;
pub mod create;
pub mod get;
pub mod models;
pub mod update;

// 子模块API
pub mod dashboard;
pub mod role;
pub mod table;
pub mod workflow;

// 导出所有API，但避免命名冲突
pub use copy::*;
pub use create::*;
#[allow(ambiguous_glob_reexports)]
pub use dashboard::*;
pub use get::*;
pub use update::*;

// 角色相关 - 选择性导入以避免命名冲突
pub use role::{
    AppRoleService, RoleMemberService, CreateAppRoleRequest, CreateAppRoleRequestBuilder, CreateAppRoleResponse,
    DeleteAppRoleRequest, DeleteAppRoleRequestBuilder, DeleteAppRoleResponse,
    ListAppRoleRequest, ListAppRoleRequestBuilder, ListAppRoleResponse,
    UpdateAppRoleRequest, UpdateAppRoleRequestBuilder, UpdateAppRoleResponse,
    BatchCreateRoleMemberRequest, BatchCreateRoleMemberRequestBuilder, BatchCreateRoleMemberResponse,
    BatchDeleteRoleMemberRequest, BatchDeleteRoleMemberRequestBuilder, BatchDeleteRoleMemberResponse,
    CreateRoleMemberRequest, CreateRoleMemberRequestBuilder, CreateRoleMemberResponse,
    RoleMemberInfo, CreateAppRoleRequestBody, UpdateAppRoleRequestBody
};

#[allow(ambiguous_glob_reexports)]
pub use table::*;

// 工作流相关 - 选择性导入以避免命名冲突
pub use workflow::{
    AppWorkflowService, ListWorkflowRequest, ListWorkflowResponse,
    UpdateWorkflowRequest, UpdateWorkflowResponse
};

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
    #[cfg(feature = "base")]
    pub fn role_service(&self) -> RoleService {
        RoleService::new(self.config.clone())
    }

    /// 获取仪表盘服务
    pub fn dashboard_service(&self) -> dashboard::AppDashboardService {
        dashboard::AppDashboardService::new(self.config.clone())
    }

    /// 获取工作流服务
    pub fn workflow_service(&self) -> workflow::AppWorkflowService {
        workflow::AppWorkflowService::new(self.config.clone())
    }
}
