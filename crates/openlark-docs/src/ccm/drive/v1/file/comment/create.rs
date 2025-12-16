use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 添加全文评论
///
/// 为文件或文档添加全文评论
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/comment/create
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::DriveApi;

/// 添加评论请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCommentRequest {
    /// 文件token
    pub file_token: String,
    /// 评论内容
    pub content: String,
    /// 评论类型
    pub comment_type: Option<String>,
}

impl CreateCommentRequest {
    /// 创建添加评论请求
    ///
    /// # 参数
    /// * `file_token` - 文件token
    /// * `content` - 评论内容
    pub fn new(file_token: impl Into<String>, content: impl Into<String>) -> Self {
        Self {
            file_token: file_token.into(),
            content: content.into(),
            comment_type: None,
        }
    }

    /// 设置评论类型
    pub fn comment_type(mut self, comment_type: impl Into<String>) -> Self {
        self.comment_type = Some(comment_type.into());
        self
    }
}

/// 评论信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentInfo {
    /// 评论ID
    pub comment_id: String,
    /// 评论内容
    pub content: String,
    /// 评论类型
    pub comment_type: String,
    /// 创建者信息
    pub creator: CreatorInfo,
    /// 创建时间
    pub create_time: i64,
}

/// 创建者信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatorInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名称
    pub name: String,
}

/// 添加评论响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCommentResponse {
    /// 评论信息
    pub data: Option<CommentInfo>,
}

impl ApiResponseTrait for CreateCommentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 添加全文评论
///
/// 为文件或文档添加全文评论
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/comment/create
pub async fn create_comment(
    request: CreateCommentRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<CreateCommentResponse>> {
    // 使用DriveApi枚举生成API端点
    let api_endpoint = DriveApi::CreateComment(request.file_token.clone());

    // 创建API请求
    let mut api_request: ApiRequest<CreateCommentResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serde_json::json!({
            "content": request.content,
            "comment_type": request.comment_type
        }));

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
    fn test_create_comment_request_builder() {
        let request = CreateCommentRequest::new("file_token", "这是一条评论");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.content, "这是一条评论");
        assert!(request.comment_type.is_none());
    }

    #[test]
    fn test_create_comment_request_with_type() {
        let request = CreateCommentRequest::new("file_token", "这是一条评论")
            .comment_type("text");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.comment_type.unwrap(), "text");
    }

    #[test]
    fn test_comment_info_structure() {
        let creator = CreatorInfo {
            user_id: "user_id".to_string(),
            name: "用户名".to_string(),
        };

        let comment = CommentInfo {
            comment_id: "comment_id".to_string(),
            content: "评论内容".to_string(),
            comment_type: "text".to_string(),
            creator,
            create_time: 1640995200,
        };

        assert_eq!(comment.comment_id, "comment_id");
        assert_eq!(comment.content, "评论内容");
        assert_eq!(comment.creator.name, "用户名");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            CreateCommentResponse::data_format(),
            ResponseFormat::Data
        );
    }
}