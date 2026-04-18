//! 获取任务详情
//!
//! docPath: https://open.feishu.cn/document/task-v2/task/get

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::task::models::GetTaskResponse;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

/// 获取任务详情请求。
#[derive(Debug, Clone)]
pub struct GetTaskRequest {
    /// 配置信息。
    config: Arc<Config>,
    /// 任务 GUID。
    task_guid: String,
    /// 用户 ID 类型。
    user_id_type: Option<String>,
}

impl GetTaskRequest {
    pub fn new(config: Arc<Config>, task_guid: impl Into<String>) -> Self {
        Self {
            config,
            task_guid: task_guid.into(),
            user_id_type: None,
        }
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    pub async fn execute(self) -> SDKResult<GetTaskResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetTaskResponse> {
        validate_required!(self.task_guid.trim(), "任务 GUID 不能为空");

        let api_endpoint = TaskApiV2::TaskGet(self.task_guid.clone());
        let mut request = ApiRequest::<GetTaskResponse>::get(api_endpoint.to_url());
        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取任务详情")
    }
}

impl ApiResponseTrait for GetTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_task_request_builder() {
        let request = GetTaskRequest::new(Arc::new(Config::default()), "task_guid_123")
            .user_id_type("open_id");
        assert_eq!(request.task_guid, "task_guid_123");
        assert_eq!(request.user_id_type.as_deref(), Some("open_id"));
    }
}
