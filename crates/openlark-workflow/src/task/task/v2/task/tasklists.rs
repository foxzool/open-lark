//! 列取任务所在清单
//!
//! docPath: https://open.feishu.cn/document/task-v2/task/tasklists

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::task::tasklists::GetTaskTasklistsResponse;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

/// 列取任务所在清单请求。
#[derive(Debug, Clone)]
pub struct GetTaskTasklistsRequest {
    /// 配置信息。
    config: Arc<Config>,
    /// 任务 GUID。
    task_guid: String,
}

impl GetTaskTasklistsRequest {
    pub fn new(config: Arc<Config>, task_guid: impl Into<String>) -> Self {
        Self {
            config,
            task_guid: task_guid.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<GetTaskTasklistsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetTaskTasklistsResponse> {
        validate_required!(self.task_guid.trim(), "任务 GUID 不能为空");

        let api_endpoint = TaskApiV2::TaskGetTasklists(self.task_guid.clone());
        let request = ApiRequest::<GetTaskTasklistsResponse>::get(api_endpoint.to_url());

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "列取任务所在清单")
    }
}

impl ApiResponseTrait for GetTaskTasklistsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_task_tasklists_request_builder() {
        let request = GetTaskTasklistsRequest::new(Arc::new(Config::default()), "task_guid_123");
        assert_eq!(request.task_guid, "task_guid_123");
    }
}
