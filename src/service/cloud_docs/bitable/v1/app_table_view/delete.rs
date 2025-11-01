use crate::core::SDKResult;use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;use serde::Deserialize;
use crate::,
{,
    core::,
{,
        BaseResponse,
        ResponseFormat,
        api_resp::{ApiResponseTrait}
    config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
};
    impl_executable_builder_config,
};
use super::AppTableViewService;
impl AppTableViewService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 删除视图,
pub async fn delete_view(
    request: DeleteViewRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<DeleteViewResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::DELETE);
api_req.api_path = BITABLE_V1_VIEW_DELETE,
        .replace("{app_token}", &request.app_token)
        .replace("{table_id}", &request.table_id)
        .replace("{view_id}", &request.view_id);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp),

/// 删除视图请求
#[derive(Debug, Clone)]
pub struct DeleteViewRequest {
    api_request: ApiRequest,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 视图的 view_id
    view_id: String}
impl DeleteViewRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct DeleteViewRequestBuilder {
    request: DeleteViewRequest}
impl DeleteViewRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_config!(,
    DeleteViewRequestBuilder,
    DeleteViewRequest,
    BaseResponse<DeleteViewResponse>,
    delete_view,
);
#[derive(Debug, Clone)]
pub struct DeleteViewResponse {
    /// 删除结果
    pub deleted: bool,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
#[test]
    fn test_delete_view_request() {
let request = DeleteViewRequest::builder(),
            .app_token()
.table_id()
            .view_id()
.build();
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.view_id, "vewTpR1urY");
#[test]
    fn test_delete_view_request_new() {
let request =,
            DeleteViewRequest::new("bascnmBA*****yGehy8", "tblsRc9GRRXKqhvW", "vewTpR1urY");

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.view_id, "vewTpR1urY");
