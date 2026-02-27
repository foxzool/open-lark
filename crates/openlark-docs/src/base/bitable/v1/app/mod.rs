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

#[allow(unused_imports)]
pub use copy::*;
#[allow(unused_imports)]
pub use create::*;
#[allow(unused_imports)]
pub use get::*;
#[allow(unused_imports)]
pub use update::*;
pub use workflow::*;

// 显式导出 models 中不与操作文件冲突的类型
pub use models::{App, AppService, AppSettings, DeleteAppResponse};

// 显式导出 dashboard（避免 list 模块名与 role/table 冲突）
pub use dashboard::{
    CopyDashboardRequest, CopyDashboardResponse, Dashboard, Creator, ListDashboardsRequest,
    ListDashboardsResponse, Permission,
};

// 显式导出 role（避免 create/delete/list/update 模块名与 table 冲突）
pub use role::{
    BlockRole, CreateAppRoleRequest, CreateAppRoleRequestBody, CreateAppRoleResponse,
    DeleteAppRoleRequest, DeleteAppRoleResponse, ListAppRoleRequest, ListAppRoleResponse, Role,
    ServiceType, TableRole, UpdateAppRoleRequest, UpdateAppRoleRequestBody,
    UpdateAppRoleResponse,
};

// 显式导出 table（避免 create/delete/list 模块名与 role 冲突）
pub use table::{
    BatchCreateTableRequest, BatchCreateTableResponse, BatchDeleteTableRequest,
    BatchDeleteTableResponse, CreateTableRequest, CreateTableResponse, DeleteTableRequest,
    DeleteTableResponse, Field, FieldDescription, FieldProperty, FieldType, ListTablesRequest,
    ListTablesResponse, PatchTableRequest, PatchTableResponse, TableData, TableField, TableInfo,
};
