/// Bitable应用管理模块
pub mod block_workflow;
/// copy 子模块。
pub mod copy;
/// create 子模块。
pub mod create;
/// dashboard 子模块。
pub mod dashboard;
/// get 子模块。
pub mod get;
/// models 子模块。
pub mod models;
/// role 子模块。
pub mod role;
/// table 子模块。
pub mod table;
/// update 子模块。
pub mod update;
/// workflow 子模块。
pub mod workflow;

/// 重新导出相关类型。
pub use block_workflow::{BlockWorkflow, ListBlockWorkflowRequest, ListBlockWorkflowResponse};
// copy 模块显式导出

/// 重新导出相关类型。
pub use copy::{CopyAppRequest, CopyAppResponse};
// create 模块显式导出
/// 重新导出相关类型。
pub use create::{CreateAppRequest, CreateAppResponse};
// get 模块显式导出
/// 重新导出相关类型。
pub use get::{GetAppRequest, GetAppResponse};
// update 模块显式导出
/// 重新导出相关类型。
pub use update::{UpdateAppRequest, UpdateAppResponse};
// workflow 模块显式导出
/// 重新导出相关类型。
pub use workflow::{
    ListWorkflowRequest, ListWorkflowResponse, UpdateWorkflowBody, UpdateWorkflowRequest,
    UpdateWorkflowResponse, Workflow, WorkflowStatus,
};

// 显式导出 models 中不与操作文件冲突的类型
/// 重新导出相关类型。
pub use models::{App, AppService, AppSettings, DeleteAppResponse};

// 显式导出 dashboard（避免 list 模块名与 role/table 冲突）
/// 重新导出相关类型。
pub use dashboard::{
    CopyDashboardRequest, CopyDashboardResponse, Creator, Dashboard, ListDashboardsRequest,
    ListDashboardsResponse, Permission,
};

// 显式导出 role（避免 create/delete/list/update 模块名与 table 冲突）
/// 重新导出相关类型。
pub use role::{
    BatchCreateRoleMemberRequest, BatchCreateRoleMemberResponse, BatchDeleteRoleMemberRequest,
    BatchDeleteRoleMemberResponse, BlockRole, CreateAppRoleRequest, CreateAppRoleRequestBody,
    CreateAppRoleResponse, CreateRoleMemberRequest, CreateRoleMemberResponse, DeleteAppRoleRequest,
    DeleteAppRoleResponse, DeleteRoleMemberRequest, DeleteRoleMemberResponse, ListAppRoleRequest,
    ListAppRoleResponse, ListRoleMembersRequest, ListRoleMembersResponse, Role, RoleMemberId,
    RoleMemberIdType, RoleMemberInfo, RoleMemberType, ServiceType, TableRole, UpdateAppRoleRequest,
    UpdateAppRoleRequestBody, UpdateAppRoleResponse,
};

// 显式导出 table（避免 create/delete/list 模块名与 role 冲突）
/// 重新导出相关类型。
pub use table::*;
