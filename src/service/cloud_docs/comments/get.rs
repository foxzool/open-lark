use SDKResult;use reqwest::Method;
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

use super::list::Comment;

/// 获取全文评论请求
///
/// 用于获取指定云文档中的单个评论的详细信息。
/// 通过评论ID来精确获取特定评论的完整信息，包括：
/// - 评论基本信息（ID、创建者、时间戳等）
/// - 评论状态（是否已解决）
/// - 回复列表
/// - 引用内容
/// - 分页信息
///
/// # 示例
/// ```
/// use open_lark::service::cloud_docs::comments::get::GetCommentRequest;
///
/// let request = GetCommentRequest::builder()
///     .file_token("doccnxxxxxx")
///     .with_doc_type()
///     .comment_id("comment_123")
///     .with_open_id()
///     .build();
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct GetCommentRequest {
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
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id_type: Option<String>,
}

impl GetCommentRequest {
    /// 创建新的获取评论请求
    ///
    /// # 参数
    /// - `file_token`: 文档标识符
    /// - `file_type`: 文档类型
    /// - `comment_id`: 评论ID
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::get::GetCommentRequest;
    ///
    /// let request = GetCommentRequest::new("doccnxxxxxx", "doc", "comment_123");
    /// ```
    pub fn new(
        file_token: impl ToString,
        file_type: impl ToString,
        comment_id: impl ToString,
    ) -> Self {
        let mut api_request = ApiRequest::new();
        api_request.api_path = COMMENT_V1_COMMENT_GET.to_string();

        Self {
            api_request,
            file_token: file_token.to_string(),
            file_type: file_type.to_string(),
            comment_id: comment_id.to_string(),
            user_id_type: None,
        }
    }

    /// 创建构建器实例
    pub fn builder() -> GetCommentRequestBuilder {
        GetCommentRequestBuilder::default()
    }
}

/// 获取评论请求构建器
///
/// 提供流式API来构建GetCommentRequest，支持链式调用。
/// 可以方便地设置文档信息、评论ID和用户ID类型。
#[derive(Debug, Clone, Default)]
pub struct GetCommentRequestBuilder {
    file_token: Option<String>,
    file_type: Option<String>,
    comment_id: Option<String>,
    user_id_type: Option<String>,
}

impl GetCommentRequestBuilder {
    /// 设置文档token
    ///
    /// # 参数
    /// - `file_token`: 文档标识符
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::get::GetCommentRequest;
    ///
    /// let builder = GetCommentRequest::builder()
    ///     .file_token("doccnxxxxxx");
    /// ```
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
    ///
    /// # 参数
    /// - `file_type`: 文档类型（doc、docx、sheet、bitable）
    pub fn file_type(mut self, file_type: impl ToString) -> Self {
        self.file_type = Some(file_type.to_string());
        self
    }

    /// 设置评论ID
    ///
    /// # 参数
    /// - `comment_id`: 评论标识符
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::get::GetCommentRequest;
    ///
    /// let builder = GetCommentRequest::builder()
    ///     .comment_id("comment_123");
    /// ```
    pub fn comment_id(mut self, comment_id: impl ToString) -> Self {
        self.comment_id = Some(comment_id.to_string());
        self
    }

