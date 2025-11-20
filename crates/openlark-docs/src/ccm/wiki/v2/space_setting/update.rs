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
/// 更新知识空间设置请求,
#[derive(Clone, Debug)]
pub struct UpdateSpaceSettingRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 知识空间id,
#[serde(skip)]
    space_id: String,
    /// 是否开启评论：true(开启)、false(关闭),
#[serde(skip_serializing_if = "Option::is_none")]
    comment_enabled: Option<bool>,
    /// 是否开启复制：true(开启)、false(关闭),
#[serde(skip_serializing_if = "Option::is_none")]
    copy_enabled: Option<bool>}
impl UpdateSpaceSettingRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone, Debug)]
pub struct UpdateSpaceSettingRequestBuilder {
    request: UpdateSpaceSettingRequest}
impl UpdateSpaceSettingRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 知识空间设置,
#[derive(Clone, Debug)]
pub struct SpaceSetting {
    /// 知识空间id
    pub space_id: String,
    /// 是否开启评论
    pub comment_enabled: Option<bool>,
    /// 是否开启复制
    pub copy_enabled: Option<bool>}
/// 更新知识空间设置响应,
#[derive(Clone, Debug)]
pub struct UpdateSpaceSettingResponse {
    /// 更新后的空间设置
    pub setting: SpaceSetting,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 更新知识空间设置,
pub async fn update_space_setting(
    request: UpdateSpaceSettingRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<UpdateSpaceSettingResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::PUT);
api_req.set_api_path(EndpointBuilder::replace_param(,
        WIKI_V2_SPACE_SETTING_UPDATE,
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
    fn test_update_space_setting_request_builder() {
let request = UpdateSpaceSettingRequest::builder(),
            .space_id()
.enable_comment()
            .disable_copy()
.build();
        assert_eq!(request.space_id, "spcxxxxxx");
        assert_eq!(request.comment_enabled, Some(true));
        assert_eq!(request.copy_enabled, Some(false));
