#![allow(unused_variables, unused_unsafe)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use openlark_core::{
    api::ApiRequest,
    core::{BaseResponse, ResponseFormat, api::ApiResponseTrait},
    config::Config,
    constants::AccessTokenType,
    endpoints::cloud_docs::*,
    http::Transport,
    reqwest::Method,
    req_option::RequestOption,
    service::bitable::v1::TableField,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// 更新字段请求
#[derive(Clone)]
pub struct UpdateFieldRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 多维表格数据表的唯一标识符
    #[serde(skip)]
    table_id: String,
    /// 字段的唯一标识符
    #[serde(skip)]
    field_id: String,
    /// 用户 ID 类型
    #[serde(skip)]
    user_id_type: Option<String>,
    /// 多维表格字段名
    #[serde(skip)]
    field_name: Option<String>,
    /// 多维表格字段类型
    #[serde(skip)]
    field_type: Option<String>,
    /// 字段属性
    #[serde(skip)]
    property: Option<Value>,
    /// 字段的描述
    #[serde(skip)]
    description: Option<String>,
    /// 字段在界面上的展示类型
    #[serde(skip)]
    ui_type: Option<String>,
}

impl UpdateFieldRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new(config, Method::PUT, "/open-apis/bitable/v1/apps/{}/tables/{}/fields/{}".to_string()),
            app_token: String::new(),
            table_id: String::new(),
            field_id: String::new(),
            user_id_type: None,
            field_name: None,
            field_type: None,
            property: None,
            description: None,
            ui_type: None,
        }
    }

    pub fn builder() -> UpdateFieldRequestBuilder {
        UpdateFieldRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct UpdateFieldRequestBuilder {
    request: UpdateFieldRequest,
}

impl UpdateFieldRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.request.app_token = app_token.into();
        self
    }

    pub fn table_id(mut self, table_id: impl Into<String>) -> Self {
        self.request.table_id = table_id.into();
        self
    }

    pub fn field_id(mut self, field_id: impl Into<String>) -> Self {
        self.request.field_id = field_id.into();
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    pub fn field_name(mut self, field_name: impl Into<String>) -> Self {
        self.request.field_name = Some(field_name.into());
        self
    }

    pub fn field_type(mut self, field_type: impl Into<String>) -> Self {
        self.request.field_type = Some(field_type.into());
        self
    }

    pub fn property(mut self, property: Value) -> Self {
        self.request.property = Some(property);
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    pub fn ui_type(mut self, ui_type: impl Into<String>) -> Self {
        self.request.ui_type = Some(ui_type.into());
        self
    }

    pub fn build(self) -> UpdateFieldRequest {
        self.request
    }
}

/// 更新字段响应
#[derive(Clone)]
pub struct UpdateFieldResponse {
    /// 更新后的字段信息
    pub field: TableField,
}

impl ApiResponseTrait for UpdateFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 请求体结构
#[derive(Serialize)]
struct UpdateFieldRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    field_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    property: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ui_type: Option<String>,
}

