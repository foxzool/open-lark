//! API 通用工具函数
//!
//! 提供 API 实现的通用工具和辅助函数，减少重复代码，提高一致性。

use openlark_core::{SDKResult, error};

const ERROR_COMPONENT: &str = "openlark-communication";

fn attach_standard_error_context(
    err: openlark_core::error::CoreError,
    operation: &str,
    resource: &str,
    request_id: Option<String>,
) -> openlark_core::error::CoreError {
    err.with_standard_context(operation, ERROR_COMPONENT, resource, request_id)
}

/// 创建“响应 data 为空”的标准错误。
pub fn missing_response_data_error(
    resource: &str,
    request_id: Option<String>,
) -> openlark_core::error::CoreError {
    attach_standard_error_context(
        error::validation_error("response.data", "服务器没有返回有效的数据"),
        "extract_response_data",
        resource,
        request_id,
    )
}

/// 创建“请求参数序列化失败”的标准错误。
pub fn request_serialization_error(
    resource: &str,
    source: impl std::fmt::Display,
) -> openlark_core::error::CoreError {
    attach_standard_error_context(
        error::validation_error("request.params", format!("无法序列化请求参数: {source}")),
        "serialize_params",
        resource,
        None,
    )
}

/// 标准化 API 响应数据提取
pub fn extract_response_data<T>(
    response: openlark_core::api::ApiResponse<T>,
    context: &str,
) -> SDKResult<T> {
    response.data.ok_or_else(|| {
        missing_response_data_error(context, response.raw_response.request_id.clone())
    })
}

/// 标准化参数序列化错误处理
pub fn serialize_params<T: serde::Serialize>(
    params: &T,
    context: &str,
) -> SDKResult<serde_json::Value> {
    serde_json::to_value(params).map_err(|e| request_serialization_error(context, e))
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
#[allow(unused_imports)]
mod tests {
    use super::*;
    use openlark_core::error::ErrorTrait;
    use serde::Serialize;

    #[derive(Serialize)]
    struct TestParams {
        name: String,
        value: i32,
    }

    struct FailingParams;

    impl Serialize for FailingParams {
        fn serialize<S>(&self, _serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            Err(serde::ser::Error::custom("boom"))
        }
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
    fn test_serialize_params_adds_standard_error_context() {
        let err = serialize_params(&FailingParams, "搜索用户").unwrap_err();
        assert_eq!(err.context().operation(), Some("serialize_params"));
        assert_eq!(err.context().component(), Some(ERROR_COMPONENT));
        assert_eq!(err.context().get_context("resource"), Some("搜索用户"));
    }

    #[test]
    fn test_extract_response_data_adds_request_id_and_resource_context() {
        let response = openlark_core::api::ApiResponse::new(
            None::<serde_json::Value>,
            openlark_core::api::RawResponse {
                request_id: Some("req-comm-123".to_string()),
                ..Default::default()
            },
        );

        let err = extract_response_data(response, "发送消息").unwrap_err();
        assert_eq!(err.context().operation(), Some("extract_response_data"));
        assert_eq!(err.context().component(), Some(ERROR_COMPONENT));
        assert_eq!(err.context().get_context("resource"), Some("发送消息"));
        assert_eq!(err.context().request_id(), Some("req-comm-123"));
    }

    #[test]
    fn test_missing_response_data_error_reuses_standard_shape() {
        let err = missing_response_data_error("发送消息", Some("req-comm-456".to_string()));
        assert_eq!(err.context().operation(), Some("extract_response_data"));
        assert_eq!(err.context().component(), Some(ERROR_COMPONENT));
        assert_eq!(err.context().get_context("resource"), Some("发送消息"));
        assert_eq!(err.context().request_id(), Some("req-comm-456"));
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
