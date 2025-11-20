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
/// 创建知识空间请求,
#[derive(Clone, Debug)]
pub struct CreateSpaceRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 知识空间名称
    name: String,
    /// 知识空间描述,
#[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>}
impl CreateSpaceRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone, Debug)]
pub struct CreateSpaceRequestBuilder {
    request: CreateSpaceRequest}
impl CreateSpaceRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    CreateSpaceRequestBuilder,
    super::cloud_docs::wiki::v2::space::SpaceService,
    CreateSpaceRequest,
    Response<CreateSpaceResponse>,
    create,
);
/// 创建的知识空间信息
#[derive(Clone, Debug)]
pub struct CreatedSpace {
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
/// 创建知识空间响应,
#[derive(Clone, Debug)]
pub struct CreateSpaceResponse {
    /// 创建的知识空间信息
    pub space: CreatedSpace,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 创建知识空间,
pub async fn create_space(
    request: CreateSpaceRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<CreateSpaceResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::POST);
api_req.set_api_path(WIKI_V2_SPACES.to_string());
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp)}

#[cfg(test)]
mod tests {
    use super::*;
#[test]
    fn test_create_space_request_builder() {
let request = CreateSpaceRequest::builder(),
            .name()
.description()
            .build();

        assert_eq!(request.name, "我的知识空间");
assert_eq!(,
            request.description,
            Some("这是一个测试知识空间".to_string()),
);
    }
