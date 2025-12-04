//! 自建应用获取 app_access_token API
//!
//! 对应CSV记录: 6995779366223822852
//! 企业自建应用通过此接口获取 app_access_token，调用接口获取应用资源时，
//! 需要使用 app_access_token 作为授权凭证。

use openlark_core::{
    config::Config,
    api::{ApiRequest, RequestData},
    error::{SDKResult, CoreError, ErrorCode, network_error},
};
use crate::models::auth::{AppAccessTokenInternalRequest, AccessTokenResponse};

/// 应用访问令牌构建器（自建应用）
#[derive(Debug)]
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

    /// 发送请求获取 app_access_token
    pub async fn send(self) -> SDKResult<AccessTokenResponse> {
        let url = format!("{}/open-apis/auth/v3/app_access_token/internal", self.config.base_url);

        let mut request = ApiRequest::<AccessTokenResponse>::post(&url);
        request.headers.insert("Content-Type".to_string(), "application/json".to_string());

        let json_data = serde_json::to_value(&self.request)
            .map_err(|e| network_error(format!("请求数据序列化失败: {}", e)))?;
        request.body = Some(RequestData::Json(json_data));

        let response = openlark_core::http::Transport::request(request, &self.config, None)
            .await
            .map_err(|e| network_error(format!("自建应用访问令牌API请求失败: {}", e)))?;

        // 处理响应
        if response.raw_response.code == 0 {
            Ok(response.data.unwrap_or_else(|| {
                // 如果没有data字段，尝试直接解析响应
                serde_json::from_value::<AccessTokenResponse>(serde_json::json!({
                    "app_access_token": response.raw_response.data.unwrap_or_default(),
                    "expires_in": 7200, // 默认2小时
                    "tenant_key": "",
                    "token_type": Some("Bearer".to_string())
                })).unwrap_or_else(|_| AccessTokenResponse {
                    app_access_token: String::new(),
                    expires_in: 7200,
                    tenant_key: String::new(),
                    token_type: Some("Bearer".to_string()),
                })
            }))
        } else {
            // API返回错误，智能映射飞书错误码
            let feishu_code = response.raw_response.code;
            let error_message = response.raw_response.msg;

            match ErrorCode::from_feishu_code(feishu_code) {
                Some(ErrorCode::AppAccessTokenInvalid) => {
                    Err(CoreError::Authentication {
                        message: "自建应用访问令牌格式或内容无效".to_string(),
                        code: ErrorCode::AppAccessTokenInvalid,
                        ctx: {
                            let mut ctx = openlark_core::error::ErrorContext::new();
                            if let Some(ref req_id) = response.raw_response.request_id {
                                ctx.set_request_id(req_id);
                            }
                            ctx.add_context("feishu_code", feishu_code.to_string());
                            ctx.add_context("endpoint", "/open-apis/auth/v3/app_access_token/internal");
                            ctx
                        },
                    })
                },
                Some(ErrorCode::PermissionMissing) => {
                    Err(CoreError::Authentication {
                        message: "应用权限不足，无法获取应用访问令牌".to_string(),
                        code: ErrorCode::PermissionMissing,
                        ctx: {
                            let mut ctx = openlark_core::error::ErrorContext::new();
                            if let Some(ref req_id) = response.raw_response.request_id {
                                ctx.set_request_id(req_id);
                            }
                            ctx.add_context("feishu_code", feishu_code.to_string());
                            ctx.add_context("endpoint", "/open-apis/auth/v3/app_access_token/internal");
                            ctx
                        },
                    })
                },
                Some(code) => {
                    Err(CoreError::Api(openlark_core::error::core_v3::ApiError {
                        status: 200, // HTTP成功但API业务错误
                        endpoint: "/open-apis/auth/v3/app_access_token/internal".into(),
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
                    Err(CoreError::Api(openlark_core::error::core_v3::ApiError {
                        status: 200,
                        endpoint: "/open-apis/auth/v3/app_access_token/internal".into(),
                        message: error_message,
                        source: None,
                        code: ErrorCode::InternalError,
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