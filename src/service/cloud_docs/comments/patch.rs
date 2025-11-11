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

/// 解决/恢复评论请求
///
/// 用于修改评论的解决状态，可以将评论标记为已解决或未解决。
///
/// # 示例
/// ```
/// use open_lark::service::cloud_docs::comments::patch::PatchCommentRequest;
///
/// let request = PatchCommentRequest::builder()
///     .file_token("doccnxxxxxx")
///     .with_doc_type()
///     .comment_id("comment_123")
///     .solve(true)
///     .with_open_id()
///     .build();
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct PatchCommentRequest {
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
    /// 是否解决
    is_solved: bool,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    user_id_type: Option<String>,
}

impl PatchCommentRequest {
    /// 创建新的修改评论状态请求
    ///
    /// # 参数
    /// - `file_token`: 文档标识符
    /// - `file_type`: 文档类型
    /// - `comment_id`: 评论ID
    /// - `is_solved`: 是否解决
    ///
    /// # 示例
    /// ```
    /// use open_lark::service::cloud_docs::comments::patch::PatchCommentRequest;
    ///
    /// let request = PatchCommentRequest::new("doccnxxxxxx", "doc", "comment_123", true);
    /// ```
    pub fn new(
        file_token: impl ToString,
        file_type: impl ToString,
        comment_id: impl ToString,
        is_solved: bool,
    ) -> Self {
        let mut api_request = ApiRequest::new();
        api_request.api_path = COMMENT_V1_PATCH.to_string();

        Self {
            api_request,
            file_token: file_token.to_string(),
            file_type: file_type.to_string(),
            comment_id: comment_id.to_string(),
            is_solved,
            user_id_type: None,
        }
    }

    /// 创建构建器实例
    pub fn builder() -> PatchCommentRequestBuilder {
        PatchCommentRequestBuilder::default()
    }
}

/// 修改评论状态请求构建器
///
/// 提供流式API来构建PatchCommentRequest，支持链式调用。
#[derive(Debug, Clone, Default)]
pub struct PatchCommentRequestBuilder {
    file_token: Option<String>,
    file_type: Option<String>,
    comment_id: Option<String>,
    is_solved: Option<bool>,
    user_id_type: Option<String>,
}

impl PatchCommentRequestBuilder {
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

    /// 设置解决状态为已解决
    pub fn solve(self) -> Self {
        self.is_solved(true)
    }

    /// 设置解决状态为未解决
    pub fn unsolve(self) -> Self {
        self.is_solved(false)
    }

    /// 设置是否解决
    pub fn is_solved(mut self, is_solved: bool) -> Self {
        self.is_solved = Some(is_solved);
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

    /// 构建PatchCommentRequest实例
    pub fn build(self) -> PatchCommentRequest {
        let file_token = self.file_token.expect("file_token is required");
        let file_type = self.file_type.expect("file_type is required");
        let comment_id = self.comment_id.expect("comment_id is required");
        let is_solved = self.is_solved.expect("is_solved is required");

        let mut request = PatchCommentRequest::new(&file_token, &file_type, &comment_id, is_solved);
        request.user_id_type = self.user_id_type;

        request
    }
}

// 应用ExecutableBuilder trait到PatchCommentRequestBuilder
impl_executable_builder_owned!(
    PatchCommentRequestBuilder,
    super::CommentsService,
    PatchCommentRequest,
    crate::core::api_resp::BaseResponse<PatchCommentResponse>,
    patch,
);

/// 解决/恢复评论响应
///
/// 包含修改评论状态后的返回信息
#[derive(Debug, Clone, Deserialize)]
pub struct PatchCommentResponse {
    /// 评论ID
    pub comment_id: String,
    /// 是否解决
    pub is_solved: bool,
    /// 解决时间（毫秒时间戳）
    pub solved_time: Option<i64>,
    /// 解决者用户ID
    pub solver_user_id: Option<String>,
}

impl PatchCommentResponse {
    /// 创建新的修改评论状态响应
    pub fn new(
        comment_id: impl ToString,
        is_solved: bool,
        solved_time: Option<i64>,
        solver_user_id: Option<String>,
    ) -> Self {
        Self {
            comment_id: comment_id.to_string(),
            is_solved,
            solved_time,
            solver_user_id,
        }
    }

