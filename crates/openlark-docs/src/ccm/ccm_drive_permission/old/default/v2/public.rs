//! 根据 filetoken 获取文档的公共设置。
//!
//! doc: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/permission/get-document-sharing-settings-v2

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetPublicPermissionRequest {
    pub token: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetPublicPermissionResponse {
    pub security_entity: Option<String>,
    pub comment_entity: Option<String>,
    pub share_entity: Option<String>,
    pub link_share_entity: Option<String>,
    pub external_access: Option<bool>,
    pub invite_external: Option<bool>,
}

impl ApiResponseTrait for GetPublicPermissionResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct GetPublicPermissionBuilder {
    api_req: ApiRequest<GetPublicPermissionRequest>,
}

impl GetPublicPermissionBuilder {
    pub fn new() -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "ccm_drive_permission_public_get".to_string();
        builder.api_req.method = "POST".to_string();
        builder.api_req.url = "https://open.feishu.cn/open-apis/drive/permission/v2/public/".to_string();
        builder.api_req.body = Some(GetPublicPermissionRequest::default());
        builder
    }

    pub fn token(mut self, token: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.token = token.to_string();
        }
        self
    }

    pub fn type_(mut self, type_: impl ToString) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.type_ = type_.to_string();
        }
        self
    }

    pub fn build(
        self,
        config: &openlark_core::config::Config,
        option: &RequestOption,
    ) -> Result<RequestBuilder, LarkAPIError> {
        let mut req = self.api_req;
        req.build(AccessTokenType::Tenant, config, option)
    }
}
