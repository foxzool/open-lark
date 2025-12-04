//! Bitable V1 更新多维表格API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    error::validation_error,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::models::{App, AppSettings, UpdateAppRequest as UpdateAppRequestBody};
use super::AppService;

/// 更新多维表格请求
pub struct UpdateAppV1Request {
    api_request: ApiRequest<UpdateAppV1Response>,
    /// 应用token
    app_token: String,
    /// 应用名称
    name: Option<String>,
    /// 应用图标
    avatar: Option<String>,
    /// 应用设置
    app_settings: Option<AppSettings>,
    /// 配置信息
    config: Config,
}

/// 更新多维表格响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateAppV1Response {
    /// 应用信息
    pub data: App,
}

impl ApiResponseTrait for UpdateAppV1Response {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl UpdateAppV1Request {
    /// 创建更新多维表格请求
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::put("/open-apis/bitable/v1/apps/:app_token"),
            app_token: String::new(),
            name: None,
            avatar: None,
            app_settings: None,
            config,
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 设置应用名称
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// 设置应用图标
    pub fn avatar(mut self, avatar: String) -> Self {
        self.avatar = Some(avatar);
        self
    }

    /// 设置应用设置
    pub fn app_settings(mut self, app_settings: AppSettings) -> Self {
        self.app_settings = Some(app_settings);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateAppV1Response> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "应用token不能为空"));
        }

        // 构建API路径
        let path = format!("/open-apis/bitable/v1/apps/{}", self.app_token);

        // 构建请求体
        let request_body = UpdateAppRequestBody {
            name: self.name.clone(),
            avatar: self.avatar.clone(),
            app_settings: self.app_settings.clone(),
        };

        // 验证请求参数
        if let Err(e) = request_body.validate() {
            return Err(validation_error("更新应用请求验证失败", e.to_string()));
        }

        // 创建API请求
        let api_request: ApiRequest<UpdateAppV1Response> = self.api_request
            .body(openlark_core::api::RequestData::Binary(serde_json::to_vec(&request_body)?));

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

/// 更新多维表格Builder
pub struct UpdateAppV1Builder {
    request: UpdateAppV1Request,
}

impl UpdateAppV1Builder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: UpdateAppV1Request::new(config),
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    /// 设置应用名称
    pub fn name(mut self, name: String) -> Self {
        self.request = self.request.name(name);
        self
    }

    /// 设置应用图标
    pub fn avatar(mut self, avatar: String) -> Self {
        self.request = self.request.avatar(avatar);
        self
    }

    /// 设置应用设置
    pub fn app_settings(mut self, app_settings: AppSettings) -> Self {
        self.request = self.request.app_settings(app_settings);
        self
    }

    /// 构建请求
    pub fn build(self) -> UpdateAppV1Request {
        self.request
    }
}

impl AppService {
    /// 创建更新多维表格请求构建器
    pub fn update_app_v1_builder(&self) -> UpdateAppV1Builder {
        UpdateAppV1Builder::new(self.config.clone())
    }

    /// 创建更新多维表格请求
    pub fn update_app_v1(&self, app_token: String) -> UpdateAppV1Request {
        UpdateAppV1Request::new(self.config.clone()).app_token(app_token)
    }
}
