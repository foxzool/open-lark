
use openlark_core::{
    api::{ApiRequest, RequestData, ResponseFormat, ApiResponseTrait},
    config::Config,
    error::validation_error,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 复制仪表盘请求
pub struct CopyDashboardRequest {
    /// 配置信息
    config: Config,
    /// 多维表格的唯一标识符
    app_token: String,
    /// 用户 ID 类型
    user_id_type: Option<String>,
    /// 格式为标准的 uuidv4，操作的唯一标识，用于幂等的进行更新操作
    client_token: Option<String>,
    /// 复制仪表盘名称
    name: String,
    /// 要复制的仪表盘ID列表
    dashboard_ids: Vec<String>,
}

impl CopyDashboardRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            app_token: String::new(),
            user_id_type: None,
            client_token: None,
            name: String::new(),
            dashboard_ids: Vec::new(),
        }
    }

    pub fn builder() -> CopyDashboardRequestBuilder {
        CopyDashboardRequestBuilder::new()
    }
}

pub struct CopyDashboardRequestBuilder {
    request: CopyDashboardRequest,
}

impl CopyDashboardRequestBuilder {
    pub fn new() -> Self {
        Self {
            request: CopyDashboardRequest::new(Config::default()),
        }
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
#[derive(Deserialize)]
pub struct CopyDashboardResponse {
    /// 复制的仪表盘列表
    pub dashboards: Vec<DashboardInfo>,
}

/// 仪表盘信息
#[derive(Deserialize)]
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
    _config: &Config,
    _option: Option<RequestOption>,
) -> SDKResult<CopyDashboardResponse> {
    // 构建API URL
    let api_url = format!("/open-apis/bitable/v1/apps/{}/dashboards/batch_copy", request.app_token);

    // 创建API请求
    let mut api_request: ApiRequest<CopyDashboardResponse> = ApiRequest::post(api_url);

    // 设置查询参数
    if let Some(user_id_type) = &request.user_id_type {
        api_request = api_request.query("user_id_type", user_id_type);
    }

    if let Some(client_token) = &request.client_token {
        api_request = api_request.query("client_token", client_token);
    }

    // 设置请求体
    let body = CopyDashboardRequestBody {
        name: request.name,
        dashboard_ids: request.dashboard_ids,
    };

    api_request = api_request.body(RequestData::Json(serde_json::to_value(&body)?));

    // 发送请求
    let response = Transport::request(api_request, &request.config, None).await?;

    // 解析响应数据
    let copy_response: CopyDashboardResponse = response.data
        .and_then(|data| serde_json::from_value(data).ok())
        .ok_or_else(|| validation_error("解析复制仪表盘响应失败", "响应数据格式不正确"))?;

    Ok(copy_response)
}

