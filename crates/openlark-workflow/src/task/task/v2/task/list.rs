//! 列取任务列表
//!
//! docPath: https://open.feishu.cn/document/task-v2/task/list

use crate::common::{api_endpoints::TaskApiV2, api_utils::*};
use crate::v2::task::models::ListTasksResponse;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    SDKResult,
};
use std::sync::Arc;

/// 列取任务列表请求。
#[derive(Debug, Clone)]
pub struct ListTasksRequest {
    /// 配置信息。
    config: Arc<Config>,
    /// 分页大小。
    page_size: Option<i32>,
    /// 分页标记。
    page_token: Option<String>,
    /// 是否只看已完成任务。
    completed: Option<bool>,
    /// 任务类型。
    task_type: Option<String>,
    /// 用户 ID 类型。
    user_id_type: Option<String>,
}

impl ListTasksRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            page_size: None,
            page_token: None,
            completed: None,
            task_type: None,
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

    pub fn completed(mut self, completed: bool) -> Self {
        self.completed = Some(completed);
        self
    }

    pub fn task_type(mut self, task_type: impl Into<String>) -> Self {
        self.task_type = Some(task_type.into());
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    pub async fn execute(self) -> SDKResult<ListTasksResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListTasksResponse> {
        let api_endpoint = TaskApiV2::TaskList;
        let mut request = ApiRequest::<ListTasksResponse>::get(api_endpoint.to_url());

        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }
        if let Some(completed) = self.completed {
            request = request.query("completed", completed.to_string());
        }
        if let Some(task_type) = self.task_type {
            request = request.query("type", task_type);
        }
        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "列取任务列表")
    }
}

impl ApiResponseTrait for ListTasksResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_tasks_request_builder() {
        let request = ListTasksRequest::new(Arc::new(Config::default()))
            .page_size(50)
            .page_token("page_token_1")
            .completed(true)
            .task_type("my_tasks")
            .user_id_type("open_id");
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token.as_deref(), Some("page_token_1"));
        assert_eq!(request.completed, Some(true));
        assert_eq!(request.task_type.as_deref(), Some("my_tasks"));
        assert_eq!(request.user_id_type.as_deref(), Some("open_id"));
    }
}
