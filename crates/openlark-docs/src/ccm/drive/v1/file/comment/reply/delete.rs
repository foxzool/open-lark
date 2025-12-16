/// 删除回复
///
/// 该接口用于删除云文档中的某条评论的回复信息。
/// docPath: https://open.feishu.cn/document/server-docs/docs/Comment/delete-2
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCommentReplyRequest {
    pub file_token: String,
    pub comment_id: String,
    pub reply_id: String,
    pub file_type: String,
}

impl DeleteCommentReplyRequest {
    pub fn new(
        file_token: impl Into<String>, 
        comment_id: impl Into<String>, 
        reply_id: impl Into<String>,
        file_type: impl Into<String>
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
) -> SDKResult<Response<DeleteCommentReplyResponse>> {
    let url = format!(
        "/open-apis/drive/v1/files/{}/comments/{}/replies/{}",
        request.file_token, request.comment_id, request.reply_id
    );
    let mut api_request: ApiRequest<DeleteCommentReplyResponse> = ApiRequest::delete(&url);
    
    api_request = api_request.query_param("file_type", &request.file_type);

    Transport::request(api_request, config, None).await
}
