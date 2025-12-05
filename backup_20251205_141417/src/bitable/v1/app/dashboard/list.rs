//! 获取多维表格仪表盘列表模块

use openlark_core::{api::ApiRequest, config::Config, error::SDKResult, http::Transport};
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

    /// 执行请求
    pub async fn execute(self) -> SDKResult<ListDashboardsResponse> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(openlark_core::error::validation_error(
                "app_token",
                "应用token不能为空",
            ));
        }

        // 构建完整的API URL
        let api_url = format!(
            "{}/open-apis/bitable/v1/apps/{}/dashboards",
            self.config.base_url, self.app_token
        );

        // 设置API URL
        let mut api_request = self.api_request;
        api_request.url = api_url;

        // 设置查询参数
        let mut separator_added = false;
        if let Some(page_size) = self.page_size {
            api_request.url = format!("{}?page_size={}", api_request.url, page_size.min(100)); // 限制最大100
            separator_added = true;
        }
        if let Some(page_token) = &self.page_token {
            let separator = if separator_added { "&" } else { "?" };
            api_request.url = format!("{}{}page_token={}", api_request.url, separator, page_token);
            separator_added = true;
        }
        if let Some(user_id_type) = &self.user_id_type {
            let separator = if separator_added { "&" } else { "?" };
            api_request.url = format!(
                "{}{}user_id_type={}",
                api_request.url, separator, user_id_type
            );
        }

        // 发送请求 - 转换为ApiRequest<()>以匹配Transport::request签名
        let request_for_transport: openlark_core::api::ApiRequest<()> =
            openlark_core::api::ApiRequest::get(api_request.url.clone())
                .body(openlark_core::api::RequestData::Empty);

        // 发送请求并解析响应
        let response = Transport::request(request_for_transport, &self.config, None).await?;

        // 手动解析响应数据
        let response_data: ListDashboardsResponse =
            serde_json::from_value(response.data.ok_or_else(|| {
                openlark_core::error::validation_error("response", "响应数据为空")
            })?)?;
        Ok(response_data)
    }
}

/// 列出仪表盘Builder
pub struct ListDashboardsBuilder {
    request: ListDashboardsRequest,
}

impl ListDashboardsBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: ListDashboardsRequest::new(config),
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request = self.request.page_size(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: String) -> Self {
        self.request = self.request.page_token(page_token);
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.request = self.request.user_id_type(user_id_type);
        self
    }

    /// 构建请求
    pub fn build(self) -> ListDashboardsRequest {
        self.request
    }
}

/// 仪表盘图标
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DashboardIcon {
    pub token: String,
    pub url: String,
}

/// 国际化数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct I18nData {
    pub zh_cn: String,
    pub en_us: String,
}

/// 仪表盘国际化信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DashboardI18n {
    pub name: I18nData,
}

/// 仪表盘项目
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

/// 获取仪表盘列表响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListDashboardsResponse {
    /// 是否还有更多项
    pub has_more: Option<bool>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 仪表盘项目列表
    pub items: Option<Vec<DashboardItem>>,
    /// 操作结果
    pub success: bool,
}
