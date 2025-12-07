//! Base 创建自定义角色API
///
/// API文档: https://open.feishu.cn/document/docs/bitable-v1/advanced-permission/app-role/create-2

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::{
    models::RoleResponse as Role,
    RoleService,
};

/// 新增自定义角色请求
pub struct CreateRoleRequest {
    app_token: String,
    /// 角色名称
    name: String,
    /// 角色描述
    description: Option<String>,
    /// 配置信息
    config: Config,
}

/// 新增自定义角色响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateRoleResponse {
    /// 角色信息
    pub data: Role,
}

impl ApiResponseTrait for CreateRoleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateRoleRequest {
    /// 创建新增自定义角色请求
    pub fn new(config: Config) -> Self {
        Self {
            app_token: String::new(),
            name: String::new(),
            description: None,
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

    /// 执行请求（集成现代化enum+builder API端点系统）
    pub async fn execute(self) -> SDKResult<CreateRoleResponse> {
        // 验证必填字段
        validate_required!(self.app_token, "应用令牌不能为空");
        validate_required!(self.name, "角色名称不能为空");

        // 🚀 使用新的enum+builder系统生成API端点
        // 替代传统的字符串拼接方式，提供类型安全和IDE自动补全
        use crate::common::api_endpoints::BaseApiV2;
        let api_endpoint = BaseApiV2::RoleCreate(self.app_token.clone());

        // 构建请求体 - 符合官方文档格式
        let request_body = serde_json::json!({
            "name": self.name,
            "description": self.description
        });

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<CreateRoleResponse> =
            ApiRequest::post(&api_endpoint.to_url()).body(
                openlark_core::api::RequestData::Json(request_body),
            );

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

impl RoleService {
    /// 创建新增自定义角色请求
    pub fn create_role_builder(
        &self,
        app_token: impl Into<String>,
        name: impl Into<String>,
    ) -> CreateRoleRequest {
        CreateRoleRequest::new(self.config.clone())
            .app_token(app_token)
            .name(name)
    }

    /// 创建新增自定义角色请求（带描述）
    pub fn create_role(
        &self,
        app_token: impl Into<String>,
        name: impl Into<String>,
        description: Option<impl Into<String>>,
    ) -> CreateRoleRequest {
        let mut request = CreateRoleRequest::new(self.config.clone())
            .app_token(app_token)
            .name(name);

        if let Some(desc) = description {
            request = request.description(desc);
        }

        request
    }
}
