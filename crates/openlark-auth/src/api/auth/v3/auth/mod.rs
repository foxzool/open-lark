//! Auth v3 /auth 路径下的API实现
//!
//! 包含企业应用认证相关的具体API实现

use openlark_core::{
    config::Config,
    api::{ApiRequest, RequestData},
    prelude::Transport,
    error::{SDKResult, api_error},
};
use crate::{models::auth::*};

// 类型别名
pub type AuthResult<T> = SDKResult<T>;

/// Auth v3 API服务
#[derive(Debug)]
pub struct AuthServiceV3 {
    config: Config,
}

impl AuthServiceV3 {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

/// 应用访问令牌构建器（商店应用）
pub struct AppAccessTokenBuilder {
    config: Config,
    request: AppAccessTokenRequest,
}

impl AppAccessTokenBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request: AppAccessTokenRequest {
                app_id: String::new(),
                app_secret: String::new(),
            },
        }
    }

    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.request.app_id = app_id.into();
        self
    }

    pub fn app_secret(mut self, app_secret: impl Into<String>) -> Self {
        self.request.app_secret = app_secret.into();
        self
    }

    pub async fn send(self) -> AuthResult<AccessTokenResponse> {
        // 构建API请求
        let url = format!("{}/open-apis/auth/v3/app_access_token", self.config.base_url);

        let request: ApiRequest<AccessTokenResponse> = ApiRequest::post(&url)
            .body(RequestData::Json(serde_json::to_value(&self.request)?))
            .header("Content-Type", "application/json");

        // 使用Transport发送请求
        let response = Transport::request(request, &self.config, None).await?;

        // 处理响应
        if response.raw_response.code == 0 {
            Ok(response.data.unwrap()) // 期望成功的响应包含数据
        } else {
            // 映射飞书错误码
            let error_code = response.raw_response.code;
            let error_message = response.raw_response.msg.clone();

            match error_code {
                99991663 => Err(api_error(400, "/open-apis/auth/v3/app_access_token", "应用访问令牌无效", None::<String>)),
                _ => Err(api_error(error_code as u16, "/open-apis/auth/v3/app_access_token", error_message, None::<String>)),
            }
        }
    }
}

/// 应用访问令牌构建器（自建应用）
pub struct AppAccessTokenInternalBuilder {
    config: Config,
    request: AppAccessTokenInternalRequest,
}

impl AppAccessTokenInternalBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request: AppAccessTokenInternalRequest {
                app_id: String::new(),
                app_secret: String::new(),
            },
        }
    }

    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.request.app_id = app_id.into();
        self
    }

    pub fn app_secret(mut self, app_secret: impl Into<String>) -> Self {
        self.request.app_secret = app_secret.into();
        self
    }

    pub async fn send(self) -> AuthResult<AccessTokenResponse> {
        // 自建应用获取app_access_token - 使用不同的端点
        let url = format!("{}/open-apis/auth/v3/app_access_token/internal", self.config.base_url);

        let request: ApiRequest<AccessTokenResponse> = ApiRequest::post(&url)
            .body(RequestData::Json(serde_json::to_value(&self.request)?))
            .header("Content-Type", "application/json");

        let response = Transport::request(request, &self.config, None).await?;

        if response.raw_response.code == 0 {
            Ok(response.data.unwrap())
        } else {
            Err(api_error(response.raw_response.code as u16, "/open-apis/auth/v3/app_access_token/internal", response.raw_response.msg.clone(), None::<String>))
        }
    }
}

/// 租户访问令牌构建器（商店应用）
pub struct TenantAccessTokenBuilder {
    config: Config,
    request: TenantAccessTokenRequest,
}

