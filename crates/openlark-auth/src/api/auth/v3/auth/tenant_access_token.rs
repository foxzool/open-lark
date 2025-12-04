//! 商店应用获取 tenant_access_token API
//!
//! 对应CSV记录: 6995779366223740932
//! 应用商店应用通过此接口获取 tenant_access_token，调用接口获取企业资源时，
//! 需要使用 tenant_access_token 作为授权凭证。

use openlark_core::{
    config::Config,
    api::{ApiRequest, RequestData},
    error::{SDKResult, CoreError, ErrorCode, network_error},
};
use crate::models::auth::{TenantAccessTokenRequest, TenantAccessTokenResponse};

/// 租户访问令牌构建器（商店应用）
#[derive(Debug)]
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
                app_ticket: String::new(),
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

    /// 设置应用票据（商店应用必需）
    pub fn app_ticket(mut self, app_ticket: impl Into<String>) -> Self {
        self.request.app_ticket = app_ticket.into();
        self
    }

    /// 发送请求获取 tenant_access_token
    pub async fn send(self) -> SDKResult<TenantAccessTokenResponse> {
        let url = format!("{}/open-apis/auth/v3/tenant_access_token", self.config.base_url);

        let mut request = ApiRequest::<TenantAccessTokenResponse>::post(&url);
        request.headers.insert("Content-Type".to_string(), "application/json".to_string());

        let json_data = serde_json::to_value(&self.request)
            .map_err(|e| network_error(format!("请求数据序列化失败: {}", e)))?;
        request.body = Some(RequestData::Json(json_data));

        let response = openlark_core::http::Transport::request(request, &self.config, None)
            .await
            .map_err(|e| network_error(format!("租户访问令牌API请求失败: {}", e)))?;

        // 处理响应
        if response.raw_response.code == 0 {
            Ok(response.data.unwrap_or_else(|| {
                // 如果没有data字段，尝试直接解析响应
                serde_json::from_value::<TenantAccessTokenResponse>(serde_json::json!({
                    "tenant_access_token": response.raw_response.data.unwrap_or_default(),
                    "expires_in": 7200, // 默认2小时
                    "token_type": Some("Bearer".to_string())
                })).unwrap_or_else(|_| TenantAccessTokenResponse {
                    tenant_access_token: String::new(),
                    expires_in: 7200,
                    token_type: Some("Bearer".to_string()),
                })
            }))
        } else {
            // API返回错误，智能映射飞书错误码
            let feishu_code = response.raw_response.code;
            let error_message = response.raw_response.msg;

            match ErrorCode::from_feishu_code(feishu_code) {
                Some(ErrorCode::TenantAccessTokenInvalid) => {
                    Err(CoreError::Authentication {
                        message: "商店应用租户访问令牌格式或内容无效".to_string(),
                        code: ErrorCode::TenantAccessTokenInvalid,
                        ctx: {
                            let mut ctx = openlark_core::error::ErrorContext::new();
                            if let Some(ref req_id) = response.raw_response.request_id {
                                ctx.set_request_id(req_id);
                            }
                            ctx.add_context("feishu_code", feishu_code.to_string());
                            ctx.add_context("endpoint", "/open-apis/auth/v3/tenant_access_token");
                            ctx
                        },
                    })
                },
                Some(ErrorCode::PermissionMissing) => {
                    Err(CoreError::Authentication {
                        message: "应用权限不足，无法获取租户访问令牌".to_string(),
                        code: ErrorCode::PermissionMissing,
                        ctx: {
                            let mut ctx = openlark_core::error::ErrorContext::new();
                            if let Some(ref req_id) = response.raw_response.request_id {
                                ctx.set_request_id(req_id);
                            }
                            ctx.add_context("feishu_code", feishu_code.to_string());
                            ctx.add_context("endpoint", "/open-apis/auth/v3/tenant_access_token");
                            ctx
                        },
                    })
                },
                Some(code) => {
                    Err(CoreError::Api(openlark_core::error::core_v3::ApiError {
                        status: 200, // HTTP成功但API业务错误
                        endpoint: "/open-apis/auth/v3/tenant_access_token".into(),
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
                        endpoint: "/open-apis/auth/v3/tenant_access_token".into(),
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