    /// 检查评论是否已解决
    pub fn is_solved_comment(&self) -> bool {
        self.is_solved
    }

    /// 获取解决时间的格式化字符串
    pub fn formatted_solved_time(&self) -> Option<String> {
        self.solved_time.and_then(|time| {
            chrono::DateTime::from_timestamp_millis(time)
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
        })
    }
}

impl ApiResponseTrait for PatchCommentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 解决/恢复评论
///
/// 修改指定云文档中评论的解决状态。
///
/// # API文档
/// <https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/cloud_docs-v1/comment/patch>
///
/// # 参数
/// - `request`: 修改评论状态请求
/// - `config`: SDK配置
/// - `option`: 可选请求参数
///
/// # 返回
/// 包含修改结果信息的响应
pub async fn patch_comment(
    request: PatchCommentRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<crate::core::api_resp::BaseResponse<PatchCommentResponse>> {
    let mut api_req = request.api_request;
    api_req.set_http_method(Method::PATCH);
    api_req.api_path = format!(
        "{}?file_type={}&file_token={}&comment_id={}",
        COMMENT_V1_PATCH, request.file_type, request.file_token, request.comment_id,
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

    #[test]
    fn test_patch_comment_request_builder() {
        let request = PatchCommentRequest::builder()
            .file_token("doccnxxxxxx")
            .with_doc_type()
            .comment_id("comment_123")
            .solve()
            .with_open_id()
            .build();

        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
        assert_eq!(request.comment_id, "comment_123");
        assert!(request.is_solved);
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_patch_comment_request_new() {
        let request = PatchCommentRequest::new("doccnxxxxxx", "doc", "comment_123", true);

        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
        assert_eq!(request.comment_id, "comment_123");
        assert!(request.is_solved);
        assert_eq!(request.user_id_type, None);
    }

    #[test]
    fn test_all_file_types() {
        let doc_request = PatchCommentRequest::builder()
            .file_token("doc_token")
            .with_doc_type()
            .comment_id("comment_id")
            .solve()
            .build();

        let docx_request = PatchCommentRequest::builder()
            .file_token("docx_token")
            .with_docx_type()
            .comment_id("comment_id")
            .solve()
            .build();

        assert_eq!(doc_request.file_type, "doc");
        assert_eq!(docx_request.file_type, "docx");
    }

    #[test]
    fn test_all_user_id_types() {
        let open_id_request = PatchCommentRequest::builder()
            .file_token("token")
            .with_doc_type()
            .comment_id("comment_id")
            .solve()
            .with_open_id()
            .build();

        let user_id_request = PatchCommentRequest::builder()
            .file_token("token")
            .with_doc_type()
            .comment_id("comment_id")
            .solve()
            .with_user_id()
            .build();

        assert_eq!(open_id_request.user_id_type, Some("open_id".to_string()));
        assert_eq!(user_id_request.user_id_type, Some("user_id".to_string()));
    }

    #[test]
    fn test_patch_comment_response() {
        let response = PatchCommentResponse::new(
            "comment_123",
            true,
            Some(1234567890),
            Some("solver_456".to_string()),
        );

        assert_eq!(response.comment_id, "comment_123");
        assert!(response.is_solved_comment());
        assert_eq!(response.solved_time, Some(1234567890));
        assert_eq!(response.solver_user_id, Some("solver_456".to_string()));
        assert_eq!(
            response.formatted_solved_time(),
            Some("2009-02-13 23:31:30".to_string())
        );
    }
}