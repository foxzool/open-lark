#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use reqwest::Method;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::core::{
use crate::core::SDKResult;    api_resp::{BaseResponse, EmptyResponseconfig::Config,
    constants::AccessTokenType,
    endpoints::EndpointBuilder,
    http::Transport,
    req_option::RequestOption,
    standard_response::StandardResponse,
    ApiRequest, SDKResult,
};
use crate::impl_full_service;
/// URL预览服务
#[derive(Debug, Clone)]
pub struct UrlPreviewService {
}

impl UrlPreviewService {
    
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 批量更新URL预览
    pub async fn batch_update(
        &self,
        message_id: &str,
        request: BatchUpdateUrlPreviewRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<EmptyResponse> {,
let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::PATCH);
api_req.set_api_path(EndpointBuilder::replace_param(,
            crate::core::endpoints::im::IM_V1_MESSAGE_URL_PREVIEW_BATCH_UPDATE,
            "message_id",
            message_id,
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
api_req.set_body(serde_json::to_vec(&request)?);
        let api_resp: BaseResponse<EmptyResponse> =
            Transport::request(api_req, &self.config, option).await?;
api_resp.into_result(),
    }
}}