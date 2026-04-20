/// 获取公开权限接口。
pub mod get;
/// 公开权限模型模块。
pub mod models;
/// 公开密码模块。
pub mod password;
/// 更新公开权限接口。
pub mod patch;

/// 重新导出获取公开权限类型。
pub use get::{GetPublicPermissionRequest, GetPublicPermissionResponse};
/// 重新导出公开权限模型。
pub use models::PermissionPublic;
/// 重新导出公开密码相关类型。
pub use password::{
    CreatePermissionPublicPasswordRequest, CreatePermissionPublicPasswordResponse,
    DeletePermissionPublicPasswordRequest, DeletePermissionPublicPasswordResponse,
    UpdatePermissionPublicPasswordRequest, UpdatePermissionPublicPasswordResponse,
};
/// 重新导出更新公开权限类型。
pub use patch::{
    PatchPublicPermissionRequest, PatchPublicPermissionResponse, PermissionPublicRequest,
};
