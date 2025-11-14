#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use SDKResult;use reqwest::Method;
use openlark_core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
use openlark_core::,
{
    core::,
{,
        BaseResponse,
        ResponseFormat,
        api_resp::{ApiResponseTrait}
    constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
};
    impl_executable_builder_owned,
};
use super::AppTableService;
impl AppTableService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 更新数据表请求,
#[derive(Clone)]
pub struct PatchTableRequest {
    api_request: ApiRequest,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 数据表的新名称
    name: Option<String>}
impl PatchTableRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone)]
pub struct PatchTableRequestBuilder {
    request: PatchTableRequest}
impl PatchTableRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    PatchTableRequestBuilder,
    AppTableService,
    PatchTableRequest,
    BaseResponse<PatchTableResponse>,
    patch,
);
#[derive(Serialize)]
struct PatchTableRequestBody {,
#[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>}

#[derive(Clone)]
pub struct PatchTableResponse {
    /// 数据表的名称
    pub name: String,
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
use serde_json::json;
    #[test]
fn test_patch_table_request() {
        let request = PatchTableRequest::builder(),
.app_token()
            .table_id()
.name()
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.name, Some("更新后的表名".to_string()));
#[test]
    ,
        let request = PatchTableRequest::new("bascnmBA*****yGehy8", "tblsRc9GRRXKqhvW");
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.name, None);
#[test]
    fn test_patch_table_request_body_serialization() {
let body = PatchTableRequestBody {,
            name: Some("新表名".to_string())};
let serialized = serde_json::to_value(&body).unwrap();
        let expected = json!({,
"name": "新表名"});

        assert_eq!(serialized, expected);
