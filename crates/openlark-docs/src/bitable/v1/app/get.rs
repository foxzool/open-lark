//! Bitable V1 获取多维表格API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::models::App;
use super::AppService;

/// 获取多维表格请求
#[derive(Debug, Clone)]
pub struct GetAppV1Request {
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
            app_token: String::new(),
            config,
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<GetAppV1Response> {
        // 验证必填字段
        validate_required!(self.app_token, "应用令牌不能为空");

        // 构建API路径
        let path = format!("/open-apis/bitable/v1/apps/{}", self.app_token);

        // 创建API请求
        let api_request: ApiRequest<GetAppV1Response> = ApiRequest::get(&path);

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

impl AppService {
    /// 创建获取多维表格请求
    pub fn get_builder(
        &self,
        app_token: impl Into<String>,
    ) -> GetAppV1Request {
        GetAppV1Request::new(self.config.clone()).app_token(app_token)
    }

    /// 创建获取多维表格请求
    pub fn get_app_v1(
        &self,
        app_token: impl Into<String>,
    ) -> GetAppV1Request {
        self.get_builder(app_token)
    }
}
