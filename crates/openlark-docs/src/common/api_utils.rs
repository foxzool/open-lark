/// API通用工具函数
///
/// 提供API实现的通用工具和辅助函数，减少重复代码，提高一致性
use openlark_core::{error, SDKResult};

const ERROR_COMPONENT: &str = "openlark-docs";

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
        error::validation_error("request.params", format!("无法序列化请求参数: {}", source)),
        "serialize_params",
        resource,
        None,
    )
}
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
        missing_response_data_error(context, response.raw_response.request_id.clone())
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
        _ => Err(attach_standard_error_context(
            error::validation_error(field_name, error_message),
            "validate_required_field",
            field_name,
            None,
        )),
    }
}

/// 标准化API端点URL生成辅助宏
///
/// # 使用示例
/// ```rust
/// use openlark_docs::api_url;
///
/// let doc_token = "doc_token";
/// let spreadsheet_token = "spreadsheet_token";
///
/// let doc_url = api_url!("/open-apis/doc/v2/{}", doc_token);
/// let sheet_url = api_url!("/open-apis/sheets/v2/spreadsheets/{}/values", spreadsheet_token);
///
/// assert!(doc_url.contains(doc_token));
/// assert!(sheet_url.contains(spreadsheet_token));
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
    serde_json::to_value(params).map_err(|e| request_serialization_error(context, e))
}

#[cfg(test)]
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
        let err = serialize_params(&FailingParams, "查询记录").unwrap_err();
        assert_eq!(err.context().operation(), Some("serialize_params"));
        assert_eq!(err.context().component(), Some(ERROR_COMPONENT));
        assert_eq!(err.context().get_context("resource"), Some("查询记录"));
    }

    #[test]
    fn test_extract_response_data_adds_request_id_and_resource_context() {
        let response = openlark_core::api::ApiResponse::new(
            None::<serde_json::Value>,
            openlark_core::api::RawResponse {
                request_id: Some("req-docs-123".to_string()),
                ..Default::default()
            },
        );

        let err = extract_response_data(response, "查询记录").unwrap_err();
        assert_eq!(err.context().operation(), Some("extract_response_data"));
        assert_eq!(err.context().component(), Some(ERROR_COMPONENT));
        assert_eq!(err.context().get_context("resource"), Some("查询记录"));
        assert_eq!(err.context().request_id(), Some("req-docs-123"));
    }

    #[test]
    fn test_missing_response_data_error_reuses_standard_shape() {
        let err = missing_response_data_error("获取多维表格", Some("req-docs-456".to_string()));
        assert_eq!(err.context().operation(), Some("extract_response_data"));
        assert_eq!(err.context().component(), Some(ERROR_COMPONENT));
        assert_eq!(err.context().get_context("resource"), Some("获取多维表格"));
        assert_eq!(err.context().request_id(), Some("req-docs-456"));
    }

    #[test]
    fn test_validate_required_field_adds_standard_context() {
        let err =
            validate_required_field("sheet_id", None::<String>, "sheet_id 不能为空").unwrap_err();
        assert_eq!(err.context().operation(), Some("validate_required_field"));
        assert_eq!(err.context().component(), Some(ERROR_COMPONENT));
        assert_eq!(err.context().get_context("resource"), Some("sheet_id"));
    }
}
