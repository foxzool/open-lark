//! 认证模块公共错误处理工具
//!
//! 提供统一的错误处理函数，减少重复代码，提高维护性

use openlark_core::error::{CoreError, ErrorCode, ErrorContext};

/// 创建认证错误，自动添加请求上下文并提供用户友好的错误消息
pub fn create_auth_error(
    feishu_code: i32,
    _error_message: String,
    endpoint: String,
    request_id: Option<String>,
) -> CoreError {
    let mut ctx = ErrorContext::new();
    if let Some(req_id) = request_id {
        ctx.set_request_id(&req_id);
    }
    ctx.add_context("feishu_code", feishu_code.to_string());
    ctx.add_context("endpoint", endpoint.clone());

    match ErrorCode::from_feishu_code(feishu_code) {
        Some(ErrorCode::AppAccessTokenInvalid) => CoreError::Authentication {
            message: "应用访问令牌格式或内容无效，请检查应用凭证".to_string(),
            code: ErrorCode::AppAccessTokenInvalid,
            ctx,
        },
        Some(ErrorCode::PermissionMissing) => CoreError::Authentication {
            message: "应用权限不足，请在开发者后台检查权限配置".to_string(),
            code: ErrorCode::PermissionMissing,
            ctx,
        },
        Some(ErrorCode::AccessTokenExpiredV2) => CoreError::Authentication {
            message: "访问令牌已过期，请重新获取令牌".to_string(),
            code: ErrorCode::AccessTokenExpiredV2,
            ctx,
        },
        Some(ErrorCode::AccessTokenInvalid) => CoreError::Authentication {
            message: "用户访问令牌无效，请重新进行用户授权".to_string(),
            code: ErrorCode::AccessTokenInvalid,
            ctx,
        },
        Some(ErrorCode::TenantAccessTokenInvalid) => CoreError::Authentication {
            message: "租户访问令牌无效，请重新获取企业访问令牌".to_string(),
            code: ErrorCode::TenantAccessTokenInvalid,
            ctx,
        },
        Some(code) => CoreError::Authentication {
            message: get_user_friendly_message(feishu_code, &endpoint),
            code,
            ctx,
        },
        None => {
            // 回退到HTTP状态码或内部业务码
            CoreError::Api(openlark_core::error::ApiError {
                status: feishu_code as u16,
                endpoint: endpoint.clone().into(),
                message: get_user_friendly_message(feishu_code, &endpoint),
                source: None,
                code: ErrorCode::from_http_status(feishu_code as u16),
                ctx,
            })
        }
    }
}

/// 获取用户友好的错误消息
fn get_user_friendly_message(feishu_code: i32, endpoint: &str) -> String {
    match feishu_code {
        // 认证相关错误码
        99991661 => "应用访问令牌格式或内容无效".to_string(),
        99991663 => "租户访问令牌无效".to_string(),
        99991664 => "应用访问令牌无效".to_string(),
        99991670 => "SSO令牌无效".to_string(),
        99991671 => "访问令牌无效".to_string(),
        99991672 => "应用权限不足".to_string(),
        99991677 => "访问令牌已过期".to_string(),
        99992351 => "用户ID格式非法".to_string(),
        99992352 => "OpenID格式非法".to_string(),
        99992353 => "UnionID格式非法".to_string(),

        // 根据端点类型提供默认消息
        _ => {
            if endpoint.contains("app_access_token") {
                "应用访问令牌获取失败，请检查应用配置和凭证".to_string()
            } else if endpoint.contains("tenant_access_token") {
                "租户访问令牌获取失败，请检查企业授权状态".to_string()
            } else if endpoint.contains("user_info") {
                "用户信息获取失败，请检查用户访问令牌有效性".to_string()
            } else if endpoint.contains("oidc") {
                "OIDC认证失败，请检查授权码和重定向配置".to_string()
            } else {
                format!("请求失败 (错误码: {})", feishu_code)
            }
        }
    }
}

/// 创建API错误，用于非认证相关的错误
pub fn create_api_error(
    feishu_code: i32,
    error_message: String,
    endpoint: String,
    request_id: Option<String>,
) -> CoreError {
    let mut ctx = ErrorContext::new();
    if let Some(req_id) = request_id {
        ctx.set_request_id(&req_id);
    }
    ctx.add_context("feishu_code", feishu_code.to_string());

    CoreError::Api(openlark_core::error::ApiError {
        status: feishu_code as u16,
        endpoint: endpoint.into(),
        message: error_message,
        source: None,
        code: ErrorCode::from_feishu_code(feishu_code)
            .unwrap_or_else(|| ErrorCode::from_http_status(feishu_code as u16)),
        ctx,
    })
}

/// 处理API响应，检查成功或返回错误
pub fn handle_api_response<T>(
    response: &openlark_core::api::ApiResponse<T>,
    endpoint: String,
) -> Result<&T, CoreError>
where
    T: serde::de::DeserializeOwned,
{
    if response.raw_response.code == 0 {
        Ok(response.data.as_ref().unwrap())
    } else {
        Err(create_auth_error(
            response.raw_response.code,
            response.raw_response.msg.clone(),
            endpoint,
            response.raw_response.request_id.clone(),
        ))
    }
}

/// 直接处理API响应并返回拥有的数据
pub fn handle_api_response_owned<T>(
    response: openlark_core::api::ApiResponse<T>,
    endpoint: String,
) -> Result<T, CoreError>
where
    T: serde::de::DeserializeOwned,
{
    if response.raw_response.code == 0 {
        Ok(response.data.unwrap())
    } else {
        Err(create_auth_error(
            response.raw_response.code,
            response.raw_response.msg.clone(),
            endpoint,
            response.raw_response.request_id.clone(),
        ))
    }
}
