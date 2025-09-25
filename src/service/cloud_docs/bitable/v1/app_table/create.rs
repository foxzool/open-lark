use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    impl_executable_builder_owned,
};

use super::AppTableService;

impl AppTableService {
    /// 新增一个数据表
    pub async fn create(
        &self,
        request: CreateTableRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<CreateTableResponse>> {
        let mut api_req = request.api_request;
        api_req.http_method = Method::POST;
        api_req.api_path = BITABLE_V1_TABLE_CREATE.replace("{app_token}", &request.app_token);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];
        api_req.body = serde_json::to_vec(&CreateTableRequestBody {
            table: request.table,
        })?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }
}

/// 新增数据表请求
#[derive(Debug, Default)]
pub struct CreateTableRequest {
    api_request: ApiRequest,
    /// 多维表格的 app_token
    app_token: String,
    /// 数据表信息
    table: TableData,
}

impl CreateTableRequest {
    pub fn builder() -> CreateTableRequestBuilder {
        CreateTableRequestBuilder::default()
    }

    /// 创建新增数据表请求
    pub fn new(app_token: impl ToString, table: TableData) -> Self {
        Self {
            api_request: ApiRequest::default(),
            app_token: app_token.to_string(),
            table,
        }
    }
}

#[derive(Default)]
pub struct CreateTableRequestBuilder {
    request: CreateTableRequest,
}

impl CreateTableRequestBuilder {
    /// 多维表格的 app_token
    pub fn app_token(mut self, app_token: impl ToString) -> Self {
        self.request.app_token = app_token.to_string();
        self
    }

    /// 数据表信息
    pub fn table(mut self, table: TableData) -> Self {
        self.request.table = table;
        self
    }

    pub fn build(self) -> CreateTableRequest {
        self.request
    }
}

impl_executable_builder_owned!(
    CreateTableRequestBuilder,
    AppTableService,
    CreateTableRequest,
    BaseResponse<CreateTableResponse>,
    create
);

/// 数据表数据
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
    /// 创建数据表数据
    pub fn new(name: impl ToString) -> Self {
        Self {
            name: name.to_string(),
            default_view_name: None,
            fields: None,
        }
    }

    /// 设置默认视图名称
    pub fn with_default_view_name(mut self, view_name: impl ToString) -> Self {
        self.default_view_name = Some(view_name.to_string());
        self
    }

    /// 设置初始字段
    pub fn with_fields(mut self, fields: Vec<TableField>) -> Self {
        self.fields = Some(fields);
        self
    }
}

/// 字段信息
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    /// 创建文本字段
    pub fn text(name: impl ToString) -> Self {
        Self {
            field_name: name.to_string(),
            field_type: 1, // 多行文本
            property: None,
        }
    }

    /// 创建数字字段
    pub fn number(name: impl ToString) -> Self {
        Self {
            field_name: name.to_string(),
            field_type: 2, // 数字
            property: None,
        }
    }

    /// 创建单选字段
    pub fn single_select(name: impl ToString, options: Vec<String>) -> Self {
        let options_value: Vec<serde_json::Value> = options
            .into_iter()
            .map(|opt| serde_json::json!({"name": opt}))
            .collect();

        Self {
            field_name: name.to_string(),
            field_type: 3, // 单选
            property: Some(serde_json::json!({"options": options_value})),
        }
    }

    /// 创建多选字段
    pub fn multi_select(name: impl ToString, options: Vec<String>) -> Self {
        let options_value: Vec<serde_json::Value> = options
            .into_iter()
            .map(|opt| serde_json::json!({"name": opt}))
            .collect();

        Self {
            field_name: name.to_string(),
            field_type: 4, // 多选
            property: Some(serde_json::json!({"options": options_value})),
        }
    }

    /// 创建日期字段
    pub fn date(name: impl ToString) -> Self {
        Self {
            field_name: name.to_string(),
            field_type: 5, // 日期
            property: None,
        }
    }
}

#[derive(Serialize)]
struct CreateTableRequestBody {
    table: TableData,
}

#[derive(Deserialize, Debug)]
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
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    #[test]
    fn test_create_table_request() {
        let table = TableData::new("测试数据表")
            .with_default_view_name("主视图")
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
    }

    #[test]
    fn test_table_field_types() {
        let text_field = TableField::text("标题");
        assert_eq!(text_field.field_type, 1);
        assert_eq!(text_field.field_name, "标题");

        let number_field = TableField::number("数量");
        assert_eq!(number_field.field_type, 2);

        let select_field =
            TableField::single_select("状态", vec!["A".to_string(), "B".to_string()]);
        assert_eq!(select_field.field_type, 3);
        assert!(select_field.property.is_some());
    }
}
