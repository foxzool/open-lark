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

/// 列出字段请求
#[derive(Clone)]
pub struct ListFieldRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 多维表格数据表的唯一标识符
    #[serde(skip)]
    table_id: String,
    /// 视图 ID
    #[serde(skip)]
    view_id: Option<String>,
    /// 控制字段描述数据的返回格式
    #[serde(skip)]
    text_field_as_array: Option<bool>,
    /// 分页标记
    #[serde(skip)]
    page_token: Option<String>,
    /// 分页大小
    #[serde(skip)]
    page_size: Option<i32>,
    /// 用户 ID 类型
    #[serde(skip)]
    user_id_type: Option<String>,
}

impl ListFieldRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new(config, Method::GET, "/open-apis/bitable/v1/apps/{}/tables/{}/fields".to_string()),
            app_token: String::new(),
            table_id: String::new(),
            view_id: None,
            text_field_as_array: None,
            page_token: None,
            page_size: None,
            user_id_type: None,
        }
    }

    pub fn builder() -> ListFieldRequestBuilder {
        ListFieldRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct ListFieldRequestBuilder {
    request: ListFieldRequest,
}

impl ListFieldRequestBuilder {
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

    pub fn view_id(mut self, view_id: impl Into<String>) -> Self {
        self.request.view_id = Some(view_id.into());
        self
    }

    pub fn text_field_as_array(mut self, text_field_as_array: bool) -> Self {
        self.request.text_field_as_array = Some(text_field_as_array);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request.page_token = Some(page_token.into());
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size.min(100)); // 限制最大100
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    pub fn build(self) -> ListFieldRequest {
        self.request
    }
}

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
    pub items: Vec<TableField>,
}

