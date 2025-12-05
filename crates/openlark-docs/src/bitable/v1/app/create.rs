//! Bitable V1 创建多维表格API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::models::{App, CreateAppRequest as CreateAppRequestBody};
use super::AppService;

/// 创建多维表格请求
pub struct CreateAppV1Request {
    /// 应用名称
    name: String,
    /// 文件夹token
    folder_token: Option<String>,
    /// 时区
    time_zone: Option<String>,
    /// 配置信息
    config: Config,
}

/// 创建多维表格响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateAppV1Response {
    /// 应用信息
    pub data: App,
}

impl ApiResponseTrait for CreateAppV1Response {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl CreateAppV1Request {
    /// 创建新增多维表格请求
    pub fn new(config: Config) -> Self {
        Self {
            name: String::new(),
            folder_token: None,
            time_zone: None,
            config,
        }
    }

    /// 设置应用名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// 设置文件夹token
    pub fn folder_token(mut self, folder_token: impl Into<String>) -> Self {
        self.folder_token = Some(folder_token.into());
        self
    }

    /// 设置时区
    pub fn time_zone(mut self, time_zone: impl Into<String>) -> Self {
        self.time_zone = Some(time_zone.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateAppV1Response> {
        // 验证必填字段
        validate_required!(self.name, "应用名称不能为空");

        // 构建请求体
        let request_body = CreateAppRequestBody {
            name: self.name.clone(),
            folder_token: self.folder_token.clone(),
            time_zone: self.time_zone.clone(),
            app_settings: None,
        };

        // 创建API请求
        let api_request: ApiRequest<CreateAppV1Response> = ApiRequest::post("/open-apis/bitable/v1/apps")
            .body(openlark_core::api::RequestData::Binary(serde_json::to_vec(&request_body)?));

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

impl AppService {
    /// 创建新增多维表格请求
    pub fn create_builder(
        &self,
        name: impl Into<String>,
    ) -> CreateAppV1Request {
        CreateAppV1Request::new(self.config.clone()).name(name)
    }

    /// 创建新增多维表格请求（带完整参数）
    pub fn create_app_v1(
        &self,
        name: impl Into<String>,
        folder_token: Option<impl Into<String>>,
        time_zone: Option<impl Into<String>>,
    ) -> CreateAppV1Request {
        let mut request = CreateAppV1Request::new(self.config.clone()).name(name);

        if let Some(folder_token) = folder_token {
            request = request.folder_token(folder_token);
        }

        if let Some(time_zone) = time_zone {
            request = request.time_zone(time_zone);
        }

        request
    }
}
