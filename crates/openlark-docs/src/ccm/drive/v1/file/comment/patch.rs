/// 解决/恢复评论
///
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-comment/patch
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 解决/恢复评论请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchCommentRequest {
    /// 文件 token
    pub file_token: String,
    /// 评论 ID
    pub comment_id: String,
    /// 评论解决标志：true=解决，false=恢复
    pub is_solved: bool,
    /// 文件类型（必填）
    pub file_type: String,
    /// 用户 ID 类型（默认 open_id）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

impl PatchCommentRequest {
    pub fn new(
        file_token: impl Into<String>,
        comment_id: impl Into<String>,
        is_solved: bool,
        file_type: impl Into<String>,
    ) -> Self {
        Self {
            file_token: file_token.into(),
            comment_id: comment_id.into(),
            is_solved,
            file_type: file_type.into(),
            user_id_type: None,
        }
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }
}

#[derive(Debug, Serialize)]
struct PatchCommentRequestBody {
    is_solved: bool,
}

/// 解决/恢复评论响应（data 为 {}）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchCommentResponse {}

impl ApiResponseTrait for PatchCommentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 解决/恢复评论
pub async fn patch_comment(
    request: PatchCommentRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<PatchCommentResponse> {
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
    super::validate_comment_file_type_for_list_like(&request.file_type)?;
    let api_endpoint = DriveApi::PatchComment(request.file_token.clone(), request.comment_id.clone());

    let mut api_request: ApiRequest<PatchCommentResponse> =
        ApiRequest::patch(&api_endpoint.to_url()).body(serialize_params(
            &PatchCommentRequestBody {
                is_solved: request.is_solved,
            },
            "解决/恢复评论",
        )?);

    api_request = api_request.query("file_type", &request.file_type);
    if let Some(user_id_type) = &request.user_id_type {
        api_request = api_request.query("user_id_type", user_id_type);
    }

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "解决/恢复评论")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patch_comment_request_builder() {
        let request = PatchCommentRequest::new("file_token", "comment_123", true, "docx");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.comment_id, "comment_123");
        assert_eq!(request.is_solved, true);
        assert_eq!(request.file_type, "docx");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(PatchCommentResponse::data_format(), ResponseFormat::Data);
    }
}