impl TenantAccessTokenBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request: TenantAccessTokenRequest {
                app_id: String::new(),
                app_secret: String::new(),
            },
        }
    }

    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.request.app_id = app_id.into();
        self
    }

    pub fn app_secret(mut self, app_secret: impl Into<String>) -> Self {
        self.request.app_secret = app_secret.into();
        self
    }

    pub async fn send(self) -> AuthResult<TenantAccessTokenResponse> {
        let url = format!("{}/open-apis/auth/v3/tenant_access_token", self.config.base_url);

        let request: ApiRequest<TenantAccessTokenResponse> = ApiRequest::post(&url)
            .body(RequestData::Json(serde_json::to_value(&self.request)?))
            .header("Content-Type", "application/json");

        let response = Transport::request(request, &self.config, None).await?;

        if response.raw_response.code == 0 {
            Ok(response.data.unwrap())
        } else {
            Err(api_error(response.raw_response.code as u16, "/open-apis/auth/v3/tenant_access_token", response.raw_response.msg.clone(), None::<String>))
        }
    }
}

/// 租户访问令牌构建器（自建应用）
pub struct TenantAccessTokenInternalBuilder {
    config: Config,
    request: TenantAccessTokenInternalRequest,
}

impl TenantAccessTokenInternalBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request: TenantAccessTokenInternalRequest {
                app_id: String::new(),
                app_secret: String::new(),
            },
        }
    }

    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.request.app_id = app_id.into();
        self
    }

    pub fn app_secret(mut self, app_secret: impl Into<String>) -> Self {
        self.request.app_secret = app_secret.into();
        self
    }

    pub async fn send(self) -> AuthResult<TenantAccessTokenResponse> {
        let url = format!("{}/open-apis/auth/v3/tenant_access_token/internal", self.config.base_url);

        let request: ApiRequest<TenantAccessTokenResponse> = ApiRequest::post(&url)
            .body(RequestData::Json(serde_json::to_value(&self.request)?))
            .header("Content-Type", "application/json");

        let response = Transport::request(request, &self.config, None).await?;

        if response.raw_response.code == 0 {
            Ok(response.data.unwrap())
        } else {
            Err(api_error(response.raw_response.code as u16, "/open-apis/auth/v3/tenant_access_token/internal", response.raw_response.msg.clone(), None::<String>))
        }
    }
}

/// 应用票据重发构建器
pub struct AppTicketResendBuilder {
    config: Config,
    request: AppTicketResendRequest,
}

impl AppTicketResendBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request: AppTicketResendRequest {
                app_id: String::new(),
                app_secret: String::new(),
            },
        }
    }

    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.request.app_id = app_id.into();
        self
    }

    pub fn app_secret(mut self, app_secret: impl Into<String>) -> Self {
        self.request.app_secret = app_secret.into();
        self
    }

    pub async fn send(self) -> AuthResult<AppTicketResponse> {
        let url = format!("{}/open-apis/auth/v3/app_ticket/resend", self.config.base_url);

        let request: ApiRequest<AppTicketResponse> = ApiRequest::post(&url)
            .body(RequestData::Json(serde_json::to_value(&self.request)?))
            .header("Content-Type", "application/json");

        let response = Transport::request(request, &self.config, None).await?;

        if response.raw_response.code == 0 {
            Ok(response.data.unwrap())
        } else {
            Err(api_error(response.raw_response.code as u16, "/open-apis/auth/v3/app_ticket/resend", response.raw_response.msg.clone(), None::<String>))
        }
    }
}

// 为AuthServiceV3提供访问各个API的方法
impl AuthServiceV3 {
    /// 商店应用获取app_access_token
    pub fn app_access_token(&self) -> AppAccessTokenBuilder {
        AppAccessTokenBuilder::new(self.config.clone())
    }

    /// 自建应用获取app_access_token
    pub fn app_access_token_internal(&self) -> AppAccessTokenInternalBuilder {
        AppAccessTokenInternalBuilder::new(self.config.clone())
    }

    /// 商店应用获取tenant_access_token
    pub fn tenant_access_token(&self) -> TenantAccessTokenBuilder {
        TenantAccessTokenBuilder::new(self.config.clone())
    }

    /// 自建应用获取tenant_access_token
    pub fn tenant_access_token_internal(&self) -> TenantAccessTokenInternalBuilder {
        TenantAccessTokenInternalBuilder::new(self.config.clone())
    }

    /// 重新获取app_ticket
    pub fn app_ticket_resend(&self) -> AppTicketResendBuilder {
        AppTicketResendBuilder::new(self.config.clone())
    }
}