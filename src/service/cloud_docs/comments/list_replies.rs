use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
};

use super::list::Reply;

/// 获取回复信息请求
///
/// 用于获取指定评论的回复列表，支持分页查询。
///
/// # 示例
/// ```
/// use open_lark::service::cloud_docs::comments::list_replies::ListRepliesRequest;
///
/// let request = ListRepliesRequest::builder()
///     .file_token("doccnxxxxxx")
///     .with_doc_type()
///     .comment_id("comment_123")
///     .page_size(20)
///     .with_open_id()
///     .build();
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct ListRepliesRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文档token
    #[serde(skip)]
    file_token: String,
    /// 文档类型：doc、docx、sheet、bitable
    #[serde(skip)]
    file_type: String,
    /// 评论ID
    #[serde(skip)]
    comment_id: String,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    page_token: Option<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id_type: Option<String>,
}

impl ListRepliesRequest {
    /// 创建新的获取回复列表请求
    ///
    /// # 参数
    /// - `file_token`: 文档标识符
    /// - `file_type`: 文档类型
    /// - `comment_id`: 评论ID
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::list_replies::ListRepliesRequest;
    ///
    /// let request = ListRepliesRequest::new("doccnxxxxxx", "doc", "comment_123");
    /// ```
    pub fn new(
        file_token: impl ToString,
        file_type: impl ToString,
        comment_id: impl ToString,
    ) -> Self {
        let mut api_request = ApiRequest::new();
        api_request.api_path = COMMENT_V1_REPLIES.to_string();

        Self {
            api_request,
            file_token: file_token.to_string(),
            file_type: file_type.to_string(),
            comment_id: comment_id.to_string(),
            page_size: None,
            page_token: None,
            user_id_type: None,
        }
    }

    /// 创建构建器实例
    pub fn builder() -> ListRepliesRequestBuilder {
        ListRepliesRequestBuilder::default()
    }
}

/// 获取回复列表请求构建器
///
/// 提供流式API来构建ListRepliesRequest，支持链式调用。
#[derive(Debug, Clone, Default)]
pub struct ListRepliesRequestBuilder {
    file_token: Option<String>,
    file_type: Option<String>,
    comment_id: Option<String>,
    page_size: Option<i32>,
    page_token: Option<String>,
    user_id_type: Option<String>,
}

impl ListRepliesRequestBuilder {
    /// 设置文档token
    pub fn file_token(mut self, file_token: impl ToString) -> Self {
        self.file_token = Some(file_token.to_string());
        self
    }

    /// 设置文档类型为doc
    pub fn with_doc_type(self) -> Self {
        self.file_type("doc")
    }

    /// 设置文档类型为docx
    pub fn with_docx_type(self) -> Self {
        self.file_type("docx")
    }

    /// 设置文档类型为sheet
    pub fn with_sheet_type(self) -> Self {
        self.file_type("sheet")
    }

    /// 设置文档类型为bitable
    pub fn with_bitable_type(self) -> Self {
        self.file_type("bitable")
    }

    /// 设置文档类型
    pub fn file_type(mut self, file_type: impl ToString) -> Self {
        self.file_type = Some(file_type.to_string());
        self
    }

