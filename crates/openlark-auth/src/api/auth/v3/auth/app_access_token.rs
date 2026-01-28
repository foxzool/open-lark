//! 商店应用获取 app_access_token API
use crate::models::auth::{AccessTokenResponse, AppAccessTokenRequest};
///
/// API文档: https://open.feishu.cn/document/server-docs/authentication-management/access-token/app_access_token
///
/// 应用商店应用通过此接口获取 app_access_token，调用接口获取应用资源时，
/// 需要使用 app_access_token 作为授权凭证。
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};



/// 商店应用获取 app_access_token 请求
pub struct AppAccessTokenBuilder {
    app_id: String,
    app_secret: String,
    /// 配置信息
    config: Config,
}

/// 商店应用获取 app_access_token 响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppAccessTokenResponseData {
    /// 应用访问令牌响应
    pub data: AccessTokenResponse,
}

impl ApiResponseTrait for AppAccessTokenResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl AppAccessTokenBuilder {
    /// 创建 app_access_token 请求
    pub fn new(config: Config) -> Self {
        Self {
            app_id: String::new(),
            app_secret: String::new(),
            config,
        }
    }

    /// 设置应用 ID
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = app_id.into();
        self
    }

    /// 设置应用密钥
    pub fn app_secret(mut self, app_secret: impl Into<String>) -> Self {
        self.app_secret = app_secret.into();
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<AppAccessTokenResponseData> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 执行请求（带选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<AppAccessTokenResponseData> {
        // 验证必填字段
        validate_required!(self.app_id, "应用ID不能为空");
        validate_required!(self.app_secret, "应用密钥不能为空");

        // 🚀 使用新的enum+builder系统生成API端点
        use crate::common::api_endpoints::AuthApiV3;
        let api_endpoint = AuthApiV3::AppAccessToken;

        // 构建请求体
        let request_body = AppAccessTokenRequest {
            app_id: self.app_id.clone(),
            app_secret: self.app_secret.clone(),
        };

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<AppAccessTokenResponseData> =
            ApiRequest::post(api_endpoint.path()).body(serde_json::to_value(&request_body)?);

        // 发送请求
        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取商店应用 access_token", "响应数据为空")
        })
    }
}
