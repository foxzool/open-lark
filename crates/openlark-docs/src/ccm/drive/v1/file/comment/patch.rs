/// 解决/恢复评论
///
/// 解决或恢复云文档中的评论。
/// docPath: https://open.feishu.cn/document/server-docs/docs/CommentAPI/patch
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 解决/恢复评论请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchCommentRequest {
    /// 文件token
    pub file_token: String,
    /// 评论ID
    pub comment_id: String,
    /// 操作类型
    pub action: String,
}

impl PatchCommentRequest {
    /// 创建解决/恢复评论请求
    ///
    /// # 参数
    /// * `file_token` - 文件token
    /// * `comment_id` - 评论ID
    /// * `action` - 操作类型，可选值：resolve（解决）、recover（恢复）
    pub fn new(
        file_token: impl Into<String>,
        comment_id: impl Into<String>,
        action: impl Into<String>,
    ) -> Self {
        Self {
            file_token: file_token.into(),
            comment_id: comment_id.into(),
            action: action.into(),
        }
    }
}

/// 解决/恢复评论响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchCommentResponse {
    /// 评论信息
    pub comment: PatchCommentInfo,
}

/// 评论信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchCommentInfo {
    /// 评论ID
    pub comment_id: String,
    /// 评论内容
    pub content: String,
    /// 评论类型
    pub comment_type: String,
    /// 评论状态
    pub status: String,
    /// 创建者信息
    pub creator: PatchCreatorInfo,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: String,
    /// 父评论ID（回复评论时）
    pub parent_comment_id: Option<String>,
    /// 回复评论数量
    pub reply_count: i32,
}

/// 创建者信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchCreatorInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名称
    pub name: String,
    /// 用户类型
    pub user_type: String,
}

impl ApiResponseTrait for PatchCommentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 解决/恢复评论
///
/// 解决或恢复云文档中的评论。
/// docPath: https://open.feishu.cn/document/server-docs/docs/CommentAPI/patch
pub async fn patch_comment(
    request: PatchCommentRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<PatchCommentResponse>> {
    // 构建请求体
    let body = serde_json::json!({
        "action": request.action
    });

    // 创建API请求
    let mut api_request: ApiRequest<PatchCommentResponse> = ApiRequest::patch(&format!(
        "/open-apis/drive/v1/files/{}/comments/{}",
        request.file_token, request.comment_id
    ))
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
    fn test_patch_comment_request_builder() {
        let request = PatchCommentRequest::new("file_token", "comment_123", "resolve");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.comment_id, "comment_123");
        assert_eq!(request.action, "resolve");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(PatchCommentResponse::data_format(), ResponseFormat::Data);
    }
}
