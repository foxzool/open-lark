//! 创建评论
//!
//! docPath: https://open.feishu.cn/document/task-v2/comment/create

use crate::common::api_utils::*;
use crate::v2::comment::models::{CreateCommentBody, CreateCommentResponse};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    validate_required, SDKResult,
};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct CreateCommentRequest {
    config: Arc<Config>,
    body: CreateCommentBody,
}

impl CreateCommentRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            body: CreateCommentBody::default(),
        }
    }

    pub fn content(mut self, content: impl Into<String>) -> Self {
        self.body.content = content.into();
        self
    }

    pub async fn execute(self) -> SDKResult<CreateCommentResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<CreateCommentResponse> {
        validate_required!(self.body.content.trim(), "评论内容不能为空");

        let request = ApiRequest::<CreateCommentResponse>::post("/open-apis/task/v2/comments")
            .body(serialize_params(&self.body, "创建评论")?);

        let response =
            openlark_core::http::Transport::request(request, &self.config, Some(option)).await?;
        extract_response_data(response, "创建评论")
    }
}

impl ApiResponseTrait for CreateCommentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}
