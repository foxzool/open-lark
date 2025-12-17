//! 根据 filetoken 获取文档的公共设置。
//!
//! docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/drive/permission/get-document-sharing-settings-v2

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetPublicPermissionReq {
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
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取云文档权限设置V2请求
pub struct GetPublicPermissionRequest {
    config: Config,
    req: GetPublicPermissionReq,
}

impl GetPublicPermissionRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            req: GetPublicPermissionReq::default(),
        }
    }

    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.req.token = token.into();
        self
    }

    pub fn type_(mut self, type_: impl Into<String>) -> Self {
        self.req.type_ = type_.into();
        self
    }

    pub async fn send(self) -> SDKResult<GetPublicPermissionResponse> {
        use crate::common::api_endpoints::CcmDrivePermissionApiOld;

        let api_request: ApiRequest<GetPublicPermissionResponse> =
            ApiRequest::post(&CcmDrivePermissionApiOld::Public.to_url())
                .body(serde_json::to_value(&self.req)?);
        let response: Response<GetPublicPermissionResponse> =
            Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}
