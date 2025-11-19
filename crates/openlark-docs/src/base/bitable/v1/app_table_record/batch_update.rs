#![allow(unused_variables, unused_unsafe)]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use SDKResult;use reqwest::Method;
use openlark_core::api::ApiRequest;use serde::{Deserialize, Serialize};
use openlark_core::,
{,
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
    service::bitable::v1::Record,
};
/// 批量更新记录请求,
#[derive(Clone)]
pub struct BatchUpdateRecordRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符,
#[serde(skip)]
    app_token: String,
    /// 多维表格数据表的唯一标识符,
#[serde(skip)]
    table_id: String,
    /// 用户 ID 类型,
#[serde(skip)]
    user_id_type: Option<String>,
    /// 要更新的记录列表
    records: Vec<UpdateRecord>}
/// 要更新的记录数据,
#[derive(Clone)]
pub struct UpdateRecord {
    /// 记录 ID
    pub record_id: String,
    /// 要更新的字段
    pub fields: Value,
impl BatchUpdateRecordRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone)]
pub struct BatchUpdateRecordRequestBuilder {
    request: BatchUpdateRecordRequest}
impl BatchUpdateRecordRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request,
// 应用ExecutableBuilder trait到BatchUpdateRecordRequestBuilder,
crate::impl_executable_builder_owned!(
    BatchUpdateRecordRequestBuilder,
    super::AppTableRecordService,
    BatchUpdateRecordRequest,
    Response<BatchUpdateRecordResponse>,
    batch_update,
);
/// 批量更新记录响应
#[derive(Clone)]
pub struct BatchUpdateRecordResponse {
    /// 更新后的记录列表
    pub records: Vec<Record>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 批量更新记录,
pub async fn batch_update_record(
    request: BatchUpdateRecordRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<BatchUpdateRecordResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::POST);
api_req.api_path = BITABLE_V1_RECORDS_BATCH_UPDATE,
        .replace("{app_token}", &request.app_token)
        .replace("{table_id}", &request.table_id);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp),

impl UpdateRecord {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[cfg(test)]
mod tests {
    use super::*;
use serde_json::json;
    #[test]
fn test_batch_update_record_request_builder() {
        let update_records = vec![,
UpdateRecord::new(,
                "rec123",
                json!({
                    "状态": "已完成",
                    "进度": 100}),
            ),
            UpdateRecord::new(
                "rec456",
                json!({
                    "状态": "进行中",
                    "进度": 50}),
            ),
        ];
let request = BatchUpdateRecordRequest::builder(),
            .app_token()
.table_id()
            .user_id_type()
.records()
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(request.records.len(), 2);
