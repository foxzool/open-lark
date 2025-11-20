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
/// 获取知识空间列表请求,
#[derive(Clone, Debug)]
pub struct ListSpaceRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 页大小,
#[serde(skip)]
    page_size: Option<i32>,
    /// 页标记，第一次请求不填，表示从头开始遍历,
#[serde(skip)]
    page_token: Option<String>}
impl ListSpaceRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone, Debug)]
pub struct ListSpaceRequestBuilder {
    request: ListSpaceRequest}
impl ListSpaceRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}if let Some(page_token) = &self.request.page_token {,
            self.request,
.api_request,
                .query_params
                .insert("page_token", page_token.clone());
self.request,
    }
/// 知识空间信息,
#[derive(Clone, Debug)]
pub struct Space {
    /// 知识空间id
    pub space_id: String,
    /// 知识空间名称
    pub name: String,
    /// 知识空间描述,
#[serde(default)]
    pub description: Option<String>,
    /// 知识空间类型
    pub space_type: Option<String>,
    /// 知识空间可见性
    pub visibility: Option<String>}
/// 获取知识空间列表响应,
#[derive(Clone, Debug)]
pub struct ListSpaceResponse {
    /// 知识空间列表
    pub items: Vec<Space>,
    /// 分页标记，当has_more为true时，会同时返回新的page_token,
#[serde(default)]
    pub page_token: Option<String>,
    /// 是否还有更多项
    pub has_more: bool,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 获取知识空间列表,
pub async fn list_spaces(
    request: ListSpaceRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<ListSpaceResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::GET);
api_req.set_api_path(WIKI_V2_SPACES.to_string());
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp)}

#[cfg(test)]
mod tests {
    use super::*;
#[test]
    fn test_list_space_request_builder() {
let request = ListSpaceRequest::builder(),
            .page_size()
.page_token()
            .build();

        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.page_token, Some("page_token_123".to_string()));
