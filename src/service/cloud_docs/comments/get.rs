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
/// 获取全文评论请求
#[derive(Debug, Clone)]
pub struct GetCommentRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 文档token,
#[serde(skip)]
    file_token: String,
    /// 文档类型：doc、docx、sheet、bitable,
#[serde(skip)]
    file_type: String,
    /// 评论ID,
#[serde(skip)]
    comment_id: String,
    /// 用户ID类型,
#[serde(skip_serializing_if = "Option::is_none")]
    user_id_type: Option<String>}
impl GetCommentRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct GetCommentRequestBuilder {
    request: GetCommentRequest}
impl GetCommentRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}// 应用ExecutableBuilder trait到GetCommentRequestBuilder,
impl_executable_builder_owned!(
    GetCommentRequestBuilder,
    super::CommentsService,
    GetCommentRequest,
    BaseResponse<GetCommentResponse>,
    get,
);
/// 获取全文评论响应
#[derive(Debug, Clone)]
pub struct GetCommentResponse {
    /// 评论信息
    pub comment: Comment,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 获取全文评论,
pub async fn get_comment(
    request: GetCommentRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<GetCommentResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::GET);
api_req.api_path = format!(,
        "{}?file_type={}&file_token={}",
        COMMENT_V1_COMMENT_GET.replace("{}", &request.comment_id),
        request.file_type,
        request.file_token,
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

impl GetCommentResponse {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
#[test]
    fn test_get_comment_request_builder() {
let request = GetCommentRequest::builder(),
            .file_token()
.with_doc_type()
            .comment_id()
.with_open_id()
            .build();

        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
        assert_eq!(request.comment_id, "comment123");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
#[test]
    ,
        let request = GetCommentRequest::new("doccnxxxxxx", "doc", "comment123");
        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
        assert_eq!(request.comment_id, "comment123");
