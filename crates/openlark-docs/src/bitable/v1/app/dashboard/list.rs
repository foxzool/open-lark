//! 获取多维表格仪表盘列表模块

use openlark_core::{
    api::{ApiRequest, RequestData},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

/// 获取仪表盘列表请求
#[derive(Debug, Clone)]
pub struct ListDashboardsRequest {
    api_request: ApiRequest<ListDashboardsResponse>,
    app_token: String,
    page_size: Option<i32>,
    page_token: Option<String>,
    user_id_type: Option<String>,
    /// 配置信息
    config: Config,
}

impl ListDashboardsRequest {
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::get(""),
            config,
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

    pub async fn execute(self) -> SDKResult<ListDashboardsResponse> {
        // 构建完整的API URL
        let api_url = format!("https://open.feishu.cn/open-apis/bitable/v1/apps/{}/dashboards", self.app_token);

        // 设置API URL
        let mut api_request = self.api_request;
        api_request.url = api_url;

        // 发送请求 - 转换为ApiRequest<()>以匹配Transport::request签名
        let mut request_for_transport: ApiRequest<()> = ApiRequest::get(api_request.url.clone())
            .body(api_request.body.unwrap_or(RequestData::Empty));

        let config = &self.config;
        let response = Transport::request(request_for_transport, config, None).await?;

        // 解析响应数据
        let data_value: serde_json::Value = response.data
            .unwrap_or(serde_json::Value::Null);

        // 尝试将JSON解析为预期的数据结构
        let data: Option<ListDashboardsResponseData> = if data_value.is_null() {
            None
        } else {
            serde_json::from_value(data_value).ok()
        };

        Ok(ListDashboardsResponse {
            data,
            success: response.raw_response.is_success(),
        })
    }
}

pub struct ListDashboardsBuilder {
    request: ListDashboardsRequest,
}

impl Default for ListDashboardsBuilder {
    fn default() -> Self {
        Self {
            request: ListDashboardsRequest::new(Config::default()),
        }
    }
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardIcon {
    pub token: String,
    pub url: String,
}

/// 国际化数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct I18nData {
    pub zh_cn: String,
    pub en_us: String,
}

/// 仪表盘国际化信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardI18n {
    pub name: I18nData,
}

/// 仪表盘项目
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDashboardsResponseData {
    pub items: Vec<DashboardItem>,
    pub page_token: Option<String>,
    pub has_more: Option<bool>,
    pub table_id: Option<String>,
    pub app_token: Option<String>,
}

/// 获取仪表盘列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDashboardsResponse {
    pub success: bool,
    pub data: Option<ListDashboardsResponseData>,
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