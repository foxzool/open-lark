//! 列出自定义角色的协作者
//!
//! doc: https://open.feishu.cn/document/server-docs/docs/bitable-v1/advanced-permission/app-role-member/list

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListRoleMemberRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ListRoleMemberResponse {
    pub items: Vec<AppRoleMember>,
    pub page_token: String,
    pub has_more: bool,
    pub total: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AppRoleMember {
    #[serde(rename = "type")]
    pub type_: String,
    pub member_id: String,
    pub name: String,
    pub en_name: String,
    pub avatar_url: String,
}

impl ApiResponseTrait for ListRoleMemberResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug)]
pub struct ListRoleMember {
    config: openlark_core::config::Config,
    app_token: String,
    role_id: String,
    req: ListRoleMemberRequest,
}

impl ListRoleMember {
    pub fn new(config: openlark_core::config::Config, app_token: impl Into<String>, role_id: impl Into<String>) -> Self {
        Self {
            config,
            app_token: app_token.into(),
            role_id: role_id.into(),
            req: ListRoleMemberRequest::default(),
        }
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.req.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.req.page_token = Some(page_token.into());
        self
    }

    pub async fn send(self) -> Result<openlark_core::response::Response<ListRoleMemberResponse>, openlark_core::error::Error> {
        let url = format!(
            "{}/open-apis/bitable/v1/apps/{}/roles/{}/members",
            self.config.base_url, self.app_token, self.role_id
        );
        let request = ApiRequest::get(&url).query(&self.req);
        let response = RequestBuilder::new(self.config, request).send().await?;
        Ok(response)
    }
}
