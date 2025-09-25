//! 权限管理模块
//!
//! 提供云文档权限相关功能：
//! - 协作者权限管理（增删改查、批量操作、权限检查、所有者转移等）
//! - 公开权限设置（v1/v2版本）

pub mod member;
pub mod public_v1;
pub mod public_v2;

use crate::core::{config::Config, req_option::RequestOption, SDKResult};

use self::{member::*, public_v1::*, public_v2::*};

/// 权限服务
pub struct PermissionService {
    config: Config,
}

impl PermissionService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 使用共享配置（实验性）
    pub fn new_from_shared(shared: std::sync::Arc<Config>) -> Self {
        Self {
            config: (*shared).clone(),
        }
    }

    /// 批量增加协作者权限
    pub async fn batch_create_member(
        &self,
        request: BatchCreatePermissionMemberRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<BatchCreatePermissionMemberResponse>> {
        batch_create_permission_member(request, &self.config, option).await
    }

    /// 转移所有者
    pub async fn transfer_owner(
        &self,
        request: TransferOwnerRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<TransferOwnerResponse>> {
        transfer_owner(request, &self.config, option).await
    }

    /// 判断当前用户是否有某权限
    pub async fn auth_permission(
        &self,
        request: AuthPermissionRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<AuthPermissionResponse>> {
        auth_permission(request, &self.config, option).await
    }

    /// 获取协作者列表
    pub async fn list_members(
        &self,
        request: ListPermissionMembersRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<ListPermissionMembersResponse>> {
        list_permission_members(request, &self.config, option).await
    }

    /// 增加协作者权限
    pub async fn create_member(
        &self,
        request: CreatePermissionMemberRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<CreatePermissionMemberResponse>> {
        create_permission_member(request, &self.config, option).await
    }

    /// 更新协作者权限
    pub async fn update_member(
        &self,
        request: UpdatePermissionMemberRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<UpdatePermissionMemberResponse>> {
        update_permission_member(request, &self.config, option).await
    }

    /// 移除协作者权限
    pub async fn delete_member(
        &self,
        request: DeletePermissionMemberRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<DeletePermissionMemberResponse>> {
        delete_permission_member(request, &self.config, option).await
    }

    /// 获取云文档权限设置
    pub async fn get_permission_public(
        &self,
        request: GetPermissionPublicRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<GetPermissionPublicResponse>> {
        get_permission_public(request, &self.config, option).await
    }

    /// 更新云文档权限设置
    pub async fn patch_permission_public(
        &self,
        request: PatchPermissionPublicRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<PatchPermissionPublicResponse>> {
        patch_permission_public(request, &self.config, option).await
    }

    /// 开启密码保护
    pub async fn create_password(
        &self,
        request: CreatePasswordRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<CreatePasswordResponse>> {
        create_password(request, &self.config, option).await
    }

    /// 刷新密码
    pub async fn update_password(
        &self,
        request: UpdatePasswordRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<UpdatePasswordResponse>> {
        update_password(request, &self.config, option).await
    }

    /// 关闭密码保护
    pub async fn delete_password(
        &self,
        request: DeletePasswordRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<DeletePasswordResponse>> {
        delete_password(request, &self.config, option).await
    }

    /// 获取云文档权限设置 (v2)
    pub async fn get_permission_public_v2(
        &self,
        request: GetPermissionPublicV2Request,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<GetPermissionPublicV2Response>> {
        get_permission_public_v2(request, &self.config, option).await
    }

    /// 更新云文档权限设置 (v2)
    pub async fn patch_permission_public_v2(
        &self,
        request: PatchPermissionPublicV2Request,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<PatchPermissionPublicV2Response>> {
        patch_permission_public_v2(request, &self.config, option).await
    }
}
