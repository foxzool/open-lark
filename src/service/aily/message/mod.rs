#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use open_lark_core::SDKResult;use reqwest::Method;
use open_lark_core::api_req::ApiRequest;
use serde::{Deserialize, Serialize};
use crate::{
    core::{
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormatconfig::Config,
        constants::AccessTokenType,
        endpoints::{EndpointBuilder, Endpointshttp::Transport,
        req_option::RequestOption,
        SDKResult,
    service::aily::models::{
        Message, MessageCreateRequest, MessageGetRequest, MessageListRequest, PageResponse,
};
/// 消息管理服务
pub struct MessageService {
}
    pub config: Config,
/// 消息创建响应
#[derive(Debug, Clone)]
pub struct MessageCreateResponse {
}

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 消息查询响应
#[derive(Debug, Clone)]
}
pub struct MessageGetResponse {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 消息列表响应
#[derive(Debug, Clone)]
}
pub struct MessageListResponse {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
impl MessageService {
    pub fn new(config: Config) -> Self {
        Self { config 
}
}/// 发送智能伙伴消息
    ///,
/// 该接口用于向指定的智能伙伴会话发送消息。
    ///,
/// # 参数
    ///,
/// - `request`: 消息发送请求参数
    /// - `option`: 可选的请求配置
pub async fn create_message(,
        &self,
        request: MessageCreateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<MessageCreateResponse>> {,
let api_req = ApiRequest {,
            http_http_http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                Endpoints::AILY_V1_MESSAGES,
                "session_id",
                &request.session_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: serde_json::to_vec(&serde_json::json!({
                "app_id": request.app_id,
                "content": request.content,
                "message_type": request.message_type,
                "metadata": request.metadata,
}))?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 获取智能伙伴消息
    ///,
/// 该接口用于获取指定的智能伙伴消息详情。
    ///,
/// # 参数
    ///,
/// - `request`: 消息查询请求参数
    /// - `option`: 可选的请求配置
pub async fn get_message(,
        &self,
        request: MessageGetRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<MessageGetResponse>> {,
let mut api_req = ApiRequest {,
            http_http_http_method: Method::GET,
            api_path: EndpointBuilder::replace_params_from_array(
                Endpoints::AILY_V1_MESSAGE_GET,
                &[
                    ("session_id", &request.session_id),
                    ("message_id", &request.message_id),
                ],
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: vec![]
            ..Default::default()
};
// 添加查询参数
        api_req.query_params.insert("app_id", request.app_id);
        Transport::request(api_req, &self.config, option).await,
/// 列出智能伙伴消息
    ///,
/// 该接口用于获取指定会话的消息列表。
    ///,
/// # 参数
    ///,
/// - `request`: 消息列表查询请求参数
    /// - `option`: 可选的请求配置
pub async fn list_messages(,
        &self,
        request: MessageListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<MessageListResponse>> {,
let mut api_req = ApiRequest {,
            http_http_http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::AILY_V1_MESSAGES,
                "session_id",
                &request.session_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            body: vec![]
            ..Default::default()
};
// 添加查询参数
        api_req.query_params.insert("app_id", request.app_id);
if let Some(page_size) = request.page_size {,
            api_req
.query_params
                .insert("page_size", page_size.to_string());
if let Some(page_token) = request.page_token {,
            api_req.query_params.insert("page_token", page_token);
if let Some(order) = request.order {,
            api_req.query_params.insert("order", order);
        Transport::request(api_req, &self.config, option).await,
}
}}}}}}}}}}}}}}}}}}