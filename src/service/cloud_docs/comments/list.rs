use open_lark_core::SDKResult;use reqwest::Method;
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

/// 获取云文档所有评论请求
///
/// 用于获取指定云文档中的所有评论。支持多种过滤条件，包括：
/// - 按评论类型过滤（全文评论/部分评论）
/// - 按解决状态过滤
/// - 分页查询
/// - 用户ID类型指定
///
/// # 示例
/// ```
/// use open_lark::service::cloud_docs::comments::list::ListCommentsRequest;
///
/// let request = ListCommentsRequest::builder()
///     .file_token("doccnxxxxxx")
///     .with_doc_type()
///     .whole_comments_only()
///     .unsolved_comments_only()
///     .page_size(20)
///     .with_open_id()
///     .build();
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct ListCommentsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文档token
    #[serde(skip)]
    file_token: String,
    /// 文档类型：doc、docx、sheet、bitable
    #[serde(skip)]
    file_type: String,
    /// 是否是全文评论，不传该参数则返回全部评论
    #[serde(skip_serializing_if = "Option::is_none")]
    is_whole: Option<bool>,
    /// 是否获取已解决的评论
    #[serde(skip_serializing_if = "Option::is_none")]
    is_solved: Option<bool>,
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

impl ListCommentsRequest {
    /// 创建新的评论列表请求
    ///
    /// # 参数
    /// - `file_token`: 文档标识符
    /// - `file_type`: 文档类型
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::list::ListCommentsRequest;
    ///
    /// let request = ListCommentsRequest::new("doccnxxxxxx", "doc");
    /// ```
    pub fn new(file_token: impl ToString, file_type: impl ToString) -> Self {
        let mut api_request = ApiRequest::new();
        api_request.api_path = COMMENT_V1_COMMENTS.to_string();

        Self {
            api_request,
            file_token: file_token.to_string(),
            file_type: file_type.to_string(),
            is_whole: None,
            is_solved: None,
            page_size: None,
            page_token: None,
            user_id_type: None,
        }
    }

    /// 创建构建器实例
    pub fn builder() -> ListCommentsRequestBuilder {
        ListCommentsRequestBuilder::default()
    }
}

/// 获取评论列表请求构建器
///
/// 提供流式API来构建ListCommentsRequest，支持链式调用。
/// 可以方便地设置过滤条件、分页参数和用户ID类型。
#[derive(Debug, Clone, Default)]
pub struct ListCommentsRequestBuilder {
    file_token: Option<String>,
    file_type: Option<String>,
    is_whole: Option<bool>,
    is_solved: Option<bool>,
    page_size: Option<i32>,
    page_token: Option<String>,
    user_id_type: Option<String>,
}

impl ListCommentsRequestBuilder {
    /// 设置文档token
    ///
    /// # 参数
    /// - `file_token`: 文档标识符
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::list::ListCommentsRequest;
    ///
    /// let builder = ListCommentsRequest::builder()
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

    /// 只获取全文评论
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::list::ListCommentsRequest;
    ///
    /// let builder = ListCommentsRequest::builder()
    ///     .whole_comments_only();
    /// ```
    pub fn whole_comments_only(self) -> Self {
        self.is_whole(true)
    }

    /// 只获取部分评论
    pub fn partial_comments_only(self) -> Self {
        self.is_whole(false)
    }

    /// 设置是否全文评论
    ///
    /// # 参数
    /// - `is_whole`: true表示全文评论，false表示部分评论
    pub fn is_whole(mut self, is_whole: bool) -> Self {
        self.is_whole = Some(is_whole);
        self
    }

    /// 只获取已解决的评论
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::list::ListCommentsRequest;
    ///
    /// let builder = ListCommentsRequest::builder()
    ///     .solved_comments_only();
    /// ```
    pub fn solved_comments_only(self) -> Self {
        self.is_solved(true)
    }

    /// 只获取未解决的评论
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::list::ListCommentsRequest;
    ///
    /// let builder = ListCommentsRequest::builder()
    ///     .unsolved_comments_only();
    /// ```
    pub fn unsolved_comments_only(self) -> Self {
        self.is_solved(false)
    }

