use reqwest::Method;
use serde::{Deserialize, Serialize};
use open_lark_core::core::api_req::ApiRequest;
use std::collections::HashMap;
use crate::{
    core::{
        api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormatconfig::Config,
        constants::AccessTokenType,
        endpoints::{helpdesk::*, EndpointBuilderhttp::Transport,
        req_option::RequestOption,
        SDKResult,
    service::helpdesk::models::{AgentSchedule, UserIdType}
};
/// 客服工作日程服务
pub struct AgentScheduleService {
}
    pub config: Config,
/// 创建客服工作日程请求
#[derive(Debug, Clone)]
pub struct CreateAgentScheduleRequest {
}
    /// 开始时间
    pub start_time: String,
    /// 结束时间
    pub end_time: String,
    /// 重复模式
#[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_type: Option<String>,
/// 创建客服工作日程响应
#[derive(Debug, Clone)]
pub struct CreateAgentScheduleResponse {
}
    /// 创建的工作日程
    pub schedule: AgentSchedule,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 更新客服工作日程请求
#[derive(Debug, Clone)]
}
pub struct UpdateAgentScheduleRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 获取客服工作日程响应
#[derive(Debug, Clone)]
}
pub struct GetAgentScheduleResponse {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 获取全部客服工作日程响应
#[derive(Debug, Clone)]
}
pub struct ListAgentSchedulesResponse {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
impl AgentScheduleService {
    pub fn new(config: Config) -> Self {
        Self { config 
}
}/// 创建客服工作日程
    pub async fn create(
        &self,
        agent_id: &str,
        request: CreateAgentScheduleRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateAgentScheduleResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_http_http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                HELPDESK_V1_AGENT_SCHEDULES,
                "agent_id",
                agent_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 删除客服工作日程
    pub async fn delete(
        &self,
        agent_id: &str,
        schedule_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_http_http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_param(,
&EndpointBuilder::replace_param(,
                    HELPDESK_V1_AGENT_SCHEDULE_DELETE,
                    "agent_id",
                    agent_id,
                ),
                "schedule_id",
                schedule_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 更新客服工作日程
    pub async fn patch(
        &self,
        agent_id: &str,
        schedule_id: &str,
        request: UpdateAgentScheduleRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateAgentScheduleResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_http_http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(,
&EndpointBuilder::replace_param(,
                    HELPDESK_V1_AGENT_SCHEDULE_DELETE,
                    "agent_id",
                    agent_id,
                ),
                "schedule_id",
                schedule_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 查询指定客服工作日程
    pub async fn get(
        &self,
        agent_id: &str,
        schedule_id: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetAgentScheduleResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_http_http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(,
&EndpointBuilder::replace_param(,
                    HELPDESK_V1_AGENT_SCHEDULE_DELETE,
                    "agent_id",
                    agent_id,
                ),
                "schedule_id",
                schedule_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 查询全部客服工作日程
    pub async fn list(
        &self,
        agent_id: &str,
        user_id_type: Option<UserIdType>,
        page_token: Option<&str>,
        page_size: Option<i32>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListAgentSchedulesResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
if let Some(page_token) = page_token {,
            query_params.insert("page_token", page_token.to_string());
if let Some(page_size) = page_size {,
            query_params.insert("page_size", page_size.to_string());
let api_req = ApiRequest {,
            http_http_http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                HELPDESK_V1_AGENT_SCHEDULES,
                "agent_id",
                agent_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
}}
}}}}}}}}}}}}}}}}}}}}}}}}}