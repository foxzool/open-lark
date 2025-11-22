//! Bitable App Role API 服务实现
//!
//! 提供多维表格角色权限管理相关的API服务，包括：
//! - 角色的创建、查询、更新、删除
//! - 权限配置和管理
//! - 完整的错误处理和参数验证
use std::collections::HashMap;

use openlark_core::{
    api::ApiRequest, config::Config, constants::AccessTokenType, error::LarkAPIError,
    http::Transport, SDKResult,
};
use serde_json::Value;

use super::models::*;

/// 多维表格角色权限管理服务
#[derive(Debug, Clone)]
pub struct AppRoleService {
    config: Config,
}

impl AppRoleService {
    /// 创建新的角色权限管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建角色
    ///
    /// 在指定的应用中创建一个角色
    ///
    /// # 参数
    /// * `request` - 创建角色请求
    ///
    /// # 返回
    /// 返回新创建的角色信息
    pub async fn create_role(&self, request: &CreateRoleRequest) -> SDKResult<CreateRoleResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "创建角色: app_token={}, role_name={}, role_type={}",
            request.app_token,
            request.role_name,
            request.role_type
        );

        // 构建请求体
        let mut body = HashMap::new();
        body.insert("role_name", serde_json::to_value(&request.role_name)?);
        body.insert("role_type", serde_json::to_value(&request.role_type)?);

        if let Some(ref description) = request.description {
            body.insert("description", serde_json::to_value(description)?);
        }
        if let Some(ref permissions) = request.permissions {
            body.insert("permissions", serde_json::to_value(permissions)?);
        }

        // 构建API请求
        let api_req = ApiRequest::post(format!("/open-apis/bitable/v1/apps/{}/roles", request.app_token))
            .body(serde_json::to_string(&body)?)
            .query(HashMap::new());

        // 发送请求
        let resp = Transport::<CreateRoleResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "创建角色完成: app_token={}, role_name={}",
            request.app_token,
            request.role_name
        );

        Ok(response)
    }

    /// 更新角色
    ///
    /// 更新指定应用中的角色
    ///
    /// # 参数
    /// * `request` - 更新角色请求
    ///
    /// # 返回
    /// 返回更新后的角色信息
    pub async fn update_role(&self, request: &UpdateRoleRequest) -> SDKResult<UpdateRoleResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "更新角色: app_token={}, role_id={}",
            request.app_token,
            request.role_id
        );

        // 构建请求体
        let mut body = HashMap::new();

        if let Some(ref role_name) = request.role_name {
            body.insert("role_name", serde_json::to_value(role_name)?);
        }
        if let Some(ref description) = request.description {
            body.insert("description", serde_json::to_value(description)?);
        }
        if let Some(ref permissions) = request.permissions {
            body.insert("permissions", serde_json::to_value(permissions)?);
        }

        // 构建API请求
        let api_req = ApiRequest::put(format!(
                "/open-apis/bitable/v1/apps/{}/roles/{}",
                request.app_token, request.role_id
            ))
            .body(serde_json::to_string(&body)?)
            .query(HashMap::new());

        // 发送请求
        let resp = Transport::<UpdateRoleResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "更新角色完成: app_token={}, role_id={}",
            request.app_token,
            request.role_id
        );

        Ok(response)
    }

    /// 删除角色
    ///
    /// 删除指定应用中的角色
    ///
    /// # 参数
    /// * `request` - 删除角色请求
    ///
    /// # 返回
    /// 返回删除操作的结果
    pub async fn delete_role(&self, request: &DeleteRoleRequest) -> SDKResult<DeleteRoleResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "删除角色: app_token={}, role_id={}",
            request.app_token,
            request.role_id
        );

        // 构建API请求
        let api_req = ApiRequest::delete(format!(
                "/open-apis/bitable/v1/apps/{}/roles/{}",
                request.app_token, request.role_id
            ))
            .query(HashMap::new());

        // 发送请求
        let resp = Transport::<DeleteRoleResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "删除角色完成: app_token={}, role_id={}",
            request.app_token,
            request.role_id
        );

        Ok(response)
    }

    /// 列出角色
    ///
    /// 获取指定应用中的角色列表
    ///
    /// # 参数
    /// * `request` - 列出角色请求
    ///
    /// # 返回
    /// 返回角色列表
    pub async fn list_roles(&self, request: &ListRolesRequest) -> SDKResult<ListRolesResponse> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!("列出角色: app_token={}", request.app_token);

        // 构建查询参数
        let mut query_params = HashMap::new();

        if let Some(page_size) = request.page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(ref page_token) = request.page_token {
            query_params.insert("page_token", page_token.clone());
        }

        // 构建API请求
        let api_req = ApiRequest::get(format!("/open-apis/bitable/v1/apps/{}/roles", request.app_token))
            .query(query_params);

        // 发送请求
        let resp = Transport::<ListRolesResponse>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "列出角色完成: app_token={}, count={}",
            request.app_token,
            response.roles.as_ref().map(|r| r.len()).unwrap_or(0)
        );

        Ok(response)
    }
}

// 构建器模式实现
pub struct CreateRoleRequestBuilder {
    request: CreateRoleRequest,
}

impl CreateRoleRequestBuilder {
    pub fn new(
        app_token: impl Into<String>,
        role_name: impl Into<String>,
        role_type: impl Into<String>,
    ) -> Self {
        Self {
            request: CreateRoleRequest {
                app_token: app_token.into(),
                role_name: role_name.into(),
                role_type: role_type.into(),
                description: None,
                permissions: None,
            },
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    pub fn permissions(mut self, permissions: Vec<PermissionConfig>) -> Self {
        self.request.permissions = Some(permissions);
        self
    }

    pub async fn execute(self, service: &AppRoleService) -> SDKResult<CreateRoleResponse> {
        service.create_role(&self.request).await
    }
}

pub struct UpdateRoleRequestBuilder {
    request: UpdateRoleRequest,
}

impl UpdateRoleRequestBuilder {
    pub fn new(app_token: impl Into<String>, role_id: impl Into<String>) -> Self {
        Self {
            request: UpdateRoleRequest {
                app_token: app_token.into(),
                role_id: role_id.into(),
                role_name: None,
                description: None,
                permissions: None,
            },
        }
    }

    pub fn role_name(mut self, role_name: impl Into<String>) -> Self {
        self.request.role_name = Some(role_name.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    pub fn permissions(mut self, permissions: Vec<PermissionConfig>) -> Self {
        self.request.permissions = Some(permissions);
        self
    }

    pub async fn execute(self, service: &AppRoleService) -> SDKResult<UpdateRoleResponse> {
        service.update_role(&self.request).await
    }
}
