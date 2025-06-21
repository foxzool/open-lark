//! 权限管理模块
//!
//! 提供云文档权限相关功能：
//! - 协作者权限管理（增删改查、批量操作、权限检查、所有者转移等）
//! - 公开权限设置（v1/v2版本）

pub mod member;

use std::sync::Arc;

use crate::core::{config::Config, req_option::RequestOption, SDKResult};

use self::member::*;

/// 权限服务
pub struct PermissionService {
    config: Arc<Config>,
}

impl PermissionService {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 批量增加协作者权限
    pub async fn batch_create_member(
        &self,
        request: &BatchCreatePermissionMemberRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<BatchCreatePermissionMemberResponse>> {
        batch_create_permission_member(request.clone(), &*self.config, option).await
    }

    /// 转移所有者
    pub async fn transfer_owner(
        &self,
        request: &TransferOwnerRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<TransferOwnerResponse>> {
        transfer_owner(request.clone(), &*self.config, option).await
    }

    /// 判断当前用户是否有某权限
    pub async fn auth_permission(
        &self,
        request: &AuthPermissionRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<AuthPermissionResponse>> {
        auth_permission(request.clone(), &*self.config, option).await
    }

    /// 获取协作者列表
    pub async fn list_members(
        &self,
        request: &ListPermissionMembersRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<ListPermissionMembersResponse>> {
        list_permission_members(request.clone(), &*self.config, option).await
    }

    /// 增加协作者权限
    pub async fn create_member(
        &self,
        request: &CreatePermissionMemberRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<CreatePermissionMemberResponse>> {
        create_permission_member(request.clone(), &*self.config, option).await
    }

    /// 更新协作者权限
    pub async fn update_member(
        &self,
        request: &UpdatePermissionMemberRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<UpdatePermissionMemberResponse>> {
        update_permission_member(request.clone(), &*self.config, option).await
    }

    /// 移除协作者权限
    pub async fn delete_member(
        &self,
        request: &DeletePermissionMemberRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<crate::core::api_resp::BaseResponse<DeletePermissionMemberResponse>> {
        delete_permission_member(request.clone(), &*self.config, option).await
    }
}
