//! Bitable V1 获取多维表格API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
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
}

impl ApiResponseTrait for GetAppV1Response {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetAppV1Request {
    /// 创建获取多维表格请求
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::get("/open-apis/bitable/v1/apps/:app_token"),
            app_token: String::new(),
            config,
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

        // 构建API路径
        let path = format!("/open-apis/bitable/v1/apps/{}", self.app_token);

        // 创建API请求
        let api_request: ApiRequest<GetAppV1Response> =
            ApiRequest::get(&format!("https://open.feishu.cn{}", path));

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            validation_error("响应数据为空", "服务器没有返回有效的数据")
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
