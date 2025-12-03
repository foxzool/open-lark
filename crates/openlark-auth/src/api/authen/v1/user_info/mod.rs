//! 用户信息API实现
//!
//! 本模块提供用户信息获取的API实现，根据meta.resource=user_info组织

use openlark_core::{
    config::Config,
    api::ApiRequest,
    prelude::Transport,
    error::{SDKResult, CoreError, ErrorCode},
};
use crate::models::authen::{UserInfoRequest, UserInfoResponse};

// 类型别名
pub type AuthenResult<T> = SDKResult<T>;

/// 用户信息获取构建器
#[derive(Debug)]
pub struct UserInfoBuilder {
    config: Config,
    request: UserInfoRequest,
}

impl UserInfoBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            request: UserInfoRequest {
                user_access_token: String::new(),
                user_id_type: None,
            },
        }
    }

    /// 设置用户访问令牌
    pub fn user_access_token(mut self, user_access_token: impl Into<String>) -> Self {
        self.request.user_access_token = user_access_token.into();
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    /// 发送请求获取用户信息
    pub async fn send(self) -> AuthenResult<UserInfoResponse> {
        // 构建API请求
        let url = format!("{}/open-apis/authen/v1/user_info", self.config.base_url);

        let request: ApiRequest<UserInfoResponse> = ApiRequest::get(&url)
            .header("Authorization", format!("Bearer {}", self.request.user_access_token));

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
                        message: "用户访问令牌无效".to_string(),
                        code: ErrorCode::AccessTokenInvalid,
                        ctx: {
                            let mut ctx = openlark_core::error::ErrorContext::new();
                            if let Some(ref req_id) = response.raw_response.request_id {
                                ctx.set_request_id(req_id);
                            }
                            ctx.add_context("feishu_code", feishu_code.to_string());
                            ctx.add_context("endpoint", "/open-apis/authen/v1/user_info");
                            ctx
                        },
                    })
                },
                Some(ErrorCode::AccessTokenExpiredV2) => {
                    Err(CoreError::Authentication {
                        message: "用户访问令牌已过期".to_string(),
                        code: ErrorCode::AccessTokenExpiredV2,
                        ctx: {
                            let mut ctx = openlark_core::error::ErrorContext::new();
                            if let Some(ref req_id) = response.raw_response.request_id {
                                ctx.set_request_id(req_id);
                            }
                            ctx.add_context("feishu_code", feishu_code.to_string());
                            ctx.add_context("endpoint", "/open-apis/authen/v1/user_info");
                            ctx
                        },
                    })
                },
                Some(ErrorCode::UserSessionInvalid) => {
                    Err(CoreError::Authentication {
                        message: "用户会话失效".to_string(),
                        code: ErrorCode::UserSessionInvalid,
                        ctx: {
                            let mut ctx = openlark_core::error::ErrorContext::new();
                            if let Some(ref req_id) = response.raw_response.request_id {
                                ctx.set_request_id(req_id);
                            }
                            ctx.add_context("feishu_code", feishu_code.to_string());
                            ctx.add_context("endpoint", "/open-apis/authen/v1/user_info");
                            ctx
                        },
                    })
                },
                Some(code) => {
                    Err(CoreError::Api(openlark_core::error::ApiError {
                        status: feishu_code as u16,
                        endpoint: "/open-apis/authen/v1/user_info".into(),
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
                        endpoint: "/open-apis/authen/v1/user_info".into(),
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

/// 用户信息API服务
#[derive(Debug)]
pub struct UserInfoService {
    config: Config,
}

impl UserInfoService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取用户信息
    pub fn get(&self) -> UserInfoBuilder {
        UserInfoBuilder::new(self.config.clone())
    }
}