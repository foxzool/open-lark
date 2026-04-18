//! 删除评论
//!
//! docPath: https://open.feishu.cn/document/task-v2/comment/delete

use crate::common::api_utils::*;
use crate::v2::comment::models::DeleteCommentResponse;
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct DeleteCommentRequest {
    config: Arc<Config>,
    comment_id: String,
}

impl DeleteCommentRequest {
    pub fn new(config: Arc<Config>, comment_id: impl Into<String>) -> Self {
        Self {
            config,
            comment_id: comment_id.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<DeleteCommentResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<DeleteCommentResponse> {
        validate_required!(self.comment_id.trim(), "评论 ID 不能为空");

        let path = format!("/open-apis/task/v2/comments/{}", self.comment_id);
        let request = ApiRequest::<DeleteCommentResponse>::delete(&path);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "删除评论")
    }
}

impl ApiResponseTrait for DeleteCommentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
