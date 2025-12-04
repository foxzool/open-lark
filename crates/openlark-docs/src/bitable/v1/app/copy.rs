//! Bitable V1 复制多维表格API

use openlark_core::{
    api::{ApiRequest, RequestData},
    config::Config,
    error::validation_error,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::models::{App, CopyAppRequest as CopyAppRequestBody};
use super::AppService;

/// 复制多维表格请求
pub struct CopyAppV1Request {
    api_request: ApiRequest<CopyAppV1Response>,
    /// 配置信息
    config: Config,
    /// 应用token
    app_token: String,
    /// 新应用名称
    name: Option<String>,
    /// 目标文件夹token
    folder_token: Option<String>,
    /// 是否复制内容（true: 不复制内容，false: 复制内容）
    without_content: Option<bool>,
    /// 时区
    time_zone: Option<String>,
}

/// 复制多维表格响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CopyAppV1Response {
    /// 应用信息
    pub data: App,
    pub success: bool,
}

impl CopyAppV1Request {
    /// 创建复制多维表格请求
    pub fn new(config: Config) -> Self {
        Self {
            api_request: ApiRequest::post(""),
            config,
            app_token: String::new(),
            name: None,
            folder_token: None,
            without_content: None,
            time_zone: None,
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.app_token = app_token;
        self
    }

    /// 设置新应用名称
    pub fn name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }

    /// 设置目标文件夹token
    pub fn folder_token(mut self, folder_token: String) -> Self {
        self.folder_token = Some(folder_token);
        self
    }

    /// 设置是否复制内容（true: 不复制内容，false: 复制内容）
    pub fn without_content(mut self, without_content: bool) -> Self {
        self.without_content = Some(without_content);
        self
    }

    /// 设置时区
    pub fn time_zone(mut self, time_zone: String) -> Self {
        self.time_zone = Some(time_zone);
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<CopyAppV1Response> {
        // 参数验证
        if self.app_token.trim().is_empty() {
            return Err(validation_error("app_token", "应用token不能为空"));
        }

        if let Some(ref name) = self.name {
            if name.trim().is_empty() {
                return Err(validation_error("name", "新应用名称不能为空"));
            }

            if name.len() > 100 {
                return Err(validation_error("name", "应用名称长度不能超过100个字符"));
            }
        }

        // 构建完整的API URL
        let api_url = format!(
            "{}/open-apis/bitable/v1/apps/{}/copy",
            self.config.base_url, self.app_token
        );

        // 构建请求体
        let request_body = CopyAppRequestBody {
            name: self.name.clone(),
            folder_token: self.folder_token.clone(),
            without_content: self.without_content,
            time_zone: self.time_zone.clone(),
        };

        // 验证请求参数
        if let Err(e) = request_body.validate() {
            return Err(validation_error("复制应用请求验证失败", e.to_string()));
        }

        // 设置API URL和请求体
        let mut api_request = self.api_request;
        api_request.url = api_url;
        api_request.body = Some(RequestData::Json(serde_json::to_value(&request_body)?));

        // 转换为ApiRequest<()>以匹配Transport::request签名
        let request_for_transport: ApiRequest<()> = ApiRequest::post(api_request.url.clone())
            .body(api_request.body.unwrap_or(RequestData::Empty));

        let config = &self.config;
        let response = Transport::request(request_for_transport, config, None).await?;

        // 解析响应数据，官方API格式为: { code: 0, data: { app: { ... } }, msg: "success" }
        let response_data: serde_json::Value = response
            .data
            .ok_or_else(|| validation_error("解析响应数据失败", "响应数据为空"))?;

        // 从data中提取app对象
        let app_data: App = serde_json::from_value(response_data.get("app")
            .ok_or_else(|| validation_error("解析应用数据失败", "响应中缺少app字段"))?
            .clone())
            .map_err(|e| validation_error("解析应用数据失败", format!("JSON解析错误: {}", e)))?;

        Ok(CopyAppV1Response {
            data: app_data,
            success: response.raw_response.is_success(),
        })
    }
}

/// 复制多维表格Builder
pub struct CopyAppV1Builder {
    request: CopyAppV1Request,
}

impl CopyAppV1Builder {
    /// 创建Builder实例
    pub fn new(config: Config) -> Self {
        Self {
            request: CopyAppV1Request::new(config),
        }
    }

    /// 设置应用token
    pub fn app_token(mut self, app_token: String) -> Self {
        self.request = self.request.app_token(app_token);
        self
    }

    /// 设置新应用名称
    pub fn name(mut self, name: String) -> Self {
        self.request = self.request.name(name);
        self
    }

    /// 设置目标文件夹token
    pub fn folder_token(mut self, folder_token: String) -> Self {
        self.request = self.request.folder_token(folder_token);
        self
    }

    /// 设置是否复制内容（true: 不复制内容，false: 复制内容）
    pub fn without_content(mut self, without_content: bool) -> Self {
        self.request = self.request.without_content(without_content);
        self
    }

    /// 设置时区
    pub fn time_zone(mut self, time_zone: String) -> Self {
        self.request = self.request.time_zone(time_zone);
        self
    }

    /// 构建请求
    pub fn build(self) -> CopyAppV1Request {
        self.request
    }
}

impl AppService {
    /// 创建复制多维表格请求构建器
    pub fn copy_app_v1_builder(&self) -> CopyAppV1Builder {
        CopyAppV1Builder::new(self.config.clone())
    }

    /// 创建复制多维表格请求
    pub fn copy_app_v1(&self, app_token: String) -> CopyAppV1Request {
        CopyAppV1Request::new(self.config.clone()).app_token(app_token)
    }
}
