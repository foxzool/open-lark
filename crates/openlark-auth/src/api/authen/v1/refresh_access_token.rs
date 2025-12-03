//! 标准用户访问令牌刷新API实现
//!
//! 对应 meta.project=authen, meta.resource=refresh_access_token, meta.name=create

use openlark_core::{
    config::Config,
    api::ApiRequest,
    prelude::Transport,
    error::{SDKResult, CoreError, ErrorCode},
};
use crate::models::authen::{RefreshUserAccessTokenV1Request, UserAccessTokenResponse};

// 类型别名
pub type AuthenResult<T> = SDKResult<T>;

/// 用户访问令牌刷新构建器（v1版本）
#[derive(Debug)]
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

    /// 设置刷新令牌
    pub fn refresh_token(mut self, refresh_token: impl Into<String>) -> Self {
        self.request.refresh_token = refresh_token.into();
        self
    }

    /// 设置应用ID
    pub fn app_id(mut self, app_id: impl Into<String>) -> Self {
        self.request.app_id = app_id.into();
        self
    }

    /// 设置应用密钥
    pub fn app_secret(mut self, app_secret: impl Into<String>) -> Self {
        self.request.app_secret = app_secret.into();
        self
    }

    /// 发送请求刷新用户访问令牌
    pub async fn send(self) -> AuthenResult<UserAccessTokenResponse> {
        // 构建API请求
        let url = format!("{}/open-apis/authen/v1/refresh_access_token", self.config.base_url);

        let request: ApiRequest<UserAccessTokenResponse> = ApiRequest::post(&url)
            .header("Content-Type", "application/json")
            .body(openlark_core::api::RequestData::Json(serde_json::to_value(&self.request)?));

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
                        message: "刷新令牌无效".to_string(),
                        code: ErrorCode::AccessTokenInvalid,
                        ctx: {
                            let mut ctx = openlark_core::error::ErrorContext::new();
                            if let Some(ref req_id) = response.raw_response.request_id {
                                ctx.set_request_id(req_id);
                            }
                            ctx.add_context("feishu_code", feishu_code.to_string());
                            ctx.add_context("endpoint", "/open-apis/authen/v1/refresh_access_token");
                            ctx
                        },
                    })
                },
                Some(ErrorCode::AccessTokenExpiredV2) => {
                    Err(CoreError::Authentication {
                        message: "刷新令牌已过期".to_string(),
                        code: ErrorCode::AccessTokenExpiredV2,
                        ctx: {
                            let mut ctx = openlark_core::error::ErrorContext::new();
                            if let Some(ref req_id) = response.raw_response.request_id {
                                ctx.set_request_id(req_id);
                            }
                            ctx.add_context("feishu_code", feishu_code.to_string());
                            ctx.add_context("endpoint", "/open-apis/authen/v1/refresh_access_token");
                            ctx
                        },
                    })
                },
                Some(code) => {
                    Err(CoreError::Api(openlark_core::error::ApiError {
                        status: feishu_code as u16,
                        endpoint: "/open-apis/authen/v1/refresh_access_token".into(),
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
                        endpoint: "/open-apis/authen/v1/refresh_access_token".into(),
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