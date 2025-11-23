//! 创建数据表模块

use openlark_core::{
    core::{
        BaseResponse,
        ResponseFormat,
        api::ApiResponseTrait,
    },
    constants::AccessTokenType,
    endpoints::cloud_docs::*,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 新增数据表请求
#[derive(Clone)]
pub struct CreateTableRequest {
    api_request: openlark_core::api::ApiRequest,
    /// 多维表格的 app_token
    pub app_token: String,
    /// 数据表信息
    pub table: TableData,
}

impl CreateTableRequest {
    pub fn new(config: openlark_core::Config) -> Self {
        Self {
            api_request: openlark_core::api::ApiRequest::new(
                config,
                reqwest::Method::POST,
                CREATE_TABLE.to_string(),
            ),
            app_token: String::new(),
            table: TableData::default(),
        }
    }

    pub fn builder() -> CreateTableRequestBuilder {
        CreateTableRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct CreateTableRequestBuilder {
    request: CreateTableRequest,
}

impl CreateTableRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.request.app_token = app_token.into();
        self
    }

    pub fn table(mut self, table: TableData) -> Self {
        self.request.table = table;
        self
    }

    pub fn build(self) -> CreateTableRequest {
        self.request
    }
}

/// 数据表数据
#[derive(Clone, Default, Serialize)]
pub struct TableData {
    /// 数据表名称
    pub name: String,
    /// 数据表的默认视图名称，不填则默认为"数据表"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_view_name: Option<String>,
    /// 数据表初始字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<Vec<TableField>>,
}

impl TableData {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            default_view_name: None,
            fields: None,
        }
    }

    /// 设置默认视图名称
    pub fn with_default_view_name(mut self, view_name: impl Into<String>) -> Self {
        self.default_view_name = Some(view_name.into());
        self
    }

    /// 设置初始字段
    pub fn with_fields(mut self, fields: Vec<TableField>) -> Self {
        self.fields = Some(fields);
        self
    }
}

/// 字段信息
#[derive(Clone, Serialize)]
pub struct TableField {
    /// 字段名称
    pub field_name: String,
    /// 字段类型
    #[serde(rename = "type")]
    pub field_type: i32,
    /// 字段属性，不同字段类型对应不同的属性结构
    #[serde(skip_serializing_if = "Option::is_none")]
    pub property: Option<serde_json::Value>,
}

impl TableField {
    pub fn new(name: impl Into<String>, field_type: i32) -> Self {
        Self {
            field_name: name.into(),
            field_type,
            property: None,
        }
    }

    /// 创建文本字段
    pub fn text(name: impl Into<String>) -> Self {
        Self::new(name, 1) // 文本
    }

    /// 创建数字字段
    pub fn number(name: impl Into<String>) -> Self {
        Self::new(name, 2) // 数字
    }

    /// 创建单选字段
    pub fn single_select(name: impl Into<String>, options: Vec<String>) -> Self {
        let options_value: Vec<serde_json::Value> = options
            .into_iter()
            .map(|opt| serde_json::json!({"name": opt}))
            .collect();

        Self {
            field_name: name.into(),
            field_type: 3, // 单选
            property: Some(serde_json::json!({"options": options_value})),
        }
    }

    /// 创建多选字段
    pub fn multi_select(name: impl Into<String>, options: Vec<String>) -> Self {
        let options_value: Vec<serde_json::Value> = options
            .into_iter()
            .map(|opt| serde_json::json!({"name": opt}))
            .collect();

        Self {
            field_name: name.into(),
            field_type: 4, // 多选
            property: Some(serde_json::json!({"options": options_value})),
        }
    }

    /// 创建日期字段
    pub fn date(name: impl Into<String>) -> Self {
        Self::new(name, 5) // 日期
    }
}

#[derive(Serialize)]
struct CreateTableRequestBody {
    table: TableData,
}

/// 创建数据表响应
#[derive(Clone)]
pub struct CreateTableResponse {
    /// 数据表信息
    pub table_id: String,
    /// 数据表的默认视图 ID
    pub default_view_id: String,
    /// 数据表初始字段的 field_id 列表
    pub field_id_list: Vec<String>,
}

impl ApiResponseTrait for CreateTableResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_table_request() {
        let table = TableData::new("测试数据表")
            .with_default_view_name("默认视图")
            .with_fields(vec![
                TableField::text("标题"),
                TableField::number("数量"),
                TableField::single_select("状态", vec!["进行中".to_string(), "已完成".to_string()]),
            ]);

        let request = CreateTableRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table(table)
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table.name, "测试数据表");
        assert_eq!(request.table.default_view_name, Some("默认视图".to_string()));
        assert!(request.table.fields.is_some());
    }

    #[test]
    fn test_table_field_types() {
        let text_field = TableField::text("标题");
        assert_eq!(text_field.field_type, 1);
        assert_eq!(text_field.field_name, "标题");

        let number_field = TableField::number("数量");
        assert_eq!(number_field.field_type, 2);

        let select_field = TableField::single_select("状态", vec!["A".to_string(), "B".to_string()]);
        assert_eq!(select_field.field_type, 3);
        assert!(select_field.property.is_some());
    }

    #[test]
    fn test_table_data_serialization() {
        let table = TableData::new("测试表")
            .with_fields(vec![
                TableField::text("字段1"),
                TableField::number("字段2"),
            ]);

        let body = CreateTableRequestBody { table };
        let serialized = serde_json::to_value(&body).unwrap();

        assert_eq!(serialized["table"]["name"], "测试表");
        assert!(serialized["table"]["fields"].is_array());
        assert_eq!(serialized["table"]["fields"].as_array().unwrap().len(), 2);
    }
}