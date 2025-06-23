use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

use super::list::Comment;

/// 获取全文评论请求
#[derive(Debug, Serialize, Default)]
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
    pub fn builder() -> GetCommentRequestBuilder {
        GetCommentRequestBuilder::default()
    }

    pub fn new(
        file_token: impl ToString,
        file_type: impl ToString,
        comment_id: impl ToString,
    ) -> Self {
        Self {
            file_token: file_token.to_string(),
            file_type: file_type.to_string(),
            comment_id: comment_id.to_string(),
            ..Default::default()
        }
    }
}

#[derive(Default)]
pub struct GetCommentRequestBuilder {
    request: GetCommentRequest,
}

impl GetCommentRequestBuilder {
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
    pub fn as_doc(mut self) -> Self {
        self.request.file_type = "doc".to_string();
        self
    }

    /// 设置为docx类型
    pub fn as_docx(mut self) -> Self {
        self.request.file_type = "docx".to_string();
        self
    }

    /// 设置为电子表格类型
    pub fn as_sheet(mut self) -> Self {
        self.request.file_type = "sheet".to_string();
        self
    }

    /// 设置为多维表格类型
    pub fn as_bitable(mut self) -> Self {
        self.request.file_type = "bitable".to_string();
        self
    }

    /// 评论ID
    pub fn comment_id(mut self, comment_id: impl ToString) -> Self {
        self.request.comment_id = comment_id.to_string();
        self
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

    pub fn build(mut self) -> GetCommentRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }

    /// 执行请求
    pub async fn execute(
        mut self,
        service: &crate::service::cloud_docs::comments::CommentsService,
    ) -> SDKResult<BaseResponse<GetCommentResponse>> {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        service.get(self.request, None).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(
        mut self,
        service: &crate::service::cloud_docs::comments::CommentsService,
        option: RequestOption,
    ) -> SDKResult<BaseResponse<GetCommentResponse>> {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        service.get(self.request, Some(option)).await
    }
}

/// 获取全文评论响应
#[derive(Debug, Deserialize)]
pub struct GetCommentResponse {
    /// 评论信息
    pub comment: Comment,
}

impl ApiResponseTrait for GetCommentResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取全文评论
pub async fn get_comment(
    request: GetCommentRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<GetCommentResponse>> {
    let mut api_req = request.api_request;
    api_req.http_method = Method::GET;
    api_req.api_path = format!(
        "/open-apis/comment/v1/comments/{}?file_type={}&file_token={}",
        request.comment_id, request.file_type, request.file_token
    );

    // 添加用户ID类型查询参数
    if let Some(user_id_type) = request.user_id_type {
        api_req.api_path = format!("{}&user_id_type={}", api_req.api_path, user_id_type);
    }

    api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

    let api_resp = Transport::request(api_req, config, option).await?;
    Ok(api_resp)
}

impl GetCommentResponse {
    /// 获取评论ID
    pub fn comment_id(&self) -> &str {
        &self.comment.comment_id
    }

    /// 获取用户ID
    pub fn user_id(&self) -> &str {
        &self.comment.user_id
    }

    /// 是否已解决
    pub fn is_solved(&self) -> bool {
        self.comment.is_solved
    }

    /// 是否为全文评论
    pub fn is_whole_comment(&self) -> bool {
        self.comment.is_whole.unwrap_or(false)
    }

    /// 是否有回复
    pub fn has_replies(&self) -> bool {
        self.comment.has_replies()
    }

    /// 获取回复数量
    pub fn reply_count(&self) -> usize {
        self.comment.reply_count()
    }

    /// 获取评论的文本内容
    pub fn get_text_content(&self) -> String {
        self.comment.get_text_content()
    }

    /// 获取创建时间
    pub fn create_time(&self) -> i64 {
        self.comment.create_time
    }

    /// 获取更新时间
    pub fn update_time(&self) -> i64 {
        self.comment.update_time
    }

    /// 获取解决时间
    pub fn solved_time(&self) -> Option<i64> {
        self.comment.solved_time
    }

    /// 获取解决者用户ID
    pub fn solver_user_id(&self) -> Option<&str> {
        self.comment.solver_user_id.as_deref()
    }

    /// 获取引用内容
    pub fn quote(&self) -> Option<&str> {
        self.comment.quote.as_deref()
    }

    /// 获取详细信息摘要
    pub fn summary(&self) -> String {
        format!(
            "评论ID: {}, 用户: {}, 状态: {}, 回复数: {}, 创建时间: {}",
            self.comment_id(),
            self.user_id(),
            if self.is_solved() {
                "已解决"
            } else {
                "未解决"
            },
            self.reply_count(),
            self.create_time()
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_comment_request_builder() {
        let request = GetCommentRequest::builder()
            .file_token("doccnxxxxxx")
            .as_doc()
            .comment_id("comment123")
            .with_open_id()
            .build();

        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
        assert_eq!(request.comment_id, "comment123");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_get_comment_new() {
        let request = GetCommentRequest::new("doccnxxxxxx", "doc", "comment123");
        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
        assert_eq!(request.comment_id, "comment123");
    }
}
