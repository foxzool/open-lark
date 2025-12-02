//! Bitable V1 删除多维表格API

use openlark_core::{
    api::{ApiRequest, RequestData},
    config::Config,
    error::validation_error,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::AppService;

/// 删除多维表格请求
#[derive(Debug, Clone)]
pub struct DeleteAppV1Request {
    api_request: ApiRequest<DeleteAppV1Response>,
    /// 应用token
    app_token: String,
    /// 配置信息
    config: Config,
}

/// 删除多维表格响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteAppV1Response {
    /// 是否成功
    pub data: bool,
    pub success: bool,
}

impl DeleteAppV1Request {
    /// 创建删除多维表格请求
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::delete(""),
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
    pub async fn execute(self) -> SDKResult<DeleteAppV1Response> {
        // 构建完整的API URL
        let api_url = format!(
            "https://open.feishu.cn/open-apis/bitable/v1/apps/{}",
            self.app_token
        );

        // 设置API URL
        let mut api_request = self.api_request;
        api_request.url = api_url;

        // 发送请求 - 转换为ApiRequest<()>以匹配Transport::request签名
        let request_for_transport: ApiRequest<()> = ApiRequest::delete(api_request.url.clone())
            .body(api_request.body.unwrap_or(RequestData::Empty));

        let config = &self.config;
        let response = Transport::request(request_for_transport, config, None).await?;

        // 解析响应
        let data: bool = response
            .data
            .and_then(|data| serde_json::from_value(data).ok())
            .ok_or_else(|| validation_error("解析失败", "数据格式不正确"))?;

        Ok(DeleteAppV1Response {
            data,
            success: response.raw_response.is_success(),
        })
    }
}

/// 删除多维表格Builder
pub struct DeleteAppV1Builder {
    request: DeleteAppV1Request,
}

impl DeleteAppV1Builder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: DeleteAppV1Request::new(config),
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    /// 构建请求
    pub fn build(self) -> DeleteAppV1Request {
        self.request
    }
}

impl AppService {
    /// 创建删除多维表格请求构建器
    pub fn delete_app_v1_builder(&self) -> DeleteAppV1Builder {
        DeleteAppV1Builder::new(self.config.clone())
    }

    /// 创建删除多维表格请求
    pub fn delete_app_v1(&self, app_token: String) -> DeleteAppV1Request {
        DeleteAppV1Request::new(self.config.clone()).app_token(app_token)
    }
}
