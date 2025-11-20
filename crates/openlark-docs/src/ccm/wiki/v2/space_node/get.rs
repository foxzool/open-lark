#![allow(unused_variables, unused_unsafe)]
use reqwest::Method;
use openlark_core::api::ApiRequest;use serde::{Deserialize, Serialize};
use openlark_core::{,
use SDKResult;    api::ApiRequest,
    api::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::{cloud_docs::*, EndpointBuilder}
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
/// 获取知识空间节点请求,
#[derive(Clone, Debug)]
pub struct GetSpaceNodeRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 知识空间id,
#[serde(skip)]
    space_id: String,
    /// 节点token,
#[serde(skip)]
    node_token: String}
impl GetSpaceNodeRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone, Debug)]
pub struct GetSpaceNodeRequestBuilder {
    request: GetSpaceNodeRequest}
impl GetSpaceNodeRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 知识空间节点信息,
#[derive(Clone, Debug)]
pub struct SpaceNode {
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
/// 获取知识空间节点响应,
#[derive(Clone, Debug)]
pub struct GetSpaceNodeResponse {
    /// 节点信息
    pub node: SpaceNode,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 获取知识空间节点,
pub async fn get_space_node(
    request: GetSpaceNodeRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<GetSpaceNodeResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::GET);
api_req.api_path = {,
        let mut path =
            EndpointBuilder::replace_param(WIKI_V2_SPACE_NODE_GET, "space_id", &request.space_id);
        path = EndpointBuilder::replace_param(&path, "node_token", &request.node_token);
path};
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp),

#[cfg(test)]
mod tests {
    use super::*;
#[test]
    fn test_get_space_node_request_builder() {
let request = GetSpaceNodeRequest::builder(),
            .space_id()
.node_token()
            .build();

        assert_eq!(request.space_id, "spcxxxxxx");
        assert_eq!(request.node_token, "wikcnxxxxxx");
