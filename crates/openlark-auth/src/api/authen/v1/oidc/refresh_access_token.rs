//! OIDC 用户访问令牌刷新API实现
//!
//! 对应 meta.project=authen, meta.resource=oidc.refresh_access_token, meta.name=create

use openlark_core::{
    config::Config,
    api::ApiRequest,
    prelude::Transport,
    error::{SDKResult, CoreError, ErrorCode},
};
use crate::models::authen::{OidcRefreshUserAccessTokenRequest, UserAccessTokenResponse};

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
    pub async fn send(self) -> SDKResult<UserAccessTokenResponse> {
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
            // 智能映射飞书错误码（优先级：飞书通用码 > HTTP状态 > 内部码）
            let feishu_code = response.raw_response.code;
            let error_message = response.raw_response.msg.clone();

            match ErrorCode::from_feishu_code(feishu_code) {
                Some(ErrorCode::AccessTokenInvalid) => {
                    Err(CoreError::Authentication {
                        message: "OIDC刷新令牌无效".to_string(),
                        code: ErrorCode::AccessTokenInvalid,
                        ctx: {
                            let mut ctx = openlark_core::error::ErrorContext::new();
                            if let Some(ref req_id) = response.raw_response.request_id {
                                ctx.set_request_id(req_id);
                            }
                            ctx.add_context("feishu_code", feishu_code.to_string());
                            ctx.add_context("endpoint", "/open-apis/authen/v1/oidc/refresh_access_token");
                            ctx
                        },
                    })
                },
                Some(ErrorCode::AccessTokenExpiredV2) => {
                    Err(CoreError::Authentication {
                        message: "OIDC刷新令牌已过期".to_string(),
                        code: ErrorCode::AccessTokenExpiredV2,
                        ctx: {
                            let mut ctx = openlark_core::error::ErrorContext::new();
                            if let Some(ref req_id) = response.raw_response.request_id {
                                ctx.set_request_id(req_id);
                            }
                            ctx.add_context("feishu_code", feishu_code.to_string());
                            ctx.add_context("endpoint", "/open-apis/authen/v1/oidc/refresh_access_token");
                            ctx
                        },
                    })
                },
                Some(code) => {
                    Err(CoreError::Api(openlark_core::error::ApiError {
                        status: feishu_code as u16,
                        endpoint: "/open-apis/authen/v1/oidc/refresh_access_token".into(),
                        message: error_message,
                        source: None,
                        code,
                        ctx: {
                            let mut ctx = openlark_core::error::ErrorContext::new();
                            if let Some(ref req_id) = response.raw_response.request_id {
                                ctx.set_request_id(req_id);
                            }
                            ctx.add_context("feishu_code", feishu_code.to_string());
                            ctx
                        },
                    }))
                },
                None => {
                    // 回退到HTTP状态码或内部业务码
                    Err(CoreError::Api(openlark_core::error::ApiError {
                        status: feishu_code as u16,
                        endpoint: "/open-apis/authen/v1/oidc/refresh_access_token".into(),
                        message: error_message,
                        source: None,
                        code: ErrorCode::from_http_status(feishu_code as u16),
                        ctx: {
                            let mut ctx = openlark_core::error::ErrorContext::new();
                            if let Some(ref req_id) = response.raw_response.request_id {
                                ctx.set_request_id(req_id);
                            }
                            ctx.add_context("feishu_code", feishu_code.to_string());
                            ctx
                        },
                    }))
                }
            }
        }
    }
}