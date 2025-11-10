#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use open_lark_core::SDKResult;use reqwest::Method;
use serde::{Deserialize, Serialize};
use open_lark_core::api_req::ApiRequest;
use std::collections::HashMap;
use crate::{
    core::{
        api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormatconfig::Config,
        constants::AccessTokenType,
        endpoints::{EndpointBuilder, Endpointshttp::Transport,
        req_option::RequestOption,
        SDKResult,
    service::task::models::{ActivitySubscription, TaskMember, UserIdType}
};
/// 清单活动订阅服务
#[derive(Debug)]
pub struct TasklistActivitySubscriptionService {
}
    pub config: Config,
/// 创建动态订阅请求
#[derive(Debug, Clone)]
pub struct CreateActivitySubscriptionRequest {
}
    /// 订阅名称
    pub name: String,
    /// 订阅者列表
#[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<TaskMember>>,
    /// 是否包含已完成任务
#[serde(skip_serializing_if = "Option::is_none")]
    pub include_completed: Option<bool>,
/// 创建动态订阅响应
#[derive(Debug, Clone)]
pub struct CreateActivitySubscriptionResponse {
}
    /// 创建的订阅
    pub subscription: ActivitySubscription,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 更新动态订阅请求
#[derive(Debug, Clone)]
}
pub struct UpdateActivitySubscriptionRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 获取动态订阅响应
#[derive(Debug, Clone)]
}
pub struct GetActivitySubscriptionResponse {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 动态订阅列表响应
#[derive(Debug, Clone)]
}
pub struct ListActivitySubscriptionsResponse {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
impl TasklistActivitySubscriptionService {
    pub fn new(config: Config) -> Self {
        Self { config 
}
}/// 创建动态订阅
    pub async fn create(
        &self,
        tasklist_guid: &str,
        request: CreateActivitySubscriptionRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateActivitySubscriptionResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_http_http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                Endpoints::TASK_V2_TASKLIST_ACTIVITY_SUBSCRIPTIONS,
                "tasklist_guid",
                tasklist_guid,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 获取动态订阅
    pub async fn get(
        &self,
        tasklist_guid: &str,
        activity_subscription_guid: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetActivitySubscriptionResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_http_http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(,
&EndpointBuilder::replace_param(,
                    Endpoints::TASK_V2_TASKLIST_ACTIVITY_SUBSCRIPTION_GET,
                    "tasklist_guid",
                    tasklist_guid,
                ),
                "activity_subscription_guid",
                activity_subscription_guid,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 列取动态订阅
    pub async fn list(
        &self,
        tasklist_guid: &str,
        page_size: Option<i32>,
        page_token: Option<&str>,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListActivitySubscriptionsResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
if let Some(page_size) = page_size {,
            query_params.insert("page_size", page_size.to_string());
if let Some(page_token) = page_token {,
            query_params.insert("page_token", page_token.to_string());
let api_req = ApiRequest {,
            http_http_http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                Endpoints::TASK_V2_TASKLIST_ACTIVITY_SUBSCRIPTIONS,
                "tasklist_guid",
                tasklist_guid,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 更新动态订阅
    pub async fn patch(
        &self,
        tasklist_guid: &str,
        activity_subscription_guid: &str,
        request: UpdateActivitySubscriptionRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateActivitySubscriptionResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_http_http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(,
&EndpointBuilder::replace_param(,
                    Endpoints::TASK_V2_TASKLIST_ACTIVITY_SUBSCRIPTION_GET,
                    "tasklist_guid",
                    tasklist_guid,
                ),
                "activity_subscription_guid",
                activity_subscription_guid,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 删除动态订阅
    pub async fn delete(
        &self,
        tasklist_guid: &str,
        activity_subscription_guid: &str,
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
                    Endpoints::TASK_V2_TASKLIST_ACTIVITY_SUBSCRIPTION_GET,
                    "tasklist_guid",
                    tasklist_guid,
                ),
                "activity_subscription_guid",
                activity_subscription_guid,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
}}
}}}}}}}}}}}}}}}}}}}}}}}}}