    /// 设置是否已解决
    ///
    /// # 参数
    /// - `is_solved`: true表示已解决，false表示未解决
    pub fn is_solved(mut self, is_solved: bool) -> Self {
        self.is_solved = Some(is_solved);
        self
    }

    /// 设置分页大小
    ///
    /// # 参数
    /// - `page_size`: 每页返回的评论数量，建议值：10-50
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::list::ListCommentsRequest;
    ///
    /// let builder = ListCommentsRequest::builder()
    ///     .page_size(20);
    /// ```
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    ///
    /// # 参数
    /// - `page_token`: 分页标记，用于获取下一页数据
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::list::ListCommentsRequest;
    ///
    /// let builder = ListCommentsRequest::builder()
    ///     .page_token("next_page_token");
    /// ```
    pub fn page_token(mut self, page_token: impl ToString) -> Self {
        self.page_token = Some(page_token.to_string());
        self
    }

    /// 设置用户ID类型
    ///
    /// # 参数
    /// - `user_id_type`: 用户ID类型
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::list::ListCommentsRequest;
    ///
    /// let builder = ListCommentsRequest::builder()
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
    /// use open_lark::service::cloud_docs::comments::list::ListCommentsRequest;
    ///
    /// let builder = ListCommentsRequest::builder()
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

    /// 构建ListCommentsRequest实例
    ///
    /// # 返回
    /// 返回配置好的ListCommentsRequest实例
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::list::ListCommentsRequest;
    ///
    /// let request = ListCommentsRequest::builder()
    ///     .file_token("doccnxxxxxx")
    ///     .with_doc_type()
    ///     .whole_comments_only()
    ///     .unsolved_comments_only()
    ///     .page_size(20)
    ///     .with_open_id()
    ///     .build();
    /// ```
    pub fn build(self) -> ListCommentsRequest {
        let file_token = self.file_token.expect("file_token is required");
        let file_type = self.file_type.expect("file_type is required");

        let mut request = ListCommentsRequest::new(&file_token, &file_type);
        request.is_whole = self.is_whole;
        request.is_solved = self.is_solved;
        request.page_size = self.page_size;
        request.page_token = self.page_token;
        request.user_id_type = self.user_id_type;

        request
    }
}

// 应用ExecutableBuilder trait到ListCommentsRequestBuilder
impl_executable_builder_owned!(
    ListCommentsRequestBuilder,
    super::CommentsService,
    ListCommentsRequest,
    crate::core::api_resp::BaseResponse<ListCommentsResponse>,
    list,
);

/// 评论信息
///
/// 包含评论的详细信息，包括创建者、时间戳、回复列表等
#[derive(Debug, Clone, Deserialize)]
pub struct Comment {
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
    /// 已解决时间（毫秒时间戳）
    pub solved_time: Option<i64>,
    /// 解决者用户ID
    pub solver_user_id: Option<String>,
    /// 是否有更多回复
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
    /// 回复列表
    pub reply_list: Option<Vec<Reply>>,
    /// 是否是全文评论
    pub is_whole: Option<bool>,
    /// 引用内容
    pub quote: Option<String>,
}

