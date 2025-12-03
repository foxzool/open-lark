//! OIDC 用户访问令牌刷新API实现
//!
//! 对应 meta.project=authen, meta.resource=oidc.refresh_access_token, meta.name=create

use openlark_core::{
    config::Config,
    api::ApiRequest,
    prelude::Transport,
    error::{SDKResult, api_error},
};
use crate::models::authen::{OidcRefreshUserAccessTokenRequest, UserAccessTokenResponse};

// 类型别名
pub type AuthenResult<T> = SDKResult<T>;

/// OIDC用户访问令牌刷新构建器
#[derive(Debug)]
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

    /// 设置刷新令牌
    pub fn refresh_token(mut self, refresh_token: impl Into<String>) -> Self {
        self.request.refresh_token = refresh_token.into();
        self
    }

    /// 设置客户端ID
    pub fn client_id(mut self, client_id: impl Into<String>) -> Self {
        self.request.client_id = Some(client_id.into());
        self
    }

    /// 设置客户端密钥
    pub fn client_secret(mut self, client_secret: impl Into<String>) -> Self {
        self.request.client_secret = Some(client_secret.into());
        self
    }

    /// 设置授权类型
    pub fn grant_type(mut self, grant_type: impl Into<String>) -> Self {
        self.request.grant_type = Some(grant_type.into());
        self
    }

    /// 发送请求刷新OIDC用户访问令牌
    pub async fn send(self) -> AuthenResult<UserAccessTokenResponse> {
        // 构建API请求
        let url = format!("{}/open-apis/authen/v1/oidc/refresh_access_token", self.config.base_url);

        // 构建表单数据
        let mut form_data = std::collections::HashMap::new();
        form_data.insert("refresh_token".to_string(), self.request.refresh_token.clone());
        if let Some(ref client_id) = self.request.client_id {
            form_data.insert("client_id".to_string(), client_id.clone());
        }
        if let Some(ref client_secret) = self.request.client_secret {
            form_data.insert("client_secret".to_string(), client_secret.clone());
        }
        if let Some(ref grant_type) = self.request.grant_type {
            form_data.insert("grant_type".to_string(), grant_type.clone());
        }

        let request: ApiRequest<UserAccessTokenResponse> = ApiRequest::post(&url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(openlark_core::api::RequestData::Form(form_data));

        // 使用Transport发送请求
        let response = Transport::request(request, &self.config, None).await?;

        // 处理响应
        if response.raw_response.code == 0 {
            Ok(response.data.unwrap())
        } else {
            // 映射飞书错误码
            let error_code = response.raw_response.code;
            let error_message = response.raw_response.msg.clone();

            match error_code {
                99991663 => Err(api_error(400, "/open-apis/authen/v1/oidc/refresh_access_token", "刷新令牌无效", None::<String>)),
                99991677 => Err(api_error(400, "/open-apis/authen/v1/oidc/refresh_access_token", "刷新令牌已过期", None::<String>)),
                _ => Err(api_error(error_code as u16, "/open-apis/authen/v1/oidc/refresh_access_token", error_message, None::<String>)),
            }
        }
    }
}