/// 更新字段
pub async fn update_field(
    request: UpdateFieldRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<UpdateFieldResponse> {
    let mut api_req = request.api_request;
    api_req.set_http_method(Method::PUT);
    api_req.api_path = BITABLE_V1_FIELD_UPDATE
        .replace("{app_token}", &request.app_token)
        .replace("{table_id}", &request.table_id)
        .replace("{field_id}", &request.field_id);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req
            .query_params
            .insert("user_id_type".to_string(), user_id_type.clone());
    }

    // 设置请求体
    let body = UpdateFieldRequestBody {
        field_name: request.field_name,
        r#type: request.field_type,
        property: request.property,
        description: request.description,
        ui_type: request.ui_type,
    };

    api_req.body = serde_json::to_vec(&body).unwrap();

    let api_resp: openlark_core::core::StandardResponse<UpdateFieldResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_update_field_request_builder() {
        let property = json!({
            "options": [
                {"name": "选项1", "value": "option1"},
                {"name": "选项2", "value": "option2"}
            ]
        });

        let request = UpdateFieldRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .field_id("fldxxxxxx")
            .user_id_type("open_id")
            .field_name("更新后的字段名称")
            .field_type("text")
            .property(property)
            .description("字段描述")
            .ui_type("text")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.field_id, "fldxxxxxx");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(request.field_name, Some("更新后的字段名称".to_string()));
        assert_eq!(request.field_type, Some("text".to_string()));
        assert_eq!(request.description, Some("字段描述".to_string()));
        assert_eq!(request.ui_type, Some("text".to_string()));
    }

    #[test]
    fn test_update_field_request_body_serialization() {
        let property = json!({
            "format": "YYYY-MM-DD",
            "include_time": false
        });

        let body = UpdateFieldRequestBody {
            field_name: Some("日期字段".to_string()),
            r#type: Some("date".to_string()),
            property: Some(property),
            description: Some("日期选择字段".to_string()),
            ui_type: Some("date".to_string()),
        };

        let serialized = serde_json::to_value(&body).unwrap();
        let expected = serde_json::json!({
            "field_name": "日期字段",
            "type": "date",
            "property": {
                "format": "YYYY-MM-DD",
                "include_time": false
            },
            "description": "日期选择字段",
            "ui_type": "date"
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_update_field_request_minimal() {
        let request = UpdateFieldRequest::builder()
            .app_token("test-token")
            .table_id("test-table")
            .field_id("test-field")
            .field_name("最小字段更新")
            .build();

        assert_eq!(request.app_token, "test-token");
        assert_eq!(request.table_id, "test-table");
        assert_eq!(request.field_id, "test-field");
        assert_eq!(request.field_name, Some("最小字段更新".to_string()));
        assert!(request.user_id_type.is_none());
        assert!(request.field_type.is_none());
        assert!(request.property.is_none());
        assert!(request.description.is_none());
        assert!(request.ui_type.is_none());
    }

    #[test]
    fn test_update_field_request_builder_chaining() {
        let request = UpdateFieldRequest::builder()
            .app_token("app123")
            .table_id("table123")
            .field_id("field123")
            .user_id_type("user_id")
            .field_name("链接字段")
            .field_type("url")
            .description("存储URL链接")
            .ui_type("url")
            .build();

        assert_eq!(request.app_token, "app123");
        assert_eq!(request.table_id, "table123");
        assert_eq!(request.field_id, "field123");
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
        assert_eq!(request.field_name, Some("链接字段".to_string()));
        assert_eq!(request.field_type, Some("url".to_string()));
        assert_eq!(request.description, Some("存储URL链接".to_string()));
        assert_eq!(request.ui_type, Some("url".to_string()));
    }

    #[test]
    fn test_update_field_response_trait() {
        assert_eq!(UpdateFieldResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_update_field_response() {
        let field = TableField {
            field_name: "更新后的字段".to_string(),
            field_type: "text".to_string(),
            ..Default::default()
        };

        let response = UpdateFieldResponse { field };
        assert_eq!(response.field.field_name, "更新后的字段");
        assert_eq!(response.field.field_type, "text");
    }

    #[test]
    fn test_update_field_request_new() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let request = UpdateFieldRequest::new(config);

        assert_eq!(request.app_token, "");
        assert_eq!(request.table_id, "");
        assert_eq!(request.field_id, "");
        assert!(request.user_id_type.is_none());
        assert!(request.field_name.is_none());
        assert!(request.field_type.is_none());
        assert!(request.property.is_none());
        assert!(request.description.is_none());
        assert!(request.ui_type.is_none());
    }

    #[test]
    fn test_update_field_complex_property() {
        let complex_property = json!({
            "multi_select": {
                "options": [
                    {"name": "选项A", "value": "opt_a"},
                    {"name": "选项B", "value": "opt_b"},
                    {"name": "选项C", "value": "opt_c"}
                ],
                "default_value": ["opt_a"]
            },
            "validation": {
                "required": true,
                "min_options": 1,
                "max_options": 3
            }
        });

        let request = UpdateFieldRequest::builder()
            .app_token("app-token")
            .table_id("table-id")
            .field_id("field-id")
            .field_name("多选字段")
            .field_type("multi_select")
            .property(complex_property.clone())
            .build();

        assert_eq!(request.app_token, "app-token");
        assert_eq!(request.table_id, "table-id");
        assert_eq!(request.field_id, "field-id");
        assert_eq!(request.field_name, Some("多选字段".to_string()));
        assert_eq!(request.field_type, Some("multi_select".to_string()));
        assert_eq!(request.property, Some(complex_property));
    }
}