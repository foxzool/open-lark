//! 获取全文评论

//!

//! docPath: https://open.feishu.cn/document/server-docs/docs/CommentAPI/get

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

use super::models::Comment;

/// 获取全文评论请求

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct GetCommentRequest {
    /// 文件 token
    pub file_token: String,

    /// 评论 ID
    pub comment_id: String,

    /// 文件类型（必填）
    pub file_type: String,

    /// 用户 ID 类型（默认 open_id）

    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl GetCommentRequest {
    pub fn new(
        file_token: impl Into<String>,

        comment_id: impl Into<String>,

        file_type: impl Into<String>,
    ) -> Self {
        Self {
            file_token: file_token.into(),

            comment_id: comment_id.into(),

            file_type: file_type.into(),

            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());

        self
    }
}

pub type GetCommentResponse = Comment;

/// 获取全文评论

pub async fn get_comment(
    request: GetCommentRequest,

    config: &Config,

    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<GetCommentResponse> {
    if request.file_token.trim().is_empty() {
        return Err(openlark_core::error::validation_error(
            "file_token",
            "file_token 不能为空",
        ));
    }

    if request.comment_id.trim().is_empty() {
        return Err(openlark_core::error::validation_error(
            "comment_id",
            "comment_id 不能为空",
        ));
    }

    if request.file_type.trim().is_empty() {
        return Err(openlark_core::error::validation_error(
            "file_type",
            "file_type 不能为空",
        ));
    }

    super::validate_comment_file_type_for_get(&request.file_type)?;

    let api_endpoint = DriveApi::GetComment(request.file_token.clone(), request.comment_id.clone());

    let mut api_request: ApiRequest<GetCommentResponse> = ApiRequest::get(&api_endpoint.to_url());

    api_request = api_request.query("file_type", &request.file_type);

    if let Some(user_id_type) = &request.user_id_type {
        api_request = api_request.query("user_id_type", user_id_type);
    }

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, Some(option)).await?;

    extract_response_data(response, "获取全文评论")
}

#[cfg(test)]

mod tests {

    use super::*;

    #[test]

    fn test_get_comment_request_builder() {
        let request = GetCommentRequest::new("file_token", "comment_123", "docx");

        assert_eq!(request.file_token, "file_token");

        assert_eq!(request.comment_id, "comment_123");

        assert_eq!(request.file_type, "docx");
    }
}
