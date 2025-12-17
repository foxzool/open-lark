use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 获取云文档所有评论
///
/// 获取文件或文档的所有评论列表
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/comment/list
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::DriveApi;

/// 获取评论列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCommentsRequest {
    /// 文件token
    pub file_token: String,
    /// 页码，从0开始
    pub page_size: Option<i32>,
    /// 分页标记
    pub page_token: Option<String>,
}

impl ListCommentsRequest {
    /// 创建获取评论列表请求
    ///
    /// # 参数
    /// * `file_token` - 文件token
    pub fn new(file_token: impl Into<String>) -> Self {
        Self {
            file_token: file_token.into(),
            page_size: None,
            page_token: None,
        }
    }

    /// 设置页面大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }
}

/// 评论回复信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentReply {
    /// 回复ID
    pub reply_id: String,
    /// 回复内容
    pub content: String,
    /// 回复者信息
    pub author: AuthorInfo,
    /// 创建时间
    pub create_time: i64,
}

/// 作者信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthorInfo {
    /// 用户ID
    pub user_id: String,
    /// 用户名称
    pub name: String,
}

/// 评论信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCommentInfo {
    /// 评论ID
    pub comment_id: String,
    /// 评论内容
    pub content: String,
    /// 评论类型
    pub comment_type: String,
    /// 作者信息
    pub author: AuthorInfo,
    /// 回复列表
    pub replies: Option<Vec<CommentReply>>,
    /// 创建时间
    pub create_time: i64,
}

/// 获取评论列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListCommentsResponse {
    /// 评论列表数据
    pub data: Option<CommentsData>,
}

/// 评论数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentsData {
    /// 评论列表
    pub items: Option<Vec<ListCommentInfo>>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 是否有更多
    pub has_more: Option<bool>,
}

impl ApiResponseTrait for ListCommentsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取云文档所有评论
///
/// 获取文件或文档的所有评论列表
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/comment/list
pub async fn list_comments(
    request: ListCommentsRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<ListCommentsResponse>> {
    // 使用DriveApi枚举生成API端点
    let api_endpoint = DriveApi::ListFileComments(request.file_token.clone());

    // 创建API请求
    let mut api_request: ApiRequest<ListCommentsResponse> =
        ApiRequest::get(&api_endpoint.to_url());

    // 添加查询参数
    if let Some(page_size) = request.page_size {
        api_request = api_request.query("page_size", &page_size.to_string());
    }
    if let Some(page_token) = &request.page_token {
        api_request = api_request.query("page_token", page_token);
    }

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
    fn test_list_comments_request_builder() {
        let request = ListCommentsRequest::new("file_token");

        assert_eq!(request.file_token, "file_token");
        assert!(request.page_size.is_none());
    }

    #[test]
    fn test_list_comments_request_with_params() {
        let request = ListCommentsRequest::new("file_token")
            .page_size(20)
            .page_token("next_page_token");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.page_size.unwrap(), 20);
        assert_eq!(request.page_token.unwrap(), "next_page_token");
    }

    #[test]
    fn test_comment_info_structure() {
        let author = AuthorInfo {
            user_id: "user_id".to_string(),
            name: "用户名".to_string(),
        };

        let comment = ListCommentInfo {
            comment_id: "comment_id".to_string(),
            content: "评论内容".to_string(),
            comment_type: "text".to_string(),
            author,
            replies: None,
            create_time: 1640995200,
        };

        assert_eq!(comment.comment_id, "comment_id");
        assert_eq!(comment.content, "评论内容");
        assert_eq!(comment.author.name, "用户名");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            ListCommentsResponse::data_format(),
            ResponseFormat::Data
        );
    }
}
