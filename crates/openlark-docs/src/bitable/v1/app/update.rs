//! Bitable V1 更新多维表格API

use openlark_core::{
    api::{ApiRequest, RequestData},
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
    /// 配置信息
    config: Config,
    api_request: ApiRequest<UpdateAppV1Response>,
    /// 应用token
    app_token: String,
    /// 应用名称
    name: Option<String>,
    /// 应用图标
    avatar: Option<String>,
    /// 应用设置
    app_settings: Option<AppSettings>,
}

/// 更新多维表格响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UpdateAppV1Response {
    /// 应用信息
    pub data: App,
    pub success: bool,
}

impl UpdateAppV1Request {
    /// 创建更新多维表格请求
    pub fn new(config: Config) -> Self {
        Self {
            config,
            api_request: ApiRequest::put(""),
            app_token: String::new(),
            name: None,
            avatar: None,
            app_settings: None,
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

        // 构建完整的API URL
        let api_url = format!(
            "{}/open-apis/bitable/v1/apps/{}",
            self.config.base_url, self.app_token
        );

        // 设置API URL
        let mut api_request = self.api_request;
        api_request.url = api_url;

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

        // 设置请求体
        api_request.body = Some(RequestData::Json(serde_json::to_value(&request_body)?));

        // 发送请求 - 转换为ApiRequest<()>以匹配Transport::request签名
        let request_for_transport: ApiRequest<()> = ApiRequest::put(api_request.url.clone())
            .body(api_request.body.unwrap_or(RequestData::Empty));

        let config = &self.config;
        let response = Transport::request(request_for_transport, config, None).await?;

        // 手动解析响应数据为App类型
        let app_data: App = response
            .data
            .and_then(|data| serde_json::from_value(data).ok())
            .ok_or_else(|| validation_error("解析应用数据失败", "响应数据格式不正确"))?;

        Ok(UpdateAppV1Response {
            data: app_data,
            success: response.raw_response.is_success(),
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
