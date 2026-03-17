pub mod create;
pub mod delete;
pub mod list;
pub mod member;
pub mod models;
pub mod tests;
pub mod update;

// create 模块显式导出

pub use create::{

    CreateAppRoleRequest,

    CreateAppRoleRequestBody,

    CreateAppRoleResponse,

};
// delete 模块显式导出
pub use delete::{
    DeleteAppRoleRequest,
    DeleteAppRoleResponse,
};
// list 模块显式导出
pub use list::{
    ListAppRoleRequest,
    ListAppRoleResponse,
};
// member 模块显式导出
pub use member::{
    BatchCreateRoleMemberRequest,
    BatchCreateRoleMemberResponse,
    BatchDeleteRoleMemberRequest,
    BatchDeleteRoleMemberResponse,
    CreateRoleMemberRequest,
    CreateRoleMemberResponse,
    DeleteRoleMemberRequest,
    DeleteRoleMemberResponse,
    ListRoleMembersRequest,
    ListRoleMembersResponse,
    RoleMemberId,
    RoleMemberIdType,
    RoleMemberInfo,
    RoleMemberType,
};
// models 模块显式导出
pub use models::{
    BlockRole,
    Role,
    TableRole,
};
#[allow(unused_imports)]
// tests 模块显式导出
pub use tests::{};
// update 模块显式导出
pub use update::{
    UpdateAppRoleRequest,
    UpdateAppRoleRequestBody,
    UpdateAppRoleResponse,
};

// 兼容历史导出（在 app/mod.rs 中被引用）
pub use create::CreateAppRoleRequestBody;
pub use update::UpdateAppRoleRequestBody;

// Type alias for compatibility
pub type ServiceType = ();
