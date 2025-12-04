//! 刷新 user_access_token（v1版本） API
//!
//! 对应CSV记录: 7180265937329520644
//! `user_access_token` 的最大有效期是 6900 秒。当 `user_access_token` 过期时，
//! 可以调用本接口获取新的 `user_access_token`。

use openlark_core::{
    config::Config,
    api::{ApiRequest, RequestData},
    error::{SDKResult, CoreError, ErrorCode, network_error},
};
use crate::models::authen::{RefreshUserAccessTokenV1Request, UserAccessTokenResponse};

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

    /// 发送请求刷新 user_access_token
    pub async fn send(self) -> SDKResult<UserAccessTokenResponse> {
        let url = format!("{}/open-apis/authen/v1/refresh_access_token", self.config.base_url);

        let mut request = ApiRequest::<UserAccessTokenResponse>::post(&url);
        request.headers.insert("Content-Type".to_string(), "application/json".to_string());

        let json_data = serde_json::to_value(&self.request)
            .map_err(|e| network_error(format!("请求数据序列化失败: {}", e)))?;
        request.body = Some(RequestData::Json(json_data));

        let response = openlark_core::http::Transport::request(request, &self.config, None)
            .await
            .map_err(|e| network_error(format!("刷新用户访问令牌API请求失败: {}", e)))?;

        if response.raw_response.code == 0 {
            Ok(response.data.unwrap())
        } else {
            // 智能映射飞书错误码（优先级：飞书通用码 > HTTP状态 > 内部码）
            let feishu_code = response.raw_response.code;
            let error_message = response.raw_response.msg.clone();

            match ErrorCode::from_feishu_code(feishu_code) {
                Some(ErrorCode::AccessTokenInvalid) => {
                    Err(CoreError::Authentication {
                        message: "刷新令牌无效或已过期".to_string(),
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
                Some(ErrorCode::PermissionMissing) => {
                    Err(CoreError::Authentication {
                        message: "应用权限不足，无法刷新用户访问令牌".to_string(),
                        code: ErrorCode::PermissionMissing,
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