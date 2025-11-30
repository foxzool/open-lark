#![allow(unused_variables, unused_unsafe)]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use SDKResult;use reqwest::Method;
use openlark_core::api::ApiRequest;use serde::{Deserialize, Serialize};
use openlark_core::,
{
    core::,
{,
        BaseResponse,
        ResponseFormat,
        api::{ApiResponseTrait}
    config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
};
    impl_executable_builder_config,
};
/// 更新自动化流程状态请求,
#[derive(Clone)]
pub struct UpdateWorkflowRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符,
#[serde(skip)]
    app_token: String,
    /// 自动化流程ID,
#[serde(skip)]
    workflow_id: String,
    /// 自动化流程状态：true-启用，false-停用
    is_enabled: bool}
impl UpdateWorkflowRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone)]
pub struct UpdateWorkflowRequestBuilder {
    request: UpdateWorkflowRequest}
impl UpdateWorkflowRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_config!(,
    UpdateWorkflowRequestBuilder,
    UpdateWorkflowRequest,
    Response<UpdateWorkflowResponse>,
    update_workflow,
);
/// 更新自动化流程状态响应
#[derive(Clone)]
pub struct UpdateWorkflowResponse {
    /// 自动化流程ID
    pub workflow_id: String,
    /// 更新后的状态：0-未启用，1-已启用
    pub is_enabled: i32,
    /// 更新时间戳（秒）
    pub updated_time: i64,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 更新自动化流程状态,
pub async fn update_workflow(
    request: UpdateWorkflowRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<UpdateWorkflowResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::PATCH);
api_req.api_path = BITABLE_V1_WORKFLOW_UPDATE,
        .replace("{app_token}", &request.app_token)
        .replace("{workflow_id}", &request.workflow_id);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp),

#[cfg(test)]
mod tests {
    use super::*;
#[test]
    fn test_update_workflow_request_builder() {
let request = UpdateWorkflowRequest::builder(),
            .app_token()
.workflow_id()
            .enable()
.build();
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.workflow_id, "wkfxxxxxx");
assert!(request.is_enabled);
    }
#[test]
    fn test_update_workflow_request_disable() {
let request = UpdateWorkflowRequest::builder(),
            .app_token()
.workflow_id()
            .disable()
.build();
        assert!(!request.is_enabled);
