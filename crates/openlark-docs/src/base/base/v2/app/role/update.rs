//! 更新自定义角色
//!
//! docPath: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/bitable-v1/advanced-permission/base-v2/app-role/update
//! doc: https://open.feishu.cn/document/docs/bitable-v1/advanced-permission/app-role/update-2

use crate::base::base::v2::models::AppRole;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 更新自定义角色
#[derive(Debug)]
pub struct Update {
    config: Config,
    app_token: String,
    role_id: String,
    req: UpdateReq,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateReq {
    /// 自定义角色的名字
    pub role_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateResp {
    /// 自定义角色
    pub role: AppRole,
}

impl Update {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            role_id: String::new(),
            req: UpdateReq {
                role_name: String::new(),
            },
        }
    }

    /// 应用 token
    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    /// 角色ID
    pub fn role_id(mut self, role_id: impl Into<String>) -> Self {
        self.role_id = role_id.into();
        self
    }

    /// 自定义角色的名字
    pub fn role_name(mut self, role_name: impl Into<String>) -> Self {
        self.req.role_name = role_name.into();
        self
    }

    pub async fn send(self) -> SDKResult<UpdateResp> {
        validate_required!(self.app_token, "app_token 不能为空");
        validate_required!(self.role_id, "role_id 不能为空");
        validate_required!(self.req.role_name, "role_name 不能为空");

        use crate::common::api_endpoints::BaseApiV2;
        let api_endpoint = BaseApiV2::RoleUpdate(self.app_token, self.role_id);

        let api_request: ApiRequest<UpdateResp> =
            ApiRequest::put(&api_endpoint.to_url()).body(serde_json::to_vec(&self.req)?);

        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("response", "响应数据为空")
        })
    }
}

impl ApiResponseTrait for UpdateResp {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
