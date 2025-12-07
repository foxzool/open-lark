//! OIDC 用户访问令牌刷新API
///
/// API文档: https://open.feishu.cn/document/server-docs/user-authentication/access-token/oidc_refresh_access_token
///
/// 通过 OIDC 刷新令牌获取新的用户访问令牌

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use crate::models::authen::UserAccessTokenResponse;

/// OIDC 用户访问令牌刷新请求
pub struct OidcRefreshAccessTokenBuilder {
    refresh_token: String,
    client_id: Option<String>,
    client_secret: Option<String>,
    grant_type: Option<String>,
    /// 配置信息
    config: Config,
}

/// OIDC 用户访问令牌刷新响应
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OidcRefreshAccessTokenResponseData {
    /// 用户访问令牌响应
    pub data: UserAccessTokenResponse,
}

impl ApiResponseTrait for OidcRefreshAccessTokenResponseData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl OidcRefreshAccessTokenBuilder {
    /// 创建 oidc_refresh_access_token 请求
    pub fn new(config: Config) -> Self {
        Self {
            refresh_token: String::new(),
            client_id: None,
            client_secret: None,
            grant_type: Some("refresh_token".to_string()),
            config,
        }
    }

    /// 设置刷新令牌
    pub fn refresh_token(mut self, refresh_token: impl Into<String>) -> Self {
        self.refresh_token = refresh_token.into();
        self
    }

    /// 设置客户端ID
    pub fn client_id(mut self, client_id: impl Into<String>) -> Self {
        self.client_id = Some(client_id.into());
        self
    }

    /// 设置客户端密钥
    pub fn client_secret(mut self, client_secret: impl Into<String>) -> Self {
        self.client_secret = Some(client_secret.into());
        self
    }

    /// 设置授权类型
    pub fn grant_type(mut self, grant_type: impl Into<String>) -> Self {
        self.grant_type = Some(grant_type.into());
        self
    }

    /// 执行请求
    pub async fn execute(self) -> SDKResult<OidcRefreshAccessTokenResponseData> {
        // 验证必填字段
        validate_required!(self.refresh_token, "刷新令牌不能为空");

        // 🚀 使用新的enum+builder系统生成API端点
        use crate::common::api_endpoints::AuthenApiV1;
        let api_endpoint = AuthenApiV1::OidcRefreshAccessToken;

        // 构建表单数据
        let mut form_data = std::collections::HashMap::new();
        form_data.insert("refresh_token".to_string(), self.refresh_token.clone());
        if let Some(ref client_id) = self.client_id {
            form_data.insert("client_id".to_string(), client_id.clone());
        }
        if let Some(ref client_secret) = self.client_secret {
            form_data.insert("client_secret".to_string(), client_secret.clone());
        }
        if let Some(ref grant_type) = self.grant_type {
            form_data.insert("grant_type".to_string(), grant_type.clone());
        }

        // 创建API请求 - 使用类型安全的URL生成
        let api_request: ApiRequest<OidcRefreshAccessTokenResponseData> =
            ApiRequest::post(&api_endpoint.to_url())
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(openlark_core::api::RequestData::Form(form_data));

        // 发送请求
        let response = Transport::request(api_request, &self.config, None).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("响应数据为空", "服务器没有返回有效的数据")
        })
    }
}