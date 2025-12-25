/// 删除回复
///
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file-comment-reply/delete
/// doc: https://open.feishu.cn/document/server-docs/docs/CommentAPI/delete
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::validation_error,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCommentReplyRequest {
    pub file_token: String,
    pub comment_id: String,
    pub reply_id: String,
    /// 云文档类型（必填）
    pub file_type: String,
}

impl DeleteCommentReplyRequest {
    pub fn new(
        file_token: impl Into<String>,
        comment_id: impl Into<String>,
        reply_id: impl Into<String>,
        file_type: impl Into<String>,
    ) -> Self {
        Self {
            file_token: file_token.into(),
            comment_id: comment_id.into(),
            reply_id: reply_id.into(),
            file_type: file_type.into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCommentReplyResponse {}

impl ApiResponseTrait for DeleteCommentReplyResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

pub async fn delete_comment_reply(
    request: DeleteCommentReplyRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<DeleteCommentReplyResponse> {
    if request.file_token.trim().is_empty() {
        return Err(validation_error("file_token", "file_token 不能为空"));
    }
    if request.comment_id.trim().is_empty() {
        return Err(validation_error("comment_id", "comment_id 不能为空"));
    }
    if request.reply_id.trim().is_empty() {
        return Err(validation_error("reply_id", "reply_id 不能为空"));
    }
    if request.file_type.trim().is_empty() {
        return Err(validation_error("file_type", "file_type 不能为空"));
    }

    let api_endpoint = DriveApi::DeleteCommentReply(
        request.file_token.clone(),
        request.comment_id.clone(),
        request.reply_id.clone(),
    );
    let mut api_request: ApiRequest<DeleteCommentReplyResponse> =
        ApiRequest::delete(&api_endpoint.to_url());

    api_request = api_request.query("file_type", &request.file_type);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "删除回复")
}
