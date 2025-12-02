//! Bitable V1 复制仪表盘API

use openlark_core::{
    api::ApiRequest,
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};

/// 复制仪表盘请求
#[derive(Debug, Clone)]
pub struct CopyDashboardRequest {
    /// 配置信息
    config: Config,
    api_request: ApiRequest<CopyDashboardResponse>,
    /// 多维表格的 app_token
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
    /// 创建复制仪表盘请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            api_request: ApiRequest::post(""),
            app_token: String::new(),
            user_id_type: None,
            client_token: None,
            name: String::new(),
            dashboard_ids: Vec::new(),
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 设置客户端token
    pub fn client_token(mut self, client_token: String) -> Self {
        self.client_token = Some(client_token);
        self
    }

    /// 设置复制仪表盘名称
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    /// 设置要复制的仪表盘ID列表
    pub fn dashboard_ids(mut self, dashboard_ids: Vec<String>) -> Self {
        self.dashboard_ids = dashboard_ids;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CopyDashboardResponse> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "应用token不能为空"));
        }
        if self.name.trim().is_empty() {
            return Err(validation_error("name", "复制仪表盘名称不能为空"));
        }
        if self.dashboard_ids.is_empty() {
            return Err(validation_error(
                "dashboard_ids",
                "要复制的仪表盘ID列表不能为空",
            ));
        }

        // 构建完整的API URL
        let api_url = format!(
            "{}/open-apis/bitable/v1/apps/{}/dashboards/batch_copy",
            self.config.base_url, self.app_token
        );

        // 设置API URL
        let mut api_request = self.api_request;
        api_request.url = api_url;

        // 设置查询参数
        let mut separator_added = false;
        if let Some(user_id_type) = &self.user_id_type {
            api_request.url = format!("{}?user_id_type={}", api_request.url, user_id_type);
            separator_added = true;
        }
        if let Some(client_token) = &self.client_token {
            let separator = if separator_added { "&" } else { "?" };
            api_request.url = format!(
                "{}{}client_token={}",
                api_request.url, separator, client_token
            );
        }

        // 设置请求体
        let body = CopyDashboardRequestBody {
            name: self.name,
            dashboard_ids: self.dashboard_ids,
        };

        api_request.body = Some(openlark_core::api::RequestData::Json(serde_json::to_value(
            body,
        )?));

        // 发送请求 - 转换为ApiRequest<()>以匹配Transport::request签名
        let request_for_transport: openlark_core::api::ApiRequest<()> =
            openlark_core::api::ApiRequest::post(api_request.url.clone()).body(
                api_request
                    .body
                    .unwrap_or(openlark_core::api::RequestData::Empty),
            );

        // 发送请求并解析响应
        let response = Transport::request(request_for_transport, &self.config, None).await?;

        // 手动解析响应数据
        let response_data: CopyDashboardResponse =
            serde_json::from_value(response.data.ok_or_else(|| {
                openlark_core::error::validation_error("response", "响应数据为空")
            })?)?;
        Ok(response_data)
    }
}

/// 复制仪表盘Builder
pub struct CopyDashboardRequestBuilder {
    request: CopyDashboardRequest,
}

impl CopyDashboardRequestBuilder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: CopyDashboardRequest::new(config),
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: String) -> Self {
        self.request = self.request.user_id_type(user_id_type);
        self
    }

    /// 设置客户端token
    pub fn client_token(mut self, client_token: String) -> Self {
        self.request = self.request.client_token(client_token);
        self
    }

    /// 设置复制仪表盘名称
    pub fn name(mut self, name: String) -> Self {
        self.request = self.request.name(name);
        self
    }

    /// 设置要复制的仪表盘ID列表
    pub fn dashboard_ids(mut self, dashboard_ids: Vec<String>) -> Self {
        self.request = self.request.dashboard_ids(dashboard_ids);
        self
    }

    /// 构建请求
    pub fn build(self) -> CopyDashboardRequest {
        self.request
    }
}

/// 请求体结构
#[derive(Serialize)]
struct CopyDashboardRequestBody {
    name: String,
    dashboard_ids: Vec<String>,
}

/// 复制仪表盘响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CopyDashboardResponse {
    /// 复制的仪表盘列表
    pub dashboards: Vec<DashboardInfo>,
}

/// 仪表盘信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
