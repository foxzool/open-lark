use reqwest::Method;
use serde::{Deserialize, Serialize};
use open_lark_core::core::api_req::ApiRequest;
use std::collections::HashMap;
use crate::{
    core::{
        api_resp::{BaseResponse, EmptyResponseconfig::Config,
        constants::AccessTokenType,
        endpoints::{approval::*, EndpointBuilderhttp::Transport,
        req_option::RequestOption,
        trait_system::Service,
        SDKResult,
    service::approval::models::UserIdType,
};
/// 原生审批任务服务
pub struct TaskService {
}

impl TaskService {
    
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 同意审批任务
    ///,
/// 同意指定的审批任务，支持添加审批意见和修改表单数据。
    /// 同意后审批流程将继续流转到下一节点或完成审批。
///,
    /// # API文档
///,
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/definition/createstance/create
pub async fn approve(,
        &self,
        task_id: &str,
        request: ApproveTaskRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(APPROVAL_V4_TASK_APPROVE, "task_id", task_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default(),
};
        Transport::request(api_req, &self.config, option).await,
/// 拒绝审批任务
    ///,
/// 拒绝指定的审批任务，需要提供拒绝原因，支持修改表单数据。
    /// 拒绝后审批流程将终止，发起人可以重新提交或修改后再次发起。
///,
    /// # API文档
///,
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/definition/createstance/create
pub async fn reject(,
        &self,
        task_id: &str,
        request: RejectTaskRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(APPROVAL_V4_TASK_REJECT, "task_id", task_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default(),
};
        Transport::request(api_req, &self.config, option).await,
/// 转交审批任务
    ///,
/// 将审批任务转交给其他用户处理，需要提供转交用户ID和转交原因。
    /// 转交后原审批人将不再负责此任务，新用户将接管审批权限。
///,
    /// # API文档
///,
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/definition/createstance/create
pub async fn transfer(,
        &self,
        task_id: &str,
        request: TransferTaskRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(APPROVAL_V4_TASK_TRANSFER, "task_id", task_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default(),
};
        Transport::request(api_req, &self.config, option).await,
/// 退回审批任务
    ///,
/// 将审批任务退回到指定节点，需要提供目标节点ID和退回原因。
    /// 用于将审批任务退回给上级或发起人重新处理，适用于需要补充材料或修正的情况。
///,
    /// # API文档
///,
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/definition/createstance/create
pub async fn rollback(,
        &self,
        task_id: &str,
        request: RollbackTaskRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(
                APPROVAL_V4_TASK_SPECIFIED_ROLLBACK,
                "task_id",
                task_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default(),
};
        Transport::request(api_req, &self.config, option).await,
/// 审批任务加签
    ///,
/// 为审批任务添加加签用户，支持设置加签类型和选择加签用户。
    /// 加签用户将在当前节点完成后参与审批，用于征求额外意见或协同审批。
///,
    /// # API文档
///,
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/definition/createstance/create
pub async fn add_sign(,
        &self,
        task_id: &str,
        request: AddSignTaskRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(APPROVAL_V4_TASK_ADD_SIGN, "task_id", task_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default(),
};
        Transport::request(api_req, &self.config, option).await,
/// 重新提交审批任务
    ///,
/// 重新提交被拒绝或退回的审批任务，支持修改表单数据和添加提交意见。
    /// 用于根据拒绝原因补充材料或修正问题后重新发起审批。
///,
    /// # API文档
///,
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/approval-v4/definition/createstance/create
pub async fn resubmit(,
        &self,
        task_id: &str,
        request: ResubmitTaskRequest,
        user_id_type: Option<UserIdType>,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {,
let mut query_params = HashMap::new();
        if let Some(user_id_type) = user_id_type {
            query_params.insert("user_id_type", user_id_type.as_str().to_string());
let api_req = ApiRequest {,
            http_method: Method::POST,
            api_path: EndpointBuilder::replace_param(APPROVAL_V4_TASK_RESUBMIT, "task_id", task_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User]
            query_params,
            body: serde_json::to_vec(&request)?,
            ..Default::default(),
};
        Transport::request(api_req, &self.config, option).await,
impl Service for TaskService {,
    fn config(&self) -> &Config {,
&self.config,
    fn service_name() -> &'static str {,
        "task",
fn service_version() -> &'static str {,
        "v4",
}
}}}}}}}}}}}}}}}}}}}