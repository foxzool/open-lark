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
/// 删除知识空间成员请求,
#[derive(Clone, Debug)]
pub struct DeleteSpaceMemberRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 知识空间id,
#[serde(skip)]
    space_id: String,
    /// 成员id,
#[serde(skip)]
    member_id: String,
    /// 成员类型：user,
#[serde(skip)]
    member_type: Option<String>}
impl DeleteSpaceMemberRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone, Debug)]
pub struct DeleteSpaceMemberRequestBuilder {
    request: DeleteSpaceMemberRequest}
impl DeleteSpaceMemberRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}self.request,
    }
/// 删除知识空间成员响应,
#[derive(Clone, Debug)]
pub struct DeleteSpaceMemberResponse {
    /// 删除的成员id
    pub member_id: String,
    /// 是否删除成功
    pub deleted: bool,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 删除知识空间成员,
pub async fn delete_space_member(
    request: DeleteSpaceMemberRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<DeleteSpaceMemberResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::DELETE);
api_req.api_path = {,
        let mut path = EndpointBuilder::replace_param(
            WIKI_V2_SPACE_MEMBER_DELETE,
            "space_id",
            &request.space_id,
        );
        path = EndpointBuilder::replace_param(&path, "member_id", &request.member_id);
path};
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp),

#[cfg(test)]
mod tests {
    use super::*;
#[test]
    fn test_delete_space_member_request_builder() {
let request = DeleteSpaceMemberRequest::builder(),
            .space_id()
.member_id()
            .member_type()
.build();
        assert_eq!(request.space_id, "spcxxxxxx");
        assert_eq!(request.member_id, "ou_xxxxxx");
        assert_eq!(request.member_type, Some("user".to_string()));
