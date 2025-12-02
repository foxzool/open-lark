//! Bitable V1 获取多维表格API

use openlark_core::{
    api::{ApiRequest, RequestData},
    config::Config,
    error::validation_error,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::models::App;
use super::AppService;

/// 获取多维表格请求
#[derive(Debug, Clone)]
pub struct GetAppV1Request {
    api_request: ApiRequest<GetAppV1Response>,
    /// 应用token
    app_token: String,
    /// 配置信息
    config: Config,
}

/// 获取多维表格响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetAppV1Response {
    /// 应用信息
    pub data: App,
    pub success: bool,
}

impl GetAppV1Request {
    /// 创建获取多维表格请求
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::get(""),
            config,
            app_token: String::new(),
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetAppV1Response> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "应用token不能为空"));
        }

        // 构建完整的API URL
        let api_url = format!(
            "{}/open-apis/bitable/v1/apps/{}",
            self.config.base_url, self.app_token
        );

        // 设置API URL
        let mut api_request = self.api_request;
        api_request.url = api_url;

        // 发送请求 - 转换为ApiRequest<()>以匹配Transport::request签名
        let request_for_transport: ApiRequest<()> = ApiRequest::get(api_request.url.clone())
            .body(api_request.body.unwrap_or(RequestData::Empty));

        let config = &self.config;
        let response = Transport::request(request_for_transport, config, None).await?;

        // 解析响应
        let data: App = response
            .data
            .and_then(|data| serde_json::from_value(data).ok())
            .ok_or_else(|| validation_error("解析失败", "数据格式不正确"))?;

        Ok(GetAppV1Response {
            data,
            success: response.raw_response.is_success(),
        })
    }
}

/// 获取多维表格Builder
pub struct GetAppV1Builder {
    request: GetAppV1Request,
}

impl GetAppV1Builder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: GetAppV1Request::new(config),
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    /// 构建请求
    pub fn build(self) -> GetAppV1Request {
        self.request
    }
}

impl AppService {
    /// 创建获取多维表格请求构建器
    pub fn get_app_v1_builder(&self) -> GetAppV1Builder {
        GetAppV1Builder::new(self.config.clone())
    }

    /// 创建获取多维表格请求
    pub fn get_app_v1(&self, app_token: String) -> GetAppV1Request {
        GetAppV1Request::new(self.config.clone()).app_token(app_token)
    }
}
