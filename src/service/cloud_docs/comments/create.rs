use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
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
#[derive(Debug, Serialize, Default, Clone)]
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
    pub fn builder() -> CreateCommentRequestBuilder {
        CreateCommentRequestBuilder::default()
    }

    pub fn new(file_token: impl ToString, file_type: impl ToString, content: ReplyContent) -> Self {
        Self {
            file_token: file_token.to_string(),
            file_type: file_type.to_string(),
            content,
            ..Default::default()
        }
    }

    /// 创建简单文本评论
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
}

#[derive(Default)]
pub struct CreateCommentRequestBuilder {
    request: CreateCommentRequest,
}

impl CreateCommentRequestBuilder {
    /// 文档token
    pub fn file_token(mut self, file_token: impl ToString) -> Self {
        self.request.file_token = file_token.to_string();
        self
    }

    /// 文档类型
    pub fn file_type(mut self, file_type: impl ToString) -> Self {
        self.request.file_type = file_type.to_string();
        self
    }

    /// 设置为文档类型
    pub fn with_doc_type(mut self) -> Self {
        self.request.file_type = "doc".to_string();
        self
    }

    /// 设置为docx类型
    pub fn with_docx_type(mut self) -> Self {
        self.request.file_type = "docx".to_string();
        self
    }

    /// 设置为电子表格类型
    pub fn with_sheet_type(mut self) -> Self {
        self.request.file_type = "sheet".to_string();
        self
    }

    /// 设置为多维表格类型
    pub fn with_bitable_type(mut self) -> Self {
        self.request.file_type = "bitable".to_string();
        self
    }

    /// 回复内容
    pub fn content(mut self, content: ReplyContent) -> Self {
        self.request.content = content;
        self
    }

    /// 添加文本内容
    pub fn text(mut self, text: impl ToString) -> Self {
        let element = ContentElement {
            element_type: "text_run".to_string(),
            text_run: Some(TextRun {
                text: text.to_string(),
                style: None,
            }),
        };
        self.request.content.elements.push(element);
        self
    }

    /// 添加带样式的文本内容
    pub fn styled_text(mut self, text: impl ToString, style: serde_json::Value) -> Self {
        let element = ContentElement {
            element_type: "text_run".to_string(),
            text_run: Some(TextRun {
                text: text.to_string(),
                style: Some(style),
            }),
        };
        self.request.content.elements.push(element);
        self
    }

    /// 添加粗体文本
    pub fn bold_text(self, text: impl ToString) -> Self {
        let style = serde_json::json!({
            "bold": true
        });
        self.styled_text(text, style)
    }

    /// 添加斜体文本
    pub fn italic_text(self, text: impl ToString) -> Self {
        let style = serde_json::json!({
            "italic": true
        });
        self.styled_text(text, style)
    }

    /// 用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl ToString) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 使用OpenID
    pub fn with_open_id(mut self) -> Self {
        self.request.user_id_type = Some("open_id".to_string());
        self
    }

    /// 使用UserID
    pub fn with_user_id(mut self) -> Self {
        self.request.user_id_type = Some("user_id".to_string());
        self
    }

    /// 使用UnionID
    pub fn with_union_id(mut self) -> Self {
        self.request.user_id_type = Some("union_id".to_string());
        self
    }

    pub fn build(mut self) -> CreateCommentRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 创建的评论信息
#[derive(Debug, Deserialize)]
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

// 应用ExecutableBuilder trait到CreateCommentRequestBuilder
impl_executable_builder_owned!(
    CreateCommentRequestBuilder,
    super::CommentsService,
    CreateCommentRequest,
    BaseResponse<CreateCommentResponse>,
    create
);

/// 添加全文评论响应
#[derive(Debug, Deserialize)]
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
/// <https://open.feishu.cn/document/server-docs/docs/comment/create>
pub async fn create_comment(
    request: CreateCommentRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<CreateCommentResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::POST;
    api_req.api_path = format!(
        "{}?file_type={}&file_token={}",
        COMMENT_V1_COMMENTS, request.file_type, request.file_token
    );

    // 添加用户ID类型查询参数
    if let Some(user_id_type) = request.user_id_type {
        api_req.api_path = format!("{}&user_id_type={}", api_req.api_path, user_id_type);
    }

    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

impl CreatedComment {
    /// 是否为全文评论
    pub fn is_whole_comment(&self) -> bool {
        self.is_whole.unwrap_or(false)
    }

    /// 获取创建时间的格式化字符串
    pub fn created_at_formatted(&self) -> String {
        format!("创建时间: {}", self.create_time)
    }

    /// 是否已解决
    pub fn is_solved(&self) -> bool {
        self.is_solved
    }
}

/// 内容构建器，用于构建复杂的评论内容
pub struct ContentBuilder {
    elements: Vec<ContentElement>,
}

impl ContentBuilder {
    pub fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    /// 添加普通文本
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
    pub fn add_bold(self, text: impl ToString) -> Self {
        let style = serde_json::json!({ "bold": true });
        self.add_styled_text(text, style)
    }

    /// 添加斜体文本
    pub fn add_italic(self, text: impl ToString) -> Self {
        let style = serde_json::json!({ "italic": true });
        self.add_styled_text(text, style)
    }

    /// 添加下划线文本
    pub fn add_underline(self, text: impl ToString) -> Self {
        let style = serde_json::json!({ "underline": true });
        self.add_styled_text(text, style)
    }

    /// 构建回复内容
    pub fn build(self) -> ReplyContent {
        ReplyContent {
            elements: self.elements,
        }
    }
}

impl Default for ContentBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_create_comment_request_builder() {
        let request = CreateCommentRequest::builder()
            .file_token("doccnxxxxxx")
            .with_doc_type()
            .text("这是一条评论")
            .bold_text("重要内容")
            .with_open_id()
            .build();

        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
        assert_eq!(request.content.elements.len(), 2);
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_create_comment_with_text() {
        let request = CreateCommentRequest::with_text("doccnxxxxxx", "doc", "简单评论");
        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
        assert_eq!(request.content.elements.len(), 1);
    }

    #[test]
    fn test_content_builder() {
        let content = ContentBuilder::new()
            .add_text("普通文本 ")
            .add_bold("粗体文本 ")
            .add_italic("斜体文本")
            .build();

        assert_eq!(content.elements.len(), 3);
        assert_eq!(content.elements[0].element_type, "text_run");
    }
}
