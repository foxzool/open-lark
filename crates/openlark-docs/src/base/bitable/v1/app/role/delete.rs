//! 删除自定义角色
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/advanced-permission/app-role/delete

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteRoleRequest {}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteRoleResponse {}

impl ApiResponseTrait for DeleteRoleResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug)]
pub struct DeleteRole {
    config: openlark_core::config::Config,
    app_token: String,
    role_id: String,
}

impl DeleteRole {
    pub fn new(config: openlark_core::config::Config, app_token: impl Into<String>, role_id: impl Into<String>) -> Self {
        Self {
            config,
            app_token: app_token.into(),
            role_id: role_id.into(),
        }
    }

    pub async fn send(self) -> Result<openlark_core::response::Response<DeleteRoleResponse>, openlark_core::error::Error> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/roles/{}",
            self.config.base_url, self.app_token, self.role_id
        );
        let request = ApiRequest::delete(&url);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}