impl Comment {
    /// 创建新的评论实例
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
            solved_time: None,
            solver_user_id: None,
            has_more: false,
            page_token: None,
            reply_list: None,
            is_whole: None,
            quote: None,
        }
    }

    /// 设置解决信息
    pub fn with_solved_info(
        mut self,
        solved_time: i64,
        solver_user_id: impl ToString,
    ) -> Self {
        self.solved_time = Some(solved_time);
        self.solver_user_id = Some(solver_user_id.to_string());
        self
    }

    /// 设置分页信息
    pub fn with_pagination(mut self, has_more: bool, page_token: Option<String>) -> Self {
        self.has_more = has_more;
        self.page_token = page_token;
        self
    }

    /// 设置回复列表
    pub fn with_replies(mut self, replies: Vec<Reply>) -> Self {
        self.reply_list = Some(replies);
        self
    }

    /// 设置是否为全文评论
    pub fn with_whole(mut self, is_whole: bool) -> Self {
        self.is_whole = Some(is_whole);
        self
    }

    /// 设置引用内容
    pub fn with_quote(mut self, quote: impl ToString) -> Self {
        self.quote = Some(quote.to_string());
        self
    }

    /// 是否有回复
    pub fn has_replies(&self) -> bool {
        self.reply_list
            .as_ref()
            .is_some_and(|replies| !replies.is_empty())
    }

    /// 获取回复数量
    pub fn reply_count(&self) -> usize {
        self.reply_list.as_ref().map_or(0, |replies| replies.len())
    }

    /// 获取第一个回复（如果存在）
    pub fn first_reply(&self) -> Option<&Reply> {
        self.reply_list.as_ref().and_then(|replies| replies.first())
    }

    /// 获取最后一个回复（如果存在）
    pub fn last_reply(&self) -> Option<&Reply> {
        self.reply_list.as_ref().and_then(|replies| replies.last())
    }

    /// 是否已解决
    pub fn is_solved_comment(&self) -> bool {
        self.is_solved
    }

    /// 是否是全文评论
    pub fn is_whole_comment(&self) -> bool {
        self.is_whole.unwrap_or(false)
    }

    /// 获取评论创建时间的格式化字符串
    pub fn formatted_create_time(&self) -> String {
        chrono::DateTime::from_timestamp_millis(self.create_time)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "Invalid timestamp".to_string())
    }

    /// 获取评论更新时间的格式化字符串
    pub fn formatted_update_time(&self) -> String {
        chrono::DateTime::from_timestamp_millis(self.update_time)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "Invalid timestamp".to_string())
    }

    /// 获取评论解决时间的格式化字符串
    pub fn formatted_solved_time(&self) -> Option<String> {
        self.solved_time.and_then(|time| {
            chrono::DateTime::from_timestamp_millis(time)
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        })
    }
}

/// 回复信息
///
/// 包含评论回复的详细信息
#[derive(Debug, Clone, Deserialize)]
pub struct Reply {
    /// 回复ID
    pub reply_id: String,
    /// 用户ID
    pub user_id: String,
    /// 创建时间（毫秒时间戳）
    pub create_time: i64,
    /// 更新时间（毫秒时间戳）
    pub update_time: i64,
    /// 回复内容
    pub content: ReplyContent,
    /// 额外字段
    pub extra: Option<serde_json::Value>,
}

impl Reply {
    /// 创建新的回复实例
    pub fn new(
        reply_id: impl ToString,
        user_id: impl ToString,
        create_time: i64,
        update_time: i64,
        content: ReplyContent,
    ) -> Self {
        Self {
            reply_id: reply_id.to_string(),
            user_id: user_id.to_string(),
            create_time,
            update_time,
            content,
            extra: None,
        }
    }

    /// 设置额外信息
    pub fn with_extra(mut self, extra: serde_json::Value) -> Self {
        self.extra = Some(extra);
        self
    }

    /// 获取回复创建时间的格式化字符串
    pub fn formatted_create_time(&self) -> String {
        chrono::DateTime::from_timestamp_millis(self.create_time)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "Invalid timestamp".to_string())
    }

    /// 获取回复更新时间的格式化字符串
    pub fn formatted_update_time(&self) -> String {
        chrono::DateTime::from_timestamp_millis(self.update_time)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "Invalid timestamp".to_string())
    }

    /// 获取回复内容的纯文本表示
    pub fn plain_text(&self) -> String {
        self.content.elements.iter().fold(String::new(), |mut acc, element| {
            if let Some(text_run) = &element.text_run {
                acc.push_str(&text_run.text);
            }
            acc
        })
    }

    /// 检查回复内容是否包含特定文本
    pub fn contains_text(&self, text: &str) -> bool {
        self.plain_text().contains(text)
    }
}

/// 回复内容
///
/// 包含回复的富文本内容，由多个内容元素组成
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplyContent {
    /// 元素列表
    pub elements: Vec<ContentElement>,
}

impl ReplyContent {
    /// 创建新的回复内容
    pub fn new(elements: Vec<ContentElement>) -> Self {
        Self { elements }
    }

