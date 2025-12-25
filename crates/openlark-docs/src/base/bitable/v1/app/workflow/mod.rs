/// 自动化流程模块
use openlark_core::config::Config;

pub mod list;
pub mod update;

pub use list::{ListWorkflowRequest, ListWorkflowResponse, Workflow};
pub use update::{UpdateWorkflowRequest, UpdateWorkflowResponse, WorkflowStatus};

/// 自动化流程服务
#[derive(Debug, Clone)]
pub struct AppWorkflowService {
    config: Config,
}

impl AppWorkflowService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 列出自动化流程
    pub fn list(&self, app_token: impl Into<String>) -> ListWorkflowRequest {
        ListWorkflowRequest::new(self.config.clone()).app_token(app_token)
    }

    /// 更新自动化流程状态
    pub fn update(
        &self,
        app_token: impl Into<String>,
        workflow_id: impl Into<String>,
        status: WorkflowStatus,
    ) -> UpdateWorkflowRequest {
        UpdateWorkflowRequest::new(self.config.clone())
            .app_token(app_token)
            .workflow_id(workflow_id)
            .status(status)
    }
}

// Type alias for compatibility
pub type ServiceType = AppWorkflowService;
