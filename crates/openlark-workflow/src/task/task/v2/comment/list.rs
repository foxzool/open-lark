//! 获取评论列表
//!
//! docPath: https://open.feishu.cn/document/task-v2/comment/list

use crate::common::api_utils::*;
use crate::task::task::v2::comment::models::ListCommentsResponse;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

/// 获取评论列表请求。
#[derive(Debug, Clone)]
pub struct ListCommentsRequest {
    config: Arc<Config>,
    resource_id: String,
    page_size: Option<i32>,
    page_token: Option<String>,
    resource_type: Option<String>,
    direction: Option<String>,
    user_id_type: Option<String>,
}

impl ListCommentsRequest {
    pub fn new(config: Arc<Config>, resource_id: impl Into<String>) -> Self {
        Self {
            config,
            resource_id: resource_id.into(),
            page_size: None,
            page_token: None,
            resource_type: None,
            direction: None,
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

    pub fn resource_type(mut self, resource_type: impl Into<String>) -> Self {
        self.resource_type = Some(resource_type.into());
        self
    }

    pub fn direction(mut self, direction: impl Into<String>) -> Self {
        self.direction = Some(direction.into());
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    pub async fn execute(self) -> SDKResult<ListCommentsResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<ListCommentsResponse> {
        validate_required!(self.resource_id.trim(), "资源 ID 不能为空");

        let mut request = ApiRequest::<ListCommentsResponse>::get("/open-apis/task/v2/comments")
            .query("resource_id", self.resource_id);

        if let Some(page_size) = self.page_size {
            request = request.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            request = request.query("page_token", page_token);
        }
        if let Some(resource_type) = self.resource_type {
            request = request.query("resource_type", resource_type);
        }
        if let Some(direction) = self.direction {
            request = request.query("direction", direction);
        }
        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取评论列表")
    }
}

impl ApiResponseTrait for ListCommentsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_comments_request_builder() {
        let config = Arc::new(openlark_core::config::Config::default());
        let request = ListCommentsRequest::new(config, "task_guid_123")
            .page_size(20)
            .page_token("token_1")
            .resource_type("task")
            .direction("asc")
            .user_id_type("open_id");

        assert_eq!(request.resource_id, "task_guid_123");
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.page_token.as_deref(), Some("token_1"));
        assert_eq!(request.resource_type.as_deref(), Some("task"));
        assert_eq!(request.direction.as_deref(), Some("asc"));
        assert_eq!(request.user_id_type.as_deref(), Some("open_id"));
    }
}
