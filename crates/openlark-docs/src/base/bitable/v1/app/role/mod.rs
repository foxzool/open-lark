/// create 子模块。
pub mod create;
/// delete 子模块。
pub mod delete;
/// list 子模块。
pub mod list;
/// member 子模块。
pub mod member;
/// models 子模块。
pub mod models;
/// tests 子模块。
pub mod tests;
/// update 子模块。
pub mod update;

// create 模块显式导出

/// 重新导出相关类型。
pub use create::{CreateAppRoleRequest, CreateAppRoleRequestBody, CreateAppRoleResponse};
// delete 模块显式导出
/// 重新导出相关类型。
pub use delete::{DeleteAppRoleRequest, DeleteAppRoleResponse};
// list 模块显式导出
/// 重新导出相关类型。
pub use list::{ListAppRoleRequest, ListAppRoleResponse};
// member 模块显式导出
/// 重新导出相关类型。
pub use member::{
    BatchCreateRoleMemberRequest, BatchCreateRoleMemberResponse, BatchDeleteRoleMemberRequest,
    BatchDeleteRoleMemberResponse, CreateRoleMemberRequest, CreateRoleMemberResponse,
    DeleteRoleMemberRequest, DeleteRoleMemberResponse, ListRoleMembersRequest,
    ListRoleMembersResponse, RoleMemberId, RoleMemberIdType, RoleMemberInfo, RoleMemberType,
};
// models 模块显式导出
/// 重新导出相关类型。
pub use models::{BlockRole, Role, TableRole};
#[allow(unused_imports)]
// tests 模块显式导出
/// 重新导出相关类型。
pub use tests::{};
// update 模块显式导出
/// 重新导出相关类型。
pub use update::{UpdateAppRoleRequest, UpdateAppRoleRequestBody, UpdateAppRoleResponse};

// Type alias for compatibility
/// 兼容旧版接口的服务类型占位符。
pub type ServiceType = ();
