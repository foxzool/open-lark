/// 获取全文评论
///
/// 获取云文档中的某条评论。
/// docPath: https://open.feishu.cn/document/server-docs/docs/CommentAPI/get
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取全文评论请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCommentRequest {
    /// 文件token
    pub file_token: String,
    /// 评论ID
    pub comment_id: String,
}

impl GetCommentRequest {
    /// 创建获取全文评论请求
    ///
    /// # 参数
    /// * `file_token` - 文件token
    /// * `comment_id` - 评论ID
    pub fn new(file_token: impl Into<String>, comment_id: impl Into<String>) -> Self {
        Self {
            file_token: file_token.into(),
            comment_id: comment_id.into(),
        }
    }
}

/// 获取全文评论响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCommentResponse {
    /// 评论信息
    pub comment: GetCommentInfo,
}

/// 评论信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCommentInfo {
    /// 评论ID
    pub comment_id: String,
    /// 评论内容
    pub content: String,
    /// 评论类型
    pub comment_type: String,
    /// 评论状态
    pub status: String,
    /// 创建者信息
    pub creator: GetCreatorInfo,
    /// 创建时间
    pub create_time: String,
    /// 更新时间
    pub update_time: String,
    /// 父评论ID（回复评论时）
    pub parent_comment_id: Option<String>,
    /// 回复评论数量
    pub reply_count: i32,
    /// 回复列表
    pub replies: Option<Vec<GetCommentInfo>>,
}

/// 创建者信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCreatorInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名称
    pub name: String,
    /// 用户类型
    pub user_type: String,
}

impl ApiResponseTrait for GetCommentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取全文评论
///
/// 获取云文档中的某条评论。
/// docPath: https://open.feishu.cn/document/server-docs/docs/CommentAPI/get
pub async fn get_comment(
    request: GetCommentRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<GetCommentResponse>> {
    // 创建API请求
    let mut api_request: ApiRequest<GetCommentResponse> = ApiRequest::get(&format!(
        "/open-apis/drive/v1/files/{}/comments/{}",
        request.file_token, request.comment_id
    ));

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
    fn test_get_comment_request_builder() {
        let request = GetCommentRequest::new("file_token", "comment_123");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.comment_id, "comment_123");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(GetCommentResponse::data_format(), ResponseFormat::Data);
    }
}
