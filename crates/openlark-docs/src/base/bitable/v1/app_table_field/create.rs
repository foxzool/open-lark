#![allow(unused_variables, unused_unsafe)]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use SDKResult;use reqwest::Method;
use openlark_core::api::ApiRequest;use serde::{Deserialize, Serialize};
use super::AppTableFieldService;
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
    impl_executable_builder_owned,
    service::bitable::v1::app_table_field::{
        AppTableField, AppTableFieldDescription, AppTableFieldProperty, FieldType, UiType}
};
/// 新增字段请求,
#[derive(Clone)]
pub struct CreateFieldRequest {
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
    /// 多维表格字段名
    field_name: String,
    /// 多维表格字段类型
    r#type: FieldType,
    /// 字段属性,
#[serde(skip_serializing_if = "Option::is_none")]
    property: Option<AppTableFieldProperty>,
    /// 字段的描述,
#[serde(skip_serializing_if = "Option::is_none")]
    description: Option<AppTableFieldDescription>,
    /// 字段在界面上的展示类型,
#[serde(skip_serializing_if = "Option::is_none")]
    ui_type: Option<UiType>}
impl CreateFieldRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone)]
pub struct CreateFieldRequestBuilder {
    request: CreateFieldRequest}
impl CreateFieldRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}if let Some(client_token) = &self.request.client_token {,
            self.request,
.api_request,
                .query_params
                .insert("client_token", client_token.clone());
self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request,
impl_executable_builder_owned!(,
    CreateFieldRequestBuilder,
    AppTableFieldService,
    CreateFieldRequest,
    Response<CreateFieldResponse>,
    create,
);
/// 新增字段响应
#[derive(Clone)]
pub struct CreateFieldResponse {
    /// 新增的字段信息
    pub field: AppTableField,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 新增字段,
pub async fn create_field(
    request: CreateFieldRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<CreateFieldResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::POST);
api_req.api_path = BITABLE_V1_FIELDS,
        .replace("{app_token}", &request.app_token)
        .replace("{table_id}", &request.table_id);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp),

#[cfg(test)]
mod tests {
    use super::*;
#[test]
    fn test_create_field_request_builder() {
let property = AppTableFieldProperty::text();
        let request = CreateFieldRequest::builder(),
.app_token()
            .table_id()
.user_id_type()
            .field_name()
.field_type()
            .property()
.build();
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(request.field_name, "测试字段");
        assert_eq!(request.r#type, FieldType::Text);
