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
/// 删除字段请求,
#[derive(Clone)]
pub struct DeleteFieldRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符,
#[serde(skip)]
    app_token: String,
    /// 多维表格数据表的唯一标识符,
#[serde(skip)]
    table_id: String,
    /// 字段的唯一标识符,
#[serde(skip)]
    field_id: String}
impl DeleteFieldRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone)]
pub struct DeleteFieldRequestBuilder {
    request: DeleteFieldRequest}
impl DeleteFieldRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_config!(,
    DeleteFieldRequestBuilder,
    DeleteFieldRequest,
    Response<DeleteFieldResponse>,
    delete_field,
);
/// 删除字段响应
#[derive(Clone)]
pub struct DeleteFieldResponse {
    /// 删除的字段 ID
    pub field_id: String,
    /// 是否删除成功
    pub deleted: bool,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 删除字段,
pub async fn delete_field(
    request: DeleteFieldRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<DeleteFieldResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::DELETE);
api_req.api_path = BITABLE_V1_FIELD_DELETE,
        .replace("{app_token}", &request.app_token)
        .replace("{table_id}", &request.table_id)
        .replace("{field_id}", &request.field_id);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp),

#[cfg(test)]
mod tests {
    use super::*;
#[test]
    fn test_delete_field_request_builder() {
let request = DeleteFieldRequest::builder(),
            .app_token()
.table_id()
            .field_id()
.build();
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.field_id, "fldxxxxxx");
