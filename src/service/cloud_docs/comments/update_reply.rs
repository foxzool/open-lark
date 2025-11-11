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

use super::list::ReplyContent;

/// 更新回复内容请求
///
/// 用于更新评论回复的内容，支持富文本格式。
///
/// # 示例
/// ```
/// use open_lark::service::cloud_docs::comments::update_reply::UpdateReplyRequest;
/// use open_lark::service::cloud_docs::comments::list::ReplyContent;
///
/// let request = UpdateReplyRequest::builder()
///     .file_token("doccnxxxxxx")
///     .with_doc_type()
///     .comment_id("comment_123")
///     .reply_id("reply_456")
///     .content(reply_content)
///     .with_open_id()
///     .build();
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct UpdateReplyRequest {
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
    /// 回复ID
    #[serde(skip)]
    reply_id: String,
    /// 回复内容
    content: ReplyContent,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id_type: Option<String>,
}

impl UpdateReplyRequest {
    /// 创建新的更新回复请求
    ///
    /// # 参数
    /// - `file_token`: 文档标识符
    /// - `file_type`: 文档类型
    /// - `comment_id`: 评论ID
    /// - `reply_id`: 回复ID
    /// - `content`: 回复内容
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::update_reply::UpdateReplyRequest;
    /// use open_lark::service::cloud_docs::comments::list::ReplyContent;
    ///
    /// let request = UpdateReplyRequest::new(
    ///     "doccnxxxxxx",
    ///     "doc",
    ///     "comment_123",
    ///     "reply_456",
    ///     reply_content
    /// );
    /// ```
    pub fn new(
        file_token: impl ToString,
        file_type: impl ToString,
        comment_id: impl ToString,
        reply_id: impl ToString,
        content: ReplyContent,
    ) -> Self {
        let mut api_request = ApiRequest::new();
        api_request.api_path = COMMENT_V1_UPDATE_REPLY.to_string();

        Self {
            api_request,
            file_token: file_token.to_string(),
            file_type: file_type.to_string(),
            comment_id: comment_id.to_string(),
            reply_id: reply_id.to_string(),
            content,
            user_id_type: None,
        }
    }

    /// 创建构建器实例
    pub fn builder() -> UpdateReplyRequestBuilder {
        UpdateReplyRequestBuilder::default()
    }
}

/// 更新回复请求构建器
///
/// 提供流式API来构建UpdateReplyRequest，支持链式调用。
#[derive(Debug, Clone, Default)]
pub struct UpdateReplyRequestBuilder {
    file_token: Option<String>,
    file_type: Option<String>,
    comment_id: Option<String>,
    reply_id: Option<String>,
    content: Option<ReplyContent>,
    user_id_type: Option<String>,
}

impl UpdateReplyRequestBuilder {
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

    /// 设置回复ID
    pub fn reply_id(mut self, reply_id: impl ToString) -> Self {
        self.reply_id = Some(reply_id.to_string());
        self
    }

    /// 设置回复内容
    pub fn content(mut self, content: ReplyContent) -> Self {
        self.content = Some(content);
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

    /// 构建UpdateReplyRequest实例
    pub fn build(self) -> UpdateReplyRequest {
        let file_token = self.file_token.expect("file_token is required");
        let file_type = self.file_type.expect("file_type is required");
        let comment_id = self.comment_id.expect("comment_id is required");
        let reply_id = self.reply_id.expect("reply_id is required");
        let content = self.content.expect("content is required");

        let mut request = UpdateReplyRequest::new(
            &file_token,
            &file_type,
            &comment_id,
            &reply_id,
            content,
        );
        request.user_id_type = self.user_id_type;

        request
    }
}

// 应用ExecutableBuilder trait到UpdateReplyRequestBuilder
impl_executable_builder_owned!(
    UpdateReplyRequestBuilder,
    super::CommentsService,
    UpdateReplyRequest,
    crate::core::api_resp::BaseResponse<UpdateReplyResponse>,
    update_reply,
);

/// 更新回复内容响应
///
/// 包含更新回复后的返回信息
#[derive(Debug, Clone, Deserialize)]
pub struct UpdateReplyResponse {
    /// 更新的回复信息
    pub reply: UpdatedReply,
}

impl UpdateReplyResponse {
    /// 创建新的更新回复响应
    pub fn new(reply: UpdatedReply) -> Self {
        Self { reply }
    }
}

/// 更新的回复信息
///
/// 包含更新后返回的回复详细信息
#[derive(Debug, Clone, Deserialize)]
pub struct UpdatedReply {
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
}

impl UpdatedReply {
    /// 创建新的更新回复信息实例
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
        }
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
}

