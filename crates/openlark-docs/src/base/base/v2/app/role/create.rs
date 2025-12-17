//! 新增自定义角色
//!
//! docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/advanced-permission/base-v2/app-role/create
//! doc: https://open.feishu.cn/document/docs/bitable-v1/advanced-permission/app-role/create-2

use crate::base::base::v2::models::AppRole;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 新增自定义角色
#[derive(Debug)]
pub struct Create {
    config: Config,
    app_token: String,
    req: CreateReq,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateReq {
    /// 自定义角色的名字
    pub role_name: String,
    /// 角色ID，以 "custom_" 开头
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateResp {
    /// 自定义角色
    pub role: AppRole,
}

impl Create {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            req: CreateReq {
                role_name: String::new(),
                role_id: None,
            },
        }
    }

    /// 应用 token
    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    /// 自定义角色的名字
    pub fn role_name(mut self, role_name: impl Into<String>) -> Self {
        self.req.role_name = role_name.into();
        self
    }

    /// 角色ID
    pub fn role_id(mut self, role_id: impl Into<String>) -> Self {
        self.req.role_id = Some(role_id.into());
        self
    }

    pub async fn send(self) -> SDKResult<CreateResp> {
        validate_required!(self.app_token, "app_token 不能为空");
        validate_required!(self.req.role_name, "role_name 不能为空");

        // 使用类型安全的端点枚举生成路径
        use crate::common::api_endpoints::BaseApiV2;
        let api_endpoint = BaseApiV2::RoleCreate(self.app_token);

        let api_request: ApiRequest<CreateResp> =
            ApiRequest::post(&api_endpoint.to_url()).body(serde_json::to_vec(&self.req)?);

        let response = Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

impl ApiResponseTrait for CreateResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
