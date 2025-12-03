//! 认证模块公共错误处理工具
//!
//! 提供统一的错误处理函数，减少重复代码，提高维护性

use openlark_core::{
    error::{CoreError, ErrorCode, ErrorContext},
};

/// 创建认证错误，自动添加请求上下文
pub fn create_auth_error(
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
    ctx.add_context("endpoint", endpoint.clone());

    match ErrorCode::from_feishu_code(feishu_code) {
        Some(code) => CoreError::Authentication {
            message: error_message,
            code,
            ctx,
        },
        None => {
            // 回退到HTTP状态码或内部业务码
            CoreError::Api(openlark_core::error::ApiError {
                status: feishu_code as u16,
                endpoint: endpoint.into(),
                message: error_message,
                source: None,
                code: ErrorCode::from_http_status(feishu_code as u16),
                ctx,
            })
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