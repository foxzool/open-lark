//! 转移拥有者
//!
//! doc: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/permission/transfer-ownership

use openlark_core::api::{ApiRequest, ApiResponseTrait, LarkAPIError, RequestBuilder};
use openlark_core::constants::AccessTokenType;
use openlark_core::req_option::RequestOption;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TransferRequest {
    pub token: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub owner_id: String,
    pub owner_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remove_old_owner: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_notify: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TransferResponse {
    pub is_success: bool,
    pub token: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub owner_id: String,
    pub owner_type: String,
}

impl ApiResponseTrait for TransferResponse {
    fn data_format() -> openlark_core::api::ResponseFormat {
        openlark_core::api::ResponseFormat::Data
    }
}

#[derive(Debug, Default)]
pub struct TransferBuilder {
    api_req: ApiRequest<TransferRequest>,
}

impl TransferBuilder {
    pub fn new(token: impl ToString, type_: impl ToString, owner_id: impl ToString, owner_type: impl ToString) -> Self {
        let mut builder = Self::default();
        builder.api_req.req_type = "drive_permission_transfer".to_string();
        builder.api_req.method = "POST".to_string();
        builder.api_req.url = "https://open.feishu.cn/open-apis/drive/permission/member/transfer".to_string();
        builder.api_req.body = Some(TransferRequest {
            token: token.to_string(),
            type_: type_.to_string(),
            owner_id: owner_id.to_string(),
            owner_type: owner_type.to_string(),
            remove_old_owner: None,
            cancel_notify: None,
        });
        builder
    }

    pub fn remove_old_owner(mut self, remove: bool) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.remove_old_owner = Some(remove);
        }
        self
    }

    pub fn cancel_notify(mut self, cancel: bool) -> Self {
        if let Some(body) = &mut self.api_req.body {
            body.cancel_notify = Some(cancel);
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
