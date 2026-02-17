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
pub fn serialize_params<T: serde::Serialize>(
    params: &T,
    context: &str,
) -> SDKResult<serde_json::Value> {
    serde_json::to_value(params).map_err(|e| {
        error::validation_error(
            format!("{}参数序列化失败", context),
            format!("无法序列化请求参数: {}", e),
        )
    })
}

/// 标准化 API 端点 URL 生成辅助宏
///
/// # 使用示例
/// ```rust
/// use openlark_communication::api_url;
///
/// let post_id = "post_id";
/// let url = api_url!("/open-apis/moments/v1/posts/{}", post_id);
/// assert!(url.contains(post_id));
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[derive(Serialize)]
    struct TestParams {
        name: String,
        value: i32,
    }

    #[test]
    fn test_serialize_params_success() {
        let params = TestParams {
            name: "test".to_string(),
            value: 42,
        };
        let result = serialize_params(&params, "test");
        assert!(result.is_ok());
        let json = result.unwrap();
        assert_eq!(json["name"], "test");
        assert_eq!(json["value"], 42);
    }

    #[test]
    fn test_api_url_macro_simple() {
        let url = api_url!("/open-apis/test/v1/resource");
        assert_eq!(url, "/open-apis/test/v1/resource");
    }

    #[test]
    fn test_api_url_macro_with_args() {
        let resource_id = "12345";
        let url = api_url!("/open-apis/test/v1/resources/{}", resource_id);
        assert!(url.contains("12345"));
    }

    #[test]
    fn test_api_url_macro_with_multiple_args() {
        let resource_id = "123";
        let action = "approve";
        let url = api_url!("/open-apis/test/v1/resources/{}/{}", resource_id, action);
        assert!(url.contains("123"));
        assert!(url.contains("approve"));
    }
}
