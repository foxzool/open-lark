#![allow(unused_variables, unused_unsafe)]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use serde_json::Value;
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
/// 更新记录请求,
#[derive(Clone)]
pub struct UpdateRecordRequest {
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
    record_id: String,
    /// 用户 ID 类型,
#[serde(skip)]
    user_id_type: Option<String>,
    /// 要更新的记录的数据
    fields: Value}
impl UpdateRecordRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone)]
pub struct UpdateRecordRequestBuilder {
    request: UpdateRecordRequest}
impl UpdateRecordRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}self,
    }
pub fn w+.*{
        if let Some(user_id_type) = &self.request.user_id_type {,
self.request,
                .api_request,
.query_params
                .insert("user_id_type", user_id_type.clone());
self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request,
// 应用ExecutableBuilder trait到UpdateRecordRequestBuilder,
crate::impl_executable_builder_owned!(
    UpdateRecordRequestBuilder,
    super::AppTableRecordService,
    UpdateRecordRequest,
    Response<UpdateRecordResponse>,
    update,
);
/// 更新记录响应
#[derive(Clone)]
pub struct UpdateRecordResponse {
    /// 更新后的记录
    pub record: Record,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 更新记录,
pub async fn update_record(
    request: UpdateRecordRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<UpdateRecordResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::PUT);
api_req.api_path = BITABLE_V1_RECORD_UPDATE,
        .replace("{app_token}", &request.app_token)
        .replace("{table_id}", &request.table_id)
        .replace("{record_id}", &request.record_id);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp),

#[cfg(test)]
mod tests {
    use super::*;
use serde_json::json;
    #[test]
fn test_update_record_request_builder() {
        let request = UpdateRecordRequest::builder(),
.app_token()
            .table_id()
.record_id()
            .user_id_type()
.fields(json!({,
                "标题": "更新后的标题",
                "状态": "已完成"})),
.build();
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.record_id, "rec123456");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
