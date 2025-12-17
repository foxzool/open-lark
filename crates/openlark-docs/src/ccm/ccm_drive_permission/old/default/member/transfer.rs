//! 转移拥有者
//!
//! docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/permission/transfer-ownership

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct TransferReq {
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
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 转移拥有者请求
pub struct TransferRequest {
    config: Config,
    req: TransferReq,
}

impl TransferRequest {
    pub fn new(
        config: Config,
        token: impl Into<String>,
        type_: impl Into<String>,
        owner_id: impl Into<String>,
        owner_type: impl Into<String>,
    ) -> Self {
        Self {
            config,
            req: TransferReq {
                token: token.into(),
                type_: type_.into(),
                owner_id: owner_id.into(),
                owner_type: owner_type.into(),
                remove_old_owner: None,
                cancel_notify: None,
            },
        }
    }

    pub fn remove_old_owner(mut self, remove: bool) -> Self {
        self.req.remove_old_owner = Some(remove);
        self
    }

    pub fn cancel_notify(mut self, cancel: bool) -> Self {
        self.req.cancel_notify = Some(cancel);
        self
    }

    pub async fn send(self) -> SDKResult<TransferResponse> {
        use crate::common::api_endpoints::CcmDrivePermissionApiOld;

        let api_request: ApiRequest<TransferResponse> =
            ApiRequest::post(&CcmDrivePermissionApiOld::MemberTransfer.to_url())
                .body(serde_json::to_value(&self.req)?);
        let response: Response<TransferResponse> =
            Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}
