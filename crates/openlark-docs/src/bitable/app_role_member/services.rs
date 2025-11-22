//! Bitable App Role Member API 服务实现
//!
//! 提供多维表格角色成员管理相关的API服务，包括：
//! - 角色成员的添加、查询、删除
//! - 批量成员操作支持
//! - 完整的错误处理和参数验证
use std::collections::HashMap;

use openlark_core::{
    api::ApiRequest, config::Config, constants::AccessTokenType, error::LarkAPIError,
    http::Transport, SDKResult,
};

use super::models::*;

/// 多维表格角色成员管理服务
#[derive(Debug, Clone)]
pub struct AppRoleMemberService {
    config: Config,
}

impl AppRoleMemberService {
    /// 创建新的角色成员管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建角色成员
    ///
    /// 在指定的角色中添加一个成员
    ///
    /// # 参数
    /// * `request` - 创建角色成员请求
    ///
    /// # 返回
    /// 返回新创建的成员信息
    pub async fn create_role_member(
        &self,
        request: &CreateRoleMemberRequest,
    ) -> SDKResult<CreateRoleMemberResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "创建角色成员: app_token={}, role_id={}, user_id={}",
            request.app_token,
            request.role_id,
            request.user_id
        );

        // 构建请求体
        let mut body = HashMap::new();
        body.insert(
            "role_id".to_string(),
            serde_json::to_value(&request.role_id)?,
        );
        body.insert(
            "user_id".to_string(),
            serde_json::to_value(&request.user_id)?,
        );

        // 构建API请求
        let api_req = ApiRequest::post(format!(
                "/open-apis/bitable/v1/apps/{}/role_members",
                request.app_token
            ))
            .body(serde_json::to_string(&body)?)
            .query(HashMap::new());

        // 发送请求
        let resp =
            Transport::<CreateRoleMemberResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "创建角色成员完成: app_token={}, role_id={}, user_id={}",
            request.app_token,
            request.role_id,
            request.user_id
        );

        Ok(response)
    }

    /// 删除角色成员
    ///
    /// 从指定角色中删除一个成员
    ///
    /// # 参数
    /// * `request` - 删除角色成员请求
    ///
    /// # 返回
    /// 返回删除操作的结果
    pub async fn delete_role_member(
        &self,
        request: &DeleteRoleMemberRequest,
    ) -> SDKResult<DeleteRoleMemberResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "删除角色成员: app_token={}, role_id={}, member_id={}",
            request.app_token,
            request.role_id,
            request.member_id
        );

        // 构建API请求
        let api_req = ApiRequest::delete(format!(
                "/open-apis/bitable/v1/apps/{}/role_members/{}",
                request.app_token, request.member_id
            ))
            .query(HashMap::new());

        // 发送请求
        let resp =
            Transport::<DeleteRoleMemberResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "删除角色成员完成: app_token={}, role_id={}, member_id={}",
            request.app_token,
            request.role_id,
            request.member_id
        );

        Ok(response)
    }

    /// 列出角色成员
    ///
    /// 获取指定角色中的成员列表
    ///
    /// # 参数
    /// * `request` - 列出角色成员请求
    ///
    /// # 返回
    /// 返回成员列表
    pub async fn list_role_members(
        &self,
        request: &ListRoleMembersRequest,
    ) -> SDKResult<ListRoleMembersResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "列出角色成员: app_token={}, role_id={}",
            request.app_token,
            request.role_id
        );

        // 构建查询参数
        let mut query_params = HashMap::new();
        query_params.insert("role_id", request.role_id.clone());

        if let Some(page_size) = request.page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(ref page_token) = request.page_token {
            query_params.insert("page_token", page_token.clone());
        }

        // 构建API请求
        let api_req = ApiRequest::get(format!(
                "/open-apis/bitable/v1/apps/{}/role_members",
                request.app_token
            ))
            .query(query_params);

        // 发送请求
        let resp =
            Transport::<ListRoleMembersResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "列出角色成员完成: app_token={}, role_id={}, count={}",
            request.app_token,
            request.role_id,
            response.members.as_ref().map(|m| m.len()).unwrap_or(0)
        );

        Ok(response)
    }

    /// 批量创建角色成员
    ///
    /// 在指定的角色中批量添加多个成员
    ///
    /// # 参数
    /// * `request` - 批量创建角色成员请求
    ///
    /// # 返回
    /// 返回新创建的成员信息列表
    pub async fn batch_create_role_member(
        &self,
        request: &BatchCreateRoleMemberRequest,
    ) -> SDKResult<BatchCreateRoleMemberResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "批量创建角色成员: app_token={}, role_id={}, count={}",
            request.app_token,
            request.role_id,
            request.members.len()
        );

        // 构建请求体
        let mut body = HashMap::new();
        body.insert(
            "role_id".to_string(),
            serde_json::to_value(&request.role_id)?,
        );
        body.insert(
            "members".to_string(),
            serde_json::to_value(&request.members)?,
        );

        // 构建API请求
        let api_req = ApiRequest::post(format!(
                "/open-apis/bitable/v1/apps/{}/role_members/batch_create",
                request.app_token
            ))
            .body(serde_json::to_string(&body)?)
            .query(HashMap::new());

        // 发送请求
        let resp = Transport::<BatchCreateRoleMemberResponse>::request(api_req, &self.config, None)
            .await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "批量创建角色成员完成: app_token={}, role_id={}",
            request.app_token,
            request.role_id
        );

        Ok(response)
    }

    /// 批量删除角色成员
    ///
    /// 从指定角色中批量删除多个成员
    ///
    /// # 参数
    /// * `request` - 批量删除角色成员请求
    ///
    /// # 返回
    /// 返回批量删除的结果
    pub async fn batch_delete_role_member(
        &self,
        request: &BatchDeleteRoleMemberRequest,
    ) -> SDKResult<BatchDeleteRoleMemberResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "批量删除角色成员: app_token={}, role_id={}, count={}",
            request.app_token,
            request.role_id,
            request.member_ids.len()
        );

        // 构建请求体
        let mut body = HashMap::new();
        body.insert(
            "role_id".to_string(),
            serde_json::to_value(&request.role_id)?,
        );
        body.insert(
            "member_ids".to_string(),
            serde_json::to_value(&request.member_ids)?,
        );

        // 构建API请求
        let api_req = ApiRequest::post(format!(
                "/open-apis/bitable/v1/apps/{}/role_members/batch_delete",
                request.app_token
            ))
            .body(serde_json::to_string(&body)?)
            .query(HashMap::new());

        // 发送请求
        let resp = Transport::<BatchDeleteRoleMemberResponse>::request(api_req, &self.config, None)
            .await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "批量删除角色成员完成: app_token={}, role_id={}",
            request.app_token,
            request.role_id
        );

        Ok(response)
    }
}

// 构建器模式实现
pub struct CreateRoleMemberRequestBuilder {
    request: CreateRoleMemberRequest,
}

impl CreateRoleMemberRequestBuilder {
    pub fn new(
        app_token: impl Into<String>,
        role_id: impl Into<String>,
        user_id: impl Into<String>,
    ) -> Self {
        Self {
            request: CreateRoleMemberRequest {
                app_token: app_token.into(),
                role_id: role_id.into(),
                user_id: user_id.into(),
            },
        }
    }

    pub async fn execute(
        self,
        service: &AppRoleMemberService,
    ) -> SDKResult<CreateRoleMemberResponse> {
        service.create_role_member(&self.request).await
    }
}
