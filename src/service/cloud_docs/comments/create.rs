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

use super::list::{ContentElement, ReplyContent, TextRun};

/// 添加全文评论请求
///
/// 用于在云文档中创建全文评论。支持富文本内容，包括粗体、斜体、下划线等样式设置。
/// 可以使用ContentBuilder来构建复杂的评论内容。
///
/// # 示例
/// ```
/// use open_lark::service::cloud_docs::comments::create::CreateCommentRequest;
/// use open_lark::service::cloud_docs::comments::create::ContentBuilder;
///
/// // 使用构建器模式
/// let request = CreateCommentRequest::builder()
///     .file_token("doccnxxxxxx")
///     .with_doc_type()
///     .content(
///         ContentBuilder::new()
///             .add_text("这是")
///             .add_bold("重要内容")
///             .add_italic("的评论")
///             .build()
///     )
///     .with_open_id()
///     .build();
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct CreateCommentRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文档token
    #[serde(skip)]
    file_token: String,
    /// 文档类型：doc、docx、sheet、bitable
    #[serde(skip)]
    file_type: String,
    /// 回复内容
    content: ReplyContent,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id_type: Option<String>,
}

impl CreateCommentRequest {
    /// 创建新的评论请求
    ///
    /// # 参数
    /// - `file_token`: 文档标识符
    /// - `file_type`: 文档类型
    /// - `content`: 评论内容
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::create::CreateCommentRequest;
    /// use open_lark::service::cloud_docs::comments::list::ReplyContent;
    ///
    /// let request = CreateCommentRequest::new(
    ///     "doccnxxxxxx",
    ///     "doc",
    ///     ReplyContent { elements: vec![] }
    /// );
    /// ```
    pub fn new(
        file_token: impl ToString,
        file_type: impl ToString,
        content: ReplyContent,
    ) -> Self {
        let mut api_request = ApiRequest::new();
        api_request.api_path = COMMENT_V1_COMMENTS.to_string();

        Self {
            api_request,
            file_token: file_token.to_string(),
            file_type: file_type.to_string(),
            content,
            user_id_type: None,
        }
    }

    /// 创建带简单文本内容的评论请求
    ///
    /// # 参数
    /// - `file_token`: 文档标识符
    /// - `file_type`: 文档类型
    /// - `text`: 评论文本内容
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::create::CreateCommentRequest;
    ///
    /// let request = CreateCommentRequest::with_text("doccnxxxxxx", "doc", "这是一个简单评论");
    /// ```
    pub fn with_text(
        file_token: impl ToString,
        file_type: impl ToString,
        text: impl ToString,
    ) -> Self {
        let content = ReplyContent {
            elements: vec![ContentElement {
                element_type: "text_run".to_string(),
                text_run: Some(TextRun {
                    text: text.to_string(),
                    style: None,
                }),
            }],
        };

        Self::new(file_token, file_type, content)
    }

    /// 创建构建器实例
    pub fn builder() -> CreateCommentRequestBuilder {
        CreateCommentRequestBuilder::default()
    }
}

/// 创建评论请求构建器
///
/// 提供流式API来构建CreateCommentRequest，支持链式调用。
/// 可以方便地设置文档信息、用户ID类型和富文本内容。
#[derive(Debug, Clone, Default)]
pub struct CreateCommentRequestBuilder {
    file_token: Option<String>,
    file_type: Option<String>,
    content: Option<ReplyContent>,
    user_id_type: Option<String>,
}

impl CreateCommentRequestBuilder {
    /// 设置文档token
    ///
    /// # 参数
    /// - `file_token`: 文档标识符
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::create::CreateCommentRequest;
    ///
    /// let builder = CreateCommentRequest::builder()
    ///     .file_token("doccnxxxxxx");
    /// ```
    pub fn file_token(mut self, file_token: impl ToString) -> Self {
        self.file_token = Some(file_token.to_string());
        self
    }

    /// 设置文档类型为doc
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::create::CreateCommentRequest;
    ///
    /// let builder = CreateCommentRequest::builder()
    ///     .with_doc_type();
    /// ```
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

