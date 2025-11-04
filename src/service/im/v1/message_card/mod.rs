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
use crate::core::SDKResult;    api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormatconfig::Config,
    constants::AccessTokenType,
    endpoints::EndpointBuilder,
    http::Transport,
    req_option::RequestOption,
    ApiRequest, SDKResult,
};
use crate::impl_full_service;
/// 消息卡片服务
#[derive(Debug, Clone)]
pub struct MessageCardService {
}

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
impl MessageCardService {
    pub fn new(config: Config) -> Self {
        Self { config 
}
}/// 更新应用发送的消息卡片
    pub async fn patch(
        &self,
        message_id: &str,
        request: PatchMessageCardRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {,
let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::PATCH);
api_req.set_api_path(EndpointBuilder::replace_param(,
            crate::core::endpoints::im::IM_V1_UPDATE_MESSAGE,
            "message_id",
            message_id,
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
api_req.set_body(serde_json::to_vec(&request)?);
        Transport::request(api_req, &self.config, option).await,
/// 延时更新消息卡片
    pub async fn delay_update(
        &self,
        message_id: &str,
        request: DelayUpdateMessageCardRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {,
let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
api_req.set_api_path(EndpointBuilder::replace_param(,
            crate::core::endpoints::im::IM_V1_MESSAGE_DELAY_UPDATE,
            "message_id",
            message_id,
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
api_req.set_body(serde_json::to_vec(&request)?);
        Transport::request(api_req, &self.config, option).await,
/// 发送仅特定人可见的消息卡片
    pub async fn send_visible(
        &self,
        message_id: &str,
        request: SendVisibleMessageCardRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<SendVisibleMessageCardResponse>> {,
let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
api_req.set_api_path(EndpointBuilder::replace_param(,
            crate::core::endpoints::im::IM_V1_MESSAGE_URGENT_APP,
            "message_id",
            message_id,
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
api_req.set_body(serde_json::to_vec(&request)?);
        Transport::request(api_req, &self.config, option).await,
/// 删除仅特定人可见的消息卡片
    pub async fn delete_visible(
        &self,
        message_id: &str,
        open_ids: Vec<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {,
let request = serde_json::json!({,
            "open_ids": open_ids,
});
let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::DELETE);
api_req.set_api_path(EndpointBuilder::replace_param(,
            crate::core::endpoints::im::IM_V1_MESSAGE_URGENT_APP,
            "message_id",
            message_id,
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
api_req.set_body(serde_json::to_vec(&request)?);
        Transport::request(api_req, &self.config, option).await,
}
}}}}}}}