    /// 创建空的回复内容
    pub fn empty() -> Self {
        Self { elements: vec![] }
    }

    /// 获取内容的纯文本表示
    pub fn plain_text(&self) -> String {
        self.elements.iter().fold(String::new(), |mut acc, element| {
            if let Some(text_run) = &element.text_run {
                acc.push_str(&text_run.text);
            }
            acc
        })
    }

    /// 检查内容是否为空
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    /// 获取内容长度
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    /// 添加内容元素
    pub fn add_element(mut self, element: ContentElement) -> Self {
        self.elements.push(element);
        self
    }
}

/// 内容元素
///
/// 表示富文本内容中的单个元素
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentElement {
    /// 元素类型
    #[serde(rename = "type")]
    pub element_type: String,
    /// 文本内容
    pub text_run: Option<TextRun>,
}

impl ContentElement {
    /// 创建新的内容元素
    pub fn new(element_type: impl ToString, text_run: Option<TextRun>) -> Self {
        Self {
            element_type: element_type.to_string(),
            text_run,
        }
    }

    /// 创建文本运行元素
    pub fn text_run(text_run: TextRun) -> Self {
        Self {
            element_type: "text_run".to_string(),
            text_run: Some(text_run),
        }
    }

    /// 获取元素的纯文本内容
    pub fn plain_text(&self) -> String {
        self.text_run
            .as_ref()
            .map(|run| run.text.clone())
            .unwrap_or_default()
    }

    /// 检查元素是否为文本类型
    pub fn is_text_run(&self) -> bool {
        self.element_type == "text_run"
    }
}

/// 文本内容
///
/// 包含文本内容和样式信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextRun {
    /// 文本内容
    pub text: String,
    /// 样式
    pub style: Option<serde_json::Value>,
}

impl TextRun {
    /// 创建新的文本运行
    pub fn new(text: impl ToString) -> Self {
        Self {
            text: text.to_string(),
            style: None,
        }
    }

    /// 创建带样式的文本运行
    pub fn with_style(text: impl ToString, style: serde_json::Value) -> Self {
        Self {
            text: text.to_string(),
            style: Some(style),
        }
    }

    /// 创建粗体文本
    pub fn bold(text: impl ToString) -> Self {
        Self::with_style(text, serde_json::json!({ "bold": true }))
    }

    /// 创建斜体文本
    pub fn italic(text: impl ToString) -> Self {
        Self::with_style(text, serde_json::json!({ "italic": true }))
    }

    /// 创建下划线文本
    pub fn underline(text: impl ToString) -> Self {
        Self::with_style(text, serde_json::json!({ "underline": true }))
    }

    /// 检查是否为粗体
    pub fn is_bold(&self) -> bool {
        self.style
            .as_ref()
            .and_then(|style| style.get("bold"))
            .and_then(|value| value.as_bool())
            .unwrap_or(false)
    }

    /// 检查是否为斜体
    pub fn is_italic(&self) -> bool {
        self.style
            .as_ref()
            .and_then(|style| style.get("italic"))
            .and_then(|value| value.as_bool())
            .unwrap_or(false)
    }

    /// 检查是否有下划线
    pub fn is_underline(&self) -> bool {
        self.style
            .as_ref()
            .and_then(|style| style.get("underline"))
            .and_then(|value| value.as_bool())
            .unwrap_or(false)
    }

    /// 获取文本颜色
    pub fn get_color(&self) -> Option<String> {
        self.style
            .as_ref()
            .and_then(|style| style.get("color"))
            .and_then(|color| color.as_str())
            .map(|s| s.to_string())
    }

    /// 获取背景颜色
    pub fn get_background_color(&self) -> Option<String> {
        self.style
            .as_ref()
            .and_then(|style| style.get("background_color"))
            .and_then(|color| color.as_str())
            .map(|s| s.to_string())
    }
}

/// 获取云文档所有评论响应
///
/// 包含评论列表信息和分页信息
#[derive(Debug, Clone, Deserialize)]
pub struct ListCommentsResponse {
    /// 评论列表
    pub items: Vec<Comment>,
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
}

