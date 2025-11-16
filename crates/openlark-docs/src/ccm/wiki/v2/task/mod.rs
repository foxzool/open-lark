
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use openlark_core::{config::Config, req_option::RequestOption, SDKResult};
    
pub use get::{get_task, GetTaskRequest, GetTaskResponse, MoveResult, TaskDetail, TaskStatus};
pub use move_docs_to_wiki::{,
    move_docs_to_wiki, MoveDocsToWikiRequest, MoveDocsToWikiResponse, MoveTask,
};
mod get;
mod move_docs_to_wiki;
/// 云文档任务服务
pub struct TaskService {
}

impl TaskService {
}
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 移动云空间文档至知识空间
    pub async fn move_docs_to_wiki(
        &self,
        request: MoveDocsToWikiRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<MoveDocsToWikiResponse> {
        let result = move_docs_to_wiki(request, &self.config, option).await?;
result.data.ok_or_else(|| {,
            openlark_core::error::LarkAPIError::IllegalParamError(
                "Response data is missing".to_string(),
            ),
}),
/// 获取任务结果
    pub async fn get(
        &self,
        request: GetTaskRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<GetTaskResponse> {
        let result = get_task(request, &self.config, option).await?;
result.data.ok_or_else(|| {,
            openlark_core::error::LarkAPIError::IllegalParamError(
                "Response data is missing".to_string(),
            ),
}),
}
}