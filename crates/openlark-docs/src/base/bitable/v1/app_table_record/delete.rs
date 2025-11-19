#![allow(unused_variables, unused_unsafe)]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
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
/// 删除记录请求,
#[derive(Clone)]
pub struct DeleteRecordRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符,
#[serde(skip)]
    app_token: String,
    /// 多维表格数据表的唯一标识符,
#[serde(skip)]
    table_id: String,
    /// 记录的唯一标识符,
#[serde(skip)]
    record_id: String}
impl DeleteRecordRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone)]
pub struct DeleteRecordRequestBuilder {
    request: DeleteRecordRequest}
impl DeleteRecordRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}// 应用ExecutableBuilder trait到DeleteRecordRequestBuilder,
crate::impl_executable_builder_owned!(
    DeleteRecordRequestBuilder,
    super::AppTableRecordService,
    DeleteRecordRequest,
    Response<DeleteRecordResponse>,
    delete,
);
/// 删除记录响应
#[derive(Clone)]
pub struct DeleteRecordResponse {
    /// 是否删除成功
    pub deleted: bool,
    /// 被删除的记录 ID
    pub record_id: String,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 删除记录,
pub async fn delete_record(
    request: DeleteRecordRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<DeleteRecordResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::DELETE);
api_req.api_path = BITABLE_V1_RECORD_DELETE,
        .replace("{app_token}", &request.app_token)
        .replace("{table_id}", &request.table_id)
        .replace("{record_id}", &request.record_id);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp),

#[cfg(test)]
mod tests {
    use super::*;
#[test]
    fn test_delete_record_request_builder() {
let request = DeleteRecordRequest::builder(),
            .app_token()
.table_id()
            .record_id()
.build();
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.record_id, "rec123456");
