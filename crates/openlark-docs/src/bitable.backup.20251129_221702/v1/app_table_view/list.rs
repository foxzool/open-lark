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

/// 列出视图请求
#[derive(Clone)]
pub struct ListViewsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的 app_token
    #[serde(skip)]
    app_token: String,
    /// 数据表的 table_id
    #[serde(skip)]
    table_id: String,
    /// 用户 ID 类型
    #[serde(skip)]
    user_id_type: Option<String>,
    /// 分页标记
    #[serde(skip)]
    page_token: Option<String>,
    /// 分页大小
    #[serde(skip)]
    page_size: Option<i32>,
}

impl ListViewsRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new(config, Method::GET, "/open-apis/bitable/v1/apps/{}/tables/{}/views".to_string()),
            app_token: String::new(),
            table_id: String::new(),
            user_id_type: None,
            page_token: None,
            page_size: None,
        }
    }

    pub fn builder() -> ListViewsRequestBuilder {
        ListViewsRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct ListViewsRequestBuilder {
    request: ListViewsRequest,
}

impl ListViewsRequestBuilder {
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

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request.page_token = Some(page_token.into());
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size.min(100)); // 限制最大100
        self
    }

    pub fn build(self) -> ListViewsRequest {
        self.request
    }
}

/// 列出视图响应
#[derive(Clone)]
pub struct ListViewsResponse {
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
    /// 视图信息列表
    pub items: Vec<View>,
}

impl ApiResponseTrait for ListViewsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 列出视图
pub async fn list_views(
    request: ListViewsRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<ListViewsResponse> {
    let mut api_req = request.api_request;
    api_req.set_http_method(Method::GET);
    api_req.api_path = BITABLE_V1_VIEWS
        .replace("{app_token}", &request.app_token)
        .replace("{table_id}", &request.table_id);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req
            .query_params
            .insert("user_id_type".to_string(), user_id_type.clone());
    }

    if let Some(page_token) = &request.page_token {
        api_req
            .query_params
            .insert("page_token".to_string(), page_token.clone());
    }

    if let Some(page_size) = &request.page_size {
        api_req
            .query_params
            .insert("page_size".to_string(), page_size.to_string());
    }

    let api_resp: openlark_core::core::StandardResponse<ListViewsResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_views_request_builder() {
        let request = ListViewsRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .table_id("tblsRc9GRRXKqhvW")
            .page_size(20)
            .user_id_type("open_id")
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_list_views_request_minimal() {
        let request = ListViewsRequest::builder()
            .app_token("test-token")
            .table_id("test-table")
            .build();

        assert_eq!(request.app_token, "test-token");
        assert_eq!(request.table_id, "test-table");
        assert!(request.user_id_type.is_none());
        assert!(request.page_token.is_none());
        assert!(request.page_size.is_none());
    }

    #[test]
    fn test_page_size_limit() {
        let request = ListViewsRequest::builder()
            .app_token("test-token")
            .table_id("test-table")
            .page_size(200) // 超过100的限制
            .build();

        assert_eq!(request.page_size, Some(100)); // 应该被限制为100
    }

    #[test]
    fn test_list_views_request_builder_chaining() {
        let request = ListViewsRequest::builder()
            .app_token("app123")
            .table_id("table123")
            .user_id_type("user_id")
            .page_token("page123")
            .page_size(50)
            .build();

        assert_eq!(request.app_token, "app123");
        assert_eq!(request.table_id, "table123");
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
        assert_eq!(request.page_token, Some("page123".to_string()));
        assert_eq!(request.page_size, Some(50));
    }

    #[test]
    fn test_list_views_response_trait() {
        assert_eq!(ListViewsResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_list_views_response() {
        let response = ListViewsResponse {
            has_more: true,
            page_token: Some("next_page_token".to_string()),
            items: vec![
                View {
                    view_id: Some("vew123".to_string()),
                    view_name: "视图1".to_string(),
                    view_type: Some("grid".to_string()),
                    ..Default::default()
                },
                View {
                    view_id: Some("vew456".to_string()),
                    view_name: "视图2".to_string(),
                    view_type: Some("kanban".to_string()),
                    ..Default::default()
                },
            ],
        };

        assert_eq!(response.has_more, true);
        assert_eq!(response.page_token, Some("next_page_token".to_string()));
        assert_eq!(response.items.len(), 2);
        assert_eq!(response.items[0].view_id, Some("vew123".to_string()));
        assert_eq!(response.items[1].view_name, "视图2");
    }

    #[test]
    fn test_list_views_request_new() {
        let config = openlark_core::Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
            .unwrap();

        let request = ListViewsRequest::new(config);

        assert_eq!(request.app_token, "");
        assert_eq!(request.table_id, "");
        assert!(request.user_id_type.is_none());
        assert!(request.page_token.is_none());
        assert!(request.page_size.is_none());
    }

    #[test]
    fn test_list_views_request_boundary_values() {
        // 测试边界值
        let request_min = ListViewsRequest::builder()
            .app_token("a")
            .table_id("t")
            .page_size(1)
            .build();

        let request_max = ListViewsRequest::builder()
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