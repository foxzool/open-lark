//! 判断协作者是否有某权限
//!
//! doc: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/permission/querying-if-a-collaborator-has-a-specific-permission

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PermittedRequest {
    pub token: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub perm: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PermittedResponse {
    pub is_permitted: bool,
}

impl ApiResponseTrait for PermittedResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct PermittedBuilder {
    api_req: ApiRequest<PermittedRequest>,
}

impl PermittedBuilder {
    pub fn new(token: impl ToString, type_: impl ToString, perm: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "drive_permission_permitted".to_string();
        builder.api_req.method = "POST".to_string();
        builder.api_req.url = "https://open.feishu.cn/open-apis/drive/permission/member/permitted".to_string();
        builder.api_req.body = Some(PermittedRequest {
            token: token.to_string(),
            type_: type_.to_string(),
            perm: perm.to_string(),
        });
        builder
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
