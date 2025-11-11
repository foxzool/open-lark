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

/// 删除回复请求
///
/// 用于删除评论的回复，支持删除指定评论下的特定回复。
///
/// # 示例
/// ```
/// use open_lark::service::cloud_docs::comments::delete_reply::DeleteReplyRequest;
///
/// let request = DeleteReplyRequest::builder()
///     .file_token("doccnxxxxxx")
///     .with_doc_type()
///     .comment_id("comment_123")
///     .reply_id("reply_456")
///     .with_open_id()
///     .build();
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct DeleteReplyRequest {
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
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id_type: Option<String>,
}

impl DeleteReplyRequest {
    /// 创建新的删除回复请求
    ///
    /// # 参数
    /// - `file_token`: 文档标识符
    /// - `file_type`: 文档类型
    /// - `comment_id`: 评论ID
    /// - `reply_id`: 回复ID
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::delete_reply::DeleteReplyRequest;
    ///
    /// let request = DeleteReplyRequest::new("doccnxxxxxx", "doc", "comment_123", "reply_456");
    /// ```
    pub fn new(
        file_token: impl ToString,
        file_type: impl ToString,
        comment_id: impl ToString,
        reply_id: impl ToString,
    ) -> Self {
        let mut api_request = ApiRequest::new();
        api_request.api_path = COMMENT_V1_DELETE_REPLY.to_string();

        Self {
            api_request,
            file_token: file_token.to_string(),
            file_type: file_type.to_string(),
            comment_id: comment_id.to_string(),
            reply_id: reply_id.to_string(),
            user_id_type: None,
        }
    }

    /// 创建构建器实例
    pub fn builder() -> DeleteReplyRequestBuilder {
        DeleteReplyRequestBuilder::default()
    }
}

/// 删除回复请求构建器
///
/// 提供流式API来构建DeleteReplyRequest，支持链式调用。
#[derive(Debug, Clone, Default)]
pub struct DeleteReplyRequestBuilder {
    file_token: Option<String>,
    file_type: Option<String>,
    comment_id: Option<String>,
    reply_id: Option<String>,
    user_id_type: Option<String>,
}

impl DeleteReplyRequestBuilder {
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

    /// 构建DeleteReplyRequest实例
    pub fn build(self) -> DeleteReplyRequest {
        let file_token = self.file_token.expect("file_token is required");
        let file_type = self.file_type.expect("file_type is required");
        let comment_id = self.comment_id.expect("comment_id is required");
        let reply_id = self.reply_id.expect("reply_id is required");

        let mut request = DeleteReplyRequest::new(
            &file_token,
            &file_type,
            &comment_id,
            &reply_id,
        );
        request.user_id_type = self.user_id_type;

        request
    }
}

// 应用ExecutableBuilder trait到DeleteReplyRequestBuilder
impl_executable_builder_owned!(
    DeleteReplyRequestBuilder,
    super::CommentsService,
    DeleteReplyRequest,
    crate::core::api_resp::BaseResponse<DeleteReplyResponse>,
    delete_reply,
);

/// 删除回复响应
///
/// 包含删除回复后的返回信息
#[derive(Debug, Clone, Deserialize)]
pub struct DeleteReplyResponse {
    /// 删除的回复信息
    pub reply: DeletedReply,
}

impl DeleteReplyResponse {
    /// 创建新的删除回复响应
    pub fn new(reply: DeletedReply) -> Self {
        Self { reply }
    }
}

/// 删除的回复信息
///
/// 包含删除后返回的回复详细信息
#[derive(Debug, Clone, Deserialize)]
pub struct DeletedReply {
    /// 回复ID
    pub reply_id: String,
    /// 用户ID
    pub user_id: String,
    /// 创建时间（毫秒时间戳）
    pub create_time: i64,
    /// 删除时间（毫秒时间戳）
    pub delete_time: i64,
}

impl DeletedReply {
    /// 创建新的删除回复信息实例
    pub fn new(
        reply_id: impl ToString,
        user_id: impl ToString,
        create_time: i64,
        delete_time: i64,
    ) -> Self {
        Self {
            reply_id: reply_id.to_string(),
            user_id: user_id.to_string(),
            create_time,
            delete_time,
        }
    }

    /// 获取回复创建时间的格式化字符串
    pub fn formatted_create_time(&self) -> String {
        chrono::DateTime::from_timestamp_millis(self.create_time)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "Invalid timestamp".to_string())
    }

    /// 获取回复删除时间的格式化字符串
    pub fn formatted_delete_time(&self) -> String {
        chrono::DateTime::from_timestamp_millis(self.delete_time)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| "Invalid timestamp".to_string())
    }

    /// 获取回复存活时间（毫秒）
    pub fn lifetime_ms(&self) -> i64 {
        self.delete_time - self.create_time
    }

    /// 获取回复存活时间的秒数
    pub fn lifetime_seconds(&self) -> i64 {
        self.lifetime_ms() / 1000
    }
}

