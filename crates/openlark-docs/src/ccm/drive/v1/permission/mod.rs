/// 协作者权限模块。
pub mod member;
/// 公开权限模块。
pub mod public;

/// 重新导出协作者权限相关类型。
pub use member::{
    AuthPermissionMemberRequest, AuthPermissionMemberResponse, BatchCreatePermissionMemberRequest,
    BatchCreatePermissionMemberResponse, CreatePermissionMemberRequest,
    CreatePermissionMemberResponse, DeletePermissionMemberRequest, DeletePermissionMemberResponse,
    ListPermissionMembersRequest, ListPermissionMembersResponse, PermissionMember,
    PermissionMemberItem, TransferOwnerRequest, TransferOwnerResponse,
    UpdatePermissionMemberRequest, UpdatePermissionMemberResponse,
};
/// 重新导出公开权限相关类型。
pub use public::{
    CreatePermissionPublicPasswordRequest, CreatePermissionPublicPasswordResponse,
    DeletePermissionPublicPasswordRequest, DeletePermissionPublicPasswordResponse,
    GetPublicPermissionRequest, GetPublicPermissionResponse, PatchPublicPermissionRequest,
    PatchPublicPermissionResponse, PermissionPublic, PermissionPublicRequest,
    UpdatePermissionPublicPasswordRequest, UpdatePermissionPublicPasswordResponse,
};
