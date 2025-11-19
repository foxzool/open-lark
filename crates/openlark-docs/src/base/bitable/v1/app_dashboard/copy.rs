#![allow(unused_variables, unused_unsafe)]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use SDKResult;use reqwest::Method;
use openlark_core::api::ApiRequest;use serde::{Deserialize, Serialize};
use openlark_core::,
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
    impl_executable_builder_config,
};
/// 复制仪表盘请求,
#[derive(Clone)]
pub struct CopyDashboardRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符,
#[serde(skip)]
    app_token: String,
    /// 仪表盘ID,
#[serde(skip)]
    block_id: String,
    /// 复制后的仪表盘名称,
#[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>}
impl CopyDashboardRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone)]
pub struct CopyDashboardRequestBuilder {
    request: CopyDashboardRequest}
impl CopyDashboardRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_config!(,
    CopyDashboardRequestBuilder,
    CopyDashboardRequest,
    Response<CopyDashboardResponse>,
    copy_dashboard,
);
/// 仪表盘信息
#[derive(Clone)]
pub struct Dashboard {
    /// 仪表盘ID
    pub block_id: String,
    /// 仪表盘名称
    pub name: String,
/// 复制仪表盘响应,
#[derive(Clone)]
pub struct CopyDashboardResponse {
    /// 复制后的仪表盘信息
    pub dashboard: Dashboard,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 复制仪表盘,
pub async fn copy_dashboard(
    request: CopyDashboardRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<CopyDashboardResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::POST);
api_req.api_path = BITABLE_V1_DASHBOARD_COPY,
        .replace("{app_token}", &request.app_token)
        .replace("{block_id}", &request.block_id);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp),

#[cfg(test)]
mod tests {
    use super::*;
#[test]
    fn test_copy_dashboard_request_builder() {
let request = CopyDashboardRequest::builder(),
            .app_token()
.block_id()
            .name()
.build();
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.block_id, "blkxxxxxx");
        assert_eq!(request.name, Some("复制的仪表盘".to_string()));
