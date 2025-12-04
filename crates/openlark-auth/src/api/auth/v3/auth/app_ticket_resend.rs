//! 重新获取 app_ticket API
//!
//! 对应CSV记录: 6995779366223757316
//! 飞书每隔 1 小时会给应用推送一次最新的 app_ticket，应用也可以主动调用此接口，
//! 触发飞书进行及时的重新推送。（该接口并不能直接获取app_ticket，而是触发事件推送）

use openlark_core::{
    config::Config,
    api::{ApiRequest, RequestData},
    error::{SDKResult, CoreError, ErrorCode, network_error},
};
use crate::models::auth::*;

/// 应用票据重发构建器
#[derive(Debug)]
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

    /// 发送请求触发 app_ticket 重新推送
    pub async fn send(self) -> SDKResult<AppTicketResponse> {
        let url = format!("{}/open-apis/auth/v3/app_ticket/resend", self.config.base_url);

        let mut request = ApiRequest::<AppTicketResponse>::post(&url);
        request.headers.insert("Content-Type".to_string(), "application/json".to_string());

        let json_data = serde_json::to_value(&self.request)
            .map_err(|e| network_error(format!("请求数据序列化失败: {}", e)))?;
        request.body = Some(RequestData::Json(json_data));

        let response = openlark_core::http::Transport::request(request, &self.config, None)
            .await
            .map_err(|e| network_error(format!("应用票据重发API请求失败: {}", e)))?;

        if response.raw_response.code == 0 {
            Ok(response.data.unwrap())
        } else {
            // 智能映射飞书错误码（优先级：飞书通用码 > HTTP状态 > 内部码）
            let feishu_code = response.raw_response.code;
            let error_message = response.raw_response.msg.clone();

            match ErrorCode::from_feishu_code(feishu_code) {
                Some(ErrorCode::PermissionMissing) => {
                    Err(CoreError::Authentication {
                        message: "应用权限不足，无法重新发送app_ticket".to_string(),
                        code: ErrorCode::PermissionMissing,
                        ctx: {
                            let mut ctx = openlark_core::error::ErrorContext::new();
                            if let Some(ref req_id) = response.raw_response.request_id {
                                ctx.set_request_id(req_id);
                            }
                            ctx.add_context("feishu_code", feishu_code.to_string());
                            ctx.add_context("endpoint", "/open-apis/auth/v3/app_ticket/resend");
                            ctx
                        },
                    })
                },
                Some(code) => {
                    Err(CoreError::Api(openlark_core::error::core_v3::ApiError {
                        status: 200, // HTTP成功但API业务错误
                        endpoint: "/open-apis/auth/v3/app_ticket/resend".into(),
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
                        endpoint: "/open-apis/auth/v3/app_ticket/resend".into(),
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