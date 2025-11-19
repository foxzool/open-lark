#![allow(unused_variables, unused_unsafe)]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use std::collections::HashMap;
use SDKResult;use log::error;
use openlark_core::api::ApiRequest;use reqwest::Method;
use serde::{Deserialize, Serialize};
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
        standard_response::StandardResponse,
        SDKResult,
};
    service::bitable::v1::Record,
};
/// 新增记录请求,
#[derive(Clone)]
pub struct CreateRecordRequest {
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
    /// 要新增的记录的数据,
#[serde(flatten)]
    fields: Record}
impl CreateRecordRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone)]
pub struct CreateRecordRequestBuilder {
    request: CreateRecordRequest}
impl CreateRecordRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}if let Some(client_token) = &self.request.client_token {,
            self.request,
.api_request,
                .query_params
                .insert("client_token", client_token.clone());
match serde_json::to_vec(&self.request) {,
            Ok(bytes) => {,
self.request.api_request.body = bytes;
            }
Err(e) => {,
                error!("Failed to serialize create record request: {}", e);
self.request.api_request.body = Vec::new();
            }
self.request,
    }
// 应用ExecutableBuilder trait到CreateRecordRequestBuilder,
crate::impl_executable_builder_owned!(
    CreateRecordRequestBuilder,
    super::AppTableRecordService,
    CreateRecordRequest,
    CreateRecordResponse,
    create,
);
/// 新增记录响应
#[derive(Clone)]
pub struct CreateRecordResponse {
    /// 新增的记录
    pub record: Record,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 新增记录,
///
/// # API文档,
///,
/// https://open.feishu.cn/document/ukTMukTMukTM/uMzM1YjLzMTN24yMzUjN/hire-v1/job/combined_create,
pub async fn create_record(,
    request: CreateRecordRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<CreateRecordResponse> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::POST);
api_req.api_path = BITABLE_V1_RECORDS,
        .replace("{app_token}", &request.app_token)
        .replace("{table_id}", &request.table_id);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
let api_resp: Response<CreateRecordResponse> =,
        Transport::request(api_req, config, option).await?;
api_resp.into_result(),

#[cfg(test)]
mod tests {
    use super::*;
use serde_json::json;
    #[test]
fn test_create_record_request_builder() {
        let record = Record {
            record_id: None,
            fields: std::collections::HashMap::from([
                ("标题".to_string(), json!("测试记录")),
                ("状态".to_string(), json!("进行中")),
            ]),
            created_by: None,
            created_time: None,
            last_modified_by: None,
            last_modified_time: None};
let request = CreateRecordRequest::builder(),
            .app_token()
.table_id()
            .user_id_type()
.fields()
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
assert!(request.fields.fields.contains_key("标题"));
    }
