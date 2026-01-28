//! API 工具函数
//!
//! 提供 API 相关的通用工具函数，包括响应数据提取、参数序列化等。

use openlark_core::{api::Response, error, SDKResult};
use serde::Serialize;

/// 标准化 API 响应数据提取
///
/// 从 API 响应中提取数据，处理空响应的情况。
///
/// # Arguments
///
/// * `response` - API 响应对象
/// * `context` - 上下文信息，用于错误消息
///
/// # Returns
///
/// 提取的数据，如果响应为空则返回错误
pub fn extract_response_data<T>(response: Response<T>, context: &str) -> SDKResult<T> {
    response.data.ok_or_else(|| {
        error::validation_error(format!("{context}响应数据为空"), "服务器没有返回有效的数据")
    })
}

/// 标准化参数序列化错误处理
///
/// 将参数序列化为 JSON，处理序列化失败的情况。
///
/// # Arguments
///
/// * `params` - 要序列化的参数
/// * `context` - 上下文信息，用于错误消息
///
/// # Returns
///
/// 序列化的 JSON 值，如果序列化失败则返回错误
pub fn serialize_params<T: Serialize>(params: &T, context: &str) -> SDKResult<serde_json::Value> {
    serde_json::to_value(params).map_err(|e| {
        error::validation_error(
            format!("{context}参数序列化失败"),
            format!("无法序列化请求参数: {e}"),
        )
    })
}

/// 构建 API 端点 URL
///
/// 将基础端点和路径参数组合成完整的 API URL。
///
/// # Arguments
///
/// * `base_endpoint` - 基础端点常量
/// * `path_params` - 路径参数键值对
///
/// # Returns
///
/// 完整的 API URL
#[macro_export]
macro_rules! api_url {
    ($base_endpoint:expr) => {{
        $base_endpoint.to_string()
    }};
    ($base_endpoint:expr, $($key:expr => $value:expr),+) => {{
        let mut url = $base_endpoint.to_string();
        $(
            url = url.replace(&format!("{{{}}}", $key), &$value);
        )+
        url
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::api::RawResponse;

    #[test]
    fn test_extract_response_data_success() {
        let response: Response<String> = Response {
            data: Some("test_data".to_string()),
            raw_response: RawResponse::success(),
        };
        let result = extract_response_data(response, "测试");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test_data");
    }

    #[test]
    fn test_extract_response_data_empty() {
        let response: Response<String> = Response {
            data: None,
            raw_response: RawResponse::success(),
        };
        let result = extract_response_data(response, "测试");
        assert!(result.is_err());
    }

    #[test]
    fn test_serialize_params_success() {
        let params = vec!["key1", "key2"];
        let result = serialize_params(&params, "测试");
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), serde_json::json!(["key1", "key2"]));
    }

    #[test]
    fn test_api_url_macro() {
        let base = "/open-apis/test/{id}/items/{item_id}";
        let url = api_url!(base, "id" => "123", "item_id" => "456");
        assert_eq!(url, "/open-apis/test/123/items/456");
    }

    #[test]
    fn test_api_url_macro_no_params() {
        let base = "/open-apis/test";
        let url = api_url!(base);
        assert_eq!(url, "/open-apis/test");
    }
}