    /// 设置用户ID类型
    ///
    /// # 参数
    /// - `user_id_type`: 用户ID类型
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::get::GetCommentRequest;
    ///
    /// let builder = GetCommentRequest::builder()
    ///     .user_id_type("open_id");
    /// ```
    pub fn user_id_type(mut self, user_id_type: impl ToString) -> Self {
        self.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 使用OpenID作为用户ID类型
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::get::GetCommentRequest;
    ///
    /// let builder = GetCommentRequest::builder()
    ///     .with_open_id();
    /// ```
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

    /// 构建GetCommentRequest实例
    ///
    /// # 返回
    /// 返回配置好的GetCommentRequest实例
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::get::GetCommentRequest;
    ///
    /// let request = GetCommentRequest::builder()
    ///     .file_token("doccnxxxxxx")
    ///     .with_doc_type()
    ///     .comment_id("comment_123")
    ///     .with_open_id()
    ///     .build();
    /// ```
    pub fn build(self) -> GetCommentRequest {
        let file_token = self.file_token.expect("file_token is required");
        let file_type = self.file_type.expect("file_type is required");
        let comment_id = self.comment_id.expect("comment_id is required");

        let mut request = GetCommentRequest::new(&file_token, &file_type, &comment_id);
        request.user_id_type = self.user_id_type;

        request
    }
}

// 应用ExecutableBuilder trait到GetCommentRequestBuilder
impl_executable_builder_owned!(
    GetCommentRequestBuilder,
    super::CommentsService,
    GetCommentRequest,
    crate::core::api_resp::BaseResponse<GetCommentResponse>,
    get,
);

/// 获取全文评论响应
///
/// 包含指定评论的完整详细信息
#[derive(Debug, Clone, Deserialize)]
pub struct GetCommentResponse {
    /// 评论信息
    pub comment: Comment,
}

impl GetCommentResponse {
    /// 创建新的评论响应
    ///
    /// # 参数
    /// - `comment`: 评论信息
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::get::GetCommentResponse;
    /// use open_lark::service::cloud_docs::comments::list::Comment;
    ///
    /// let comment = Comment::new("comment123", "user456", 1234567890, 1234567891, false);
    /// let response = GetCommentResponse::new(comment);
    /// ```
    pub fn new(comment: Comment) -> Self {
        Self { comment }
    }

    /// 获取评论的纯文本内容
    ///
    /// # 返回
    /// 返回评论内容（如果有回复的话，只取第一个回复的内容）
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::get::GetCommentResponse;
    ///
    /// let response = GetCommentResponse::new(comment);
    /// let text_content = response.plain_text_content();
    /// ```
    pub fn plain_text_content(&self) -> String {
        self.comment
            .first_reply()
            .map(|reply| reply.plain_text())
            .unwrap_or_default()
    }

    /// 检查评论是否包含特定文本
    ///
    /// # 参数
    /// - `text`: 要搜索的文本
    ///
    /// # 返回
    /// 如果评论的任何回复包含指定文本则返回true
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::get::GetCommentResponse;
    ///
    /// let response = GetCommentResponse::new(comment);
    /// let contains_important = response.contains_text("重要");
    /// ```
    pub fn contains_text(&self, text: &str) -> bool {
        if let Some(reply_list) = &self.comment.reply_list {
            reply_list.iter().any(|reply| reply.contains_text(text))
        } else {
            false
        }
    }

    /// 获取评论的总回复数
    ///
    /// # 返回
    /// 返回评论的回复数量
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::get::GetCommentResponse;
    ///
    /// let response = GetCommentResponse::new(comment);
    /// let reply_count = response.reply_count();
    /// ```
    pub fn reply_count(&self) -> usize {
        self.comment.reply_count()
    }

    /// 检查评论是否有更多回复需要分页获取
    ///
    /// # 返回
    /// 如果评论有更多回复则返回true
    pub fn has_more_replies(&self) -> bool {
        self.comment.has_more
    }

    /// 获取下一页回复的分页标记
    ///
    /// # 返回
    /// 如果有更多回复则返回分页标记
    pub fn next_page_token(&self) -> Option<&String> {
        self.comment.page_token.as_ref()
    }

    /// 检查评论是否已解决
    ///
    /// # 返回
    /// 如果评论已解决则返回true
    pub fn is_resolved(&self) -> bool {
        self.comment.is_solved_comment()
    }

    /// 检查评论是否为全文评论
    ///
    /// # 返回
    /// 如果评论是全文评论则返回true
    pub fn is_whole_comment(&self) -> bool {
        self.comment.is_whole_comment()
    }

    /// 获取评论的引用内容
    ///
    /// # 返回
    /// 如果评论有引用内容则返回
    pub fn get_quote(&self) -> Option<&String> {
        self.comment.quote.as_ref()
    }

    /// 获取评论的解决时间
    ///
    /// # 返回
    /// 如果评论已解决则返回解决时间的格式化字符串
    pub fn formatted_solved_time(&self) -> Option<String> {
        self.comment.formatted_solved_time()
    }

    /// 获取评论创建时间的格式化字符串
    ///
    /// # 返回
    /// 返回评论创建时间的格式化字符串
    pub fn formatted_create_time(&self) -> String {
        self.comment.formatted_create_time()
    }

