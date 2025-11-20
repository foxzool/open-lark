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
/// 添加知识空间成员请求,
#[derive(Clone, Debug)]
pub struct CreateSpaceMemberRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 知识空间id,
#[serde(skip)]
    space_id: String,
    /// 成员类型：user
    member_type: String,
    /// 成员id，根据member_type决定
    member_id: String,
    /// 成员权限角色：admin(管理员)、edit_member(协作者)、view_member(阅读者)
    role: String}
impl CreateSpaceMemberRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone, Debug)]
pub struct CreateSpaceMemberRequestBuilder {
    request: CreateSpaceMemberRequest}
impl CreateSpaceMemberRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    CreateSpaceMemberRequestBuilder,
    super::cloud_docs::wiki::v2::space_member::SpaceMemberService,
    CreateSpaceMemberRequest,
    Response<CreateSpaceMemberResponse>,
    create,
);
/// 添加的成员信息
#[derive(Clone, Debug)]
pub struct CreatedMember {
    /// 成员类型：user
    pub member_type: String,
    /// 成员id
    pub member_id: String,
    /// 成员权限角色
    pub role: String,
/// 添加知识空间成员响应,
#[derive(Clone, Debug)]
pub struct CreateSpaceMemberResponse {
    /// 添加的成员信息
    pub member: CreatedMember,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 添加知识空间成员,
pub async fn create_space_member(
    request: CreateSpaceMemberRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<CreateSpaceMemberResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::POST);
api_req.set_api_path(EndpointBuilder::replace_param(,
        WIKI_V2_SPACE_MEMBER_CREATE,
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
    fn test_create_space_member_request_builder() {
let request = CreateSpaceMemberRequest::builder(),
            .space_id()
.member_type()
            .member_id()
.as_editor()
            .build();

        assert_eq!(request.space_id, "spcxxxxxx");
        assert_eq!(request.member_type, "user");
        assert_eq!(request.member_id, "ou_xxxxxx");
        assert_eq!(request.role, "edit_member");
