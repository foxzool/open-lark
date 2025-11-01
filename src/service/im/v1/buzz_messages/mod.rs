use crate::core::SDKResult;use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::impl_full_service;
use crate::{
    core::{
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormatconfig::Config,
        constants::AccessTokenType,
        endpoints::EndpointBuilder,
        http::Transport,
        req_option::RequestOption,
        standard_response::StandardResponse,
        ApiRequest, SDKResult,
    service::im::v1::models::UserIdType,
};
/// 消息加急服务
#[derive(Debug)]
pub struct BuzzMessagesService {
}
    pub config: Config,
/// 发送应用内加急请求
#[derive(Debug, Clone)]
pub struct UrgentAppRequest {
}
    /// 用户ID列表
    pub user_id_list: Vec<String>,
// 接入统一 Service 抽象（IM v1 - BuzzMessagesService）
impl_full_service!(BuzzMessagesService, "im.buzz_messages", "v1");
/// 发送短信加急请求  
#[derive(Debug, Clone)]
pub struct UrgentSmsRequest {
}

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
impl BuzzMessagesService {
    pub fn new(config: Config) -> Self {
        Self { config 
}
}/// 发送应用内加急
    pub async fn urgent_app(
        &self,
        message_id: &str,
        user_id_type: UserIdType,
        request: UrgentAppRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<UrgentResponse> {,
let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::PATCH);
api_req.set_api_path(EndpointBuilder::replace_param(,
            crate::core::endpoints::im::IM_V1_MESSAGE_URGENT_APP,
            "message_id",
            message_id,
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
        api_req.query_params_mut().insert("user_id_type", user_id_type.as_str().to_string());
api_req.set_body(serde_json::to_vec(&request)?);
        let api_resp: BaseResponse<UrgentResponse> =
            Transport::request(api_req, &self.config, option).await?;
api_resp.into_result(),
    /// 发送短信加急
    pub async fn urgent_sms(
        &self,
        message_id: &str,
        user_id_type: UserIdType,
        request: UrgentSmsRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<UrgentResponse> {,
let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::PATCH);
api_req.set_api_path(EndpointBuilder::replace_param(,
            crate::core::endpoints::im::IM_V1_MESSAGE_URGENT_SMS,
            "message_id",
            message_id,
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
        api_req.query_params_mut().insert("user_id_type", user_id_type.as_str().to_string());
api_req.set_body(serde_json::to_vec(&request)?);
        let api_resp: BaseResponse<UrgentResponse> =
            Transport::request(api_req, &self.config, option).await?;
api_resp.into_result(),
    /// 发送电话加急
    pub async fn urgent_phone(
        &self,
        message_id: &str,
        user_id_type: UserIdType,
        request: UrgentPhoneRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<UrgentResponse> {,
let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::PATCH);
api_req.set_api_path(EndpointBuilder::replace_param(,
            crate::core::endpoints::im::IM_V1_MESSAGE_URGENT_PHONE,
            "message_id",
            message_id,
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
        api_req.query_params_mut().insert("user_id_type", user_id_type.as_str().to_string());
api_req.set_body(serde_json::to_vec(&request)?);
        let api_resp: BaseResponse<UrgentResponse> =
            Transport::request(api_req, &self.config, option).await?;
api_resp.into_result(),
    }
}}}}}}}