//! 获取任务的子任务列表
//!
//! docPath: https://open.feishu.cn/document/task-v2/task-subtask/list

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::task::subtask::list::ListSubtasksResponse;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

/// 获取任务的子任务列表请求。
#[derive(Debug, Clone)]
pub struct ListSubtasksRequest {
    /// 配置信息。
    config: Arc<Config>,
    /// 父任务 GUID。
    task_guid: String,
    /// 分页大小。
    page_size: Option<i32>,
    /// 分页标记。
    page_token: Option<String>,
    /// 用户 ID 类型。
    user_id_type: Option<String>,
}

impl ListSubtasksRequest {
    pub fn new(config: Arc<Config>, task_guid: impl Into<String>) -> Self {
        Self {
            config,
            task_guid: task_guid.into(),
            page_size: None,
            page_token: None,
            user_id_type: None,
        }
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    pub async fn execute(self) -> SDKResult<ListSubtasksResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListSubtasksResponse> {
        validate_required!(self.task_guid.trim(), "任务 GUID 不能为空");

        let api_endpoint = TaskApiV2::SubtaskList(self.task_guid.clone());
        let mut request = ApiRequest::<ListSubtasksResponse>::get(api_endpoint.to_url());
        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }
        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取任务的子任务列表")
    }
}

impl ApiResponseTrait for ListSubtasksResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_subtasks_request_builder() {
        let request = ListSubtasksRequest::new(Arc::new(Config::default()), "task_guid_123")
            .page_size(20)
            .page_token("page_token_1")
            .user_id_type("open_id");
        assert_eq!(request.task_guid, "task_guid_123");
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.page_token.as_deref(), Some("page_token_1"));
        assert_eq!(request.user_id_type.as_deref(), Some("open_id"));
    }
}