impl ApiResponseTrait for DeleteReplyResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除回复
///
/// 删除指定云文档评论的回复。
///
/// # API文档
/// <https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/cloud_docs-v1/comment/delete_reply>
///
/// # 参数
/// - `request`: 删除回复请求
/// - `config`: SDK配置
/// - `option`: 可选请求参数
///
/// # 返回
/// 包含删除结果信息的响应
pub async fn delete_reply(
    request: DeleteReplyRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<crate::core::api_resp::BaseResponse<DeleteReplyResponse>> {
    let mut api_req = request.api_request;
    api_req.set_http_method(Method::DELETE);
    api_req.api_path = format!(
        "{}?file_type={}&file_token={}&comment_id={}&reply_id={}",
        COMMENT_V1_DELETE_REPLY, request.file_type, request.file_token, request.comment_id, request.reply_id,
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

    #[test]
    fn test_delete_reply_request_builder() {
        let request = DeleteReplyRequest::builder()
            .file_token("doccnxxxxxx")
            .with_doc_type()
            .comment_id("comment_123")
            .reply_id("reply_456")
            .with_open_id()
            .build();

        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
        assert_eq!(request.comment_id, "comment_123");
        assert_eq!(request.reply_id, "reply_456");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_delete_reply_request_new() {
        let request = DeleteReplyRequest::new("doccnxxxxxx", "doc", "comment_123", "reply_456");

        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
        assert_eq!(request.comment_id, "comment_123");
        assert_eq!(request.reply_id, "reply_456");
        assert_eq!(request.user_id_type, None);
    }

    #[test]
    fn test_all_file_types() {
        let doc_request = DeleteReplyRequest::builder()
            .file_token("doc_token")
            .with_doc_type()
            .comment_id("comment_id")
            .reply_id("reply_id")
            .build();

        let docx_request = DeleteReplyRequest::builder()
            .file_token("docx_token")
            .with_docx_type()
            .comment_id("comment_id")
            .reply_id("reply_id")
            .build();

        assert_eq!(doc_request.file_type, "doc");
        assert_eq!(docx_request.file_type, "docx");
    }

    #[test]
    fn test_all_user_id_types() {
        let open_id_request = DeleteReplyRequest::builder()
            .file_token("token")
            .with_doc_type()
            .comment_id("comment_id")
            .reply_id("reply_id")
            .with_open_id()
            .build();

        let user_id_request = DeleteReplyRequest::builder()
            .file_token("token")
            .with_doc_type()
            .comment_id("comment_id")
            .reply_id("reply_id")
            .with_user_id()
            .build();

        assert_eq!(open_id_request.user_id_type, Some("open_id".to_string()));
        assert_eq!(user_id_request.user_id_type, Some("user_id".to_string()));
    }

    #[test]
    fn test_deleted_reply() {
        let reply = DeletedReply::new("reply123", "user456", 1234567890, 1234567999);

        assert_eq!(reply.reply_id, "reply123");
        assert_eq!(reply.user_id, "user456");
        assert_eq!(reply.create_time, 1234567890);
        assert_eq!(reply.delete_time, 1234567999);
        assert_eq!(reply.lifetime_ms(), 109);
        assert_eq!(reply.lifetime_seconds(), 0);

        assert_eq!(
            reply.formatted_create_time(),
            "2009-02-13 23:31:30".to_string()
        );
        assert_eq!(
            reply.formatted_delete_time(),
            "2009-02-13 23:33:19".to_string()
        );
    }

    #[test]
    fn test_lifetime_calculation() {
        // 测试存活时间计算
        let create_time = 1234567890;
        let delete_time = 1234568000; // 110秒后删除

        let reply = DeletedReply::new("reply123", "user456", create_time, delete_time);

        assert_eq!(reply.lifetime_ms(), 110000);
        assert_eq!(reply.lifetime_seconds(), 110);

        // 测试同一天不同时间
        let same_day_reply = DeletedReply::new("reply123", "user456", 1640995200, 1640998800); // 2022-01-01 00:00:00 到 01:00:00

        assert_eq!(same_day_reply.lifetime_seconds(), 3600); // 1小时 = 3600秒
    }

    #[test]
    fn test_delete_reply_response() {
        let reply = DeletedReply::new("reply123", "user456", 1000, 1001);
        let response = DeleteReplyResponse::new(reply);

        assert_eq!(response.reply.reply_id, "reply123");
        assert_eq!(response.reply.user_id, "user456");
        assert_eq!(response.reply.create_time, 1000);
        assert_eq!(response.reply.delete_time, 1001);
    }

    #[test]
    fn test_invalid_timestamp_formatting() {
        // 测试无效时间戳的处理
        let reply = DeletedReply::new("reply123", "user456", -1, -1); // 无效时间戳

        assert_eq!(reply.formatted_create_time(), "Invalid timestamp".to_string());
        assert_eq!(reply.formatted_delete_time(), "Invalid timestamp".to_string());
        assert_eq!(reply.lifetime_ms(), 0); // 应该返回0而不是负数
    }
}