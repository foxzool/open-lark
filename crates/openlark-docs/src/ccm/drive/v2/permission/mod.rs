/// 公开权限接口模块。
pub mod public;

/// 重新导出公开权限相关类型。
pub use public::{
    GetPermissionPublicRequest, GetPermissionPublicResponse, PermissionPublic,
    UpdatePermissionPublicRequest, UpdatePermissionPublicResponse,
};
