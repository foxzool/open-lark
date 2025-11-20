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
/// 获取知识空间成员列表请求,
#[derive(Clone, Debug)]
pub struct ListSpaceMemberRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 知识空间id,
#[serde(skip)]
    space_id: String,
    /// 页大小,
#[serde(skip)]
    page_size: Option<i32>,
    /// 页标记，第一次请求不填，表示从头开始遍历,
#[serde(skip)]
    page_token: Option<String>}
impl ListSpaceMemberRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone, Debug)]
pub struct ListSpaceMemberRequestBuilder {
    request: ListSpaceMemberRequest}
impl ListSpaceMemberRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}if let Some(page_token) = &self.request.page_token {,
            self.request,
.api_request,
                .query_params
                .insert("page_token", page_token.clone());
self.request,
    }
/// 知识空间成员信息,
#[derive(Clone, Debug)]
pub struct SpaceMember {
    /// 成员类型：user
    pub member_type: String,
    /// 成员id，根据member_type决定
    pub member_id: String,
    /// 成员权限角色：admin(管理员)、edit_member(协作者)、view_member(阅读者)
    pub role: String,
    /// 成员类型：user
    pub r#type: Option<String>}
/// 获取知识空间成员列表响应,
#[derive(Clone, Debug)]
pub struct ListSpaceMemberResponse {
    /// 成员列表
    pub items: Vec<SpaceMember>,
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
/// 获取知识空间成员列表,
pub async fn list_space_members(
    request: ListSpaceMemberRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<ListSpaceMemberResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::GET);
api_req.set_api_path(EndpointBuilder::replace_param(,
        WIKI_V2_SPACE_MEMBERS,
        "space_id",
        &request.space_id,
    ));
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp)}

#[cfg(test)]
mod tests {
    use super::*;
#[test]
    fn test_list_space_member_request_builder() {
let request = ListSpaceMemberRequest::builder(),
            .space_id()
.page_size()
            .page_token()
.build();
        assert_eq!(request.space_id, "spcxxxxxx");
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.page_token, Some("page_token_123".to_string()));
