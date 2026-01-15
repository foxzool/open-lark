//! 获取 user_access_token（v1版本） API
use crate::models::authen::{UserAccessTokenResponse, UserAccessTokenV1Request};
///
/// API文档: https://open.feishu.cn/document/server-docs/user-authentication/access-token/access_token
///
/// 根据登录预授权码获取 user_access_token
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 用户访问令牌请求（v1版本）
pub struct UserAccessTokenV1Builder {
    grant_code: String,
    app_id: String,
    app_secret: String,
    /// 配置信息
    config: Config,
}

/// 用户访问令牌响应（v1版本）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserAccessTokenV1ResponseData {
    /// 用户访问令牌响应
    pub data: UserAccessTokenResponse,
}

impl ApiResponseTrait for UserAccessTokenV1ResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl UserAccessTokenV1Builder {
    /// 创建 access_token 请求
    pub fn new(config: Config) -> Self {
        Self {
            grant_code: String::new(),
            app_id: String::new(),
            app_secret: String::new(),
            config,
        }
    }

    /// 设置授权码
    pub fn grant_code(mut self, grant_code: impl Into<String>) -> Self {
        self.grant_code = grant_code.into();
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
    pub async fn execute(self) -> SDKResult<UserAccessTokenV1ResponseData> {
        // 验证必填字段
        validate_required!(self.grant_code, "授权码不能为空");
        validate_required!(self.app_id, "应用ID不能为空");
        validate_required!(self.app_secret, "应用密钥不能为空");

        // 🚀 使用新的enum+builder系统生成API端点
        use crate::common::api_endpoints::AuthenApiV1;
        let api_endpoint = AuthenApiV1::AccessToken;

        // 构建请求体
        let request_body = UserAccessTokenV1Request {
            grant_code: self.grant_code.clone(),
            app_id: self.app_id.clone(),
            app_secret: self.app_secret.clone(),
        };

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<UserAccessTokenV1ResponseData> =
            ApiRequest::post(api_endpoint.path()).body(serde_json::to_value(&request_body)?);

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}
