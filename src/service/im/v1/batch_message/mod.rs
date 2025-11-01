use crate::core::SDKResult;use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::impl_full_service;
use open_lark_core::core::api_req::ApiRequest;
use crate::{
    core::{,
        api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormatconfig::Config,
        constants::AccessTokenType,
        endpoints::EndpointBuilder,
        http::Transport,
        req_option::RequestOption,
        standard_response::StandardResponse,
        SDKResult,
    service::im::v1::models::{BatchMessageStatus, ReceiveIdType, UserIdType}
};
/// 批量消息服务
#[derive(Debug)]
pub struct BatchMessageService {
}
    pub config: Config,
/// 批量发送消息请求
#[derive(Debug, Clone)]
pub struct BatchSendMessageRequest {
}
    /// 消息接收者ID列表
    pub receive_id_list: Vec<String>,
    /// 消息内容
    pub msg_type: String,
    /// 消息内容
    pub content: String,
    /// 用户ID类型，可选值为open_id、user_id、union_id
#[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
// 接入统一 Service 抽象（IM v1 - BatchMessageService）
impl_full_service!(BatchMessageService, "im.batch_message", "v1");
/// 批量发送消息响应
#[derive(Debug, Clone)]
pub struct BatchSendMessageResponse {
}
    /// 批量消息ID
    pub batch_message_id: String,
    /// 无效的接收者ID列表
    pub invalid_receive_ids: Vec<String>,
impl ApiResponseTrait for.* {
}
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 查询批量消息进度响应
#[derive(Debug, Clone)]
pub struct GetBatchProgressResponse {
}

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 查询批量消息推送和阅读人数响应
#[derive(Debug, Clone)]
}
pub struct GetBatchReadUserResponse {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
impl BatchMessageService {
    pub fn new(config: Config) -> Self {
        Self { config 
}
}/// 批量发送消息
    pub async fn send(
        &self,
        receive_id_type: ReceiveIdType,
        request: BatchSendMessageRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BatchSendMessageResponse> {,
let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::POST);
api_req.set_api_path(crate::core::endpoints::im::IM_V1_BATCH_MESSAGES.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
        api_req.query_params_mut().insert("receive_id_type", receive_id_type.as_str().to_string());
api_req.set_body(serde_json::to_vec(&request)?);
        let api_resp: BaseResponse<BatchSendMessageResponse> =
            Transport::request(api_req, &self.config, option).await?;
api_resp.into_result(),
    /// 批量撤回消息
    pub async fn delete(
        &self,
        batch_message_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<EmptyResponse> {,
let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::DELETE);
api_req.set_api_path(EndpointBuilder::replace_param(,
            crate::core::endpoints::im::IM_V1_DELETE_BATCH_MESSAGE,
            "batch_message_id",
            batch_message_id,
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
let api_resp: BaseResponse<EmptyResponse> =,
            Transport::request(api_req, &self.config, option).await?;
api_resp.into_result(),
    /// 查询批量消息整体进度
    pub async fn get_progress(
        &self,
        batch_message_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<GetBatchProgressResponse> {,
let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
api_req.set_api_path(EndpointBuilder::replace_param(,
            crate::core::endpoints::im::IM_V1_BATCH_MESSAGE_PROGRESS,
            "batch_message_id",
            batch_message_id,
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
let api_resp: BaseResponse<GetBatchProgressResponse> =,
            Transport::request(api_req, &self.config, option).await?;
api_resp.into_result(),
    /// 查询批量消息推送和阅读人数
    pub async fn read_user(
        &self,
        batch_message_id: &str,
        user_id_type: Option<UserIdType>,
        page_size: Option<i32>,
        page_token: Option<String>,
        option: Option<RequestOption>,
    ) -> SDKResult<GetBatchReadUserResponse> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
if let Some(page_size) = page_size {,
            query_params.insert("page_size", page_size.to_string());
if let Some(page_token) = page_token {,
            query_params.insert("page_token", page_token);
let mut api_req = ApiRequest::default();
        api_req.set_http_method(Method::GET);
api_req.set_api_path(EndpointBuilder::replace_param(,
            crate::core::endpoints::im::IM_V1_BATCH_MESSAGE_READ_USER,
            "batch_message_id",
            batch_message_id,
        ));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
api_req.query_params_mut().extend(query_params);
        let api_resp: BaseResponse<GetBatchReadUserResponse> =
            Transport::request(api_req, &self.config, option).await?;
api_resp.into_result(),
    }
}}}}}}}}}}}}}}}}