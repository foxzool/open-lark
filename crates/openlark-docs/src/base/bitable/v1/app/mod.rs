/// Bitable应用管理模块
pub mod copy;
pub mod create;
pub mod dashboard;
pub mod get;
pub mod models;
pub mod role;
pub mod table;
pub mod update;
pub mod workflow;

// copy 模块显式导出

pub use copy::{

    CopyAppRequest,

    CopyAppResponse,

};
// create 模块显式导出
pub use create::{
    CreateAppRequest,
    CreateAppResponse,
};
// get 模块显式导出
pub use get::{
    GetAppRequest,
    GetAppResponse,
};
// update 模块显式导出
pub use update::{
    UpdateAppRequest,
    UpdateAppResponse,
};
// workflow 模块显式导出
pub use workflow::{
    ListWorkflowRequest,
    ListWorkflowResponse,
    UpdateWorkflowBody,
    UpdateWorkflowRequest,
    UpdateWorkflowResponse,
    Workflow,
    WorkflowStatus,
};

// 显式导出 models 中不与操作文件冲突的类型
pub use models::{App, AppService, AppSettings, DeleteAppResponse};

// 显式导出 dashboard（避免 list 模块名与 role/table 冲突）
pub use dashboard::{
    CopyDashboardRequest, CopyDashboardResponse, Creator, Dashboard, ListDashboardsRequest,
    ListDashboardsResponse, Permission,
};

// 显式导出 role（避免 create/delete/list/update 模块名与 table 冲突）
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
pub use table::*;
