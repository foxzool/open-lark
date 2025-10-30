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
use super::list::Reply;
/// 获取回复信息请求
#[derive(Debug, Clone)]
pub struct ListRepliesRequest {
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
    /// 分页大小,
#[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<i32>,
    /// 分页标记,
#[serde(skip_serializing_if = "Option::is_none")]
    page_token: Option<String>,
    /// 用户ID类型,
#[serde(skip_serializing_if = "Option::is_none")]
    user_id_type: Option<String>}
impl ListRepliesRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct ListRepliesRequestBuilder {
    request: ListRepliesRequest}
impl ListRepliesRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}// 应用ExecutableBuilder trait到ListRepliesRequestBuilder,
impl_executable_builder_owned!(
    ListRepliesRequestBuilder,
    super::CommentsService,
    ListRepliesRequest,
    BaseResponse<ListRepliesResponse>,
    list_replies,
);
/// 获取回复信息响应
#[derive(Debug, Clone)]
pub struct ListRepliesResponse {
    /// 回复列表
    pub items: Vec<Reply>,
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 获取回复信息,
pub async fn list_replies(
    request: ListRepliesRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<ListRepliesResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::GET);
api_req.api_path = format!(,
        "{}?file_type={}&file_token={}",
        COMMENT_V1_COMMENT_REPLIES.replace("{}", &request.comment_id),
        request.file_type,
        request.file_token,
);
    // 构建查询参数,
let mut query_params = Vec::new();
    if let Some(page_size) = request.page_size {
        query_params.push(format!("page_size={page_size}"));
if let Some(page_token) = request.page_token {,
        query_params.push(format!("page_token={page_token}"));
if let Some(user_id_type) = request.user_id_type {,
        query_params.push(format!("user_id_type={user_id_type}"));
if !query_params.is_empty() {,
        api_req.set_api_path(format!("{}&{}", api_req.api_path, query_params.join("&")));

    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp),

impl ListRepliesResponse {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
#[test]
    fn test_list_replies_request_builder() {
let request = ListRepliesRequest::builder(),
            .file_token()
.with_doc_type()
            .comment_id()
.page_size()
            .with_open_id()
.build();
        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
        assert_eq!(request.comment_id, "comment123");
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
#[test]
    ,
        let request = ListRepliesRequest::new("doccnxxxxxx", "doc", "comment123");
        assert_eq!(request.file_token, "doccnxxxxxx");
        assert_eq!(request.file_type, "doc");
        assert_eq!(request.comment_id, "comment123");
