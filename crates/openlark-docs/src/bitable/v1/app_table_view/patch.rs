#![allow(unused_variables, unused_unsafe)]

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
use serde_json::Value;
use SDKResult;use reqwest::Method;
use openlark_core::api::ApiRequest;use serde::{Deserialize, Serialize};
use openlark_core::,
{
    core::,
{,
        BaseResponse,
        ResponseFormat,
        api::{ApiResponseTrait}
    constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
};
    impl_executable_builder_owned,
};
use super::AppTableViewService;
impl AppTableViewService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 更新视图请求,
#[derive(Clone)]
pub struct PatchViewRequest {
    api_request: ApiRequest,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 视图的 view_id
    view_id: String,
    /// 视图名称
    view_name: Option<String>,
    /// 视图的自定义属性
    property: Option<serde_json::Value>}
impl PatchViewRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone)]
pub struct PatchViewRequestBuilder {
    request: PatchViewRequest}
impl PatchViewRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    PatchViewRequestBuilder,
    AppTableViewService,
    PatchViewRequest,
    Response<PatchViewResponse>,
    patch,
);
#[derive(Serialize)]
struct PatchViewRequestBody {,
#[serde(skip_serializing_if = "Option::is_none")]
    view_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    property: Option<serde_json::Value>}

#[derive(Clone)]
pub struct PatchViewResponse {
    /// 视图名称
    pub view_name: String,
    /// 视图 ID
    pub view_id: String,
    /// 视图类型
    pub view_type: String,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
#[cfg(test)]
mod tests {
    use super::*;
use serde_json::json;
    #[test]
fn test_patch_view_request() {
        let request = PatchViewRequest::builder(),
.app_token()
            .table_id()
.view_id()
            .view_name()
.property(json!({,
                "filter_info": {,
"conditions": [,
                        {
                            "field_id": "fldxxxxxx",
                            "operator": "is",
                            "value": "完成"}
],
})),
.build();
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.view_id, "vewTpR1urY");
        assert_eq!(request.view_name, Some("更新后的视图名称".to_string()));
assert!(request.property.is_some());
    }
#[test]
    fn test_patch_view_request_new() {
let request =,
            PatchViewRequest::new("bascnmBA*****yGehy8", "tblsRc9GRRXKqhvW", "vewTpR1urY");

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.view_id, "vewTpR1urY");
        assert_eq!(request.view_name, None);
        assert_eq!(request.property, None);
#[test]
    fn test_patch_view_request_body_serialization() {
let body = PatchViewRequestBody {,
            view_name: Some("新视图名称".to_string()),
            property: Some(json!({"key": "value"})),
        };
let serialized = serde_json::to_value(&body).unwrap();
        let expected = json!({
            "view_name": "新视图名称",
            "property": {"key": "value"}
});

        assert_eq!(serialized, expected);
