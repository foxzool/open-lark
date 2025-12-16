/// 更新回复的内容
///
/// 更新云文档中的某条回复。
/// docPath: https://open.feishu.cn/document/server-docs/docs/CommentAPI/update
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};


/// 更新回复请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateReplyRequest {
    /// 文件token
    pub file_token: String,
    /// 评论ID
    pub comment_id: String,
    /// 回复ID
    pub reply_id: String,
    /// 回复内容
    pub content: String,
}

impl UpdateReplyRequest {
    /// 创建更新回复请求
    ///
    /// # 参数
    /// * `file_token` - 文件token
    /// * `comment_id` - 评论ID
    /// * `reply_id` - 回复ID
    /// * `content` - 回复内容
    pub fn new(
        file_token: impl Into<String>,
        comment_id: impl Into<String>,
        reply_id: impl Into<String>,
        content: impl Into<String>,
    ) -> Self {
        Self {
            file_token: file_token.into(),
            comment_id: comment_id.into(),
            reply_id: reply_id.into(),
            content: content.into(),
        }
    }
}

/// 更新回复响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateReplyResponse {
    /// 回复信息
    pub reply: ReplyInfo,
}

/// 回复信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyInfo {
    /// 回复ID
    pub reply_id: String,
    /// 回复内容
    pub content: String,
    /// 创建者信息
    pub creator: CreatorInfo,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: String,
    /// 父评论ID
    pub parent_comment_id: String,
}

/// 创建者信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatorInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名称
    pub name: String,
    /// 用户类型
    pub user_type: String,
}

impl ApiResponseTrait for UpdateReplyResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新回复的内容
///
/// 更新云文档中的某条回复。
/// docPath: https://open.feishu.cn/document/server-docs/docs/CommentAPI/update
pub async fn update_reply(
    request: UpdateReplyRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<UpdateReplyResponse>> {
    // 构建请求体
    let body = serde_json::json!({
        "content": request.content
    });

    // 创建API请求
    let mut api_request: ApiRequest<UpdateReplyResponse> =
        ApiRequest::put(&format!("/open-apis/drive/v1/files/{}/comments/{}/replies/{}",
            request.file_token, request.comment_id, request.reply_id))
            .body(body);

    // 如果有请求选项，应用它们
    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    // 发送请求
    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_update_reply_request_builder() {
        let request = UpdateReplyRequest::new("file_token", "comment_123", "reply_456", "更新的回复内容");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.comment_id, "comment_123");
        assert_eq!(request.reply_id, "reply_456");
        assert_eq!(request.content, "更新的回复内容");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(UpdateReplyResponse::data_format(), ResponseFormat::Data);
    }
}