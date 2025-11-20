#![allow(unused_variables, unused_unsafe)]
use reqwest::Method;
use openlark_core::api::ApiRequest;use serde::{Deserialize, Serialize};
use openlark_core::{,
use SDKResult;    api::ApiRequest,
    api::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::cloud_docs::*,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
/// 移动云空间文档至知识空间请求,
#[derive(Clone, Debug)]
pub struct MoveDocsToWikiRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 知识空间id
    space_id: String,
    /// 移动的云空间文档token列表
    obj_tokens: Vec<String>,
    /// 目标父节点token，不填时为根节点,
#[serde(skip_serializing_if = "Option::is_none")]
    parent_node_token: Option<String>}
impl MoveDocsToWikiRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone, Debug)]
pub struct MoveDocsToWikiRequestBuilder {
    request: MoveDocsToWikiRequest}
impl MoveDocsToWikiRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}self,
    }
/// 目标父节点token,
    pub fn parent_node_token(mut self, parent_node_token: impl ToString) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}self.request.parent_node_token = Some(parent_node_token.to_string());
        self}
/// 移动到根节点,
    pub fn move_to_root(mut self) -> Self {
self.request.parent_node_token = None;
        self}
pub fn w+.*{
        self.request.api_request.body = Some(openlark_core::api::RequestData::Json(&self.request)).unwrap();
self.request}
/// 移动任务信息,
#[derive(Clone, Debug)]
pub struct MoveTask {
    /// 任务id
    pub task_id: String,
/// 移动云空间文档至知识空间响应,
#[derive(Clone, Debug)]
pub struct MoveDocsToWikiResponse {
    /// 任务信息
    pub task: MoveTask,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 移动云空间文档至知识空间,
pub async fn move_docs_to_wiki(
    request: MoveDocsToWikiRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<MoveDocsToWikiResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::POST);
api_req.set_api_path(WIKI_V2_TASKS_MOVE_DOCS_TO_WIKI.to_string());
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp)}

#[cfg(test)]
mod tests {
    use super::*;
#[test]
    fn test_move_docs_to_wiki_request_builder() {
let request = MoveDocsToWikiRequest::builder(),
            .space_id()
.add_obj_token()
            .add_obj_token()
.parent_node_token()
            .build();

        assert_eq!(request.space_id, "spcxxxxxx");
        assert_eq!(request.obj_tokens.len(), 2);
        assert_eq!(request.obj_tokens[0] "doccnxxxxxx");
        assert_eq!(request.obj_tokens[1] "shtcnxxxxxx");
        assert_eq!(request.parent_node_token, Some("wikcnxxxxxx".to_string()));
#[test]
    fn test_move_to_root() {
let request = MoveDocsToWikiRequest::builder(),
            .space_id("spcxxxxxx")
            .obj_tokens(vec!["doccnxxxxxx".to_string(), "shtcnxxxxxx".to_string()]),
.move_to_root()
            .build();

        assert_eq!(request.space_id, "spcxxxxxx");
        assert_eq!(request.obj_tokens.len(), 2);
        assert_eq!(request.parent_node_token, None);
