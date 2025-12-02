
//! Bitable V1 删除角色API

use openlark_core::{
    api::{ApiRequest, RequestData},
    config::Config,
    error::{SDKResult, validation_error},
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 删除角色请求
#[derive(Debug, Clone)]
pub struct DeleteAppRoleRequest {
    /// 配置信息
    config: Config,
    api_request: ApiRequest<DeleteAppRoleResponse>,
    /// 多维表格的 app_token
    app_token: String,
    /// 角色的ID
    role_id: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
}

impl DeleteAppRoleRequest {
    /// 创建删除角色请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            api_request: ApiRequest::delete(""),
            app_token: String::new(),
            role_id: String::new(),
            user_id_type: None,
        }
    }

    /// 设置应用 token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 设置角色 ID
    pub fn role_id(mut self, role_id: String) -> Self {
        self.role_id = role_id;
        self
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<DeleteAppRoleResponse> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "应用token不能为空"));
        }

        if self.role_id.trim().is_empty() {
            return Err(validation_error("role_id", "角色ID不能为空"));
        }

        // 构建完整的API URL
        let api_url = format!("{}/open-apis/bitable/v1/apps/{}/roles/{}",
                             self.config.base_url, self.app_token, self.role_id);

        // 设置API URL和查询参数
        let mut api_request = self.api_request;
        api_request = api_request.api_path(api_url);

        // 设置查询参数
        if let Some(user_id_type) = &self.user_id_type {
            api_request = api_request.query("user_id_type", user_id_type);
        }

        // 发送请求
        let response: DeleteAppRoleResponse =
            Transport::request(api_request, &self.config, None).await?;

        Ok(response)
    }
}

/// 删除角色Builder
pub struct DeleteAppRoleRequestBuilder {
    request: DeleteAppRoleRequest,
}

impl DeleteAppRoleRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: DeleteAppRoleRequest::new(config),
        }
    }

    /// 设置应用 token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    /// 设置角色 ID
    pub fn role_id(mut self, role_id: String) -> Self {
        self.request = self.request.role_id(role_id);
        self
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.request = self.request.user_id_type(user_id_type);
        self
    }

    /// 构建请求
    pub fn build(self) -> DeleteAppRoleRequest {
        self.request
    }
}

/// 删除角色响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteAppRoleResponse {
    /// 删除的角色ID
    pub role_id: Option<String>,
    /// 是否删除成功
    pub success: bool,
}

