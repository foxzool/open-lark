//! Base V2 更新自定义角色API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    prelude::validation_error,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::models::UpdateRoleRequest;
use super::RoleService;

// 类型别名，用于兼容
use super::models::RoleResponse as Role;

/// 更新自定义角色请求
pub struct UpdateRoleV2Request {
    api_request: ApiRequest<UpdateRoleV2Response>,
    app_token: String,
    role_id: String,
    /// 角色名称
    name: Option<String>,
    /// 角色描述
    description: Option<String>,
    /// 角色权限列表
    permissions: Option<Vec<String>>,
    /// 角色类型
    role_type: Option<String>,
    /// 配置信息
    config: Config,
}

/// 更新自定义角色响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateRoleV2Response {
    pub data: Role,
    pub success: bool,
}

impl ApiResponseTrait for UpdateRoleV2Response {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl UpdateRoleV2Request {
    /// 创建更新自定义角色请求
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::put("/open-apis/base/v2/apps/:app_token/roles/:role_id"),
            app_token: String::new(),
            role_id: String::new(),
            name: None,
            description: None,
            permissions: None,
            role_type: None,
            config,
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

    /// 设置角色名称
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// 设置角色描述
    pub fn description(mut self, description: String) -> Self {
        self.description = Some(description);
        self
    }

    /// 设置角色权限列表
    pub fn permissions(mut self, permissions: Vec<String>) -> Self {
        self.permissions = Some(permissions);
        self
    }

    /// 设置角色类型
    pub fn role_type(mut self, role_type: String) -> Self {
        self.role_type = Some(role_type);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateRoleV2Response> {
        // 构建API路径
        let path = format!(
            "/open-apis/base/v2/apps/{}/roles/{}",
            self.app_token, self.role_id
        );

        // 构建请求体 - 修正字段名称映射
        let request_body = UpdateRoleRequest {
            role_name: self.name.clone(),
            table_roles: None, // 根据实际API需求设置
        };

        // 验证请求参数
        if let Err(e) = request_body.validate() {
            return Err(validation_error("更新角色请求验证失败", e.to_string()));
        }

        // 创建新的API请求
        let api_request: ApiRequest<UpdateRoleV2Response> =
            ApiRequest::put(&format!("https://open.feishu.cn{}", path)).body(
                openlark_core::api::RequestData::Binary(serde_json::to_vec(&request_body)?),
            );

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

/// 更新自定义角色Builder
pub struct UpdateRoleV2Builder {
    request: UpdateRoleV2Request,
}

impl UpdateRoleV2Builder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: UpdateRoleV2Request::new(config),
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

    /// 设置角色名称
    pub fn name(mut self, name: String) -> Self {
        self.request = self.request.name(name);
        self
    }

    /// 设置角色描述
    pub fn description(mut self, description: String) -> Self {
        self.request = self.request.description(description);
        self
    }

    /// 设置角色权限列表
    pub fn permissions(mut self, permissions: Vec<String>) -> Self {
        self.request = self.request.permissions(permissions);
        self
    }

    /// 设置角色类型
    pub fn role_type(mut self, role_type: String) -> Self {
        self.request = self.request.role_type(role_type);
        self
    }

    /// 构建请求
    pub fn build(self) -> UpdateRoleV2Request {
        self.request
    }
}

impl RoleService {
    /// 创建更新自定义角色请求构建器
    pub fn update_role_v2_builder(&self) -> UpdateRoleV2Builder {
        UpdateRoleV2Builder::new(self.config.clone())
    }

    /// 创建更新自定义角色请求
    pub fn update_role_v2(
        &self,
        app_token: String,
        role_id: String,
        name: Option<String>,
        description: Option<String>,
        permissions: Option<Vec<String>>,
        role_type: Option<String>,
    ) -> UpdateRoleV2Request {
        let mut request = UpdateRoleV2Request::new(self.config.clone())
            .app_token(app_token)
            .role_id(role_id);

        if let Some(n) = name {
            request = request.name(n);
        }

        if let Some(desc) = description {
            request = request.description(desc);
        }

        if let Some(perms) = permissions {
            request = request.permissions(perms);
        }

        if let Some(rtype) = role_type {
            request = request.role_type(rtype);
        }

        request
    }
}
