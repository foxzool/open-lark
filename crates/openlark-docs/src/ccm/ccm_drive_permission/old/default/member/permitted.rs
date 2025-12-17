//! 判断协作者是否有某权限
//!
//! docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/permission/querying-if-a-collaborator-has-a-specific-permission

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct PermittedReq {
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
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 判断协作者是否有某权限请求
pub struct PermittedRequest {
    config: Config,
    req: PermittedReq,
}

impl PermittedRequest {
    pub fn new(config: Config, token: impl Into<String>, type_: impl Into<String>, perm: impl Into<String>) -> Self {
        Self {
            config,
            req: PermittedReq {
                token: token.into(),
                type_: type_.into(),
                perm: perm.into(),
            },
        }
    }

    pub async fn send(self) -> SDKResult<PermittedResponse> {
        use crate::common::api_endpoints::CcmDrivePermissionApiOld;

        let api_request: ApiRequest<PermittedResponse> =
            ApiRequest::post(&CcmDrivePermissionApiOld::MemberPermitted.to_url())
                .body(serde_json::to_value(&self.req)?);
        let response: Response<PermittedResponse> =
            Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("response", "响应数据为空")
        })
    }
}
