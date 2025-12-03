//! OIDC 用户访问令牌获取API实现
//!
//! 对应 meta.project=authen, meta.resource=oidc.access_token, meta.name=create

use openlark_core::{
    config::Config,
    api::ApiRequest,
    prelude::Transport,
    error::{SDKResult, CoreError, ErrorCode},
};
use crate::models::authen::{OidcUserAccessTokenRequest, UserAccessTokenResponse};

/// OIDC用户访问令牌构建器
#[derive(Debug)]
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

    /// 设置授权码
    pub fn code(mut self, code: impl Into<String>) -> Self {
        self.request.code = code.into();
        self
    }

    /// 设置授权验证码 (PKCE)
    pub fn code_verifier(mut self, code_verifier: impl Into<String>) -> Self {
        self.request.code_verifier = Some(code_verifier.into());
        self
    }

    /// 设置重定向URI
    pub fn redirect_uri(mut self, redirect_uri: impl Into<String>) -> Self {
        self.request.redirect_uri = Some(redirect_uri.into());
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

    /// 发送请求获取OIDC用户访问令牌
    pub async fn send(self) -> SDKResult<UserAccessTokenResponse> {
        // 构建API请求
        let url = format!("{}/open-apis/authen/v1/oidc/access_token", self.config.base_url);

        // 构建表单数据
        let mut form_data = std::collections::HashMap::new();
        form_data.insert("code".to_string(), self.request.code.clone());
        if let Some(ref code_verifier) = self.request.code_verifier {
            form_data.insert("code_verifier".to_string(), code_verifier.clone());
        }
        if let Some(ref redirect_uri) = self.request.redirect_uri {
            form_data.insert("redirect_uri".to_string(), redirect_uri.clone());
        }
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
                        message: "OIDC授权码无效".to_string(),
                        code: ErrorCode::AccessTokenInvalid,
                        ctx: {
                            let mut ctx = openlark_core::error::ErrorContext::new();
                            if let Some(ref req_id) = response.raw_response.request_id {
                                ctx.set_request_id(req_id);
                            }
                            ctx.add_context("feishu_code", feishu_code.to_string());
                            ctx.add_context("endpoint", "/open-apis/authen/v1/oidc/access_token");
                            ctx
                        },
                    })
                },
                Some(ErrorCode::UserIdentityInvalid) => {
                    Err(CoreError::Authentication {
                        message: "用户身份解析失败".to_string(),
                        code: ErrorCode::UserIdentityInvalid,
                        ctx: {
                            let mut ctx = openlark_core::error::ErrorContext::new();
                            if let Some(ref req_id) = response.raw_response.request_id {
                                ctx.set_request_id(req_id);
                            }
                            ctx.add_context("feishu_code", feishu_code.to_string());
                            ctx.add_context("endpoint", "/open-apis/authen/v1/oidc/access_token");
                            ctx
                        },
                    })
                },
                Some(ErrorCode::UserTypeNotSupportedV2) => {
                    Err(CoreError::Authentication {
                        message: "用户类型不支持OIDC授权".to_string(),
                        code: ErrorCode::UserTypeNotSupportedV2,
                        ctx: {
                            let mut ctx = openlark_core::error::ErrorContext::new();
                            if let Some(ref req_id) = response.raw_response.request_id {
                                ctx.set_request_id(req_id);
                            }
                            ctx.add_context("feishu_code", feishu_code.to_string());
                            ctx.add_context("endpoint", "/open-apis/authen/v1/oidc/access_token");
                            ctx
                        },
                    })
                },
                Some(ErrorCode::UserIdentityMismatch) => {
                    Err(CoreError::Authentication {
                        message: "用户身份不匹配".to_string(),
                        code: ErrorCode::UserIdentityMismatch,
                        ctx: {
                            let mut ctx = openlark_core::error::ErrorContext::new();
                            if let Some(ref req_id) = response.raw_response.request_id {
                                ctx.set_request_id(req_id);
                            }
                            ctx.add_context("feishu_code", feishu_code.to_string());
                            ctx.add_context("endpoint", "/open-apis/authen/v1/oidc/access_token");
                            ctx
                        },
                    })
                },
                Some(code) => {
                    Err(CoreError::Api(openlark_core::error::ApiError {
                        status: feishu_code as u16,
                        endpoint: "/open-apis/authen/v1/oidc/access_token".into(),
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
                        endpoint: "/open-apis/authen/v1/oidc/access_token".into(),
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