    /// 设置评论ID
    pub fn comment_id(mut self, comment_id: impl ToString) -> Self {
        self.comment_id = Some(comment_id.to_string());
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        self.page_token = Some(page_token.to_string());
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl ToString) -> Self {
        self.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 使用OpenID作为用户ID类型
    pub fn with_open_id(self) -> Self {
        self.user_id_type("open_id")
    }

    /// 使用UserID作为用户ID类型
    pub fn with_user_id(self) -> Self {
        self.user_id_type("user_id")
    }

    /// 使用UnionID作为用户ID类型
    pub fn with_union_id(self) -> Self {
        self.user_id_type("union_id")
    }

    /// 构建ListRepliesRequest实例
    pub fn build(self) -> ListRepliesRequest {
        let file_token = self.file_token.expect("file_token is required");
        let file_type = self.file_type.expect("file_type is required");
        let comment_id = self.comment_id.expect("comment_id is required");

        let mut request = ListRepliesRequest::new(&file_token, &file_type, &comment_id);
        request.page_size = self.page_size;
        request.page_token = self.page_token;
        request.user_id_type = self.user_id_type;

        request
    }
}

// 应用ExecutableBuilder trait到ListRepliesRequestBuilder
impl_executable_builder_owned!(
    ListRepliesRequestBuilder,
    super::CommentsService,
    ListRepliesRequest,
    crate::core::api_resp::BaseResponse<ListRepliesResponse>,
    list_replies,
);

/// 获取回复信息响应
///
/// 包含评论回复列表信息和分页信息
#[derive(Debug, Clone, Deserialize)]
pub struct ListRepliesResponse {
    /// 回复列表
    pub items: Vec<Reply>,
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
}

impl ListRepliesResponse {
    /// 创建新的回复列表响应
    pub fn new(items: Vec<Reply>, has_more: bool, page_token: Option<String>) -> Self {
        Self {
            items,
            has_more,
            page_token,
        }
    }

    /// 获取回复数量
    pub fn reply_count(&self) -> usize {
        self.items.len()
    }

    /// 检查是否有更多回复
    pub fn has_more_replies(&self) -> bool {
        self.has_more
    }

    /// 获取下一页分页标记
    pub fn next_page_token(&self) -> Option<&String> {
        self.page_token.as_ref()
    }

    /// 获取第一个回复
    pub fn first_reply(&self) -> Option<&Reply> {
        self.items.first()
    }

    /// 获取最后一个回复
    pub fn last_reply(&self) -> Option<&Reply> {
        self.items.last()
    }

    /// 按创建时间排序回复（最新的在前）
    pub fn sorted_by_create_time_desc(&self) -> Vec<&Reply> {
        let mut replies: Vec<&Reply> = self.items.iter().collect();
        replies.sort_by(|a, b| b.create_time.cmp(&a.create_time));
        replies
    }

    /// 按创建时间排序回复（最旧的在前）
    pub fn sorted_by_create_time_asc(&self) -> Vec<&Reply> {
        let mut replies: Vec<&Reply> = self.items.iter().collect();
        replies.sort_by(|a, b| a.create_time.cmp(&b.create_time));
        replies
    }

    /// 获取指定用户创建的回复
    pub fn replies_by_user(&self, user_id: &str) -> Vec<&Reply> {
        self.items
            .iter()
            .filter(|reply| reply.user_id == user_id)
            .collect()
    }

