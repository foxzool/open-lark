/// Bitable 更新工作流API
///
/// API文档: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/workflow/update
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
};
use serde::{Deserialize, Serialize};

/// 更新自动化流程状态请求
#[allow(dead_code)]
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct UpdateWorkflowRequest {
    api_request: ApiRequest<UpdateWorkflowResponse>,
    app_token: String,
    workflow_id: String,
    is_enabled: bool,
}

/// 更新自动化流程状态响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateWorkflowResponse {
    /// 更新的工作流信息
    pub workflow: Workflow,
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
    /// 是否启用
    pub is_enabled: bool,
}

impl ApiResponseTrait for UpdateWorkflowResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl UpdateWorkflowRequest {
    pub fn new(_config: Config) -> Self {
        Self {
            api_request: ApiRequest::put(
                "/open-apis/bitable/v1/apps/{app_token}/workflows/{workflow_id}",
            ),
            app_token: String::new(),
            workflow_id: String::new(),
            is_enabled: false,
        }
    }

    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    pub fn workflow_id(mut self, workflow_id: String) -> Self {
        self.workflow_id = workflow_id;
        self
    }

    pub fn is_enabled(mut self, is_enabled: bool) -> Self {
        self.is_enabled = is_enabled;
        self
    }

    pub fn builder(config: Config) -> UpdateWorkflowRequestBuilder {
        UpdateWorkflowRequestBuilder::new(config)
    }
}

/// 更新工作流请求构建器
pub struct UpdateWorkflowRequestBuilder {
    request: UpdateWorkflowRequest,
}

impl UpdateWorkflowRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            request: UpdateWorkflowRequest::new(config),
        }
    }

    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    pub fn workflow_id(mut self, workflow_id: String) -> Self {
        self.request = self.request.workflow_id(workflow_id);
        self
    }

    pub fn is_enabled(mut self, is_enabled: bool) -> Self {
        self.request = self.request.is_enabled(is_enabled);
        self
    }

    pub fn build(self) -> UpdateWorkflowRequest {
        self.request
    }
}
