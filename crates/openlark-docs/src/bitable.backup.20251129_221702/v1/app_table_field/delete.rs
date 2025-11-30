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

/// 删除字段请求
#[derive(Clone)]
pub struct DeleteFieldRequest {
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
}

impl DeleteFieldRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new(config, Method::DELETE, "/open-apis/bitable/v1/apps/{}/tables/{}/fields/{}".to_string()),
            app_token: String::new(),
            table_id: String::new(),
            field_id: String::new(),
            user_id_type: None,
        }
    }

    pub fn builder() -> DeleteFieldRequestBuilder {
        DeleteFieldRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct DeleteFieldRequestBuilder {
    request: DeleteFieldRequest,
}

impl DeleteFieldRequestBuilder {
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

    pub fn build(self) -> DeleteFieldRequest {
        self.request
    }
}

/// 删除字段响应
#[derive(Clone)]
pub struct DeleteFieldResponse {
    /// 删除的字段信息
    pub field: TableField,
}

impl ApiResponseTrait for DeleteFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除字段
pub async fn delete_field(
    request: DeleteFieldRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<DeleteFieldResponse> {
    let mut api_req = request.api_request;
    api_req.set_http_method(Method::DELETE);
    api_req.api_path = BITABLE_V1_FIELD_DELETE
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

    let api_resp: openlark_core::core::StandardResponse<DeleteFieldResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_delete_field_request_builder() {
        let request = DeleteFieldRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .field_id("fldxxxxxx")
            .user_id_type("open_id")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.field_id, "fldxxxxxx");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_delete_field_request_minimal() {
        let request = DeleteFieldRequest::builder()
            .app_token("test-token")
            .table_id("test-table")
            .field_id("test-field")
            .build();

        assert_eq!(request.app_token, "test-token");
        assert_eq!(request.table_id, "test-table");
        assert_eq!(request.field_id, "test-field");
        assert!(request.user_id_type.is_none());
    }

    #[test]
    fn test_delete_field_request_builder_chaining() {
        let request = DeleteFieldRequest::builder()
            .app_token("app123")
            .table_id("table123")
            .field_id("field123")
            .user_id_type("user_id")
            .build();

        assert_eq!(request.app_token, "app123");
        assert_eq!(request.table_id, "table123");
        assert_eq!(request.field_id, "field123");
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
    }

    #[test]
    fn test_delete_field_response_trait() {
        assert_eq!(DeleteFieldResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_delete_field_response() {
        let field = TableField {
            field_name: "删除的字段".to_string(),
            field_type: "text".to_string(),
            ..Default::default()
        };

        let response = DeleteFieldResponse { field };
        assert_eq!(response.field.field_name, "删除的字段");
        assert_eq!(response.field.field_type, "text");
    }

    #[test]
    fn test_delete_field_request_new() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let request = DeleteFieldRequest::new(config);

        assert_eq!(request.app_token, "");
        assert_eq!(request.table_id, "");
        assert_eq!(request.field_id, "");
        assert!(request.user_id_type.is_none());
    }

    #[test]
    fn test_delete_field_request_different_user_id_types() {
        let open_id_request = DeleteFieldRequest::builder()
            .app_token("app-token")
            .table_id("table-id")
            .field_id("field-id")
            .user_id_type("open_id")
            .build();

        let user_id_request = DeleteFieldRequest::builder()
            .app_token("app-token")
            .table_id("table-id")
            .field_id("field-id")
            .user_id_type("user_id")
            .build();

        let union_id_request = DeleteFieldRequest::builder()
            .app_token("app-token")
            .table_id("table-id")
            .field_id("field-id")
            .user_id_type("union_id")
            .build();

        assert_eq!(open_id_request.user_id_type, Some("open_id".to_string()));
        assert_eq!(user_id_request.user_id_type, Some("user_id".to_string()));
        assert_eq!(union_id_request.user_id_type, Some("union_id".to_string()));
    }

    #[test]
    fn test_delete_field_request_empty_strings() {
        let request = DeleteFieldRequest::builder()
            .app_token("")
            .table_id("")
            .field_id("")
            .build();

        assert_eq!(request.app_token, "");
        assert_eq!(request.table_id, "");
        assert_eq!(request.field_id, "");
        assert!(request.user_id_type.is_none());
    }

    #[test]
    fn test_delete_field_request_long_ids() {
        let long_app_token = "bascn".repeat(20); // 长app_token
        let long_table_id = "tbl".repeat(10); // 长table_id
        let long_field_id = "fld".repeat(15); // 长field_id

        let request = DeleteFieldRequest::builder()
            .app_token(&long_app_token)
            .table_id(&long_table_id)
            .field_id(&long_field_id)
            .build();

        assert_eq!(request.app_token, long_app_token);
        assert_eq!(request.table_id, long_table_id);
        assert_eq!(request.field_id, long_field_id);
    }
}