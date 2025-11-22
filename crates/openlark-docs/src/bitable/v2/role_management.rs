//! Bitable V2 角色管理模块
//!
//! 提供多维表格 V2 版本的角色管理功能，包括：
//! - 创建自定义角色
//! - 查询角色列表
//! - 更新角色信息
//!
//! V2 角色管理 API 提供了更强大的权限控制功能和更灵活的角色配置选项。

use openlark_core::{
    api::{ApiRequest, HttpMethod},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 角色信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoleV2 {
    /// 角色 ID
    pub role_id: String,
    /// 角色名称
    pub name: String,
    /// 角色描述
    pub description: Option<String>,
    /// 角色权限
    pub permissions: Vec<String>,
    /// 创建时间
    pub create_time: Option<i64>,
    /// 更新时间
    pub update_time: Option<i64>,
}

/// 创建角色 V2 请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoleV2Request {
    /// 角色名称
    pub name: String,
    /// 角色描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 角色权限列表
    pub permissions: Vec<String>,
}

impl CreateRoleV2Request {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("角色名称不能为空".to_string());
        }
        if self.name.len() > 100 {
            return Err("角色名称不能超过100个字符".to_string());
        }
        if self.permissions.is_empty() {
            return Err("角色权限不能为空".to_string());
        }
        Ok(())
    }
}

/// 更新角色 V2 请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRoleV2Request {
    /// 角色名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 角色描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 角色权限列表
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub permissions: Vec<String>,
}

impl UpdateRoleV2Request {
    /// 验证请求参数
    pub fn validate(&self) -> Result<(), String> {
        if let Some(ref name) = self.name {
            if name.trim().is_empty() {
                return Err("角色名称不能为空".to_string());
            }
            if name.len() > 100 {
                return Err("角色名称不能超过100个字符".to_string());
            }
        }
        Ok(())
    }
}

/// 创建角色 V2 响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoleV2Response {
    /// 角色 ID
    pub role_id: String,
    /// 角色信息
    pub role: Option<RoleV2>,
}

/// 列出角色 V2 响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRolesV2Response {
    /// 角色列表
    pub roles: Vec<RoleV2>,
    /// 分页 token
    pub page_token: Option<String>,
    /// 是否还有更多数据
    pub has_more: Option<bool>,
}

/// 更新角色 V2 响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateRoleV2Response {
    /// 角色信息
    pub role: Option<RoleV2>,
}

/// Bitable V2 角色管理服务
#[derive(Debug, Clone)]
pub struct RoleManagementV2Service {
    config: Config,
}

impl RoleManagementV2Service {
    /// 创建角色管理 V2 服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建自定义角色 V2
    ///
    /// # 参数
    /// * `app_token` - 多维表格的 app_token
    /// * `request` - 创建角色请求
    ///
    /// # 返回
    /// 返回新创建的角色信息
    ///
    /// # 示例
    /// ```rust
    /// let request = CreateRoleV2Request {
    ///     name: "编辑者".to_string(),
    ///     description: Some("可以编辑和查看数据的角色".to_string()),
    ///     permissions: vec!["table:read".to_string(), "table:write".to_string()],
    /// };
    ///
    /// let response = service.create_role_v2("app_token_xxx", &request).await?;
    /// ```
    pub async fn create_role_v2(
        &self,
        app_token: &str,
        request: &CreateRoleV2Request,
    ) -> SDKResult<CreateRoleV2Response> {
        // 验证请求参数
        request.validate()?;

        let api_request = ApiRequest::post("")
            .method(reqwest::Method::POST)
            .path(&format!("/open-apis/base/v2/apps/{}/roles", app_token))
            .body(serde_json::to_value(request)?);

        let transport = Transport::new();
        Transport::request(api_request, &self.config, None).await
    }

    /// 列出自定义角色 V2
    ///
    /// # 参数
    /// * `app_token` - 多维表格的 app_token
    /// * `page_size` - 页面大小（可选）
    /// * `page_token` - 分页 token（可选）
    ///
    /// # 返回
    /// 返回角色列表
    ///
    /// # 示例
    /// ```rust
    /// let response = service.list_roles_v2("app_token_xxx", Some(20), None).await?;
    /// for role in response.roles {
    ///     println!("角色: {}", role.name);
    /// }
    /// ```
    pub async fn list_roles_v2(
        &self,
        app_token: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
    ) -> SDKResult<ListRolesV2Response> {
        let mut query_params = Vec::new();

        if let Some(size) = page_size {
            query_params.push(("page_size".to_string(), size.to_string()));
        }

        if let Some(token) = page_token {
            query_params.push(("page_token".to_string(), token.to_string()));
        }

        let mut api_request = ApiRequest::post("")
            .method(reqwest::Method::GET)
            .path(&format!("/open-apis/base/v2/apps/{}/roles", app_token));

        for (key, value) in query_params {
            api_request = api_request.query_param(&key, &value);
        }

        let transport = Transport::new();
        Transport::request(api_request, &self.config, None).await
    }

    /// 更新自定义角色 V2
    ///
    /// # 参数
    /// * `app_token` - 多维表格的 app_token
    /// * `role_id` - 角色 ID
    /// * `request` - 更新角色请求
    ///
    /// # 返回
    /// 返回更新后的角色信息
    ///
    /// # 示例
    /// ```rust
    /// let request = UpdateRoleV2Request {
    ///     name: Some("高级编辑者".to_string()),
    ///     description: Some("具有高级编辑权限的角色".to_string()),
    ///     permissions: vec!["table:read".to_string(), "table:write".to_string(), "table:delete".to_string()],
    /// };
    ///
    /// let response = service.update_role_v2("app_token_xxx", "role_id_xxx", &request).await?;
    /// ```
    pub async fn update_role_v2(
        &self,
        app_token: &str,
        role_id: &str,
        request: &UpdateRoleV2Request,
    ) -> SDKResult<UpdateRoleV2Response> {
        // 验证请求参数
        request.validate()?;

        let api_request = ApiRequest::post("")
            .method(reqwest::Method::PUT)
            .path(&format!(
                "/open-apis/base/v2/apps/{}/roles/{}",
                app_token, role_id
            ))
            .body(serde_json::to_value(request)?);

        let transport = Transport::new();
        Transport::request(api_request, &self.config, None).await
    }
}
