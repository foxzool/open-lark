/// Bitable 列出工作流API
///
/// API文档: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/workflow/list
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
};
use serde::{Deserialize, Serialize};

/// 列出自动化流程请求
#[allow(dead_code)]
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct ListWorkflowRequest {
    api_request: ApiRequest<ListWorkflowResponse>,
    app_token: String,
    page_size: Option<i32>,
    page_token: Option<String>,
}

/// 列出自动化流程响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListWorkflowResponse {
    /// 自动化流程列表
    pub items: Vec<Workflow>,
    /// 分页标记
    #[serde(default)]
    pub page_token: Option<String>,
    /// 是否还有更多项
    pub has_more: bool,
    /// 总数
    #[serde(default)]
    pub total: Option<i32>,
}

/// 工作流信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    /// 工作流ID
    pub workflow_id: String,
    /// 工作流名称
    pub name: String,
    /// 工作流状态
    pub status: String,
}

impl ApiResponseTrait for ListWorkflowResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ListWorkflowRequest {
    pub fn new(_config: Config) -> Self {
        Self {
            api_request: ApiRequest::get("/open-apis/bitable/v1/apps/{app_token}/workflows"),
            app_token: String::new(),
            page_size: None,
            page_token: None,
        }
    }

    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: String) -> Self {
        self.page_token = Some(page_token);
        self
    }

    pub fn builder(config: Config) -> ListWorkflowRequestBuilder {
        ListWorkflowRequestBuilder::new(config)
    }
}

/// 列出工作流请求构建器
pub struct ListWorkflowRequestBuilder {
    request: ListWorkflowRequest,
}

impl ListWorkflowRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            request: ListWorkflowRequest::new(config),
        }
    }

    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request = self.request.page_size(page_size);
        self
    }

    pub fn page_token(mut self, page_token: String) -> Self {
        self.request = self.request.page_token(page_token);
        self
    }

    pub fn build(self) -> ListWorkflowRequest {
        self.request
    }
}
