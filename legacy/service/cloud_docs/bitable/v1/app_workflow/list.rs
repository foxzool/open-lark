use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::cloud_docs::*,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

/// 列出自动化流程请求
#[derive(Debug, Serialize, Default, Clone)]
pub struct ListWorkflowRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 页大小
    #[serde(skip)]
    page_size: Option<i32>,
    /// 页标记，第一次请求不填，表示从头开始遍历
    #[serde(skip)]
    page_token: Option<String>,
}

impl ListWorkflowRequest {
    pub fn builder() -> ListWorkflowRequestBuilder {
        ListWorkflowRequestBuilder::default()
    }

    pub fn new(app_token: impl ToString) -> Self {
        Self {
            app_token: app_token.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct ListWorkflowRequestBuilder {
    request: ListWorkflowRequest,
}

impl ListWorkflowRequestBuilder {
    /// 多维表格的唯一标识符
    pub fn app_token(mut self, app_token: impl ToString) -> Self {
        self.request.app_token = app_token.to_string();
        self
    }

    /// 页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 页标记
    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        self.request.page_token = Some(page_token.to_string());
        self
    }

    pub fn build(mut self) -> ListWorkflowRequest {
        if let Some(page_size) = self.request.page_size {
            self.request
                .api_request
                .query_params
                .insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = &self.request.page_token {
            self.request
                .api_request
                .query_params
                .insert("page_token", page_token.clone());
        }
        self.request
    }
}

// 应用ExecutableBuilder trait到ListWorkflowRequestBuilder
crate::impl_executable_builder_owned!(
    ListWorkflowRequestBuilder,
    super::AppWorkflowService,
    ListWorkflowRequest,
    BaseResponse<ListWorkflowResponse>,
    list
);

/// 自动化流程信息
#[derive(Debug, Deserialize)]
pub struct Workflow {
    /// 自动化流程ID
    pub id: String,
    /// 自动化流程名称
    pub name: String,
    /// 自动化流程状态：0-未启用，1-已启用
    pub is_enabled: i32,
    /// 自动化流程创建时间戳（秒）
    pub created_time: i64,
    /// 自动化流程更新时间戳（秒）
    pub updated_time: i64,
    /// 自动化流程最后执行时间戳（秒）
    #[serde(default)]
    pub last_execution_time: Option<i64>,
    /// 流程触发器类型
    pub trigger_type: String,
    /// 流程描述
    #[serde(default)]
    pub description: Option<String>,
}

/// 列出自动化流程响应
#[derive(Debug, Deserialize)]
pub struct ListWorkflowResponse {
    /// 自动化流程列表
    pub items: Vec<Workflow>,
    /// 分页标记，当has_more为true时，会同时返回新的page_token
    #[serde(default)]
    pub page_token: Option<String>,
    /// 是否还有更多项
    pub has_more: bool,
    /// 总数
    #[serde(default)]
    pub total: Option<i32>,
}

impl ApiResponseTrait for ListWorkflowResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 列出自动化流程
pub async fn list_workflows(
    request: ListWorkflowRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<ListWorkflowResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::GET;
    api_req.api_path = BITABLE_V1_WORKFLOWS.replace("{app_token}", &request.app_token);
    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_list_workflow_request_builder() {
        let request = ListWorkflowRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .page_size(20)
            .page_token("page_token_123")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.page_token, Some("page_token_123".to_string()));
    }
}
