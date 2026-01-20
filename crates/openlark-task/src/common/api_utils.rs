/// API通用工具函数
///
/// 提供API实现的通用工具和辅助函数，减少重复代码，提高一致性
use openlark_core::{error, SDKResult};

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
    response: openlark_core::api::Response<T>,
    context: &str,
) -> SDKResult<T> {
    response.data.ok_or_else(|| {
        error::validation_error(
            &format!("{}响应数据为空", context),
            "服务器没有返回有效的数据",
        )
    })
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
            &format!("无法序列化请求参数: {}", e),
        )
    })
}

/// 标准化API端点URL生成辅助宏
///
/// # 使用示例
/// ```rust
/// # fn main() {
/// use openlark_task::api_url;
///
/// let task_guid = "task_guid";
/// let url = api_url!("/open-apis/task/v2/tasks/{}", task_guid);
/// assert!(url.contains(task_guid));
/// # }
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
