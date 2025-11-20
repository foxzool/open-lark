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
        endpoints::{cloud_docs::*, EndpointBuilder};
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    }
    impl_executable_builder_owned,
};
/// 获取知识空间子节点列表请求,
#[derive(Clone, Debug)]
pub struct ListSpaceNodeRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 知识空间id,
#[serde(skip)]
    space_id: String,
    /// 分页大小,
#[serde(skip_serializing_if = "Option::is_none")]
    page_size: Option<i32>,
    /// 分页标记,
#[serde(skip_serializing_if = "Option::is_none")]
    page_token: Option<String>,
    /// 父节点token，获取其子节点。不传时获取根节点,
#[serde(skip_serializing_if = "Option::is_none")]
    parent_node_token: Option<String>}
impl ListSpaceNodeRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone, Debug)]
pub struct ListSpaceNodeRequestBuilder {
    request: ListSpaceNodeRequest}
impl ListSpaceNodeRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    ListSpaceNodeRequestBuilder,
    super::cloud_docs::wiki::v2::space_node::SpaceNodeService,
    ListSpaceNodeRequest,
    ListSpaceNodeResponse,
    list,
);
/// 知识空间节点信息
#[derive(Clone, Debug)]
pub struct NodeItem {
    /// 知识空间id
    pub space_id: String,
    /// 节点token
    pub node_token: String,
    /// 文档类型
    pub obj_type: String,
    /// 父节点token
    pub parent_node_token: Option<String>,
    /// 节点类型
    pub node_type: Option<String>,
    /// 原始文档token
    pub obj_token: Option<String>,
    /// 文档标题
    pub title: Option<String>,
    /// 文档创建时间（毫秒时间戳）
    pub obj_create_time: Option<String>,
    /// 文档更新时间（毫秒时间戳）
    pub obj_edit_time: Option<String>,
    /// 节点创建时间（毫秒时间戳）
    pub node_create_time: Option<String>,
    /// 节点创建者
    pub node_creator: Option<String>,
    /// 是否有子节点
    pub has_child: Option<bool>}
/// 获取知识空间子节点列表响应,
#[derive(Clone, Debug)]
pub struct ListSpaceNodeResponse {
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记，当 has_more 为 true 时，会同时返回新的 page_token
    pub page_token: Option<String>,
    /// 节点列表
    pub items: Vec<NodeItem>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 获取知识空间子节点列表,
pub async fn list_space_node(
    request: ListSpaceNodeRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<ListSpaceNodeResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::GET);
api_req.set_api_path(EndpointBuilder::replace_param(,
        WIKI_V2_SPACE_NODES,
        "space_id",
        &request.space_id,
    ));
// 构建查询参数,
    let mut query_params = vec![];
if let Some(page_size) = request.page_size {,
        query_params.push(format!("page_size={page_size}"));
if let Some(page_token) = request.page_token {,
        query_params.push(format!("page_token={page_token}"));
if let Some(parent_node_token) = request.parent_node_token {,
        query_params.push(format!("parent_node_token={parent_node_token}"));
if !query_params.is_empty() {,
        api_req.set_api_path(format!("{}?{}", api_req.api_path, query_params.join("&")));

    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp),

#[cfg(test)]
mod tests {
    use super::*;
#[test]
    fn test_list_space_node_request_builder() {
let request = ListSpaceNodeRequest::builder(),
            .space_id()
.page_size()
            .parent_node_token()
.build();
        assert_eq!(request.space_id, "spcxxxxxx");
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.parent_node_token, Some("wikcnxxxxxx".to_string()));
#[test]
    fn test_list_root_nodes() {
let request = ListSpaceNodeRequest::builder(),
            .space_id()
.root_nodes()
            .build();

        assert_eq!(request.space_id, "spcxxxxxx");
        assert_eq!(request.parent_node_token, None);