impl ApiResponseTrait for ListFieldResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 列出字段
pub async fn list_field(
    request: ListFieldRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<ListFieldResponse> {
    let mut api_req = request.api_request;
    api_req.set_http_method(Method::GET);
    api_req.api_path = BITABLE_V1_FIELDS
        .replace("{app_token}", &request.app_token)
        .replace("{table_id}", &request.table_id);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    // 设置查询参数
    if let Some(view_id) = &request.view_id {
        api_req
            .query_params
            .insert("view_id".to_string(), view_id.clone());
    }

    if let Some(text_field_as_array) = &request.text_field_as_array {
        api_req
            .query_params
            .insert("text_field_as_array", text_field_as_array.to_string());
    }

    if let Some(page_token) = &request.page_token {
        api_req
            .query_params
            .insert("page_token", page_token.clone());
    }

    if let Some(page_size) = &request.page_size {
        api_req
            .query_params
            .insert("page_size", page_size.to_string());
    }

    if let Some(user_id_type) = &request.user_id_type {
        api_req
            .query_params
            .insert("user_id_type".to_string(), user_id_type.clone());
    }

    let api_resp: openlark_core::core::StandardResponse<ListFieldResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_field_request_builder() {
        let request = ListFieldRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .view_id("vew123456")
            .page_size(20)
            .text_field_as_array(true)
            .user_id_type("open_id")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.view_id, Some("vew123456".to_string()));
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.text_field_as_array, Some(true));
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_list_field_request_minimal() {
        let request = ListFieldRequest::builder()
            .app_token("test-token")
            .table_id("test-table")
            .build();

        assert_eq!(request.app_token, "test-token");
        assert_eq!(request.table_id, "test-table");
        assert!(request.view_id.is_none());
        assert!(request.page_token.is_none());
        assert!(request.page_size.is_none());
        assert!(request.text_field_as_array.is_none());
        assert!(request.user_id_type.is_none());
    }

    #[test]
    fn test_page_size_limit() {
        let request = ListFieldRequest::builder()
            .app_token("test-token")
            .table_id("test-table")
            .page_size(200) // 超过100的限制
            .build();

        assert_eq!(request.page_size, Some(100)); // 应该被限制为100
    }

    #[test]
    fn test_list_field_request_builder_chaining() {
        let request = ListFieldRequest::builder()
            .app_token("app123")
            .table_id("table123")
            .view_id("view123")
            .page_token("page123")
            .page_size(50)
            .text_field_as_array(false)
            .user_id_type("user_id")
            .build();

        assert_eq!(request.app_token, "app123");
        assert_eq!(request.table_id, "table123");
        assert_eq!(request.view_id, Some("view123".to_string()));
        assert_eq!(request.page_token, Some("page123".to_string()));
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.text_field_as_array, Some(false));
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
    }

    #[test]
    fn test_list_field_response_trait() {
        assert_eq!(ListFieldResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_list_field_response() {
        let response = ListFieldResponse {
            has_more: true,
            page_token: Some("next_page_token".to_string()),
            total: 100,
            items: vec![
                TableField {
                    field_name: "字段1".to_string(),
                    field_type: "text".to_string(),
                    ..Default::default()
                },
                TableField {
                    field_name: "字段2".to_string(),
                    field_type: "number".to_string(),
                    ..Default::default()
                },
            ],
        };

        assert_eq!(response.has_more, true);
        assert_eq!(response.page_token, Some("next_page_token".to_string()));
        assert_eq!(response.total, 100);
        assert_eq!(response.items.len(), 2);
        assert_eq!(response.items[0].field_name, "字段1");
        assert_eq!(response.items[0].field_type, "text");
        assert_eq!(response.items[1].field_name, "字段2");
        assert_eq!(response.items[1].field_type, "number");
    }

    #[test]
    fn test_list_field_request_new() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let request = ListFieldRequest::new(config);

        assert_eq!(request.app_token, "");
        assert_eq!(request.table_id, "");
        assert!(request.view_id.is_none());
        assert!(request.page_token.is_none());
        assert!(request.page_size.is_none());
        assert!(request.text_field_as_array.is_none());
        assert!(request.user_id_type.is_none());
    }

    #[test]
    fn test_list_field_request_empty_strings() {
        let request = ListFieldRequest::builder()
            .app_token("")
            .table_id("")
            .view_id("")
            .page_token("")
            .build();

        assert_eq!(request.app_token, "");
        assert_eq!(request.table_id, "");
        assert_eq!(request.view_id, Some("".to_string()));
        assert_eq!(request.page_token, Some("".to_string()));
        assert!(request.page_size.is_none());
        assert!(request.text_field_as_array.is_none());
        assert!(request.user_id_type.is_none());
    }

    #[test]
    fn test_list_field_request_long_ids() {
        let long_app_token = "bascn".repeat(20); // 长app_token
        let long_table_id = "tbl".repeat(10); // 长table_id
        let long_view_id = "vew".repeat(15); // 长view_id
        let long_page_token = "page".repeat(12); // 长page_token

        let request = ListFieldRequest::builder()
            .app_token(&long_app_token)
            .table_id(&long_table_id)
            .view_id(&long_view_id)
            .page_token(&long_page_token)
            .build();

        assert_eq!(request.app_token, long_app_token);
        assert_eq!(request.table_id, long_table_id);
        assert_eq!(request.view_id, Some(long_view_id));
        assert_eq!(request.page_token, Some(long_page_token));
    }

    #[test]
    fn test_list_field_request_different_user_id_types() {
        let open_id_request = ListFieldRequest::builder()
            .app_token("app-token")
            .table_id("table-id")
            .user_id_type("open_id")
            .build();

        let user_id_request = ListFieldRequest::builder()
            .app_token("app-token")
            .table_id("table-id")
            .user_id_type("user_id")
            .build();

        let union_id_request = ListFieldRequest::builder()
            .app_token("app-token")
            .table_id("table-id")
            .user_id_type("union_id")
            .build();

        assert_eq!(open_id_request.user_id_type, Some("open_id".to_string()));
        assert_eq!(user_id_request.user_id_type, Some("user_id".to_string()));
        assert_eq!(union_id_request.user_id_type, Some("union_id".to_string()));
    }

    #[test]
    fn test_list_field_request_boundary_values() {
        // 测试边界值
        let request_min = ListFieldRequest::builder()
            .app_token("a")
            .table_id("t")
            .page_size(1)
            .build();

        let request_max = ListFieldRequest::builder()
            .app_token("a".repeat(100))
            .table_id("t".repeat(100))
            .page_size(100)
            .build();

        assert_eq!(request_min.app_token, "a");
        assert_eq!(request_min.table_id, "t");
        assert_eq!(request_min.page_size, Some(1));

        assert_eq!(request_max.app_token, "a".repeat(100));
        assert_eq!(request_max.table_id, "t".repeat(100));
        assert_eq!(request_max.page_size, Some(100));
    }
}