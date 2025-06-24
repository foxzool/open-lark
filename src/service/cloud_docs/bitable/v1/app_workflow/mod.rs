use crate::core::config::Config;

pub use list::*;
pub use update::*;

mod list;
mod update;

/// 自动化流程服务
pub struct AppWorkflowService {
    config: Config,
}

impl AppWorkflowService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 列出自动化流程
    pub async fn list(
        &self,
        request: &ListWorkflowRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<ListWorkflowResponse>> {
        list_workflows(request.clone(), &self.config, option).await
    }

    /// 更新自动化流程状态
    pub async fn update(
        &self,
        request: UpdateWorkflowRequest,
        option: Option<crate::core::req_option::RequestOption>,
    ) -> crate::core::SDKResult<crate::core::api_resp::BaseResponse<UpdateWorkflowResponse>> {
        update_workflow(request, &self.config, option).await
    }
}
