/// Drive v2 API 模块
///
/// 提供云空间文件管理的增强功能
pub mod file;
pub mod permission;

// 使用通配符导出所有子模块
// file 模块显式导出
pub use file::{
    FileLike,
    ListFileLikesRequest,
    ListFileLikesResponse,
};
// permission 模块显式导出
pub use permission::{
    GetPermissionPublicRequest,
    GetPermissionPublicResponse,
    PermissionPublic,
    UpdatePermissionPublicRequest,
    UpdatePermissionPublicResponse,
};