    /// 设置评论内容
    ///
    /// # 参数
    /// - `content`: 回复内容结构体
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::create::CreateCommentRequest;
    /// use open_lark::service::cloud_docs::comments::list::ReplyContent;
    ///
    /// let builder = CreateCommentRequest::builder()
    ///     .content(ReplyContent { elements: vec![] });
    /// ```
    pub fn content(mut self, content: ReplyContent) -> Self {
        self.content = Some(content);
        self
    }

    /// 添加文本内容
    ///
    /// # 参数
    /// - `text`: 文本内容
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::create::CreateCommentRequest;
    ///
    /// let builder = CreateCommentRequest::builder()
    ///     .text("这是一个评论");
    /// ```
    pub fn text(mut self, text: impl ToString) -> Self {
        if self.content.is_none() {
            self.content = Some(ReplyContent { elements: vec![] });
        }

        if let Some(ref mut content) = self.content {
            content.elements.push(ContentElement {
                element_type: "text_run".to_string(),
                text_run: Some(TextRun {
                    text: text.to_string(),
                    style: None,
                }),
            });
        }

        self
    }

    /// 添加带样式的文本内容
    ///
    /// # 参数
    /// - `text`: 文本内容
    /// - `style`: 样式JSON对象
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::create::CreateCommentRequest;
    /// use serde_json::json;
    ///
    /// let builder = CreateCommentRequest::builder()
    ///     .styled_text("重要内容", json!({"bold": true}));
    /// ```
    pub fn styled_text(mut self, text: impl ToString, style: serde_json::Value) -> Self {
        if self.content.is_none() {
            self.content = Some(ReplyContent { elements: vec![] });
        }

        if let Some(ref mut content) = self.content {
            content.elements.push(ContentElement {
                element_type: "text_run".to_string(),
                text_run: Some(TextRun {
                    text: text.to_string(),
                    style: Some(style),
                }),
            });
        }

        self
    }

    /// 添加粗体文本
    ///
    /// # 参数
    /// - `text`: 要加粗的文本
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::create::CreateCommentRequest;
    ///
    /// let builder = CreateCommentRequest::builder()
    ///     .bold_text("这是粗体文本");
    /// ```
    pub fn bold_text(self, text: impl ToString) -> Self {
        let style = serde_json::json!({
            "bold": true
        });
        self.styled_text(text, style)
    }

    /// 添加斜体文本
    ///
    /// # 参数
    /// - `text`: 要斜体显示的文本
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::create::CreateCommentRequest;
    ///
    /// let builder = CreateCommentRequest::builder()
    ///     .italic_text("这是斜体文本");
    /// ```
    pub fn italic_text(self, text: impl ToString) -> Self {
        let style = serde_json::json!({
            "italic": true
        });
        self.styled_text(text, style)
    }

    /// 添加下划线文本
    ///
    /// # 参数
    /// - `text`: 要添加下划线的文本
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::create::CreateCommentRequest;
    ///
    /// let builder = CreateCommentRequest::builder()
    ///     .underline_text("这是带下划线的文本");
    /// ```
    pub fn underline_text(self, text: impl ToString) -> Self {
        let style = serde_json::json!({
            "underline": true
        });
        self.styled_text(text, style)
    }

    /// 设置用户ID类型
    ///
    /// # 参数
    /// - `user_id_type`: 用户ID类型
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::create::CreateCommentRequest;
    ///
    /// let builder = CreateCommentRequest::builder()
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
    /// use open_lark::service::cloud_docs::comments::create::CreateCommentRequest;
    ///
    /// let builder = CreateCommentRequest::builder()
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

    /// 构建CreateCommentRequest实例
    ///
    /// # 返回
    /// 返回配置好的CreateCommentRequest实例
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::create::CreateCommentRequest;
    ///
    /// let request = CreateCommentRequest::builder()
    ///     .file_token("doccnxxxxxx")
    ///     .with_doc_type()
    ///     .text("这是一个评论")
    ///     .with_open_id()
    ///     .build();
    /// ```
    pub fn build(self) -> CreateCommentRequest {
        let file_token = self.file_token.expect("file_token is required");
        let file_type = self.file_type.expect("file_type is required");
        let content = self.content.unwrap_or_else(|| ReplyContent { elements: vec![] });

        let mut request = CreateCommentRequest::new(&file_token, &file_type, content);
        request.user_id_type = self.user_id_type;

        request
    }
}