    /// 获取评论更新时间的格式化字符串
    ///
    /// # 返回
    /// 返回评论更新时间的格式化字符串
    pub fn formatted_update_time(&self) -> String {
        self.comment.formatted_update_time()
    }
}

impl ApiResponseTrait for GetCommentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取全文评论
///
/// 获取指定云文档中的单个评论的详细信息。
///
/// # API文档
/// <https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/cloud_docs-v1/comment/get>
///
/// # 参数
/// - `request`: 获取评论请求
/// - `config`: SDK配置
/// - `option`: 可选请求参数
///
/// # 返回
/// 包含评论详细信息的响应
///
/// # 示例
/// ```
/// use open_lark::service::cloud_docs::comments::get::{get_comment, GetCommentRequest};
/// use open_lark::core::config::Config;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = Config::new("app_id", "app_secret");
///     let request = GetCommentRequest::builder()
///         .file_token("doccnxxxxxx")
///         .with_doc_type()
///         .comment_id("comment_123")
///         .with_open_id()
///         .build();
///
///     let response = get_comment(request, &config, None).await?;
///     println!("评论ID: {}", response.data.comment.comment_id);
///     println!("回复数: {}", response.reply_count());
///
///     Ok(())
/// }
/// ```
pub async fn get_comment(
    request: GetCommentRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<crate::core::api_resp::BaseResponse<GetCommentResponse>> {
    let mut api_req = request.api_request;
    api_req.set_http_method(Method::GET);

    // 构建API路径，包含评论ID
    api_req.api_path = format!(
        "{}?file_type={}&file_token={}",
        COMMENT_V1_COMMENT_GET.replace("{comment_id}", &request.comment_id),
        request.file_type,
        request.file_token,
    );

    // 添加用户ID类型查询参数
    if let Some(user_id_type) = request.user_id_type {
        api_req.api_path = format!(
            "{}&user_id_type={}",
            api_req.api_path, user_id_type,
        );
    }

    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use crate::service::cloud_docs::comments::list::{Reply, ReplyContent, ContentElement, TextRun};

    #[test]
    fn test_get_comment_request_builder() {
        let request = GetCommentRequest::builder()
            .file_token("doccnxxxxxx")
            .with_doc_type()
            .comment_id("comment_123")
            .with_open_id()
            .build();

        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
        assert_eq!(request.comment_id, "comment_123");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_get_comment_request_new() {
        let request = GetCommentRequest::new("doccnxxxxxx", "doc", "comment_123");

        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
        assert_eq!(request.comment_id, "comment_123");
        assert_eq!(request.user_id_type, None);
    }

    #[test]
    fn test_all_file_types() {
        let doc_request = GetCommentRequest::builder()
            .file_token("doc_token")
            .with_doc_type()
            .comment_id("comment_id")
            .build();

        let docx_request = GetCommentRequest::builder()
            .file_token("docx_token")
            .with_docx_type()
            .comment_id("comment_id")
            .build();

        let sheet_request = GetCommentRequest::builder()
            .file_token("sheet_token")
            .with_sheet_type()
            .comment_id("comment_id")
            .build();

        let bitable_request = GetCommentRequest::builder()
            .file_token("bitable_token")
            .with_bitable_type()
            .comment_id("comment_id")
            .build();

        assert_eq!(doc_request.file_type, "doc");
        assert_eq!(docx_request.file_type, "docx");
        assert_eq!(sheet_request.file_type, "sheet");
        assert_eq!(bitable_request.file_type, "bitable");
    }

    #[test]
    fn test_all_user_id_types() {
        let open_id_request = GetCommentRequest::builder()
            .file_token("token")
            .with_doc_type()
            .comment_id("comment_id")
            .with_open_id()
            .build();

        let user_id_request = GetCommentRequest::builder()
            .file_token("token")
            .with_doc_type()
            .comment_id("comment_id")
            .with_user_id()
            .build();

        let union_id_request = GetCommentRequest::builder()
            .file_token("token")
            .with_doc_type()
            .comment_id("comment_id")
            .with_union_id()
            .build();

        assert_eq!(open_id_request.user_id_type, Some("open_id".to_string()));
        assert_eq!(user_id_request.user_id_type, Some("user_id".to_string()));
        assert_eq!(union_id_request.user_id_type, Some("union_id".to_string()));
    }

    #[test]
    fn test_get_comment_response() {
        // 创建一个简单的评论
        let comment = Comment::new("comment123", "user456", 1234567890, 1234567891, false)
            .with_whole(true)
            .with_quote("引用内容");

        let response = GetCommentResponse::new(comment);

        assert_eq!(response.comment.comment_id, "comment123");
        assert_eq!(response.comment.user_id, "user456");
        assert!(!response.is_resolved());
        assert!(response.is_whole_comment());
        assert_eq!(response.get_quote(), Some(&"引用内容".to_string()));
        assert_eq!(response.reply_count(), 0);
        assert_eq!(response.plain_text_content(), ""); // 没有回复时返回空字符串
    }

    #[test]
    fn test_get_comment_response_with_replies() {
        // 创建带有回复的评论
        let reply_content = ReplyContent {
            elements: vec![
                ContentElement::text_run(TextRun::new("这是")),
                ContentElement::text_run(TextRun::bold("重要")),
                ContentElement::text_run(TextRun::new("回复")),
            ],
        };

        let reply = Reply::new("reply123", "user789", 1234567892, 1234567893, reply_content);
        let comment = Comment::new("comment123", "user456", 1234567890, 1234567891, false)
            .with_replies(vec![reply]);

        let response = GetCommentResponse::new(comment);

        assert_eq!(response.reply_count(), 1);
        assert_eq!(response.plain_text_content(), "这是重要回复");
        assert!(response.contains_text("重要"));
        assert!(response.contains_text("这是"));
        assert!(!response.contains_text("不存在的文本"));
    }

    #[test]
    fn test_get_comment_response_with_pagination() {
        let comment = Comment::new("comment123", "user456", 1234567890, 1234567891, false)
            .with_pagination(true, Some("next_page_token".to_string()));

        let response = GetCommentResponse::new(comment);

        assert!(response.has_more_replies());
        assert_eq!(response.next_page_token(), Some(&"next_page_token".to_string()));
    }

    #[test]
    fn test_get_comment_response_solved() {
        let comment = Comment::new("comment123", "user456", 1234567890, 1234567891, true)
            .with_solved_info(1234567900, "solver789");

        let response = GetCommentResponse::new(comment);

        assert!(response.is_resolved());
        assert_eq!(response.formatted_solved_time(), Some("2009-02-13 23:31:40".to_string()));
        assert_eq!(response.formatted_create_time(), "2009-02-13 23:31:30".to_string());
        assert_eq!(response.formatted_update_time(), "2009-02-13 23:31:31".to_string());
    }

    #[test]
    fn test_request_validation() {
        // 测试缺失必需参数的panic情况
        let result = std::panic::catch_unwind(|| {
            GetCommentRequest::builder()
                .file_token("doc_token")
                .with_doc_type()
                // 缺少comment_id
                .build();
        });
        assert!(result.is_err());

        let result = std::panic::catch_unwind(|| {
            GetCommentRequest::builder()
                .comment_id("comment_id")
                // 缺少file_token和file_type
                .build();
        });
        assert!(result.is_err());
    }

    #[test]
    fn test_complex_scenario() {
        // 测试复杂场景：带有多回复的已解决评论
        let reply1_content = ReplyContent {
            elements: vec![ContentElement::text_run(TextRun::new("第一个回复"))],
        };
        let reply2_content = ReplyContent {
            elements: vec![ContentElement::text_run(TextRun::new("第二个回复"))],
        };

        let reply1 = Reply::new("reply1", "user1", 1000, 1001, reply1_content);
        let reply2 = Reply::new("reply2", "user2", 1002, 1003, reply2_content);

        let comment = Comment::new("comment123", "user456", 1234567890, 1234567891, true)
            .with_solved_info(1234567900, "solver789")
            .with_replies(vec![reply1, reply2])
            .with_quote("这是一个复杂的评论")
            .with_whole(false);

        let response = GetCommentResponse::new(comment);

        assert_eq!(response.reply_count(), 2);
        assert!(response.is_resolved());
        assert!(!response.is_whole_comment());
        assert_eq!(response.get_quote(), Some(&"这是一个复杂的评论".to_string()));
        assert_eq!(response.plain_text_content(), "第一个回复"); // 取第一个回复的内容
        assert!(response.contains_text("回复"));
    }
}