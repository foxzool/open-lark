//! 根据 filetoken 获取文档的公共设置。
//!
//! docPath: /document/ukTMukTMukTM/uITM3YjLyEzN24iMxcjN
//! doc: https://open.feishu.cn/document/ukTMukTMukTM/uITM3YjLyEzN24iMxcjN

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::*;

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
    #[serde(rename = "permission_version")]
    pub permission_version: Option<String>,
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

    pub async fn execute(self) -> SDKResult<GetPublicPermissionResponse> {
        validate_required!(self.req.token, "token 不能为空");
        validate_required!(self.req.type_, "type 不能为空");

        use crate::common::api_endpoints::CcmDrivePermissionApiOld;

        let api_request: ApiRequest<GetPublicPermissionResponse> =
            ApiRequest::post(&CcmDrivePermissionApiOld::Public.to_url())
                .body(serialize_params(&self.req, "获取云文档权限设置V2")?);
        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "获取云文档权限设置V2")
    }
}
