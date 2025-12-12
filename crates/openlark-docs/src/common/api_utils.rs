//! API通用工具函数
//!
//! 提供API实现的通用工具和辅助函数，减少重复代码，提高一致性

use openlark_core::{SDKResult, error};

/// 标准化API响应数据提取
///
/// # 参数
/// - `response`: API响应对象
/// - `context`: 错误上下文描述
///
/// # 返回
/// - `Ok(T)`: 成功提取的数据
/// - `Err(SDKError)`: 包含上下文信息的错误
pub fn extract_response_data<T>(
    response: openlark_core::api::ApiResponse<T>,
    context: &str,
) -> SDKResult<T> {
    response.data.ok_or_else(|| {
        error::validation_error(
            &format!("{}响应数据为空", context),
            "服务器没有返回有效的数据"
        )
    })
}

/// 标准化参数验证错误处理
///
/// # 参数
/// - `field_name`: 字段名称
/// - `field_value`: 字段值
/// - `error_message`: 错误消息
///
/// # 返回
/// - `Ok(())`: 验证通过
/// - `Err(SDKError)`: 验证失败错误
pub fn validate_required_field<T: AsRef<str>>(
    field_name: &str,
    field_value: Option<T>,
    error_message: &str,
) -> SDKResult<()> {
    match field_value {
        Some(value) if !value.as_ref().trim().is_empty() => Ok(()),
        _ => Err(error::validation_error(field_name, error_message)),
    }
}

/// 标准化API端点URL生成辅助宏
///
/// # 使用示例
/// ```rust
/// api_url!("/open-apis/doc/v2/{}", doc_token)
/// api_url!("/open-apis/sheets/v2/spreadsheets/{}/values", spreadsheet_token)
/// ```
#[macro_export]
macro_rules! api_url {
    ($base_url:expr) => {
        $base_url.to_string()
    };
    ($base_url:expr, $($arg:expr),+) => {
        format!($base_url, $($arg),+)
    };
}

/// 标准化参数序列化错误处理
///
/// # 参数
/// - `params`: 要序列化的参数
/// - `context`: 序列化上下文
///
/// # 返回
/// - `Ok(serde_json::Value)`: 序列化成功
/// - `Err(SDKError)`: 包含详细信息的序列化错误
pub fn serialize_params<T: serde::Serialize>(
    params: &T,
    context: &str,
) -> SDKResult<serde_json::Value> {
    serde_json::to_value(params).map_err(|e| {
        error::validation_error(
            &format!("{}参数序列化失败", context),
            &format!("无法序列化请求参数: {}", e)
        )
    })
}