impl ListCommentsResponse {
    /// 创建新的评论列表响应
    pub fn new(items: Vec<Comment>, has_more: bool, page_token: Option<String>) -> Self {
        Self {
            items,
            has_more,
            page_token,
        }
    }

    /// 获取评论数量
    pub fn comment_count(&self) -> usize {
        self.items.len()
    }

    /// 获取已解决的评论数量
    pub fn solved_comment_count(&self) -> usize {
        self.items.iter().filter(|comment| comment.is_solved).count()
    }

    /// 获取未解决的评论数量
    pub fn unsolved_comment_count(&self) -> usize {
        self.items.iter().filter(|comment| !comment.is_solved).count()
    }

    /// 获取全文评论数量
    pub fn whole_comment_count(&self) -> usize {
        self.items
            .iter()
            .filter(|comment| comment.is_whole_comment())
            .count()
    }

    /// 获取部分评论数量
    pub fn partial_comment_count(&self) -> usize {
        self.items
            .iter()
            .filter(|comment| !comment.is_whole_comment())
            .count()
    }

    /// 获取第一个评论（如果存在）
    pub fn first_comment(&self) -> Option<&Comment> {
        self.items.first()
    }

    /// 获取最后一个评论（如果存在）
    pub fn last_comment(&self) -> Option<&Comment> {
        self.items.last()
    }

    /// 根据评论ID查找评论
    pub fn find_comment_by_id(&self, comment_id: &str) -> Option<&Comment> {
        self.items.iter().find(|comment| comment.comment_id == comment_id)
    }

    /// 获取指定用户创建的评论
    pub fn comments_by_user(&self, user_id: &str) -> Vec<&Comment> {
        self.items
            .iter()
            .filter(|comment| comment.user_id == user_id)
            .collect()
    }

    /// 按创建时间排序评论（最新的在前）
    pub fn sorted_by_create_time_desc(&self) -> Vec<&Comment> {
        let mut comments: Vec<&Comment> = self.items.iter().collect();
        comments.sort_by(|a, b| b.create_time.cmp(&a.create_time));
        comments
    }

    /// 按创建时间排序评论（最旧的在前）
    pub fn sorted_by_create_time_asc(&self) -> Vec<&Comment> {
        let mut comments: Vec<&Comment> = self.items.iter().collect();
        comments.sort_by(|a, b| a.create_time.cmp(&b.create_time));
        comments
    }
}

