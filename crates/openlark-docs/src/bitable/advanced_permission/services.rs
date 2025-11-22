//! Bitable Advanced Permission API 服务实现
//!
//! 提供多维表格高级权限管理相关的API服务，包括：
//! - V2版本的角色管理
//! - 高级权限控制
//! - 细粒度权限设置
//! - 完整的错误处理和参数验证
use std::collections::HashMap;

use openlark_core::{
    api::ApiRequest, config::Config, constants::AccessTokenType, error::LarkAPIError,
    http::Transport, SDKResult,
};

use super::models::*;

/// 多维表格高级权限管理服务
#[derive(Debug, Clone)]
pub struct AdvancedPermissionService {
    config: Config,
}

impl AdvancedPermissionService {
    /// 创建新的高级权限管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 列出V2角色
    ///
    /// 获取指定应用中的V2角色列表
    ///
    /// # 参数
    /// * `request` - 列出V2角色请求
    ///
    /// # 返回
    /// 返回V2角色列表
    pub async fn list_roles_v2(
        &self,
        request: &ListRolesV2Request,
    ) -> SDKResult<ListRolesV2Response> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!("列出V2角色: app_token={}", request.app_token);

        // 构建查询参数
        let mut query_params = HashMap::new();

        if let Some(page_size) = request.page_size {
            query_params.insert("page_size", page_size.to_string());
        }
        if let Some(ref page_token) = request.page_token {
            query_params.insert("page_token", page_token.clone());
        }
        if let Some(ref role_type) = request.role_type {
            query_params.insert("role_type", serde_json::to_value(role_type)?.to_string());
        }

        // 构建API请求
        let api_req = ApiRequest::get(format!("/open-apis/bitable/v1/apps/{}/roles_v2", request.app_token))
            .query(HashMap::new());

        // 发送请求
        let resp = Transport::<ListRolesV2Response>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "列出V2角色完成: app_token={}, count={}",
            request.app_token,
            response.roles.as_ref().map(|r| r.len()).unwrap_or(0)
        );

        Ok(response)
    }

    /// 创建V2角色
    ///
    /// 创建新的V2角色并配置权限
    ///
    /// # 参数
    /// * `request` - 创建V2角色请求
    ///
    /// # 返回
    /// 返回创建的V2角色信息
    pub async fn create_role_v2(
        &self,
        request: &CreateRoleV2Request,
    ) -> SDKResult<CreateRoleV2Response> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "创建V2角色: app_token={}, name={}, role_type={:?}",
            request.app_token,
            request.name,
            request.role_type
        );

        // 构建请求体
        let mut body = HashMap::new();

        body.insert("name".to_string(), serde_json::to_value(&request.name)?);
        body.insert(
            "role_type".to_string(),
            serde_json::to_value(&request.role_type)?,
        );
        body.insert(
            "permissions".to_string(),
            serde_json::to_value(&request.permissions)?,
        );

        if let Some(ref description) = request.description {
            body.insert(
                "description".to_string(),
                serde_json::to_value(description)?,
            );
        }
        if let Some(is_system) = request.is_system {
            body.insert("is_system".to_string(), serde_json::to_value(is_system)?);
        }

        // 构建API请求
        let api_req = ApiRequest::post(format!("/open-apis/bitable/v1/apps/{}/roles_v2", request.app_token))
            .body(serde_json::to_string(&body)?)
            .query(HashMap::new());

        // 发送请求
        let resp = Transport::<CreateRoleV2Response>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "创建V2角色完成: app_token={}, role_id={:?}",
            request.app_token,
            response.role.as_ref().and_then(|r| r.role_id.clone())
        );

        Ok(response)
    }

    /// 更新V2角色
    ///
    /// 更新指定V2角色的配置信息
    ///
    /// # 参数
    /// * `request` - 更新V2角色请求
    ///
    /// # 返回
    /// 返回更新后的V2角色信息
    pub async fn update_role_v2(
        &self,
        request: &UpdateRoleV2Request,
    ) -> SDKResult<UpdateRoleV2Response> {
        // 验证请求参数
        request
            .validate()
            .map_err(|e| LarkAPIError::illegal_param(format!("请求参数验证失败: {}", e)))?;

        log::info!(
            "更新V2角色: app_token={}, role_id={}",
            request.app_token,
            request.role_id
        );

        // 构建请求体
        let mut body = HashMap::new();

        if let Some(ref name) = request.name {
            body.insert("name".to_string(), serde_json::to_value(name)?);
        }
        if let Some(ref description) = request.description {
            body.insert(
                "description".to_string(),
                serde_json::to_value(description)?,
            );
        }
        if let Some(ref permissions) = request.permissions {
            body.insert(
                "permissions".to_string(),
                serde_json::to_value(permissions)?,
            );
        }
        if let Some(ref status) = request.status {
            body.insert("status".to_string(), serde_json::to_value(status)?);
        }

        // 构建API请求
        let api_req = ApiRequest::put(format!(
                "/open-apis/bitable/v1/apps/{}/roles_v2/{}",
                request.app_token, request.role_id
            ))
            .body(serde_json::to_string(&body)?)
            .query(HashMap::new());

        // 发送请求
        let resp = Transport::<UpdateRoleV2Response>::request(api_req, &self.config, None).await?;
        let response = resp.data.unwrap_or_default();

        log::info!(
            "更新V2角色完成: app_token={}, role_id={}",
            request.app_token,
            request.role_id
        );

        Ok(response)
    }
}

