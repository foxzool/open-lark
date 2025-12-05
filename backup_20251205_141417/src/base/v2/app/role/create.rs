//! Base V2 创建自定义角色API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::{
    models::{CreateRoleRequest, RoleResponse as Role},
    RoleService,
};

/// 新增自定义角色请求
pub struct CreateRoleV2Request {
    app_token: String,
    /// 角色名称
    name: String,
    /// 角色描述
    description: Option<String>,
    /// 角色权限列表
    permissions: Vec<String>,
    /// 角色类型
    role_type: Option<String>,
    /// 配置信息
    config: Config,
}

/// 新增自定义角色响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateRoleV2Response {
    /// 角色信息
    pub data: Role,
}

impl ApiResponseTrait for CreateRoleV2Response {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateRoleV2Request {
    /// 创建新增自定义角色请求
    pub fn new(config: Config) -> Self {
        Self {
            app_token: String::new(),
            name: String::new(),
            description: None,
            permissions: Vec::new(),
            role_type: None,
            config,
        }
    }

    /// 设置应用 token
    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    /// 设置角色名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// 设置角色描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// 设置角色权限列表
    pub fn permissions(mut self, permissions: Vec<impl Into<String>>) -> Self {
        self.permissions = permissions.into_iter().map(|p| p.into()).collect();
        self
    }

    /// 设置角色类型
    pub fn role_type(mut self, role_type: impl Into<String>) -> Self {
        self.role_type = Some(role_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateRoleV2Response> {
        // 验证必填字段
        validate_required!(self.app_token, "应用令牌不能为空");
        validate_required!(self.name, "角色名称不能为空");

        // 构建API路径
        let path = format!("/open-apis/base/v2/apps/{}/roles", self.app_token);

        // 构建请求体 - 使用实际的字段结构
        let request_body = CreateRoleRequest {
            role_name: self.name.clone(),
            table_roles: None, // 基础角色创建不需要表格权限
        };

        // 创建API请求
        let api_request: ApiRequest<CreateRoleV2Response> =
            ApiRequest::post(&path).body(
                openlark_core::api::RequestData::Binary(serde_json::to_vec(&request_body)?),
            );

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

impl RoleService {
    /// 创建新增自定义角色请求
    pub fn create_role_v2_builder(
        &self,
        app_token: impl Into<String>,
        name: impl Into<String>,
    ) -> CreateRoleV2Request {
        CreateRoleV2Request::new(self.config.clone())
            .app_token(app_token)
            .name(name)
    }

    /// 创建新增自定义角色请求（带完整参数）
    pub fn create_role_v2(
        &self,
        app_token: impl Into<String>,
        name: impl Into<String>,
        description: Option<impl Into<String>>,
        permissions: Vec<impl Into<String>>,
        role_type: Option<impl Into<String>>,
    ) -> CreateRoleV2Request {
        let mut request = CreateRoleV2Request::new(self.config.clone())
            .app_token(app_token)
            .name(name)
            .permissions(permissions);

        if let Some(desc) = description {
            request = request.description(desc);
        }

        if let Some(rtype) = role_type {
            request = request.role_type(rtype);
        }

        request
    }
}
