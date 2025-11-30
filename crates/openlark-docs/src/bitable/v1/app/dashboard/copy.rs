
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, HttpMethod},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 复制仪表盘请求
#[derive(Clone)]
pub struct CopyDashboardRequest {
    #[serde(skip)]
    api_request: ApiRequest<CopyDashboardResponse>,
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
            api_request: ApiRequest::new()
                .method(HttpMethod::Post)
                .api_path("/open-apis/bitable/v1/apps/{}/dashboards/batch_copy".to_string())
                .config(config)
                .build(),
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
    let mut api_request = request.api_request
        .api_path(format!(/open-apis/bitable/v1/apps/{}/dashboards/{}/copy, &request.app_token, &request.dashboard_id));
    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_req
            .query_params
            .insert(user_id_type.to_string(), user_id_type.clone());
    }

    if let Some(client_token) = &request.client_token {
        api_req
            .query_params
            .insert(client_token.to_string(), client_token.clone());
    }

    // 设置请求体
    let body = CopyDashboardRequestBody {
        name: request.name,
        dashboard_ids: request.dashboard_ids,
    };

    let api_request = api_request.body(serde_json::to_vec(&body).unwrap());

    let response: CopyDashboardResponse =
        Transport::request(api_request, config, option).await?;
    response
}

