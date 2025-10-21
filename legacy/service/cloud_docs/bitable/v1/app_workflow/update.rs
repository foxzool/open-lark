use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_config,
};

/// 更新自动化流程状态请求
#[derive(Debug, Serialize, Default)]
pub struct UpdateWorkflowRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 自动化流程ID
    #[serde(skip)]
    workflow_id: String,
    /// 自动化流程状态：true-启用，false-停用
    is_enabled: bool,
}

impl UpdateWorkflowRequest {
    pub fn builder() -> UpdateWorkflowRequestBuilder {
        UpdateWorkflowRequestBuilder::default()
    }

    pub fn new(app_token: impl ToString, workflow_id: impl ToString, is_enabled: bool) -> Self {
        Self {
            app_token: app_token.to_string(),
            workflow_id: workflow_id.to_string(),
            is_enabled,
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct UpdateWorkflowRequestBuilder {
    request: UpdateWorkflowRequest,
}

impl UpdateWorkflowRequestBuilder {
    /// 多维表格的唯一标识符
    pub fn app_token(mut self, app_token: impl ToString) -> Self {
        self.request.app_token = app_token.to_string();
        self
    }

    /// 自动化流程ID
    pub fn workflow_id(mut self, workflow_id: impl ToString) -> Self {
        self.request.workflow_id = workflow_id.to_string();
        self
    }

    /// 自动化流程状态：true-启用，false-停用
    pub fn set_enabled(mut self, is_enabled: bool) -> Self {
        self.request.is_enabled = is_enabled;
        self
    }

    /// 启用自动化流程
    pub fn enable(mut self) -> Self {
        self.request.is_enabled = true;
        self
    }

    /// 停用自动化流程
    pub fn disable(mut self) -> Self {
        self.request.is_enabled = false;
        self
    }

    pub fn build(mut self) -> UpdateWorkflowRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl_executable_builder_config!(
    UpdateWorkflowRequestBuilder,
    UpdateWorkflowRequest,
    BaseResponse<UpdateWorkflowResponse>,
    update_workflow
);

/// 更新自动化流程状态响应
#[derive(Debug, Deserialize)]
pub struct UpdateWorkflowResponse {
    /// 自动化流程ID
    pub workflow_id: String,
    /// 更新后的状态：0-未启用，1-已启用
    pub is_enabled: i32,
    /// 更新时间戳（秒）
    pub updated_time: i64,
}

impl ApiResponseTrait for UpdateWorkflowResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新自动化流程状态
pub async fn update_workflow(
    request: UpdateWorkflowRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<UpdateWorkflowResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::PATCH;
    api_req.api_path = BITABLE_V1_WORKFLOW_UPDATE
        .replace("{app_token}", &request.app_token)
        .replace("{workflow_id}", &request.workflow_id);
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_update_workflow_request_builder() {
        let request = UpdateWorkflowRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .workflow_id("wkfxxxxxx")
            .enable()
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.workflow_id, "wkfxxxxxx");
        assert!(request.is_enabled);
    }

    #[test]
    fn test_update_workflow_request_disable() {
        let request = UpdateWorkflowRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .workflow_id("wkfxxxxxx")
            .disable()
            .build();

        assert!(!request.is_enabled);
    }
}
