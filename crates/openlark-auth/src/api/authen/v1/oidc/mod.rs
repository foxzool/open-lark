//! OIDC API实现

use openlark_core::{
    config::Config,
    api::{ApiRequest, RequestData},
    prelude::Transport,
    error::{SDKResult, api_error},
};
use crate::{models::authen::*};

// 类型别名
pub type AuthResult<T> = SDKResult<T>;

// OIDC访问令牌构建器
pub struct OidcAccessTokenBuilder {
    config: Config,
    request: OidcUserAccessTokenRequest,
}

impl OidcAccessTokenBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request: OidcUserAccessTokenRequest {
                code: String::new(),
                code_verifier: None,
                redirect_uri: None,
                client_id: None,
                client_secret: None,
                grant_type: Some("authorization_code".to_string()),
            },
        }
    }

    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.request.code = code.into();
        self
    }

    pub fn redirect_uri(mut self, uri: impl Into<String>) -> Self {
        self.request.redirect_uri = Some(uri.into());
        self
    }

    pub async fn send(self) -> AuthResult<UserAccessTokenResponse> {
        // 构建API请求
        let url = format!("{}/open-apis/authen/v1/oidc/access_token", self.config.base_url);

        let request: ApiRequest<UserAccessTokenResponse> = ApiRequest::post(&url)
            .body(RequestData::Json(serde_json::to_value(&self.request)?))
            .header("Content-Type", "application/json");

        // 使用Transport发送请求
        let response = Transport::request(request, &self.config, None).await?;

        // 处理响应
        if response.raw_response.code == 0 {
            Ok(response.data.unwrap())
        } else {
            Err(api_error(
                response.raw_response.code as u16,
                "/open-apis/authen/v1/oidc/access_token",
                response.raw_response.msg.clone(),
                None::<String>
            ))
        }
    }
}

// OIDC刷新令牌构建器
pub struct OidcRefreshAccessTokenBuilder {
    config: Config,
    request: OidcRefreshUserAccessTokenRequest,
}

impl OidcRefreshAccessTokenBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request: OidcRefreshUserAccessTokenRequest {
                refresh_token: String::new(),
                client_id: None,
                client_secret: None,
                grant_type: Some("refresh_token".to_string()),
            },
        }
    }

    pub fn refresh_token(mut self, token: impl Into<String>) -> Self {
        self.request.refresh_token = token.into();
        self
    }

    pub async fn send(self) -> AuthResult<UserAccessTokenResponse> {
        // 构建API请求
        let url = format!("{}/open-apis/authen/v1/oidc/refresh_access_token", self.config.base_url);

        let request: ApiRequest<UserAccessTokenResponse> = ApiRequest::post(&url)
            .body(RequestData::Json(serde_json::to_value(&self.request)?))
            .header("Content-Type", "application/json");

        // 使用Transport发送请求
        let response = Transport::request(request, &self.config, None).await?;

        // 处理响应
        if response.raw_response.code == 0 {
            Ok(response.data.unwrap())
        } else {
            Err(api_error(
                response.raw_response.code as u16,
                "/open-apis/authen/v1/oidc/refresh_access_token",
                response.raw_response.msg.clone(),
                None::<String>
            ))
        }
    }
}