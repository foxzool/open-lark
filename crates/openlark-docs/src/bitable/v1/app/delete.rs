//! Bitable V1 删除多维表格API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, RequestData, ResponseFormat},
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

/// 删除多维表格数据
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteAppData {
    /// 应用token
    pub app_token: String,
}

/// 删除多维表格响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DeleteAppV1Response {
    /// 删除多维表格数据
    pub data: DeleteAppData,
}

impl ApiResponseTrait for DeleteAppV1Response {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
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
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "应用token不能为空"));
        }

        // 构建API路径
        let path = format!("/open-apis/bitable/v1/apps/{}", self.app_token);

        // 创建API请求
        let api_request: ApiRequest<DeleteAppV1Response> =
            ApiRequest::delete(&format!("https://open.feishu.cn{}", path));

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            validation_error("响应数据为空", "服务器没有返回有效的数据")
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
