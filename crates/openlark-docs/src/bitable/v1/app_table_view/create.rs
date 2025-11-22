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
}/// 新增视图请求,
#[derive(Clone)]
pub struct CreateViewRequest {
    api_request: ApiRequest,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表的 table_id
    table_id: String,
    /// 视图信息
    view: ViewData}
impl CreateViewRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone)]
pub struct CreateViewRequestBuilder {
    request: CreateViewRequest}
impl CreateViewRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    CreateViewRequestBuilder,
    super::AppTableViewService,
    CreateViewRequest,
    Response<CreateViewResponse>,
    create,
);
/// 视图数据
#[derive(Clone)]
pub struct ViewData {
    /// 视图名称
    pub view_name: String,
    /// 视图类型，可选值：grid (表格视图)、kanban (看板视图)、gallery (画册视图)、gantt (甘特视图),
#[serde(skip_serializing_if = "Option::is_none")]
    pub view_type: Option<String>,
    /// 视图的自定义属性，当前支持的视图自定义属性参考视图类型,
#[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<serde_json::Value>}
impl ViewData {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 创建表格视图,
    pub fn grid_view(view_name: impl ToString) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}Self {
            view_name: view_name.to_string(),
            view_type: Some("grid".to_string()),
            property: None}
/// 创建看板视图,
    pub fn kanban_view(view_name: impl ToString) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}Self {
            view_name: view_name.to_string(),
            view_type: Some("kanban".to_string()),
            property: None}
/// 创建画册视图,
    pub fn gallery_view(view_name: impl ToString) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}Self {
            view_name: view_name.to_string(),
            view_type: Some("gallery".to_string()),
            property: None}
/// 创建甘特视图,
    pub fn gantt_view(view_name: impl ToString) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}Self {
            view_name: view_name.to_string(),
            view_type: Some("gantt".to_string()),
            property: None}
/// 设置视图类型,
    pub fn with_view_type(mut self, view_type: impl ToString) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}self.view_type = Some(view_type.to_string());
        self}
/// 设置视图属性,
    pub fn with_property(mut self, property: serde_json::Value) -> Self {
self.property = Some(property);
        self}
#[derive(Clone)]
struct CreateViewRequestBody {
    view: ViewData}

#[derive(Clone)]
pub struct CreateViewResponse {
    /// 视图 ID
    pub view_id: String,
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
fn test_create_view_request() {
        let view = ViewData::grid_view("测试表格视图");
let request = CreateViewRequest::builder(),
            .app_token()
.table_id()
            .view()
.build();
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.view.view_name, "测试表格视图");
        assert_eq!(request.view.view_type, Some("grid".to_string()));
#[test]
    fn test_view_data_types() {
let grid_view = ViewData::grid_view("表格视图");
        assert_eq!(grid_view.view_type, Some("grid".to_string()));
let kanban_view = ViewData::kanban_view("看板视图");
        assert_eq!(kanban_view.view_type, Some("kanban".to_string()));
let gallery_view = ViewData::gallery_view("画册视图");
        assert_eq!(gallery_view.view_type, Some("gallery".to_string()));
let gantt_view = ViewData::gantt_view("甘特视图");
        assert_eq!(gantt_view.view_type, Some("gantt".to_string()));
#[test]
    fn test_view_data_with_property() {
let view = ViewData::new("自定义视图"),
            .with_view_type()
.with_property(json!({,
                "filter_info": {,
"conditions": []
}));

        assert_eq!(view.view_name, "自定义视图");
        assert_eq!(view.view_type, Some("grid".to_string()));
assert!(view.property.is_some());
    }