// 应用ExecutableBuilder trait到CreateCommentRequestBuilder
impl_executable_builder_owned!(
    CreateCommentRequestBuilder,
    super::CommentsService,
    CreateCommentRequest,
    crate::core::api_resp::BaseResponse<CreateCommentResponse>,
    create,
);

/// 创建的评论信息
///
/// 包含创建后返回的评论详细信息
#[derive(Debug, Clone, Deserialize)]
pub struct CreatedComment {
    /// 评论ID
    pub comment_id: String,
    /// 用户ID
    pub user_id: String,
    /// 创建时间（毫秒时间戳）
    pub create_time: i64,
    /// 更新时间（毫秒时间戳）
    pub update_time: i64,
    /// 是否解决
    pub is_solved: bool,
    /// 是否是全文评论
    pub is_whole: Option<bool>,
}

impl CreatedComment {
    /// 创建新的评论信息实例
    pub fn new(
        comment_id: impl ToString,
        user_id: impl ToString,
        create_time: i64,
        update_time: i64,
        is_solved: bool,
    ) -> Self {
        Self {
            comment_id: comment_id.to_string(),
            user_id: user_id.to_string(),
            create_time,
            update_time,
            is_solved,
            is_whole: None,
        }
    }

    /// 设置是否为全文评论
    pub fn with_whole(mut self, is_whole: bool) -> Self {
        self.is_whole = Some(is_whole);
        self
    }

    /// 是否已解决
    pub fn is_solved(&self) -> bool {
        self.is_solved
    }

    /// 是否是全文评论
    pub fn is_whole_comment(&self) -> bool {
        self.is_whole.unwrap_or(false)
    }
}

/// 添加全文评论响应
///
/// 包含创建评论后的返回信息
#[derive(Debug, Clone, Deserialize)]
pub struct CreateCommentResponse {
    /// 创建的评论信息
    pub comment: CreatedComment,
}