impl ApiResponseTrait for ListCommentsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取云文档所有评论
///
/// 获取指定云文档中的所有评论，支持多种过滤条件和分页查询。
///
/// # 参数
/// - `request`: 获取评论列表请求
/// - `config`: SDK配置
/// - `option`: 可选请求参数
///
/// # 返回
/// 包含评论列表信息的响应
///
/// # 示例
/// ```
/// use open_lark::service::cloud_docs::comments::list::{list_comments, ListCommentsRequest};
/// use open_lark::core::config::Config;
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     let config = Config::new("app_id", "app_secret");
///     let request = ListCommentsRequest::builder()
///         .file_token("doccnxxxxxx")
///         .with_doc_type()
///         .page_size(20)
///         .with_open_id()
///         .build();
///
///     let response = list_comments(request, &config, None).await?;
///     println!("获取到 {} 条评论", response.data.items.len());
///
///     Ok(())
/// }
/// ```
pub async fn list_comments(
    request: ListCommentsRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<crate::core::api_resp::BaseResponse<ListCommentsResponse>> {
    let mut api_req = request.api_request;
    api_req.set_http_method(Method::GET);
    api_req.api_path = format!(
        "{}?file_type={}&file_token={}",
        COMMENT_V1_COMMENTS, request.file_type, request.file_token,
    );

    // 构建查询参数
    let mut query_params = Vec::new();

    if let Some(is_whole) = request.is_whole {
        query_params.push(format!("is_whole={is_whole}"));
    }
    if let Some(is_solved) = request.is_solved {
        query_params.push(format!("is_solved={is_solved}"));
    }
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
    use serde_json::json;

    #[test]
    fn test_list_comments_request_builder() {
        let request = ListCommentsRequest::builder()
            .file_token("doccnxxxxxx")
            .with_doc_type()
            .whole_comments_only()
            .unsolved_comments_only()
            .page_size(20)
            .with_open_id()
            .build();

        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
        assert_eq!(request.is_whole, Some(true));
        assert_eq!(request.is_solved, Some(false));
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_comment_creation() {
        let comment = Comment::new("comment123", "user456", 1234567890, 1234567891, false)
            .with_solved_info(1234567900, "solver789")
            .with_whole(true)
            .with_quote("引用内容");

        assert_eq!(comment.comment_id, "comment123");
        assert_eq!(comment.user_id, "user456");
        assert_eq!(comment.is_solved, true);
        assert_eq!(comment.solved_time, Some(1234567900));
        assert_eq!(comment.solver_user_id, Some("solver789".to_string()));
        assert_eq!(comment.is_whole_comment(), true);
        assert_eq!(comment.quote, Some("引用内容".to_string()));
    }

    #[test]
    fn test_reply_content() {
        let content = ReplyContent {
            elements: vec![
                ContentElement::text_run(TextRun::bold("这是")),
                ContentElement::text_run(TextRun::new("重要")),
                ContentElement::text_run(TextRun::italic("内容")),
            ],
        };

        assert_eq!(content.len(), 3);
        assert_eq!(content.plain_text(), "这是重要内容");
        assert!(!content.is_empty());
    }

    #[test]
    fn test_text_run() {
        let bold_text = TextRun::bold("粗体文本");
        let italic_text = TextRun::italic("斜体文本");
        let underline_text = TextRun::underline("下划线文本");

        assert!(bold_text.is_bold());
        assert!(!bold_text.is_italic());
        assert!(italic_text.is_italic());
        assert!(!italic_text.is_bold());
        assert!(underline_text.is_underline());
        assert!(!underline_text.is_bold());
    }

    #[test]
    fn test_list_comments_response() {
        let comment1 = Comment::new("comment1", "user1", 1000, 1001, true)
            .with_whole(true);
        let comment2 = Comment::new("comment2", "user2", 1002, 1003, false)
            .with_whole(false);

        let response = ListCommentsResponse::new(vec![comment1, comment2], true, Some("next_token"));

        assert_eq!(response.comment_count(), 2);
        assert_eq!(response.solved_comment_count(), 1);
        assert_eq!(response.unsolved_comment_count(), 1);
        assert_eq!(response.whole_comment_count(), 1);
        assert_eq!(response.partial_comment_count(), 1);
        assert!(response.has_more);
        assert_eq!(response.page_token, Some("next_token".to_string()));
    }

    #[test]
    fn test_all_file_types() {
        let doc_request = ListCommentsRequest::builder()
            .file_token("doc_token")
            .with_doc_type()
            .build();

        let docx_request = ListCommentsRequest::builder()
            .file_token("docx_token")
            .with_docx_type()
            .build();

        let sheet_request = ListCommentsRequest::builder()
            .file_token("sheet_token")
            .with_sheet_type()
            .build();

        let bitable_request = ListCommentsRequest::builder()
            .file_token("bitable_token")
            .with_bitable_type()
            .build();

        assert_eq!(doc_request.file_type, "doc");
        assert_eq!(docx_request.file_type, "docx");
        assert_eq!(sheet_request.file_type, "sheet");
        assert_eq!(bitable_request.file_type, "bitable");
    }

    #[test]
    fn test_complex_reply() {
        let content = ReplyContent {
            elements: vec![
                ContentElement::text_run(TextRun::with_style(
                    "重要",
                    json!({
                        "bold": true,
                        "color": "#FF0000"
                    }),
                )),
                ContentElement::text_run(TextRun::new("内容")),
            ],
        };

        let reply = Reply::new("reply123", "user456", 1234567890, 1234567891, content)
            .with_extra(json!({"priority": "high"}));

        assert_eq!(reply.reply_id, "reply123");
        assert_eq!(reply.plain_text(), "重要内容");
        assert!(reply.extra.is_some());
    }
}