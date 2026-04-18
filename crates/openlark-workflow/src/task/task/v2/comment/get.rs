//! 获取评论详情
//!
//! docPath: https://open.feishu.cn/document/task-v2/comment/get

use crate::common::api_utils::*;
use crate::task::task::v2::comment::models::GetCommentResponse;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

/// 获取评论详情请求。
#[derive(Debug, Clone)]
pub struct GetCommentRequest {
    config: Arc<Config>,
    comment_id: String,
    user_id_type: Option<String>,
}

impl GetCommentRequest {
    pub fn new(config: Arc<Config>, comment_id: impl Into<String>) -> Self {
        Self {
            config,
            comment_id: comment_id.into(),
            user_id_type: None,
        }
    }

    /// 设置用户 ID 类型。
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    pub async fn execute(self) -> SDKResult<GetCommentResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetCommentResponse> {
        validate_required!(self.comment_id.trim(), "评论 ID 不能为空");

        let path = format!("/open-apis/task/v2/comments/{}", self.comment_id);
        let mut request = ApiRequest::<GetCommentResponse>::get(&path);
        if let Some(user_id_type) = self.user_id_type {
            request = request.query("user_id_type", user_id_type);
        }

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "获取评论详情")
    }
}

impl ApiResponseTrait for GetCommentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_comment_url() {
        let path = format!("/open-apis/task/v2/comments/{}", "comment_123");
        assert_eq!(path, "/open-apis/task/v2/comments/comment_123");
    }

    #[test]
    fn test_get_comment_request_builder() {
        let config = Arc::new(openlark_core::config::Config::default());
        let request = GetCommentRequest::new(config, "comment_123").user_id_type("open_id");
        assert_eq!(request.comment_id, "comment_123");
        assert_eq!(request.user_id_type.as_deref(), Some("open_id"));
    }
}