impl ApiResponseTrait for UpdateReplyResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新回复的内容
///
/// 更新指定云文档评论的回复内容。
///
/// # API文档
/// <https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/cloud_docs-v1/comment/update_reply>
///
/// # 参数
/// - `request`: 更新回复请求
/// - `config`: SDK配置
/// - `option`: 可选请求参数
///
/// # 返回
/// 包含更新结果信息的响应
pub async fn update_reply(
    request: UpdateReplyRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<crate::core::api_resp::BaseResponse<UpdateReplyResponse>> {
    let mut api_req = request.api_request;
    api_req.set_http_method(Method::PUT);
    api_req.api_path = format!(
        "{}?file_type={}&file_token={}&comment_id={}&reply_id={}",
        COMMENT_V1_UPDATE_REPLY, request.file_type, request.file_token, request.comment_id, request.reply_id,
    );

    // 添加用户ID类型查询参数
    if let Some(user_id_type) = request.user_id_type {
        api_req.api_path = format!(
            "{}&user_id_type={}",
            api_req.api_path, user_id_type,
        );
    }

    // 设置请求体
    api_req.body = Some(serde_json::to_vec(&request)?);

    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use crate::service::cloud_docs::comments::list::{ContentElement, TextRun};

    #[test]
    fn test_update_reply_request_builder() {
        let content = ReplyContent {
            elements: vec![ContentElement::text_run(TextRun::new("更新的内容"))],
        };

        let request = UpdateReplyRequest::builder()
            .file_token("doccnxxxxxx")
            .with_doc_type()
            .comment_id("comment_123")
            .reply_id("reply_456")
            .content(content)
            .with_open_id()
            .build();

        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
        assert_eq!(request.comment_id, "comment_123");
        assert_eq!(request.reply_id, "reply_456");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_update_reply_request_new() {
        let content = ReplyContent {
            elements: vec![ContentElement::text_run(TextRun::new("测试内容"))],
        };

        let request = UpdateReplyRequest::new(
            "doccnxxxxxx",
            "doc",
            "comment_123",
            "reply_456",
            content,
        );

        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
        assert_eq!(request.comment_id, "comment_123");
        assert_eq!(request.reply_id, "reply_456");
        assert_eq!(request.user_id_type, None);
    }

    #[test]
    fn test_all_file_types() {
        let content = ReplyContent {
            elements: vec![ContentElement::text_run(TextRun::new("测试"))],
        };

        let doc_request = UpdateReplyRequest::builder()
            .file_token("doc_token")
            .with_doc_type()
            .comment_id("comment_id")
            .reply_id("reply_id")
            .content(content.clone())
            .build();

        let docx_request = UpdateReplyRequest::builder()
            .file_token("docx_token")
            .with_docx_type()
            .comment_id("comment_id")
            .reply_id("reply_id")
            .content(content)
            .build();

        assert_eq!(doc_request.file_type, "doc");
        assert_eq!(docx_request.file_type, "docx");
    }

    #[test]
    fn test_all_user_id_types() {
        let content = ReplyContent {
            elements: vec![ContentElement::text_run(TextRun::new("测试"))],
        };

        let open_id_request = UpdateReplyRequest::builder()
            .file_token("token")
            .with_doc_type()
            .comment_id("comment_id")
            .reply_id("reply_id")
            .content(content.clone())
            .with_open_id()
            .build();

        let user_id_request = UpdateReplyRequest::builder()
            .file_token("token")
            .with_doc_type()
            .comment_id("comment_id")
            .reply_id("reply_id")
            .content(content)
            .with_user_id()
            .build();

        assert_eq!(open_id_request.user_id_type, Some("open_id".to_string()));
        assert_eq!(user_id_request.user_id_type, Some("user_id".to_string()));
    }

    #[test]
    fn test_updated_reply() {
        let content = ReplyContent {
            elements: vec![ContentElement::text_run(TextRun::new("更新的回复内容"))],
        };

        let reply = UpdatedReply::new("reply123", "user456", 1234567890, 1234567891, content);

        assert_eq!(reply.reply_id, "reply123");
        assert_eq!(reply.user_id, "user456");
        assert_eq!(reply.create_time, 1234567890);
        assert_eq!(reply.update_time, 1234567891);
        assert_eq!(reply.plain_text(), "更新的回复内容");
        assert!(reply.contains_text("回复"));
        assert!(reply.contains_text("更新"));
        assert!(!reply.contains_text("不存在"));

        assert_eq!(
            reply.formatted_create_time(),
            "2009-02-13 23:31:30".to_string()
        );
        assert_eq!(
            reply.formatted_update_time(),
            "2009-02-13 23:31:31".to_string()
        );
    }

    #[test]
    fn test_update_reply_response() {
        let content = ReplyContent {
            elements: vec![ContentElement::text_run(TextRun::new("测试回复"))],
        };

        let reply = UpdatedReply::new("reply123", "user456", 1000, 1001, content);
        let response = UpdateReplyResponse::new(reply);

        assert_eq!(response.reply.reply_id, "reply123");
        assert_eq!(response.reply.user_id, "user456");
        assert_eq!(response.reply.plain_text(), "测试回复");
    }
}