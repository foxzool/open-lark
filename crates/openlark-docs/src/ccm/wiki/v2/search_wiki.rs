#![allow(unused_variables, unused_unsafe)]
use SDKResult;use reqwest::Method;
use openlark_core::api::ApiRequest;use serde::{Deserialize, Serialize};
,
{
    core::,
{,
        BaseResponse,
        ResponseFormat,
        api::{ApiResponseTrait}
    config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
};
    impl_executable_builder_owned,
};
/// 搜索Wiki请求,
#[derive(Clone, Debug)]
pub struct SearchWikiRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 搜索关键词
    query: String,
    /// 分页大小，最大值为50,
#[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<i32>,
    /// 分页标记,
#[serde(skip_serializing_if = "Option::is_none")]
    page_token: Option<String>,
    /// 指定搜索的知识空间id列表，不填时搜索所有有权限的知识空间,
#[serde(skip_serializing_if = "Option::is_none")]
    space_ids: Option<Vec<String>>}
impl SearchWikiRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone, Debug)]
pub struct SearchWikiRequestBuilder {
    request: SearchWikiRequest}
impl SearchWikiRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}if let Some(ref mut space_ids) = self.request.space_ids {,
            space_ids.push(space_id.to_string());
self,
    }
/// 搜索所有有权限的知识空间,
    pub fn search_all_spaces(mut self) -> Self {
self.request.space_ids = None;
        self}
pub fn w+.*{
        self.request.api_request.body = Some(openlark_core::api::RequestData::Json(&self.request)).unwrap();
self.request}
impl_executable_builder_owned!(,
    SearchWikiRequestBuilder,
    super::cloud_docs::wiki::v2::V2,
    SearchWikiRequest,
    SearchWikiResponse,
    search_wiki,
);
/// 搜索结果项
#[derive(Clone, Debug)]
pub struct WikiSearchItem {
    /// 节点token
    pub node_token: String,
    /// 知识空间id
    pub space_id: String,
    /// 文档标题
    pub title: Option<String>,
    /// 文档类型
    pub obj_type: Option<String>,
    /// 原始文档token
    pub obj_token: Option<String>,
    /// 父节点token
    pub parent_node_token: Option<String>,
    /// 知识空间名称
    pub space_name: Option<String>,
    /// 匹配的文本片段
    pub snippet: Option<String>,
    /// 最后编辑时间（毫秒时间戳）
    pub obj_edit_time: Option<String>,
    /// 最后编辑者
    pub obj_edit_user: Option<String>}
/// 搜索Wiki响应,
#[derive(Clone, Debug)]
pub struct SearchWikiResponse {
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token
    pub page_token: Option<String>,
    /// 搜索结果列表
    pub items: Vec<WikiSearchItem>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 搜索Wiki,
pub async fn search_wiki(
    request: SearchWikiRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<SearchWikiResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::POST);
api_req.set_api_path(WIKI_V2_SEARCH.to_string());
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp)}

impl WikiSearchItem {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 是否有匹配的文本片段,
    pub fn w+.*{
self.snippet.is_some()}
/// 获取显示标题（优先使用title，否则使用obj_token）,
    pub fn w+.*{
self.title.as_ref().cloned().unwrap_or_else()
                .cloned()
.unwrap_or_else(|| self.node_token.clone())}),
#[cfg(test)]
mod tests {
    use super::*;
#[test]
    fn test_search_wiki_request_builder() {
let request = SearchWikiRequest::builder(),
            .query()
.page_size()
            .add_space_id()
.add_space_id()
            .build();

        assert_eq!(request.query, "测试搜索");
        assert_eq!(request.page_size, Some(20));
assert_eq!(,
            request.space_ids,
            Some(vec!["spcxxxxxx".to_string(), "spcyyyyyy".to_string()]),
);
    }
#[test]
    fn test_search_all_spaces() {
let request = SearchWikiRequest::builder(),
            .query()
.search_all_spaces()
            .build();

        assert_eq!(request.query, "测试搜索");
        assert_eq!(request.space_ids, None);
#[test]
    fn test_wiki_search_item_methods() {
let item = WikiSearchItem {,
            node_token: "wikcnxxxxxx".to_string(),
            space_id: "spcxxxxxx".to_string(),
            title: Some("测试文档".to_string()),
            obj_type: Some("doc".to_string()),
            obj_token: Some("doccnxxxxxx".to_string()),
            parent_node_token: None,
            space_name: Some("测试空间".to_string()),
            snippet: Some("这是匹配的文本片段".to_string()),
            obj_edit_time: None,
            obj_edit_user: None};

        assert_eq!(item.display_title(), "测试文档");
assert!(item.has_snippet());
        assert_eq!(
            item.get_doc_url(),
            Some("https://feishu.cn/docs/doccnxxxxxx".to_string()),
);
    }
