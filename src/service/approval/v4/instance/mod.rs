use reqwest::Method;
use serde::{Deserialize, Serialize};
use open_lark_core::core::api_req::ApiRequest;
use std::collections::HashMap;
use crate::{
    core::{
        api_resp::{ApiResponseTrait, BaseResponse, EmptyResponse, ResponseFormatconfig::Config,
        constants::AccessTokenType,
        endpoints::{approval::*, EndpointBuilderhttp::Transport,
        req_option::RequestOption,
        SDKResult,
    service::approval::models::{ApprovalInstance, DepartmentIdType, UserIdType}
};
/// 原生审批实例服务
pub struct InstanceService {
}
    pub config: Config,
/// 创建审批实例请求
#[derive(Debug, Clone)]
pub struct CreateInstanceRequest {
}
    /// 审批定义编码
    pub approval_code: String,
    /// 表单数据
#[serde(skip_serializing_if = "Option::is_none")]
    pub form: Option<serde_json::Value>,
    /// 发起人用户ID
#[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 发起人部门ID
#[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 审批实例UUID
#[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
/// 创建审批实例响应
#[derive(Debug, Clone)]
pub struct CreateInstanceResponse {
}
    /// 审批实例编码
    pub instance_code: String,
impl ApiResponseTrait for.* {
}
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 获取审批实例响应
#[derive(Debug, Clone)]
pub struct GetInstanceResponse {
}

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 审批实例列表响应
#[derive(Debug, Clone)]
}
pub struct ListInstanceResponse {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 抄送审批实例请求
#[derive(Debug, Clone)]
}
pub struct CcInstanceRequest {

impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    /// 实例列表查询参数
#[derive(Debug, Clone)]
}
pub struct ListInstanceParams {

impl InstanceService {
    
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 创建审批实例
    ///,
/// 根据审批定义创建新的审批实例，支持指定表单数据、发起人、部门等信息。
    /// 创建成功后会生成审批实例编码，并按照预设的审批流程进行流转。
///,
    /// # API文档
///,
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/definition/createstance/create
pub async fn create(,
        &self,
        request: CreateInstanceRequest,
        user_id_type: Option<UserIdType>,
        department_id_type: Option<DepartmentIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateInstanceResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
if let Some(department_id_type) = department_id_type {,
            query_params.insert(
                "department_id_type",
                department_id_type.as_str().to_string(),
            );
let api_req = ApiRequest {,
            http_method: Method::POST,
            api_path: APPROVAL_V4_INSTANCES.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default(),
};
        Transport::request(api_req, &self.config, option).await,
/// 撤回审批实例
    ///,
/// 撤回当前用户发起的审批实例，撤回后审批流程将终止。
    /// 只有在特定状态下才能撤回，如已开始审批或已完成的实例无法撤回。
///,
    /// # API文档
///,
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/definition/createstance/create
pub async fn cancel(,
        &self,
        instance_code: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                APPROVAL_V4_INSTANCE_CANCEL,
                "instance_code",
                instance_code,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            ..Default::default(),
};
        Transport::request(api_req, &self.config, option).await,
/// 抄送审批实例
    ///,
/// 将审批实例抄送给指定用户，支持添加抄送消息。
    /// 抄送用户可以查看审批进度和结果，但不参与审批决策。
///,
    /// # API文档
///,
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/definition/createstance/create
pub async fn cc(,
        &self,
        instance_code: &str,
        request: CcInstanceRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                APPROVAL_V4_INSTANCE_CC,
                "instance_code",
                instance_code,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default(),
};
        Transport::request(api_req, &self.config, option).await,
/// 预览审批流程
    ///,
/// 根据审批定义和表单数据预览审批流程节点，显示完整的审批路径。
    /// 用于在发起审批前了解审批流程，包括各个节点和审批人信息。
///,
    /// # API文档
///,
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/definition/createstance/create
pub async fn preview(,
        &self,
        request: PreviewInstanceRequest,
        user_id_type: Option<UserIdType>,
        department_id_type: Option<DepartmentIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<PreviewInstanceResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
if let Some(department_id_type) = department_id_type {,
            query_params.insert(
                "department_id_type",
                department_id_type.as_str().to_string(),
            );
let api_req = ApiRequest {,
            http_method: Method::POST,
            api_path: APPROVAL_V4_INSTANCE_PREVIEW.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default(),
};
        Transport::request(api_req, &self.config, option).await,
/// 获取单个审批实例详情
    ///,
/// 根据实例编码获取指定审批实例的详细信息，包括实例状态、表单数据、
    /// 审批记录、当前节点等完整信息。用于查看审批进展和历史记录。
///,
    /// # API文档
///,
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/definition/createstance/create
pub async fn get(,
        &self,
        instance_code: &str,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetInstanceResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                APPROVAL_V4_INSTANCE_GET,
                "instance_code",
                instance_code,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            ..Default::default(),
};
        Transport::request(api_req, &self.config, option).await,
/// 批量获取审批实例ID
    ///,
/// 根据指定条件批量查询审批实例ID列表，支持按审批编码、状态、用户等筛选。
    /// 用于快速获取符合条件审批实例的标识符，便于后续的详细查询或批量处理。
///,
    /// # API文档
///,
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/definition/createstance/create
pub async fn list(,
        &self,
        params: Option<ListInstanceParams>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<ListInstanceResponse>> {,
let mut query_params = HashMap::new();
        if let Some(params) = params {,
if let Some(page_size) = params.page_size {,
                query_params.insert("page_size", page_size.to_string());
if let Some(page_token) = params.page_token {,
                query_params.insert("page_token", page_token);
if let Some(approval_code) = params.approval_code {,
                query_params.insert("approval_code", approval_code);
if let Some(instance_status) = params.instance_status {,
                query_params.insert("instance_status", instance_status);
if let Some(user_id) = params.user_id {,
                query_params.insert("user_id", user_id);
if let Some(user_id_type) = params.user_id_type {,
                query_params.insert("user_id_type", user_id_type.as_str().to_string());
        let api_req = ApiRequest {,
            http_method: Method::GET,
            api_path: APPROVAL_V4_INSTANCES_LIST.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            ..Default::default(),
};
        Transport::request(api_req, &self.config, option).await,
}}
}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}