use super::models::ReplyInfo;
/// 获取回复列表
///
/// 该接口用于获取云文档中的某条评论的回复信息。
/// docPath: https://open.feishu.cn/document/server-docs/docs/Comment/list-2
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCommentReplyRequest {
    pub file_token: String,
    pub comment_id: String,
    pub file_type: String,
    pub page_token: Option<String>,
    pub page_size: Option<i32>,
    pub user_id_type: Option<String>,
}

impl ListCommentReplyRequest {
    pub fn new(
        file_token: impl Into<String>,
        comment_id: impl Into<String>,
        file_type: impl Into<String>,
    ) -> Self {
        Self {
            file_token: file_token.into(),
            comment_id: comment_id.into(),
            file_type: file_type.into(),
            page_token: None,
            page_size: None,
            user_id_type: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCommentReplyResponse {
    pub items: Vec<ReplyInfo>,
    pub page_token: Option<String>,
    pub has_more: bool,
}

impl ApiResponseTrait for ListCommentReplyResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

pub async fn list_comment_reply(
    request: ListCommentReplyRequest,
    config: &Config,
) -> SDKResult<Response<ListCommentReplyResponse>> {
    let url = format!(
        "/open-apis/drive/v1/files/{}/comments/{}/replies",
        request.file_token, request.comment_id
    );
    let mut api_request: ApiRequest<ListCommentReplyResponse> = ApiRequest::get(&url);

    api_request = api_request.query_param("file_type", &request.file_type);

    if let Some(token) = request.page_token {
        api_request = api_request.query_param("page_token", &token);
    }
    if let Some(size) = request.page_size {
        api_request = api_request.query_param("page_size", &size.to_string());
    }
    if let Some(user_type) = request.user_id_type {
        api_request = api_request.query_param("user_id_type", &user_type);
    }

    Transport::request(api_request, config, None).await
}
