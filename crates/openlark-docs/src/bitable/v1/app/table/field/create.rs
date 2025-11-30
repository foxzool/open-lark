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
    service::bitable::v1::{TableField, FieldType, FieldProperty},
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 新增字段请求
#[derive(Clone)]
pub struct CreateFieldRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 多维表格数据表的唯一标识符
    #[serde(skip)]
    table_id: String,
    /// 用户 ID 类型
    #[serde(skip)]
    user_id_type: Option<String>,
    /// 格式为标准的 uuidv4，操作的唯一标识，用于幂等的进行更新操作
    #[serde(skip)]
    client_token: Option<String>,
    /// 多维表格字段名
    field_name: String,
    /// 多维表格字段类型
    r#type: FieldType,
    /// 字段属性
    #[serde(skip_serializing_if = "Option::is_none")]
    property: Option<FieldProperty>,
    /// 字段的描述
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    /// 字段在界面上的展示类型
    #[serde(skip_serializing_if = "Option::is_none")]
    ui_type: Option<String>,
}

impl CreateFieldRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new(config, Method::POST, "/open-apis/bitable/v1/apps/{}/tables/{}/fields".to_string()),
            app_token: String::new(),
            table_id: String::new(),
            user_id_type: None,
            client_token: None,
            field_name: String::new(),
            r#type: FieldType::Text,
            property: None,
            description: None,
            ui_type: None,
        }
    }

    pub fn builder() -> CreateFieldRequestBuilder {
        CreateFieldRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct CreateFieldRequestBuilder {
    request: CreateFieldRequest,
}

impl CreateFieldRequestBuilder {
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

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    pub fn client_token(mut self, client_token: impl Into<String>) -> Self {
        self.request.client_token = Some(client_token.into());
        self
    }

    pub fn field_name(mut self, field_name: impl Into<String>) -> Self {
        self.request.field_name = field_name.into();
        self
    }

    pub fn field_type(mut self, field_type: FieldType) -> Self {
        self.request.r#type = field_type;
        self
    }

    pub fn property(mut self, property: FieldProperty) -> Self {
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

    pub fn build(self) -> CreateFieldRequest {
        self.request
    }
}

/// 请求体结构
#[derive(Serialize)]
struct CreateFieldRequestBody {
    field_name: String,
    r#type: FieldType,
    #[serde(skip_serializing_if = "Option::is_none")]
    property: Option<FieldProperty>,
    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ui_type: Option<String>,
}

/// 新增字段响应
#[derive(Clone)]
pub struct CreateFieldResponse {
    /// 新增的字段信息
    pub field: TableField,
}

impl ApiResponseTrait for CreateFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 新增字段
pub async fn create_field(
    request: CreateFieldRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<CreateFieldResponse> {
    let mut api_req = request.api_request;
    api_req.set_http_method(Method::POST);
    api_req.api_path = BITABLE_V1_FIELDS
        .replace("{app_token}", &request.app_token)
        .replace("{table_id}", &request.table_id);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req
            .query_params
            .insert("user_id_type".to_string(), user_id_type.clone());
    }

    if let Some(client_token) = &request.client_token {
        api_req
            .query_params
            .insert("client_token".to_string(), client_token.clone());
    }

    // 设置请求体
    let body = CreateFieldRequestBody {
        field_name: request.field_name,
        r#type: request.r#type,
        property: request.property,
        description: request.description,
        ui_type: request.ui_type,
    };

    api_req.body = serde_json::to_vec(&body).unwrap();

    let api_resp: openlark_core::core::StandardResponse<CreateFieldResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_field_request_builder() {
        let property = FieldProperty::text();
        let request = CreateFieldRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .user_id_type("open_id")
            .field_name("测试字段")
            .field_type(FieldType::Text)
            .property(property)
            .description("测试字段描述")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(request.field_name, "测试字段");
        assert_eq!(request.r#type, FieldType::Text);
        assert!(request.property.is_some());
        assert_eq!(request.description, Some("测试字段描述".to_string()));
    }

    #[test]
    fn test_create_field_request_body_serialization() {
        let property = FieldProperty::text();
        let body = CreateFieldRequestBody {
            field_name: "测试字段".to_string(),
            r#type: FieldType::Text,
            property: Some(property),
            description: Some("测试描述".to_string()),
            ui_type: Some("text".to_string()),
        };

        let serialized = serde_json::to_value(&body).unwrap();
        let expected = serde_json::json!({
            "field_name": "测试字段",
            "type": "text",
            "property": {},
            "description": "测试描述",
            "ui_type": "text"
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_create_field_request_minimal() {
        let request = CreateFieldRequest::builder()
            .app_token("test-token")
            .table_id("test-table")
            .field_name("field_name")
            .build();

        assert_eq!(request.app_token, "test-token");
        assert_eq!(request.table_id, "test-table");
        assert_eq!(request.field_name, "field_name");
        assert_eq!(request.r#type, FieldType::Text);
        assert!(request.user_id_type.is_none());
        assert!(request.property.is_none());
        assert!(request.description.is_none());
        assert!(request.ui_type.is_none());
    }

    #[test]
    fn test_create_field_request_builder_chaining() {
        let request = CreateFieldRequest::builder()
            .app_token("app123")
            .table_id("table123")
            .user_id_type("user_id")
            .client_token("client123")
            .field_name("字段名称")
            .field_type(FieldType::Number)
            .description("字段描述")
            .ui_type("number")
            .build();

        assert_eq!(request.app_token, "app123");
        assert_eq!(request.table_id, "table123");
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
        assert_eq!(request.client_token, Some("client123".to_string()));
        assert_eq!(request.field_name, "字段名称");
        assert_eq!(request.r#type, FieldType::Number);
        assert_eq!(request.description, Some("字段描述".to_string()));
        assert_eq!(request.ui_type, Some("number".to_string()));
    }

    #[test]
    fn test_create_field_response_trait() {
        assert_eq!(CreateFieldResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_create_field_response() {
        let field = TableField {
            field_name: "测试字段".to_string(),
            field_type: "text".to_string(),
            ..Default::default()
        };

        let response = CreateFieldResponse { field };
        assert_eq!(response.field.field_name, "测试字段");
        assert_eq!(response.field.field_type, "text");
    }

    #[test]
    fn test_create_field_request_new() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let request = CreateFieldRequest::new(config);

        assert_eq!(request.app_token, "");
        assert_eq!(request.table_id, "");
        assert_eq!(request.field_name, "");
        assert_eq!(request.r#type, FieldType::Text);
        assert!(request.user_id_type.is_none());
        assert!(request.client_token.is_none());
        assert!(request.property.is_none());
        assert!(request.description.is_none());
        assert!(request.ui_type.is_none());
    }
}