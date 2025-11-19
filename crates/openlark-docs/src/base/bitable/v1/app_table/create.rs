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
use super::AppTableService;
impl AppTableService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 新增数据表请求,
#[derive(Clone)]
pub struct CreateTableRequest {
    api_request: ApiRequest,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表信息
    table: TableData}
impl CreateTableRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone)]
pub struct CreateTableRequestBuilder {
    request: CreateTableRequest}
impl CreateTableRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    CreateTableRequestBuilder,
    AppTableService,
    CreateTableRequest,
    Response<CreateTableResponse>,
    create,
);
/// 数据表数据
#[derive(Clone)]
pub struct TableData {
    /// 数据表名称
    pub name: String,
    /// 数据表的默认视图名称，不填则默认为"数据表",
#[serde(skip_serializing_if = "Option::is_none")]
    pub default_view_name: Option<String>,
    /// 数据表初始字段,
#[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<TableField>>}
impl TableData {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 设置默认视图名称,
    pub fn with_default_view_name(mut self, view_name: impl ToString) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}self.default_view_name = Some(view_name.to_string());
        self}
/// 设置初始字段,
    pub fn with_fields(mut self, fields: Vec<TableField>) -> Self {
self.fields = Some(fields);
        self}
/// 字段信息,
#[derive(Clone)]
pub struct TableField {
    /// 字段名称
    pub field_name: String,
    /// 字段类型,
#[serde(rename = "type")]
    pub field_type: i32,
    /// 字段属性，不同字段类型对应不同的属性结构,
#[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<serde_json::Value>}
impl TableField {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 创建数字字段,
    pub fn number(name: impl ToString) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}Self {
            field_name: name.to_string(),
            field_type: 2, // 数字
            property: None}
/// 创建单选字段,
    pub fn single_select(name: impl ToString, options: Vec<String>) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}let options_value: Vec<serde_json::Value> = options,
            .into_iter()
            .map(|opt| serde_json::json!({"name": opt})),
.collect();
        Self {
            field_name: name.to_string(),
            field_type: 3, // 单选
            property: Some(serde_json::json!({"options": options_value})),
        }
/// 创建多选字段,
    pub fn multi_select(name: impl ToString, options: Vec<String>) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}let options_value: Vec<serde_json::Value> = options,
            .into_iter()
            .map(|opt| serde_json::json!({"name": opt})),
.collect();
        Self {
            field_name: name.to_string(),
            field_type: 4, // 多选
            property: Some(serde_json::json!({"options": options_value})),
        }
/// 创建日期字段,
    pub fn date(name: impl ToString) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}Self {
            field_name: name.to_string(),
            field_type: 5, // 日期
            property: None}
#[derive(Clone)]
struct CreateTableRequestBody {
    table: TableData}

#[derive(Clone)]
pub struct CreateTableResponse {
    /// 数据表信息
    pub table_id: String,
    /// 数据表的默认视图 ID
    pub default_view_id: String,
    /// 数据表初始字段的 field_id 列表
    pub field_id_list: Vec<String>}
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
#[cfg(test)]
mod tests {
    use super::*;
#[test]
    fn test_create_table_request() {
let table = TableData::new("测试数据表"),
            .with_default_view_name()
.with_fields(vec![,
                TableField::text("标题"),
                TableField::number("数量"),
                TableField::single_select("状态", vec!["进行中".to_string(), "已完成".to_string()]),
            ]);
let request = CreateTableRequest::builder(),
            .app_token()
.table()
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table.name, "测试数据表");
#[test]
    fn test_table_field_types() {
let text_field = TableField::text("标题");
        assert_eq!(text_field.field_type, 1);
        assert_eq!(text_field.field_name, "标题");
let number_field = TableField::number("数量");
        assert_eq!(number_field.field_type, 2);
let select_field =,
            TableField::single_select("状态", vec!["A".to_string(), "B".to_string()]);
        assert_eq!(select_field.field_type, 3);
assert!(select_field.property.is_some());
    }
