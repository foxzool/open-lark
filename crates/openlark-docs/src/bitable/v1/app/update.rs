#![allow(unused_variables, unused_unsafe)]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use SDKResult;use reqwest::Method;
use openlark_core::api::ApiRequest;use serde::{Deserialize, Serialize};
use openlark_core::,
{
    core::,
{,
        BaseResponse,
        ResponseFormat,
        api::{ApiResponseTrait}
    constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
};
    impl_executable_builder_owned,
};
use super::AppService;
impl AppService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 更新多维表格元数据请求,
#[derive(Clone)]
pub struct UpdateAppRequest {
    api_request: ApiRequest,
    /// 多维表格的 app_token
    app_token: String,
    /// 多维表格的名字
    name: Option<String>,
    /// 多维表格是否开启高级权限
    is_advanced: Option<bool>}
impl UpdateAppRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone)]
pub struct UpdateAppRequestBuilder {
    request: UpdateAppRequest}
impl UpdateAppRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    UpdateAppRequestBuilder,
    AppService,
    UpdateAppRequest,
    Response<UpdateAppResponse>,
    update,
);
#[derive(Serialize)]
struct UpdateAppRequestBody {,
#[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    is_advanced: Option<bool>}

#[derive(Clone)]
pub struct UpdateAppResponse {
    /// 多维表格的 app 信息
    pub app: UpdateAppResponseData,

#[derive(Clone)]
pub struct UpdateAppResponseData {
    /// 多维表格的 app_token
    pub app_token: String,
    /// 多维表格的名字
    pub name: String,
    /// 多维表格的版本号
    pub revision: i32,
    /// 多维表格是否开启了高级权限
    pub is_advanced: bool,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
#[cfg(test)]
mod tests {
    use super::*;
use serde_json::json;
    #[test]
fn test_update_app_request() {
        let request = UpdateAppRequest::builder(),
.app_token()
            .name()
.is_advanced()
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.name, Some("新的表格名称".to_string()));
        assert_eq!(request.is_advanced, Some(true));
#[test]
    fn test_update_app_request_new() {
let request = UpdateAppRequest::new("bascnmBA*****yGehy8");
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.name, None);
        assert_eq!(request.is_advanced, None);
#[test]
    fn test_update_app_request_body_serialization() {
let body = UpdateAppRequestBody {,
            name: Some("新的表格名称".to_string()),
            is_advanced: Some(true)};
let serialized = serde_json::to_value(&body).unwrap();
        let expected = json!({
            "name": "新的表格名称",
            "is_advanced": true});

        assert_eq!(serialized, expected);
