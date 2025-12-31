//! API 通用工具函数
//!
//! 提供 API 实现的通用工具和辅助函数，减少重复代码，提高一致性。

use openlark_core::{error, SDKResult};

/// 标准化 API 响应数据提取
pub fn extract_response_data<T>(
    response: openlark_core::api::ApiResponse<T>,
    context: &str,
) -> SDKResult<T> {
    response.data.ok_or_else(|| {
        error::validation_error(
            format!("{}响应数据为空", context),
            "服务器没有返回有效的数据",
        )
    })
}

/// 标准化参数序列化错误处理
pub fn serialize_params<T: serde::Serialize>(params: &T, context: &str) -> SDKResult<serde_json::Value> {
    serde_json::to_value(params).map_err(|e| {
        error::validation_error(
            format!("{}参数序列化失败", context),
            format!("无法序列化请求参数: {}", e),
        )
    })
}

