//! 获取多维表格仪表盘列表模块

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, HttpMethod},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取仪表盘列表请求
#[derive(Clone)]
pub struct ListDashboardsRequest {
    api_request: ApiRequest<ListDashboardsResponse>,
    app_token: String,
    page_size: Option<i32>,
    page_token: Option<String>,
    user_id_type: Option<String>,
}

impl ListDashboardsRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::new()
                .method(HttpMethod::Get)
                .api_path("/open-apis/bitable/v1/apps/{}/dashboards".to_string())
                .config(config)
                .build(),
            app_token: String::new(),
            page_size: None,
            page_token: None,
            user_id_type: None,
        }
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.user_id_type = Some(user_id_type.into());
        self
    }

    pub async fn execute(mut self) -> SDKResult<ListDashboardsResponse> {
        let path = format!("/open-apis/bitable/v1/apps/{}/dashboards", self.app_token);
        self.api_request = self.api_request.api_path(path);

        let config = self.api_request.config();
        let response = Transport::request(self.api_request, &config.clone(), None).await?;
        Ok(response)
    }
}

#[derive(Default)]
pub struct ListDashboardsBuilder {
    request: ListDashboardsRequest,
}

impl ListDashboardsBuilder {
    pub fn new(app_token: impl Into<String>) -> Self {
        let mut builder = Self::default();
        builder.request.app_token = app_token.into();
        builder
    }

    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.request.app_token = app_token.into();
        self
    }

    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request.page_token = Some(page_token.into());
        self
    }

    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    pub fn build(self) -> ListDashboardsRequest {
        self.request
    }
}

/// 仪表盘图标
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DashboardIcon {
    pub token: String,
    pub url: String,
}

/// 国际化数据
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct I18nData {
    pub zh_cn: String,
    pub en_us: String,
}

/// 仪表盘国际化信息
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DashboardI18n {
    pub name: I18nData,
}

/// 仪表盘项目
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DashboardItem {
    pub dashboard_id: String,
    pub name: String,
    pub icon: DashboardIcon,
    pub i18n: DashboardI18n,
    pub link: String,
    pub created_time: String,
    pub modified_time: String,
    pub revision: i32,
}

/// 获取仪表盘列表响应数据
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ListDashboardsResponseData {
    pub items: Vec<DashboardItem>,
    pub page_token: Option<String>,
    pub has_more: Option<bool>,
    pub table_id: Option<String>,
    pub app_token: Option<String>,
}

/// 获取仪表盘列表响应
#[derive(Clone)]
pub struct ListDashboardsResponse {
    pub success: bool,
    pub data: Option<ListDashboardsResponseData>,
}

impl ApiResponseTrait for ListDashboardsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 仪表盘服务
pub struct AppDashboardService {
    pub config: Config,
}

impl AppDashboardService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn list_dashboards_builder(&self, app_token: impl Into<String>) -> ListDashboardsBuilder {
        ListDashboardsBuilder::new(app_token)
    }
}