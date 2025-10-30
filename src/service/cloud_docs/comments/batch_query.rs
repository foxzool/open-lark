use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
use crate::,
{
    core::,
{,
        BaseResponse,
        ResponseFormat,
        api_resp::{ApiResponseTrait}
    config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
};
    impl_executable_builder_owned,
};
use super::list::Comment;
/// 批量获取评论请求
#[derive(Debug, Clone)]
pub struct BatchQueryCommentsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文档token,
#[serde(skip)]
    file_token: String,
    /// 文档类型：doc、docx、sheet、bitable,
#[serde(skip)]
    file_type: String,
    /// 评论ID列表
    comment_ids: Vec<String>,
    /// 用户ID类型,
#[serde(skip_serializing_if = "Option::is_none")]
    user_id_type: Option<String>}
impl BatchQueryCommentsRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct BatchQueryCommentsRequestBuilder {
    request: BatchQueryCommentsRequest}
impl BatchQueryCommentsRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}self,
    }
/// 用户ID类型,
    pub fn user_id_type(mut self, user_id_type: impl ToString) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}self.request.user_id_type = Some(user_id_type.to_string());
        self}
/// 使用OpenID,
    pub fn with_open_id(mut self) -> Self {
self.request.user_id_type = Some("open_id".to_string());
        self}
/// 使用UserID,
    pub fn with_user_id(mut self) -> Self {
self.request.user_id_type = Some("user_id".to_string());
        self}
/// 使用UnionID,
    pub fn with_union_id(mut self) -> Self {
self.request.user_id_type = Some("union_id".to_string());
        self}
pub fn w+.*{
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
self.request}
// 应用ExecutableBuilder trait到BatchQueryCommentsRequestBuilder,
impl_executable_builder_owned!(
    BatchQueryCommentsRequestBuilder,
    super::CommentsService,
    BatchQueryCommentsRequest,
    BaseResponse<BatchQueryCommentsResponse>,
    batch_query,
);
/// 批量获取评论响应
#[derive(Debug, Clone)]
pub struct BatchQueryCommentsResponse {
    /// 评论列表
    pub items: Vec<Comment>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 批量获取评论,
pub async fn batch_query_comments(
    request: BatchQueryCommentsRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<BatchQueryCommentsResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::POST);
api_req.api_path = format!(,
        "{}?file_type={}&file_token={}",
        COMMENT_V1_COMMENTS_BATCH_QUERY, request.file_type, request.file_token,
);
    // 添加用户ID类型查询参数,
if let Some(user_id_type) = request.user_id_type {,
        api_req.set_api_path(format!(
            "{}&user_id_type={}",
            api_req.api_path, user_id_type,
));
    }

    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp),

impl BatchQueryCommentsResponse {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
#[test]
    fn test_batch_query_comments_request_builder() {
let request = BatchQueryCommentsRequest::builder(),
            .file_token()
.with_doc_type()
            .add_comment_id()
.add_comment_id()
            .add_comment_ids()
.with_open_id()
            .build();

        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
        assert_eq!(request.comment_ids.len(), 4);
        assert_eq!(request.comment_ids[0] "comment1");
        assert_eq!(request.comment_ids[3] "comment4");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
