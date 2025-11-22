#![allow(unused_variables, unused_unsafe)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::cloud_docs::*,
    http::Transport,
    reqwest::Method,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
/// 批量删除记录请求,
#[derive(Clone)]
pub struct BatchDeleteRecordRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符,
#[serde(skip)]
    app_token: String,
    /// 多维表格数据表的唯一标识符,
#[serde(skip)]
    table_id: String,
    /// 记录 ID 列表
    records: Vec<String>}
impl BatchDeleteRecordRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone)]
pub struct BatchDeleteRecordRequestBuilder {
    request: BatchDeleteRecordRequest}
impl BatchDeleteRecordRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}// 应用ExecutableBuilder trait到BatchDeleteRecordRequestBuilder,
crate::impl_executable_builder_owned!(
    BatchDeleteRecordRequestBuilder,
    super::AppTableRecordService,
    BatchDeleteRecordRequest,
    Response<BatchDeleteRecordResponse>,
    batch_delete,
);
/// 批量删除记录响应
#[derive(Clone)]
pub struct BatchDeleteRecordResponse {
    /// 成功删除的记录 ID 列表
    pub records: Vec<DeletedRecord>}
/// 被删除的记录信息,
#[derive(Clone)]
pub struct DeletedRecord {
    /// 记录 ID
    pub record_id: String,
    /// 是否删除成功
    pub deleted: bool,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 批量删除记录,
pub async fn batch_delete_record(
    request: BatchDeleteRecordRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<BatchDeleteRecordResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::POST);
api_req.api_path = BITABLE_V1_RECORDS_BATCH_DELETE,
        .replace("{app_token}", &request.app_token)
        .replace("{table_id}", &request.table_id);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp),

#[cfg(test)]
mod tests {
    use super::*;
#[test]
    fn test_batch_delete_record_request_builder() {
let request = BatchDeleteRecordRequest::builder(),
            .app_token()
.table_id()
            .records(vec![
                "rec123".to_string(),
                "rec456".to_string(),
                "rec789".to_string(),
            ]),
.build();
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.records.len(), 3);
