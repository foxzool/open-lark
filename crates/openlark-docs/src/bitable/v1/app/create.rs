//! Bitable V1 创建多维表格API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::{validation_error, SDKResult},
    http::Transport,
};
use serde::{Deserialize, Serialize};

use super::models::{App, CreateAppRequest as CreateAppRequestBody};
use super::AppService;

/// 创建多维表格请求
pub struct CreateAppV1Request {
    api_request: ApiRequest<CreateAppV1Response>,
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
            api_request: ApiRequest::post("/open-apis/bitable/v1/apps"),
            name: String::new(),
            folder_token: None,
            time_zone: None,
            config,
        }
    }

    /// 设置应用名称
    pub fn name(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    /// 设置文件夹token
    pub fn folder_token(mut self, folder_token: String) -> Self {
        self.folder_token = Some(folder_token);
        self
    }

    /// 设置时区
    pub fn time_zone(mut self, time_zone: String) -> Self {
        self.time_zone = Some(time_zone);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CreateAppV1Response> {
        // 参数验证
        if self.name.trim().is_empty() {
            return Err(validation_error("name", "应用名称不能为空"));
        }

        if self.name.len() > 100 {
            return Err(validation_error("name", "应用名称长度不能超过100个字符"));
        }

        // 构建请求体
        let request_body = CreateAppRequestBody {
            name: self.name.clone(),
            folder_token: self.folder_token.clone(),
            time_zone: self.time_zone.clone(),
            app_settings: None,
        };

        // 验证请求参数
        if let Err(e) = request_body.validate() {
            return Err(validation_error("创建应用请求验证失败", e.to_string()));
        }

        // 创建API请求
        let api_request: ApiRequest<CreateAppV1Response> = self.api_request
            .body(openlark_core::api::RequestData::Binary(serde_json::to_vec(&request_body)?));

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

/// 创建多维表格Builder
pub struct CreateAppV1Builder {
    request: CreateAppV1Request,
}

impl CreateAppV1Builder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: CreateAppV1Request::new(config),
        }
    }

    /// 设置应用名称
    pub fn name(mut self, name: String) -> Self {
        self.request = self.request.name(name);
        self
    }

    /// 设置文件夹token
    pub fn folder_token(mut self, folder_token: String) -> Self {
        self.request = self.request.folder_token(folder_token);
        self
    }

    /// 设置时区
    pub fn time_zone(mut self, time_zone: String) -> Self {
        self.request = self.request.time_zone(time_zone);
        self
    }

    /// 构建请求
    pub fn build(self) -> CreateAppV1Request {
        self.request
    }
}

impl AppService {
    /// 创建新增多维表格请求构建器
    pub fn create_app_v1_builder(&self) -> CreateAppV1Builder {
        CreateAppV1Builder::new(self.config.clone())
    }

    /// 创建新增多维表格请求
    pub fn create_app_v1(
        &self,
        name: String,
        folder_token: Option<String>,
        time_zone: Option<String>,
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
