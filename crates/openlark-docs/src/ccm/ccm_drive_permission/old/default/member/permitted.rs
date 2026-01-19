//! 判断协作者是否有某权限
//!
//! docPath: /document/ukTMukTMukTM/uYzN3UjL2czN14iN3cTN
//! doc: https://open.feishu.cn/document/ukTMukTMukTM/uYzN3UjL2czN14iN3cTN

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::*;

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
    pub fn new(
        config: Config,
        token: impl Into<String>,
        type_: impl Into<String>,
        perm: impl Into<String>,
    ) -> Self {
        Self {
            config,
            req: PermittedReq {
                token: token.into(),
                type_: type_.into(),
                perm: perm.into(),
            },
        }
    }

    pub async fn execute(self) -> SDKResult<PermittedResponse> {
            self.execute_with_options(openlark_core::req_option::RequestOption::default()).await
        }

        pub async fn execute_with_options(
            self,
            option: openlark_core::req_option::RequestOption,
        ) -> SDKResult<PermittedResponse> {

        validate_required!(self.req.token, "token 不能为空");
        validate_required!(self.req.type_, "type 不能为空");
        validate_required!(self.req.perm, "perm 不能为空");

        use crate::common::api_endpoints::CcmDrivePermissionApiOld;

        let api_request: ApiRequest<PermittedResponse> =
            ApiRequest::post(&CcmDrivePermissionApiOld::MemberPermitted.to_url())
                .body(serialize_params(&self.req, "判断协作者是否有某权限")?);
        
            let response = Transport::request(api_request, &self.config, 
Some(option)).await?;
                extract_response_data(response, "权限")}
}