impl ApiResponseTrait for CreateCommentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 添加全文评论
///
/// 在指定的云文档中创建全文评论。
///
/// # API文档
/// <https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/cloud_docs-v1/comment/create>
///
/// # 参数
/// - `request`: 创建评论请求
/// - `config`: SDK配置
/// - `option`: 可选请求参数
///
/// # 返回
/// 包含创建的评论信息的响应
///
/// # 示例
/// ```
/// use open_lark::service::cloud_docs::comments::create::{create_comment, CreateCommentRequest};
/// use open_lark::service::cloud_docs::comments::list::ReplyContent;
/// use open_lark::core::config::Config;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = Config::new("app_id", "app_secret");
///     let request = CreateCommentRequest::with_text("doccnxxxxxx", "doc", "这是一个评论");
///
///     let response = create_comment(request, &config, None).await?;
///     println!("创建的评论ID: {}", response.data.comment.comment_id);
///
///     Ok(())
/// }
/// ```
pub async fn create_comment(
    request: CreateCommentRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<crate::core::api_resp::BaseResponse<CreateCommentResponse>> {
    let mut api_req = request.api_request;
    api_req.set_http_method(Method::POST);
    api_req.api_path = format!(
        "{}?file_type={}&file_token={}",
        COMMENT_V1_COMMENTS, request.file_type, request.file_token,
    );

    // 添加用户ID类型查询参数
    if let Some(user_id_type) = request.user_id_type {
        api_req.set_api_path(format!(
            "{}&user_id_type={}",
            api_req.api_path, user_id_type,
        ));
    }

    // 设置请求体
    api_req.body = Some(serde_json::to_vec(&request)?);

    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

/// 内容构建器，用于构建复杂的评论内容
///
/// 提供便捷的方法来构建包含多种样式元素的富文本评论内容。
/// 支持普通文本、粗体、斜体、下划线等文本样式。
///
/// # 示例
/// ```
/// use open_lark::service::cloud_docs::comments::create::ContentBuilder;
///
/// let content = ContentBuilder::new()
///     .add_text("这是")
///     .add_bold("重要内容")
///     .add_italic("的评论")
///     .add_underline("非常重要")
///     .build();
/// ```
#[derive(Debug, Clone, Default)]
pub struct ContentBuilder {
    elements: Vec<ContentElement>,
}

impl ContentBuilder {
    /// 创建新的内容构建器
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::create::ContentBuilder;
    ///
    /// let builder = ContentBuilder::new();
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// 添加普通文本
    ///
    /// # 参数
    /// - `text`: 文本内容
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::create::ContentBuilder;
    ///
    /// let builder = ContentBuilder::new()
    ///     .add_text("普通文本");
    /// ```
    pub fn add_text(mut self, text: impl ToString) -> Self {
        self.elements.push(ContentElement {
            element_type: "text_run".to_string(),
            text_run: Some(TextRun {
                text: text.to_string(),
                style: None,
            }),
        });
        self
    }

    /// 添加带样式的文本
    ///
    /// # 参数
    /// - `text`: 文本内容
    /// - `style`: 样式JSON对象
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::create::ContentBuilder;
    /// use serde_json::json;
    ///
    /// let builder = ContentBuilder::new()
    ///     .add_styled_text("样式文本", json!({"bold": true, "color": "red"}));
    /// ```
    pub fn add_styled_text(mut self, text: impl ToString, style: serde_json::Value) -> Self {
        self.elements.push(ContentElement {
            element_type: "text_run".to_string(),
            text_run: Some(TextRun {
                text: text.to_string(),
                style: Some(style),
            }),
        });
        self
    }

    /// 添加粗体文本
    ///
    /// # 参数
    /// - `text`: 要加粗的文本
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::create::ContentBuilder;
    ///
    /// let builder = ContentBuilder::new()
    ///     .add_bold("粗体文本");
    /// ```
    pub fn add_bold(self, text: impl ToString) -> Self {
        let style = serde_json::json!({ "bold": true });
        self.add_styled_text(text, style)
    }

    /// 添加斜体文本
    ///
    /// # 参数
    /// - `text`: 要斜体显示的文本
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::create::ContentBuilder;
    ///
    /// let builder = ContentBuilder::new()
    ///     .add_italic("斜体文本");
    /// ```
    pub fn add_italic(self, text: impl ToString) -> Self {
        let style = serde_json::json!({ "italic": true });
        self.add_styled_text(text, style)
    }

    /// 添加下划线文本
    ///
    /// # 参数
    /// - `text`: 要添加下划线的文本
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::create::ContentBuilder;
    ///
    /// let builder = ContentBuilder::new()
    ///     .add_underline("下划线文本");
    /// ```
    pub fn add_underline(self, text: impl ToString) -> Self {
        let style = serde_json::json!({ "underline": true });
        self.add_styled_text(text, style)
    }

    /// 添加删除线文本
    ///
    /// # 参数
    /// - `text`: 要添加删除线的文本
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::create::ContentBuilder;
    ///
    /// let builder = ContentBuilder::new()
    ///     .add_strikethrough("删除线文本");
    /// ```
    pub fn add_strikethrough(self, text: impl ToString) -> Self {
        let style = serde_json::json!({ "strikethrough": true });
        self.add_styled_text(text, style)
    }

    /// 构建回复内容
    ///
    /// # 返回
    /// 返回构建好的ReplyContent实例
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::create::ContentBuilder;
    ///
    /// let content = ContentBuilder::new()
    ///     .add_text("这是")
    ///     .add_bold("重要内容")
    ///     .build();
    /// ```
    pub fn build(self) -> ReplyContent {
        ReplyContent {
            elements: self.elements,
        }
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_create_comment_request_builder() {
        let request = CreateCommentRequest::builder()
            .file_token("doccnxxxxxx")
            .with_doc_type()
            .text("这是一个评论")
            .bold_text("重要内容")
            .with_open_id()
            .build();

        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
        assert_eq!(request.content.elements.len(), 2);
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_create_comment_request_with_text() {
        let request = CreateCommentRequest::with_text("doccnxxxxxx", "doc", "简单评论");
        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
        assert_eq!(request.content.elements.len(), 1);
    }

    #[test]
    fn test_content_builder() {
        let content = ContentBuilder::new()
            .add_text("这是")
            .add_bold("重要内容")
            .add_italic("的评论")
            .add_underline("非常重要")
            .build();

        assert_eq!(content.elements.len(), 4);
        assert_eq!(content.elements[0].element_type, "text_run");
        assert_eq!(content.elements[1].element_type, "text_run");
        assert_eq!(content.elements[2].element_type, "text_run");
        assert_eq!(content.elements[3].element_type, "text_run");

        // 验证样式
        if let Some(text_run) = &content.elements[1].text_run {
            assert_eq!(text_run.text, "重要内容");
            assert!(text_run.style.is_some());
            if let Some(style) = &text_run.style {
                assert_eq!(style.get("bold"), Some(&json!(true)));
            }
        }
    }

    #[test]
    fn test_created_comment() {
        let comment = CreatedComment::new("comment123", "user456", 1234567890, 1234567891, false)
            .with_whole(true);

        assert_eq!(comment.comment_id, "comment123");
        assert_eq!(comment.user_id, "user456");
        assert_eq!(comment.create_time, 1234567890);
        assert_eq!(comment.update_time, 1234567891);
        assert!(!comment.is_solved());
        assert!(comment.is_whole_comment());
    }

    #[test]
    fn test_all_file_types() {
        let doc_request = CreateCommentRequest::builder()
            .file_token("doc_token")
            .with_doc_type()
            .build();

        let docx_request = CreateCommentRequest::builder()
            .file_token("docx_token")
            .with_docx_type()
            .build();

        let sheet_request = CreateCommentRequest::builder()
            .file_token("sheet_token")
            .with_sheet_type()
            .build();

        let bitable_request = CreateCommentRequest::builder()
            .file_token("bitable_token")
            .with_bitable_type()
            .build();

        assert_eq!(doc_request.file_type, "doc");
        assert_eq!(docx_request.file_type, "docx");
        assert_eq!(sheet_request.file_type, "sheet");
        assert_eq!(bitable_request.file_type, "bitable");
    }

    #[test]
    fn test_all_user_id_types() {
        let open_id_request = CreateCommentRequest::builder()
            .file_token("token")
            .with_doc_type()
            .with_open_id()
            .build();

        let user_id_request = CreateCommentRequest::builder()
            .file_token("token")
            .with_doc_type()
            .with_user_id()
            .build();

        let union_id_request = CreateCommentRequest::builder()
            .file_token("token")
            .with_doc_type()
            .with_union_id()
            .build();

        assert_eq!(open_id_request.user_id_type, Some("open_id".to_string()));
        assert_eq!(user_id_request.user_id_type, Some("user_id".to_string()));
        assert_eq!(union_id_request.user_id_type, Some("union_id".to_string()));
    }

    #[test]
    fn test_complex_content_builder() {
        let content = ContentBuilder::new()
            .add_text("这是一个")
            .add_bold("重要")
            .add_italic("的")
            .add_underline("文档")
            .add_strikethrough("评论")
            .add_text("，需要认真处理。")
            .add_styled_text(
                "自定义样式",
                json!({
                    "bold": true,
                    "italic": true,
                    "underline": true,
                    "color": "#FF0000"
                }),
            )
            .build();

        assert_eq!(content.elements.len(), 7);

        // 验证自定义样式
        if let Some(text_run) = &content.elements[6].text_run {
            assert_eq!(text_run.text, "自定义样式");
            if let Some(style) = &text_run.style {
                assert_eq!(style.get("bold"), Some(&json!(true)));
                assert_eq!(style.get("italic"), Some(&json!(true)));
                assert_eq!(style.get("underline"), Some(&json!(true)));
                assert_eq!(style.get("color"), Some(&json!("#FF0000")));
            }
        }
    }
}