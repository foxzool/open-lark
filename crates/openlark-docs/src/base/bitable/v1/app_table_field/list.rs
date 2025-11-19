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
    service::bitable::v1::app_table_field::AppTableField,
};
/// 列出字段请求,
#[derive(Clone)]
pub struct ListFieldRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符,
#[serde(skip)]
    app_token: String,
    /// 多维表格数据表的唯一标识符,
#[serde(skip)]
    table_id: String,
    /// 视图 ID,
#[serde(skip)]
    view_id: Option<String>,
    /// 控制字段描述数据的返回格式,
#[serde(skip)]
    text_field_as_array: Option<bool>,
    /// 分页标记,
#[serde(skip)]
    page_token: Option<String>,
    /// 分页大小,
#[serde(skip)]
    page_size: Option<i32>}
impl ListFieldRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone)]
pub struct ListFieldRequestBuilder {
    request: ListFieldRequest}
impl ListFieldRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}if let Some(text_field_as_array) = &self.request.text_field_as_array {,
            self.request,
.api_request,
                .query_params
                .insert("text_field_as_array", text_field_as_array.to_string());
if let Some(page_token) = &self.request.page_token {,
            self.request,
.api_request,
                .query_params
                .insert("page_token", page_token.clone());
if let Some(page_size) = &self.request.page_size {,
            self.request,
.api_request,
                .query_params
                .insert("page_size", page_size.to_string());
self.request,
    }
// 应用ExecutableBuilder trait到ListFieldRequestBuilder,
crate::impl_executable_builder_owned!(
    ListFieldRequestBuilder,
    super::AppTableFieldService,
    ListFieldRequest,
    Response<ListFieldResponse>,
    list,
);
/// 列出字段响应
#[derive(Clone)]
pub struct ListFieldResponse {
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
    /// 总数
    pub total: i32,
    /// 字段信息列表
    pub items: Vec<AppTableField>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 列出字段,
pub async fn list_field(
    request: ListFieldRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<ListFieldResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::GET);
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
    fn test_list_field_request_builder() {
let request = ListFieldRequest::builder(),
            .app_token()
.table_id()
            .page_size()
.text_field_as_array()
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.text_field_as_array, Some(true));