    /// 获取包含指定文本的回复
    pub fn replies_containing_text(&self, text: &str) -> Vec<&Reply> {
        self.items
            .iter()
            .filter(|reply| reply.contains_text(text))
            .collect()
    }
}

impl ApiResponseTrait for ListRepliesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取回复信息
///
/// 获取指定云文档评论的回复列表。
///
/// # API文档
/// <https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/cloud_docs-v1/comment/list_replies>
///
/// # 参数
/// - `request`: 获取回复列表请求
/// - `config`: SDK配置
/// - `option`: 可选请求参数
///
/// # 返回
/// 包含回复列表信息的响应
pub async fn list_replies(
    request: ListRepliesRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<crate::core::api_resp::BaseResponse<ListRepliesResponse>> {
    let mut api_req = request.api_request;
    api_req.set_http_method(Method::GET);
    api_req.api_path = format!(
        "{}?file_type={}&file_token={}&comment_id={}",
        COMMENT_V1_REPLIES, request.file_type, request.file_token, request.comment_id,
    );

    // 构建查询参数
    let mut query_params = Vec::new();

    if let Some(page_size) = request.page_size {
        query_params.push(format!("page_size={page_size}"));
    }
    if let Some(page_token) = request.page_token {
        query_params.push(format!("page_token={page_token}"));
    }
    if let Some(user_id_type) = request.user_id_type {
        query_params.push(format!("user_id_type={user_id_type}"));
    }

    if !query_params.is_empty() {
        api_req.api_path = format!("{}&{}", api_req.api_path, query_params.join("&"));
    }

    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use crate::service::cloud_docs::comments::list::{ReplyContent, ContentElement, TextRun};

    #[test]
    fn test_list_replies_request_builder() {
        let request = ListRepliesRequest::builder()
            .file_token("doccnxxxxxx")
            .with_doc_type()
            .comment_id("comment_123")
            .page_size(20)
            .with_open_id()
            .build();

        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
        assert_eq!(request.comment_id, "comment_123");
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_list_replies_request_new() {
        let request = ListRepliesRequest::new("doccnxxxxxx", "doc", "comment_123");

        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
        assert_eq!(request.comment_id, "comment_123");
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
        assert_eq!(request.user_id_type, None);
    }

    #[test]
    fn test_all_file_types() {
        let doc_request = ListRepliesRequest::builder()
            .file_token("doc_token")
            .with_doc_type()
            .comment_id("comment_id")
            .build();

        let docx_request = ListRepliesRequest::builder()
            .file_token("docx_token")
            .with_docx_type()
            .comment_id("comment_id")
            .build();

        assert_eq!(doc_request.file_type, "doc");
        assert_eq!(docx_request.file_type, "docx");
    }

    #[test]
    fn test_all_user_id_types() {
        let open_id_request = ListRepliesRequest::builder()
            .file_token("token")
            .with_doc_type()
            .comment_id("comment_id")
            .with_open_id()
            .build();

        let user_id_request = ListRepliesRequest::builder()
            .file_token("token")
            .with_doc_type()
            .comment_id("comment_id")
            .with_user_id()
            .build();

        assert_eq!(open_id_request.user_id_type, Some("open_id".to_string()));
        assert_eq!(user_id_request.user_id_type, Some("user_id".to_string()));
    }

    #[test]
    fn test_list_replies_response() {
        let reply_content = ReplyContent {
            elements: vec![ContentElement::text_run(TextRun::new("测试回复"))],
        };

        let reply1 = Reply::new("reply1", "user1", 1000, 1001, reply_content.clone());
        let reply2 = Reply::new("reply2", "user2", 1002, 1003, reply_content);

        let response = ListRepliesResponse::new(
            vec![reply1, reply2],
            true,
            Some("next_token".to_string()),
        );

        assert_eq!(response.reply_count(), 2);
        assert!(response.has_more_replies());
        assert_eq!(response.next_page_token(), Some(&"next_token".to_string()));
        assert_eq!(response.first_reply().unwrap().reply_id, "reply1");
        assert_eq!(response.last_reply().unwrap().reply_id, "reply2");
    }

    #[test]
    fn test_response_methods() {
        let reply_content1 = ReplyContent {
            elements: vec![ContentElement::text_run(TextRun::new("重要内容"))],
        };
        let reply_content2 = ReplyContent {
            elements: vec![ContentElement::text_run(TextRun::new("普通内容"))],
        };

        let reply1 = Reply::new("reply1", "user1", 1000, 1001, reply_content1);
        let reply2 = Reply::new("reply2", "user1", 1002, 1003, reply_content2);
        let reply3 = Reply::new("reply3", "user2", 1004, 1005, reply_content1);

        let response = ListRepliesResponse::new(vec![reply1, reply2, reply3], false, None);

        // 测试按用户筛选
        let user1_replies = response.replies_by_user("user1");
        assert_eq!(user1_replies.len(), 2);

        // 测试按文本内容筛选
        let important_replies = response.replies_containing_text("重要");
        assert_eq!(important_replies.len(), 2);

        // 测试排序
        let desc_sorted = response.sorted_by_create_time_desc();
        assert_eq!(desc_sorted[0].reply_id, "reply3");
        assert_eq!(desc_sorted[1].reply_id, "reply2");
        assert_eq!(desc_sorted[2].reply_id, "reply1");

        let asc_sorted = response.sorted_by_create_time_asc();
        assert_eq!(asc_sorted[0].reply_id, "reply1");
        assert_eq!(asc_sorted[1].reply_id, "reply2");
        assert_eq!(asc_sorted[2].reply_id, "reply3");
    }
}