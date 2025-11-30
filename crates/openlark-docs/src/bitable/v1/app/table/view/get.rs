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
    service::bitable::v1::View,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取视图请求
#[derive(Clone)]
pub struct GetViewRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的 app_token
    #[serde(skip)]
    app_token: String,
    /// 数据表的 table_id
    #[serde(skip)]
    table_id: String,
    /// 视图的 view_id
    #[serde(skip)]
    view_id: String,
    /// 用户 ID 类型
    #[serde(skip)]
    user_id_type: Option<String>,
}

impl GetViewRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new(config, Method::GET, "/open-apis/bitable/v1/apps/{}/tables/{}/views/{}".to_string()),
            app_token: String::new(),
            table_id: String::new(),
            view_id: String::new(),
            user_id_type: None,
        }
    }

    pub fn builder() -> GetViewRequestBuilder {
        GetViewRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct GetViewRequestBuilder {
    request: GetViewRequest,
}

impl GetViewRequestBuilder {
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
        self.request.view_id = view_id.into();
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    pub fn build(self) -> GetViewRequest {
        self.request
    }
}

/// 获取视图响应
#[derive(Clone)]
pub struct GetViewResponse {
    /// 视图信息
    pub view: View,
}

impl ApiResponseTrait for GetViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取视图
pub async fn get_view(
    request: GetViewRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<GetViewResponse> {
    let mut api_req = request.api_request;
    api_req.set_http_method(Method::GET);
    api_req.api_path = BITABLE_V1_VIEW_GET
        .replace("{app_token}", &request.app_token)
        .replace("{table_id}", &request.table_id)
        .replace("{view_id}", &request.view_id);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req
            .query_params
            .insert("user_id_type".to_string(), user_id_type.clone());
    }

    let api_resp: openlark_core::core::StandardResponse<GetViewResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_view_request_builder() {
        let request = GetViewRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .view_id("vew123456")
            .user_id_type("open_id")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.view_id, "vew123456");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_get_view_request_minimal() {
        let request = GetViewRequest::builder()
            .app_token("test-token")
            .table_id("test-table")
            .view_id("test-view")
            .build();

        assert_eq!(request.app_token, "test-token");
        assert_eq!(request.table_id, "test-table");
        assert_eq!(request.view_id, "test-view");
        assert!(request.user_id_type.is_none());
    }

    #[test]
    fn test_get_view_request_builder_chaining() {
        let request = GetViewRequest::builder()
            .app_token("app123")
            .table_id("table123")
            .view_id("view123")
            .user_id_type("user_id")
            .build();

        assert_eq!(request.app_token, "app123");
        assert_eq!(request.table_id, "table123");
        assert_eq!(request.view_id, "view123");
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
    }

    #[test]
    fn test_get_view_response_trait() {
        assert_eq!(GetViewResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_get_view_response() {
        let view = View {
            view_id: Some("vew123".to_string()),
            view_name: "测试视图".to_string(),
            view_type: Some("grid".to_string()),
            ..Default::default()
        };

        let response = GetViewResponse { view };
        assert_eq!(response.view.view_id, Some("vew123".to_string()));
        assert_eq!(response.view.view_name, "测试视图");
        assert_eq!(response.view.view_type, Some("grid".to_string()));
    }

    #[test]
    fn test_get_view_request_new() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let request = GetViewRequest::new(config);

        assert_eq!(request.app_token, "");
        assert_eq!(request.table_id, "");
        assert_eq!(request.view_id, "");
        assert!(request.user_id_type.is_none());
    }

    #[test]
    fn test_get_view_request_different_user_id_types() {
        let open_id_request = GetViewRequest::builder()
            .app_token("app-token")
            .table_id("table-id")
            .view_id("view-id")
            .user_id_type("open_id")
            .build();

        let user_id_request = GetViewRequest::builder()
            .app_token("app-token")
            .table_id("table-id")
            .view_id("view-id")
            .user_id_type("user_id")
            .build();

        let union_id_request = GetViewRequest::builder()
            .app_token("app-token")
            .table_id("table-id")
            .view_id("view-id")
            .user_id_type("union_id")
            .build();

        assert_eq!(open_id_request.user_id_type, Some("open_id".to_string()));
        assert_eq!(user_id_request.user_id_type, Some("user_id".to_string()));
        assert_eq!(union_id_request.user_id_type, Some("union_id".to_string()));
    }

    #[test]
    fn test_get_view_request_empty_strings() {
        let request = GetViewRequest::builder()
            .app_token("")
            .table_id("")
            .view_id("")
            .build();

        assert_eq!(request.app_token, "");
        assert_eq!(request.table_id, "");
        assert_eq!(request.view_id, "");
        assert!(request.user_id_type.is_none());
    }

    #[test]
    fn test_get_view_request_long_ids() {
        let long_app_token = "bascn".repeat(20); // 长app_token
        let long_table_id = "tbl".repeat(10); // 长table_id
        let long_view_id = "vew".repeat(15); // 长view_id

        let request = GetViewRequest::builder()
            .app_token(&long_app_token)
            .table_id(&long_table_id)
            .view_id(&long_view_id)
            .build();

        assert_eq!(request.app_token, long_app_token);
        assert_eq!(request.table_id, long_table_id);
        assert_eq!(request.view_id, long_view_id);
    }
}