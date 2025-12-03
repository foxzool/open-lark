//! Authen v1 API实现

pub mod oidc;
pub mod user_info;

use openlark_core::{
    config::Config,
    api::{ApiRequest, RequestData},
    prelude::Transport,
    error::{SDKResult, api_error},
};
use crate::{models::authen::*};

// 类型别名
pub type AuthResult<T> = SDKResult<T>;

// AuthenServiceV1结构体
#[derive(Debug)]
pub struct AuthenServiceV1 {
    config: Config,
}

impl AuthenServiceV1 {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

// 用户信息获取构建器
pub struct UserInfoGetBuilder {
    config: Config,
    request: UserInfoRequest,
}

impl UserInfoGetBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request: UserInfoRequest {
                user_access_token: String::new(),
                user_id_type: None,
            },
        }
    }

    pub fn user_access_token(mut self, token: impl Into<String>) -> Self {
        self.request.user_access_token = token.into();
        self
    }

    pub async fn send(self) -> AuthResult<UserInfoResponse> {
        // 构建API请求
        let url = format!("{}/open-apis/authen/v1/user_info", self.config.base_url);

        let request: ApiRequest<UserInfoResponse> = ApiRequest::get(&url)
            .header("Authorization", format!("Bearer {}", self.request.user_access_token))
            .query("user_id_type", self.request.user_id_type.as_deref().unwrap_or("open_id"));

        // 使用Transport发送请求
        let response = Transport::request(request, &self.config, None).await?;

        // 处理响应
        if response.raw_response.code == 0 {
            Ok(response.data.unwrap())
        } else {
            Err(api_error(
                response.raw_response.code as u16,
                "/open-apis/authen/v1/user_info",
                response.raw_response.msg.clone(),
                None::<String>
            ))
        }
    }
}

/// 用户访问令牌构建器（v1版本）
pub struct UserAccessTokenV1Builder {
    config: Config,
    request: UserAccessTokenV1Request,
}

impl UserAccessTokenV1Builder {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request: UserAccessTokenV1Request {
                grant_code: String::new(),
                app_id: String::new(),
                app_secret: String::new(),
            },
        }
    }

    pub fn grant_code(mut self, code: impl Into<String>) -> Self {
        self.request.grant_code = code.into();
        self
    }

    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.request.app_id = app_id.into();
        self
    }

    pub fn app_secret(mut self, app_secret: impl Into<String>) -> Self {
        self.request.app_secret = app_secret.into();
        self
    }

    pub async fn send(self) -> AuthResult<UserAccessTokenResponse> {
        // 构建API请求
        let url = format!("{}/open-apis/authen/v1/access_token", self.config.base_url);

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
                "/open-apis/authen/v1/access_token",
                response.raw_response.msg.clone(),
                None::<String>
            ))
        }
    }
}

/// 用户访问令牌刷新构建器（v1版本）
pub struct RefreshUserAccessTokenV1Builder {
    config: Config,
    request: RefreshUserAccessTokenV1Request,
}

impl RefreshUserAccessTokenV1Builder {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request: RefreshUserAccessTokenV1Request {
                refresh_token: String::new(),
                app_id: String::new(),
                app_secret: String::new(),
            },
        }
    }

    pub fn refresh_token(mut self, token: impl Into<String>) -> Self {
        self.request.refresh_token = token.into();
        self
    }

    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.request.app_id = app_id.into();
        self
    }

    pub fn app_secret(mut self, app_secret: impl Into<String>) -> Self {
        self.request.app_secret = app_secret.into();
        self
    }

    pub async fn send(self) -> AuthResult<UserAccessTokenResponse> {
        // 构建API请求
        let url = format!("{}/open-apis/authen/v1/refresh_access_token", self.config.base_url);

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
                "/open-apis/authen/v1/refresh_access_token",
                response.raw_response.msg.clone(),
                None::<String>
            ))
        }
    }
}

impl super::AuthenServiceV1 {
    /// 用户信息服务
    pub fn user_info(&self) -> UserInfoGetBuilder {
        UserInfoGetBuilder::new(self.config.clone())
    }

    /// 用户访问令牌服务（v1版本）
    pub fn access_token(&self) -> UserAccessTokenV1Builder {
        UserAccessTokenV1Builder::new(self.config.clone())
    }

    /// 用户访问令牌刷新服务（v1版本）
    pub fn refresh_access_token(&self) -> RefreshUserAccessTokenV1Builder {
        RefreshUserAccessTokenV1Builder::new(self.config.clone())
    }

    /// OIDC访问令牌服务
    pub fn oidc_access_token(&self) -> crate::api::authen::v1::oidc::OidcAccessTokenBuilder {
        crate::api::authen::v1::oidc::OidcAccessTokenBuilder::new(self.config.clone())
    }

    /// OIDC刷新访问令牌服务
    pub fn oidc_refresh_access_token(&self) -> crate::api::authen::v1::oidc::OidcRefreshAccessTokenBuilder {
        crate::api::authen::v1::oidc::OidcRefreshAccessTokenBuilder::new(self.config.clone())
    }
}