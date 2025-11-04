use crate::core::SDKResult;use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
use crate::,
{
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
    service::bitable::v1::Record,
};
/// 批量新增记录请求,
#[derive(Debug, Clone)]
pub struct BatchCreateRecordRequest {
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
    /// 格式为标准的 uuidv4，操作的唯一标识，用于幂等的进行更新操作,
#[serde(skip)]
    client_token: Option<String>,
    /// 要新增的记录列表
    records: Vec<Record>}
impl BatchCreateRecordRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct BatchCreateRecordRequestBuilder {
    request: BatchCreateRecordRequest}
impl BatchCreateRecordRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}if let Some(client_token) = &self.request.client_token {,
            self.request,
.api_request,
                .query_params
                .insert("client_token", client_token.clone());
self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request,
// 应用ExecutableBuilder trait到BatchCreateRecordRequestBuilder,
crate::impl_executable_builder_owned!(
    BatchCreateRecordRequestBuilder,
    super::AppTableRecordService,
    BatchCreateRecordRequest,
    BaseResponse<BatchCreateRecordResponse>,
    batch_create,
);
/// 批量新增记录响应
#[derive(Debug, Clone)]
pub struct BatchCreateRecordResponse {
    /// 新增的记录列表
    pub records: Vec<Record>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 批量新增记录,
pub async fn batch_create_record(
    request: BatchCreateRecordRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<BaseResponse<BatchCreateRecordResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::POST);
api_req.api_path = BITABLE_V1_RECORDS_BATCH_CREATE,
        .replace("{app_token}", &request.app_token)
        .replace("{table_id}", &request.table_id);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp),

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
use serde_json::json;
    use std::collections::HashMap;
#[test]
    fn test_batch_create_record_request_builder() {
let record1 = Record {,
            record_id: None,
            fields: HashMap::from([
                ("标题".to_string(), json!("记录1")),
                ("状态".to_string(), json!("待处理")),
            ]),
            created_by: None,
            created_time: None,
            last_modified_by: None,
            last_modified_time: None};
let record2 = Record {,
            record_id: None,
            fields: HashMap::from([
                ("标题".to_string(), json!("记录2")),
                ("状态".to_string(), json!("进行中")),
            ]),
            created_by: None,
            created_time: None,
            last_modified_by: None,
            last_modified_time: None};
let request = BatchCreateRecordRequest::builder(),
            .app_token()
.table_id()
            .user_id_type("open_id")
            .records()
.build();
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(request.records.len(), 2);
