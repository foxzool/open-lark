/// 权限管理模块
pub mod member;
pub mod public;

// 重新导出所有API函数
// member 模块显式导出
pub use member::{
    AuthPermissionMemberRequest, AuthPermissionMemberResponse, BatchCreatePermissionMemberRequest,
    BatchCreatePermissionMemberResponse, CreatePermissionMemberRequest,
    CreatePermissionMemberResponse, DeletePermissionMemberRequest, DeletePermissionMemberResponse,
    ListPermissionMembersRequest, ListPermissionMembersResponse, PermissionMember,
    PermissionMemberItem, TransferOwnerRequest, TransferOwnerResponse,
    UpdatePermissionMemberRequest, UpdatePermissionMemberResponse,
};
// public 模块显式导出
pub use public::{
    CreatePermissionPublicPasswordRequest, CreatePermissionPublicPasswordResponse,
    DeletePermissionPublicPasswordRequest, DeletePermissionPublicPasswordResponse,
    GetPublicPermissionRequest, GetPublicPermissionResponse, PatchPublicPermissionRequest,
    PatchPublicPermissionResponse, PermissionPublic, PermissionPublicRequest,
    UpdatePermissionPublicPasswordRequest, UpdatePermissionPublicPasswordResponse,
};