// 构建器模式实现
pub struct ListRolesV2RequestBuilder {
    request: ListRolesV2Request,
}

impl ListRolesV2RequestBuilder {
    pub fn new(app_token: impl Into<String>) -> Self {
        Self {
            request: ListRolesV2Request {
                app_token: app_token.into(),
                page_size: None,
                page_token: None,
                role_type: None,
            },
        }
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request.page_token = Some(page_token.into());
        self
    }

    pub fn role_type(mut self, role_type: RoleTypeV2) -> Self {
        self.request.role_type = Some(role_type);
        self
    }

    pub async fn execute(
        self,
        service: &AdvancedPermissionService,
    ) -> SDKResult<ListRolesV2Response> {
        service.list_roles_v2(&self.request).await
    }
}

pub struct CreateRoleV2RequestBuilder {
    request: CreateRoleV2Request,
}

impl CreateRoleV2RequestBuilder {
    pub fn new(
        app_token: impl Into<String>,
        name: impl Into<String>,
        role_type: RoleTypeV2,
    ) -> Self {
        Self {
            request: CreateRoleV2Request {
                app_token: app_token.into(),
                name: name.into(),
                role_type,
                description: None,
                permissions: Vec::new(),
                is_system: None,
            },
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    pub fn permission(mut self, permission: PermissionConfig) -> Self {
        self.request.permissions.push(permission);
        self
    }

    pub fn permissions(mut self, permissions: Vec<PermissionConfig>) -> Self {
        self.request.permissions = permissions;
        self
    }

    pub fn is_system(mut self, is_system: bool) -> Self {
        self.request.is_system = Some(is_system);
        self
    }

    pub async fn execute(
        self,
        service: &AdvancedPermissionService,
    ) -> SDKResult<CreateRoleV2Response> {
        service.create_role_v2(&self.request).await
    }
}

pub struct UpdateRoleV2RequestBuilder {
    request: UpdateRoleV2Request,
}

impl UpdateRoleV2RequestBuilder {
    pub fn new(app_token: impl Into<String>, role_id: impl Into<String>) -> Self {
        Self {
            request: UpdateRoleV2Request {
                app_token: app_token.into(),
                role_id: role_id.into(),
                name: None,
                description: None,
                permissions: None,
                status: None,
            },
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request.name = Some(name.into());
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

    pub fn status(mut self, status: RoleStatusV2) -> Self {
        self.request.status = Some(status);
        self
    }

    pub async fn execute(
        self,
        service: &AdvancedPermissionService,
    ) -> SDKResult<UpdateRoleV2Response> {
        service.update_role_v2(&self.request).await
    }
}
