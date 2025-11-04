#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use crate::core::SDKResult;use reqwest::Method;
use serde::{Deserialize, Serialize};
use open_lark_core::core::api_req::ApiRequest;
use std::collections::HashMap;
use crate::{
    core::{
        api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormatconfig::Config,
        constants::AccessTokenType,
        endpoints::{EndpointBuilder, Endpointshttp::Transport,
        req_option::RequestOption,
        trait_system::Service,
        SDKResult,
    service::task::models::{Dependency, Reminder, Task, TaskMember, Tasklist, UserIdType}
};
/// 任务服务
#[derive(Debug)]
pub struct TaskService {
}
    pub config: Config,
/// 创建任务请求
#[derive(Debug, Clone)]
pub struct CreateTaskRequest {
}
    /// 任务标题
    pub summary: String,
    /// 任务描述
#[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 清单GUID
#[serde(skip_serializing_if = "Option::is_none")]
    pub tasklist_guid: Option<String>,
    /// 截止时间
#[serde(skip_serializing_if = "Option::is_none")]
    pub due: Option<TaskDue>,
    /// 开始时间
#[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<TaskStart>,
    /// 任务成员列表
#[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<TaskMember>>,
    /// 重复规则
#[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_rule: Option<String>,
    /// 自定义完成配置
#[serde(skip_serializing_if = "Option::is_none")]
    pub custom_complete: Option<TaskCustomComplete>,
    /// 任务来源
#[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<i32>,
#[derive(Debug, Clone)]
pub struct TaskDue {
}
    /// 截止时间戳(毫秒)
#[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// 是否为全天任务
#[serde(skip_serializing_if = "Option::is_none")]
    pub is_all_day: Option<bool>,
#[derive(Debug, Clone)]
pub struct TaskStart {
}
    /// 开始时间戳(毫秒)
#[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// 是否为全天任务
#[serde(skip_serializing_if = "Option::is_none")]
    pub is_all_day: Option<bool>,
#[derive(Debug, Clone)]
pub struct TaskCustomComplete {
}
    /// 完成模式
#[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// 完成设置
#[serde(skip_serializing_if = "Option::is_none")]
    pub complete_setting: Option<TaskCompleteSetting>,
#[derive(Debug, Clone)]
pub struct TaskCompleteSetting {
}
    /// 子任务完成数量
#[serde(skip_serializing_if = "Option::is_none")]
    pub subtask_count: Option<i32>,
/// 创建任务响应
#[derive(Debug, Clone)]
pub struct CreateTaskResponse {
}
    /// 创建的任务
    pub task: Task,
impl ApiResponseTrait for.* {
}
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 更新任务请求
#[derive(Debug, Clone)]
pub struct UpdateTaskRequest {
}
    /// 任务标题
#[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// 任务描述
#[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 截止时间
#[serde(skip_serializing_if = "Option::is_none")]
    pub due: Option<TaskDue>,
    /// 开始时间
#[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<TaskStart>,
    /// 完成时间
#[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    /// 重复规则
#[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_rule: Option<String>,
    /// 自定义完成配置
#[serde(skip_serializing_if = "Option::is_none")]
    pub custom_complete: Option<TaskCustomComplete>,
/// 更新任务响应
#[derive(Debug, Clone)]
pub struct UpdateTaskResponse {
}
    /// 更新后的任务
    pub task: Task,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 获取任务响应
#[derive(Debug, Clone)]
}
pub struct GetTaskResponse {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 任务列表响应
#[derive(Debug, Clone)]
}
pub struct ListTasksResponse {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 任务加入清单请求
#[derive(Debug, Clone)]
}
pub struct AddTaskTasklistRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 添加任务成员请求
#[derive(Debug, Clone)]
}
pub struct AddTaskMembersRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 移除任务成员请求
#[derive(Debug, Clone)]
}
pub struct RemoveTaskMembersRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 添加任务提醒请求
#[derive(Debug, Clone)]
}
pub struct AddTaskRemindersRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 移除任务提醒请求
#[derive(Debug, Clone)]
}
pub struct RemoveTaskRemindersRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 添加任务依赖请求
#[derive(Debug, Clone)]
}
pub struct AddTaskDependenciesRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 移除任务依赖请求
#[derive(Debug, Clone)]
}
pub struct RemoveTaskDependenciesRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
impl TaskService {
    pub fn new(config: Config) -> Self {
        Self { config 
}
}/// 创建任务
    pub async fn create(
        &self,
        request: CreateTaskRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateTaskResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let mut api_req = ApiRequest {,
                        http_http_http_method: Method::POST,
            api_path: Endpoints::TASK_V2_TASKS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 更新任务
    pub async fn patch(
        &self,
        task_guid: &str,
        request: UpdateTaskRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UpdateTaskResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let mut api_req = ApiRequest {,
            http_http_http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(Endpoints::TASK_V2_TASK_GET,
                "task_guid",
                task_guid,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            body: vec![]
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 获取任务详情
    pub async fn get(
        &self,
        task_guid: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetTaskResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let mut api_req = ApiRequest {,
            http_http_http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(Endpoints::TASK_V2_TASK_GET,
                "task_guid",
                task_guid,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            body: vec![]
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 删除任务
    pub async fn delete(
        &self,
        task_guid: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let mut api_req = ApiRequest {,
            http_http_http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_param(Endpoints::TASK_V2_TASK_GET,
                "task_guid",
                task_guid,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            body: vec![]
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 列取任务列表
    #[allow(clippy::too_many_arguments)]
pub async fn list(,
        &self,
        page_size: Option<i32>,
        page_token: Option<&str>,
        completed: Option<bool>,
        created_from: Option<&str>,
        created_to: Option<&str>,
        updated_from: Option<&str>,
        updated_to: Option<&str>,
        due_from: Option<&str>,
        due_to: Option<&str>,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListTasksResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
if let Some(page_size) = page_size {,
            query_params.insert("page_size", page_size.to_string());
if let Some(page_token) = page_token {,
            query_params.insert("page_token", page_token.to_string());
if let Some(completed) = completed {,
            query_params.insert("completed", completed.to_string());
if let Some(created_from) = created_from {,
            query_params.insert("created_from", created_from.to_string());
if let Some(created_to) = created_to {,
            query_params.insert("created_to", created_to.to_string());
if let Some(updated_from) = updated_from {,
            query_params.insert("updated_from", updated_from.to_string());
if let Some(updated_to) = updated_to {,
            query_params.insert("updated_to", updated_to.to_string());
if let Some(due_from) = due_from {,
            query_params.insert("due_from", due_from.to_string());
if let Some(due_to) = due_to {,
            query_params.insert("due_to", due_to.to_string());
let mut api_req = ApiRequest {,
            http_http_http_method: Method::GET,
            api_path: Endpoints::TASK_V2_TASKS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            body: vec![]
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 添加任务成员
    pub async fn add_members(
        &self,
        task_guid: &str,
        request: AddTaskMembersRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<AddTaskMembersResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let mut api_req = ApiRequest {,
            http_http_http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(Endpoints::TASK_V2_TASK_GET,
                "task_guid",
                task_guid,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            body: vec![]
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 移除任务成员
    pub async fn remove_members(
        &self,
        task_guid: &str,
        request: RemoveTaskMembersRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<RemoveTaskMembersResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let mut api_req = ApiRequest {,
            http_http_http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(Endpoints::TASK_V2_TASK_GET,
                "task_guid",
                task_guid,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            body: vec![]
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 添加任务提醒
    pub async fn add_reminders(
        &self,
        task_guid: &str,
        request: AddTaskRemindersRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<AddTaskRemindersResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let mut api_req = ApiRequest {,
            http_http_http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(Endpoints::TASK_V2_TASK_GET,
                "task_guid",
                task_guid,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            body: vec![]
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 任务加入清单
    pub async fn add_tasklist(
        &self,
        task_guid: &str,
        request: AddTaskTasklistRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<AddTaskTasklistResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let mut api_req = ApiRequest {,
            http_http_http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(Endpoints::TASK_V2_TASK_GET,
                "task_guid",
                task_guid,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            body: vec![]
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 移除任务提醒
    pub async fn remove_reminders(
        &self,
        task_guid: &str,
        request: RemoveTaskRemindersRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<RemoveTaskRemindersResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let mut api_req = ApiRequest {,
            http_http_http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(Endpoints::TASK_V2_TASK_GET,
                "task_guid",
                task_guid,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            body: vec![]
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 添加任务依赖
    pub async fn add_dependencies(
        &self,
        task_guid: &str,
        request: AddTaskDependenciesRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<AddTaskDependenciesResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let mut api_req = ApiRequest {,
            http_http_http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(Endpoints::TASK_V2_TASK_GET,
                "task_guid",
                task_guid,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            body: vec![]
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
/// 移除任务依赖
    pub async fn remove_dependencies(
        &self,
        task_guid: &str,
        request: RemoveTaskDependenciesRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<RemoveTaskDependenciesResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let mut api_req = ApiRequest {,
            http_http_http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(Endpoints::TASK_V2_TASK_GET,
                "task_guid",
                task_guid,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            body: vec![]
            ..Default::default()
};
        Transport::request(api_req, &self.config, option).await,
impl Service for TaskService {,
    fn config(&self) -> &Config {,
&self.config,
    fn service_name() -> &'static str {,
        "task",
fn service_version() -> &'static str {,
        "v2",
}}}}
}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}