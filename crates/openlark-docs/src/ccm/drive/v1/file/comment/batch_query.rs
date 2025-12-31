/// 批量获取评论
///
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-comment/batch_query
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::validation_error,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

use super::models::Comment;

/// 批量获取评论请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchQueryCommentRequest {
    /// 文件 token
    pub file_token: String,
    /// 文件类型（必填）
    pub file_type: String,
    /// 评论 ID 列表
    pub comment_ids: Vec<String>,
    /// 用户 ID 类型（默认 open_id）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl BatchQueryCommentRequest {
    pub fn new(
        file_token: impl Into<String>,
        file_type: impl Into<String>,
        comment_ids: Vec<String>,
    ) -> Self {
        Self {
            file_token: file_token.into(),
            file_type: file_type.into(),
            comment_ids,
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }
}

#[derive(Debug, Serialize)]
struct BatchQueryCommentRequestBody {
    comment_ids: Vec<String>,
}

/// 批量获取评论响应（data）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchQueryCommentResponse {
    #[serde(default)]
    pub items: Vec<Comment>,
}

impl ApiResponseTrait for BatchQueryCommentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量获取评论
pub async fn batch_query_comment(
    request: BatchQueryCommentRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<BatchQueryCommentResponse> {
    if request.file_token.trim().is_empty() {
        return Err(validation_error("file_token", "file_token 不能为空"));
    }
    if request.file_type.trim().is_empty() {
        return Err(validation_error("file_type", "file_type 不能为空"));
    }
    super::validate_comment_file_type_for_list_like(&request.file_type)?;
    if request.comment_ids.is_empty() {
        return Err(validation_error("comment_ids", "comment_ids 不能为空"));
    }

    let api_endpoint = DriveApi::BatchQueryComments(request.file_token.clone());

    let mut api_request: ApiRequest<BatchQueryCommentResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(
            &BatchQueryCommentRequestBody {
                comment_ids: request.comment_ids,
            },
            "批量获取评论",
        )?);

    api_request = api_request.query("file_type", &request.file_type);
    if let Some(user_id_type) = &request.user_id_type {
        api_request = api_request.query("user_id_type", user_id_type);
    }

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "批量获取评论")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_query_comment_request_builder() {
        let comment_ids = vec!["comment_1".to_string(), "comment_2".to_string()];
        let request = BatchQueryCommentRequest::new("file_token", "docx", comment_ids);

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.file_type, "docx");
        assert_eq!(request.comment_ids.len(), 2);
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            BatchQueryCommentResponse::data_format(),
            ResponseFormat::Data
        );
    }
}
