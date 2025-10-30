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
/// 解决/恢复评论请求,
#[derive(Debug, Clone)]
pub struct PatchCommentRequest {
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
    /// 是否解决
    is_solved: bool,
    /// 用户ID类型,
#[serde(skip_serializing_if = "Option::is_none")]
    user_id_type: Option<String>}
impl PatchCommentRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct PatchCommentRequestBuilder {
    request: PatchCommentRequest}
impl PatchCommentRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}// 应用ExecutableBuilder trait到PatchCommentRequestBuilder,
impl_executable_builder_owned!(
    PatchCommentRequestBuilder,
    super::CommentsService,
    PatchCommentRequest,
    BaseResponse<PatchCommentResponse>,
    patch,
);
/// 解决/恢复评论响应
#[derive(Debug, Clone)]
pub struct PatchCommentResponse {
    /// 评论ID
    pub comment_id: String,
    /// 是否解决
    pub is_solved: bool,
    /// 解决时间（毫秒时间戳）
    pub solved_time: Option<i64>,
    /// 解决者用户ID
    pub solver_user_id: Option<String>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 解决/恢复评论,
pub async fn patch_comment(
    request: PatchCommentRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<PatchCommentResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::PATCH);
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

impl PatchCommentResponse {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
#[test]
    fn test_patch_comment_request_builder() {
let request = PatchCommentRequest::builder(),
            .file_token()
.with_doc_type()
            .comment_id()
.solve_comment()
            .with_open_id()
.build();
        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
        assert_eq!(request.comment_id, "comment123");
assert!(request.is_solved);
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
#[test]
    ,
        let solve_request = PatchCommentRequest::solve("doccnxxxxxx", "doc", "comment123");
assert!(solve_request.is_solved);
        let restore_request = PatchCommentRequest::restore("doccnxxxxxx", "doc", "comment123");
assert!(!restore_request.is_solved);
    }
