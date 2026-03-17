/// 公开密码管理模块
pub mod create;
pub mod delete;
pub mod update;

// 使用通配符导出所有子模块,避免维护大量重复的导出列表
// create 模块显式导出
pub use create::{
    CreatePermissionPublicPasswordRequest,
    CreatePermissionPublicPasswordResponse,
};
// delete 模块显式导出
pub use delete::{
    DeletePermissionPublicPasswordRequest,
    DeletePermissionPublicPasswordResponse,
};
// update 模块显式导出
pub use update::{
    UpdatePermissionPublicPasswordRequest,
    UpdatePermissionPublicPasswordResponse,
};
