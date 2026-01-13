//! API 通用工具函数

use openlark_core::{error, SDKResult};

/// 标准化 API 响应数据提取
pub fn extract_response_data<T>(
    response: openlark_core::api::ApiResponse<T>,
    context: &str,
) -> SDKResult<T> {
    response.data.ok_or_else(|| {
        error::validation_error(format!("{context}响应数据为空"), "服务器没有返回有效的数据")
    })
}

/// 标准化参数序列化错误处理
pub fn serialize_params<T: serde::Serialize>(
    params: &T,
    context: &str,
) -> SDKResult<serde_json::Value> {
    serde_json::to_value(params).map_err(|e| {
        error::validation_error(
            format!("{context}参数序列化失败"),
            format!("无法序列化请求参数: {e}"),
        )
    })
}

/// 无 data 的接口：仅用 code==0 判断成功
pub fn ensure_success(
    response: openlark_core::api::ApiResponse<serde_json::Value>,
    context: &str,
) -> SDKResult<()> {
    if response.is_success() {
        Ok(())
    } else {
        Err(error::validation_error(
            format!("{context}失败: {}", response.code()),
            response.msg().to_string(),
        ))
    }
}

/// 标准化 API 端点 URL 生成辅助宏
#[macro_export]
macro_rules! api_url {
    ($base_url:expr) => {
        $base_url.to_string()
    };
    ($base_url:expr, $($arg:expr),+) => {
        format!($base_url, $($arg),+)
    };
}
