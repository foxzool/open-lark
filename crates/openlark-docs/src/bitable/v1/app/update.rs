//! Bitable V1 更新多维表格API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::models::{App, AppSettings, UpdateAppRequest as UpdateAppRequestBody};
use super::AppService;

/// 更新多维表格请求
pub struct UpdateAppV1Request {
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
            app_token: String::new(),
            name: None,
            avatar: None,
            app_settings: None,
            config,
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: impl Into<String>) -> Self {
        self.app_token = app_token.into();
        self
    }

    /// 设置应用名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    /// 设置应用图标
    pub fn avatar(mut self, avatar: impl Into<String>) -> Self {
        self.avatar = Some(avatar.into());
        self
    }

    /// 设置应用设置
    pub fn app_settings(mut self, app_settings: AppSettings) -> Self {
        self.app_settings = Some(app_settings);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<UpdateAppV1Response> {
        // 验证必填字段
        validate_required!(self.app_token, "应用令牌不能为空");

        // 构建API路径
        let path = format!("/open-apis/bitable/v1/apps/{}", self.app_token);

        // 构建请求体
        let request_body = UpdateAppRequestBody {
            name: self.name.clone(),
            avatar: self.avatar.clone(),
            app_settings: self.app_settings.clone(),
        };

        // 创建API请求
        let api_request: ApiRequest<UpdateAppV1Response> = ApiRequest::put(&path)
            .body(openlark_core::api::RequestData::Binary(serde_json::to_vec(&request_body)?));

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}

impl AppService {
    /// 创建更新多维表格请求
    pub fn update_builder(
        &self,
        app_token: impl Into<String>,
    ) -> UpdateAppV1Request {
        UpdateAppV1Request::new(self.config.clone()).app_token(app_token)
    }

    /// 创建更新多维表格请求（带完整参数）
    pub fn update_app_v1(
        &self,
        app_token: impl Into<String>,
        name: Option<impl Into<String>>,
        avatar: Option<impl Into<String>>,
        app_settings: Option<AppSettings>,
    ) -> UpdateAppV1Request {
        let mut request = UpdateAppV1Request::new(self.config.clone()).app_token(app_token);

        if let Some(name) = name {
            request = request.name(name);
        }

        if let Some(avatar) = avatar {
            request = request.avatar(avatar);
        }

        if let Some(app_settings) = app_settings {
            request = request.app_settings(app_settings);
        }

        request
    }
}
