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
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 复制仪表盘请求
#[derive(Clone)]
pub struct CopyDashboardRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符
    #[serde(skip)]
    app_token: String,
    /// 用户 ID 类型
    #[serde(skip)]
    user_id_type: Option<String>,
    /// 格式为标准的 uuidv4，操作的唯一标识，用于幂等的进行更新操作
    #[serde(skip)]
    client_token: Option<String>,
    /// 复制仪表盘名称
    name: String,
    /// 要复制的仪表盘ID列表
    dashboard_ids: Vec<String>,
}

impl CopyDashboardRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new(config, Method::POST, "/open-apis/bitable/v1/apps/{}/dashboards:batch_copy".to_string()),
            app_token: String::new(),
            user_id_type: None,
            client_token: None,
            name: String::new(),
            dashboard_ids: Vec::new(),
        }
    }

    pub fn builder() -> CopyDashboardRequestBuilder {
        CopyDashboardRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct CopyDashboardRequestBuilder {
    request: CopyDashboardRequest,
}

impl CopyDashboardRequestBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.request.app_token = app_token.into();
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

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request.name = name.into();
        self
    }

    pub fn dashboard_ids(mut self, dashboard_ids: Vec<String>) -> Self {
        self.request.dashboard_ids = dashboard_ids;
        self
    }

    pub fn build(self) -> CopyDashboardRequest {
        self.request
    }
}

/// 复制仪表盘响应
#[derive(Clone)]
pub struct CopyDashboardResponse {
    /// 复制的仪表盘列表
    pub dashboards: Vec<DashboardInfo>,
}

/// 仪表盘信息
#[derive(Clone)]
pub struct DashboardInfo {
    /// 仪表盘ID
    pub dashboard_id: String,
    /// 仪表盘名称
    pub name: String,
    /// 是否复制成功
    pub success: bool,
    /// 错误信息（如果有）
    pub error: Option<String>,
}

impl ApiResponseTrait for CopyDashboardResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 请求体结构
#[derive(Serialize)]
struct CopyDashboardRequestBody {
    name: String,
    dashboard_ids: Vec<String>,
}

/// 复制仪表盘
pub async fn copy_dashboard(
    request: CopyDashboardRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<CopyDashboardResponse> {
    let mut api_req = request.api_request;
    api_req.set_http_method(Method::POST);
    api_req.api_path = BITABLE_V1_DASHBOARDS_COPY
        .replace("{app_token}", &request.app_token);
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
    let body = CopyDashboardRequestBody {
        name: request.name,
        dashboard_ids: request.dashboard_ids,
    };

    api_req.body = serde_json::to_vec(&body).unwrap();

    let api_resp: openlark_core::core::StandardResponse<CopyDashboardResponse> =
        Transport::request(api_req, config, option).await?;
    api_resp.into_result()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_copy_dashboard_request_builder() {
        let request = CopyDashboardRequest::builder()
            .app_token("bascnmBA*****yGehy8")
            .user_id_type("open_id")
            .name("复制的仪表盘")
            .dashboard_ids(vec![
                "dash123".to_string(),
                "dash456".to_string(),
            ])
            .build();

        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(request.name, "复制的仪表盘");
        assert_eq!(request.dashboard_ids.len(), 2);
        assert!(request.dashboard_ids.contains(&"dash123".to_string()));
        assert!(request.dashboard_ids.contains(&"dash456".to_string()));
    }

    #[test]
    fn test_copy_dashboard_request_body_serialization() {
        let body = CopyDashboardRequestBody {
            name: "复制的仪表盘".to_string(),
            dashboard_ids: vec!["dash123".to_string(), "dash456".to_string()],
        };

        let serialized = serde_json::to_value(&body).unwrap();
        let expected = serde_json::json!({
            "name": "复制的仪表盘",
            "dashboard_ids": ["dash123", "dash456"]
        });

        assert_eq!(serialized, expected);
    }

    #[test]
    fn test_copy_dashboard_request_minimal() {
        let request = CopyDashboardRequest::builder()
            .app_token("test-token")
            .name("仪表盘名称")
            .dashboard_ids(vec!["dash123".to_string()])
            .build();

        assert_eq!(request.app_token, "test-token");
        assert_eq!(request.name, "仪表盘名称");
        assert_eq!(request.dashboard_ids, vec!["dash123".to_string()]);
        assert!(request.user_id_type.is_none());
        assert!(request.client_token.is_none());
    }

    #[test]
    fn test_copy_dashboard_request_empty_dashboard_ids() {
        let request = CopyDashboardRequest::builder()
            .app_token("test-token")
            .name("仪表盘名称")
            .dashboard_ids(vec![])
            .build();

        assert_eq!(request.app_token, "test-token");
        assert_eq!(request.name, "仪表盘名称");
        assert!(request.dashboard_ids.is_empty());
    }

    #[test]
    fn test_dashboard_info() {
        let dashboard = DashboardInfo {
            dashboard_id: "dash123".to_string(),
            name: "测试仪表盘".to_string(),
            success: true,
            error: None,
        };

        assert_eq!(dashboard.dashboard_id, "dash123");
        assert_eq!(dashboard.name, "测试仪表盘");
        assert!(dashboard.success);
        assert!(dashboard.error.is_none());
    }

    #[test]
    fn test_copy_dashboard_response_trait() {
        assert_eq!(CopyDashboardResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_copy_dashboard_request_builder_chaining() {
        let request = CopyDashboardRequest::builder()
            .app_token("app123")
            .user_id_type("user_id")
            .client_token("client123")
            .name("仪表盘副本")
            .dashboard_ids(vec!["dash1".to_string(), "dash2".to_string()])
            .build();

        assert_eq!(request.app_token, "app123");
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
        assert_eq!(request.client_token, Some("client123".to_string()));
        assert_eq!(request.name, "仪表盘副本");
        assert_eq!(request.dashboard_ids.len(), 2);
        assert!(request.dashboard_ids.contains(&"dash1".to_string()));
        assert!(request.dashboard_ids.contains(&"dash2".to_string()));
    }

    #[test]
    fn test_copy_dashboard_response() {
        let response = CopyDashboardResponse {
            dashboards: vec![
                DashboardInfo {
                    dashboard_id: "dash123".to_string(),
                    name: "仪表盘1".to_string(),
                    success: true,
                    error: None,
                },
                DashboardInfo {
                    dashboard_id: "dash456".to_string(),
                    name: "仪表盘2".to_string(),
                    success: false,
                    error: Some("复制失败".to_string()),
                },
            ],
        };

        assert_eq!(response.dashboards.len(), 2);
        assert_eq!(response.dashboards[0].dashboard_id, "dash123");
        assert!(response.dashboards[0].success);
        assert_eq!(response.dashboards[1].dashboard_id, "dash456");
        assert!(!response.dashboards[1].success);
        assert_eq!(response.dashboards[1].error, Some("复制失败".to_string()));
    }
}