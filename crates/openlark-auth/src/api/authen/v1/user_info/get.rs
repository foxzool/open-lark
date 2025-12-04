//! 获取用户信息 API
//!
//! 对应CSV记录: 7180265937329537028
//! 通过 `user_access_token` 获取登录用户的信息。

use openlark_core::{
    config::Config,
    error::{SDKResult, CoreError, ErrorCode, network_error},
};
use crate::models::authen::{UserInfoRequest, UserInfoResponse};

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
    pub async fn send(self) -> SDKResult<UserInfoResponse> {
        // 构建API请求
        let mut request = openlark_core::api::ApiRequest::<UserInfoResponse>::get(
            format!("{}/open-apis/authen/v1/user_info", self.config.base_url)
        );

        // 添加Authorization头
        request.headers.insert("Authorization".to_string(),
            format!("Bearer {}", self.request.user_access_token));

        // 添加查询参数
        if let Some(ref user_id_type) = self.request.user_id_type {
            request.query.insert("user_id_type".to_string(), user_id_type.clone());
        }

        // 使用Transport发送请求
        let response = openlark_core::http::Transport::request(request, &self.config, None)
            .await
            .map_err(|e| network_error(format!("用户信息API请求失败: {}", e)))?;

        // 处理响应
        if response.raw_response.code == 0 {
            Ok(response.data.unwrap_or_else(|| {
                // 如果没有data字段，尝试直接解析响应
                serde_json::from_value::<UserInfoResponse>(serde_json::json!({
                    "data": response.raw_response.data.unwrap_or_default()
                })).unwrap_or_else(|_| UserInfoResponse {
                    data: crate::models::authen::UserInfo {
                        open_id: String::new(),
                        union_id: None,
                        user_id: None,
                        name: None,
                        en_name: None,
                        email: None,
                        mobile: None,
                        avatar_url: None,
                        avatar: None,
                        status: None,
                        department_ids: None,
                        group_ids: None,
                        positions: None,
                        employee_no: None,
                        dingtalk_user_id: None,
                        enterprise_extension: None,
                        custom_attrs: None,
                        tenant_key: None,
                    }
                })
            }))
        } else {
            // API返回错误，智能映射飞书错误码
            let feishu_code = response.raw_response.code;
            let error_message = response.raw_response.msg;

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
                Some(ErrorCode::UserIdInvalid) => {
                    Err(CoreError::Validation {
                        field: "user_id_type".into(),
                        message: "用户ID类型参数不正确".to_string(),
                        code: ErrorCode::UserIdInvalid,
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
                    Err(CoreError::Api(openlark_core::error::core_v3::ApiError {
                        status: 200, // HTTP成功但API业务错误
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
                    Err(CoreError::Api(openlark_core::error::core_v3::ApiError {
                        status: 200,
                        endpoint: "/open-apis/authen/v1/user_info".into(),
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