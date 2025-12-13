//! 刷新 user_access_token（v1版本） API
use crate::models::authen::{RefreshUserAccessTokenV1Request, UserAccessTokenResponse};
///
/// API文档: https://open.feishu.cn/document/server-docs/user-authentication/access-token/refresh_access_token
///
/// user_access_token 的最大有效期是 6900 秒。当 user_access_token 过期时，
/// 可以调用本接口获取新的 user_access_token
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 用户访问令牌刷新请求（v1版本）
pub struct RefreshUserAccessTokenV1Builder {
    refresh_token: String,
    app_id: String,
    app_secret: String,
    /// 配置信息
    config: Config,
}

/// 用户访问令牌刷新响应（v1版本）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RefreshUserAccessTokenV1ResponseData {
    /// 用户访问令牌响应
    pub data: UserAccessTokenResponse,
}

impl ApiResponseTrait for RefreshUserAccessTokenV1ResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl RefreshUserAccessTokenV1Builder {
    /// 创建 refresh_access_token 请求
    pub fn new(config: Config) -> Self {
        Self {
            refresh_token: String::new(),
            app_id: String::new(),
            app_secret: String::new(),
            config,
        }
    }

    /// 设置刷新令牌
    pub fn refresh_token(mut self, refresh_token: impl Into<String>) -> Self {
        self.refresh_token = refresh_token.into();
        self
    }

    /// 设置应用ID
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
    pub async fn execute(self) -> SDKResult<RefreshUserAccessTokenV1ResponseData> {
        // 验证必填字段
        validate_required!(self.refresh_token, "刷新令牌不能为空");
        validate_required!(self.app_id, "应用ID不能为空");
        validate_required!(self.app_secret, "应用密钥不能为空");

        // 🚀 使用新的enum+builder系统生成API端点
        use crate::common::api_endpoints::AuthenApiV1;
        let api_endpoint = AuthenApiV1::RefreshAccessToken;

        // 构建请求体
        let request_body = RefreshUserAccessTokenV1Request {
            refresh_token: self.refresh_token.clone(),
            app_id: self.app_id.clone(),
            app_secret: self.app_secret.clone(),
        };

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<RefreshUserAccessTokenV1ResponseData> =
            ApiRequest::post(&api_endpoint.to_url()).body(openlark_core::api::RequestData::Json(
                serde_json::to_value(&request_body)?,
            ));

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
