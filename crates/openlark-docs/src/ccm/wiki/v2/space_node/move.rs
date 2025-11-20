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
/// 移动知识空间节点请求,
#[derive(Clone, Debug)]
pub struct MoveSpaceNodeRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 知识空间id,
#[serde(skip)]
    space_id: String,
    /// 节点token,
#[serde(skip)]
    node_token: String,
    /// 目标父节点token，移动到根目录时可以为空,
#[serde(skip_serializing_if = "Option::is_none")]
    target_parent_token: Option<String>,
    /// 目标位置，移动到目标父节点的指定位置，不填时追加到末尾,
#[serde(skip_serializing_if = "Option::is_none")]
    target_prev_token: Option<String>}
impl MoveSpaceNodeRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone, Debug)]
pub struct MoveSpaceNodeRequestBuilder {
    request: MoveSpaceNodeRequest}
impl MoveSpaceNodeRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 移动后的节点信息,
#[derive(Clone, Debug)]
pub struct MovedNode {
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
    pub title: Option<String>}
/// 移动知识空间节点响应,
#[derive(Clone, Debug)]
pub struct MoveSpaceNodeResponse {
    /// 移动后的节点信息
    pub node: MovedNode,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 移动知识空间节点,
pub async fn move_space_node(
    request: MoveSpaceNodeRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<MoveSpaceNodeResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::POST);
api_req.api_path = {,
        let mut path =
            EndpointBuilder::replace_param(WIKI_V2_SPACE_NODE_MOVE, "space_id", &request.space_id);
        path = EndpointBuilder::replace_param(&path, "node_token", &request.node_token);
path};
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp),

#[cfg(test)]
mod tests {
    use super::*;
#[test]
    fn test_move_space_node_request_builder() {
let request = MoveSpaceNodeRequest::builder(),
            .space_id()
.node_token()
            .target_parent_token()
.target_prev_token()
            .build();

        assert_eq!(request.space_id, "spcxxxxxx");
        assert_eq!(request.node_token, "wikcnxxxxxx");
assert_eq!(,
            request.target_parent_token,
            Some("wikcnyyyyyyy".to_string()),
);
        assert_eq!(request.target_prev_token, Some("wikcnzzzzzzz".to_string()));
#[test]
    fn test_move_to_root() {
let request = MoveSpaceNodeRequest::builder(),
            .space_id()
.node_token()
            .move_to_root()
.append_to_end()
            .build();

        assert_eq!(request.space_id, "spcxxxxxx");
        assert_eq!(request.node_token, "wikcnxxxxxx");
        assert_eq!(request.target_parent_token, None);
        assert_eq!(request.target_prev_token, None);
