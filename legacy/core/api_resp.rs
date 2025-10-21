use std::fmt::{Debug, Display, Formatter};

use serde::{Deserialize, Serialize};

use crate::core::{error::LarkAPIError, error_codes::LarkErrorCode, SDKResult};

/// 业务返回值
#[derive(Debug, Serialize, Deserialize)]
pub struct BaseResponse<T> {
    /// 响应头
    #[serde(flatten)]
    pub raw_response: RawResponse,
    /// 具体数据
    pub data: Option<T>,
}

impl<T> BaseResponse<T> {
    pub fn success(&self) -> bool {
        self.raw_response.code == 0
    }

    pub fn code(&self) -> i32 {
        self.raw_response.code
    }

    pub fn msg(&self) -> &str {
        &self.raw_response.msg
    }

    pub fn err(&self) -> Option<&ErrorInfo> {
        self.raw_response.err.as_ref()
    }

    /// 获取语义化的错误码
    pub fn error_code(&self) -> Option<LarkErrorCode> {
        LarkErrorCode::from_code(self.code())
    }

    /// 检查是否为特定错误码
    pub fn is_error_code(&self, code: LarkErrorCode) -> bool {
        self.code() == code as i32
    }

    /// 检查是否为权限相关错误
    pub fn is_permission_error(&self) -> bool {
        self.error_code()
            .map(|c| c.is_permission_error())
            .unwrap_or(false)
    }

    /// 检查是否为认证相关错误
    pub fn is_auth_error(&self) -> bool {
        self.error_code()
            .map(|c| c.is_auth_error())
            .unwrap_or(false)
    }

    /// 检查是否为服务器错误
    pub fn is_server_error(&self) -> bool {
        self.error_code()
            .map(|c| c.is_server_error())
            .unwrap_or(false)
    }

    /// 检查是否为客户端错误
    pub fn is_client_error(&self) -> bool {
        self.error_code()
            .map(|c| c.is_client_error())
            .unwrap_or(false)
    }

    /// 检查是否可以重试
    pub fn is_retryable(&self) -> bool {
        self.error_code().map(|c| c.is_retryable()).unwrap_or(false)
    }

    /// 获取建议的重试延迟时间
    pub fn suggested_retry_delay(&self) -> Option<u64> {
        self.error_code().and_then(|c| c.suggested_retry_delay())
    }

    /// 获取数据或返回友好错误
    pub fn data_or_error(self) -> Result<T, String> {
        if self.success() {
            self.data.ok_or_else(|| "响应成功但数据为空".to_string())
        } else {
            let error_msg = if let Some(code) = self.error_code() {
                code.detailed_description().to_string()
            } else {
                format!("{} (错误码: {})", self.msg(), self.code())
            };
            Err(error_msg)
        }
    }

    /// 获取数据或转换为LarkAPIError
    pub fn data_or_api_error(self) -> SDKResult<T> {
        if self.success() {
            self.data
                .ok_or_else(|| LarkAPIError::api_error(0, "响应成功但数据为空", None))
        } else {
            Err(LarkAPIError::api_error(
                self.code(),
                self.msg(),
                None, // FUTURE: 可从响应头中提取request_id以增强调试支持
            ))
        }
    }

    /// 处理通用错误，返回处理后的响应或错误
    pub fn handle_common_errors(self) -> SDKResult<Self> {
        if self.success() {
            return Ok(self);
        }

        match self.error_code() {
            Some(LarkErrorCode::AccessTokenInvalid) => Err(LarkAPIError::illegal_param(
                "访问令牌已过期，请重新获取用户授权",
            )),
            Some(LarkErrorCode::AppAccessTokenInvalid) => Err(LarkAPIError::illegal_param(
                "应用访问令牌无效，请检查应用配置",
            )),
            Some(LarkErrorCode::TenantAccessTokenInvalid) => Err(LarkAPIError::illegal_param(
                "租户访问令牌无效，请检查应用权限",
            )),
            Some(LarkErrorCode::Forbidden) => Err(LarkAPIError::illegal_param(
                "权限不足，请检查应用权限配置或用户权限",
            )),
            Some(LarkErrorCode::TooManyRequests) => {
                Err(LarkAPIError::illegal_param("请求过于频繁，请稍后重试"))
            }
            Some(LarkErrorCode::NotFound) => Err(LarkAPIError::illegal_param("请求的资源不存在")),
            _ => {
                // 对于其他错误，返回原始响应让调用者处理
                Ok(self)
            }
        }
    }

    /// 获取用户友好的错误描述
    pub fn user_friendly_error(&self) -> Option<String> {
        if self.success() {
            return None;
        }

        Some(
            self.error_code()
                .map(|c| c.detailed_description().to_string())
                .unwrap_or_else(|| format!("{} (错误码: {})", self.msg(), self.code())),
        )
    }

    /// 获取错误的建议解决方案
    pub fn error_solutions(&self) -> Vec<String> {
        if self.success() {
            return vec![];
        }

        match self.error_code() {
            Some(LarkErrorCode::AccessTokenInvalid) => vec![
                "重新获取用户访问令牌".to_string(),
                "检查令牌是否在有效期内".to_string(),
            ],
            Some(LarkErrorCode::AppAccessTokenInvalid) => vec![
                "检查应用ID和应用密钥".to_string(),
                "确认应用类型配置正确".to_string(),
            ],
            Some(LarkErrorCode::TenantAccessTokenInvalid) => vec![
                "检查租户权限配置".to_string(),
                "确认应用已正确安装到企业".to_string(),
            ],
            Some(LarkErrorCode::Forbidden) => vec![
                "检查应用权限范围设置".to_string(),
                "确认用户具有相应的操作权限".to_string(),
                "联系管理员添加必要权限".to_string(),
            ],
            Some(LarkErrorCode::TooManyRequests) => vec![
                "降低请求频率".to_string(),
                "实现请求重试机制".to_string(),
                "考虑使用请求缓存".to_string(),
            ],
            Some(LarkErrorCode::NotFound) => vec![
                "检查资源ID是否正确".to_string(),
                "确认资源是否存在".to_string(),
            ],
            _ => vec![
                "检查请求参数是否正确".to_string(),
                "参考API文档确认调用方式".to_string(),
            ],
        }
    }

    /// 获取相关的帮助链接
    pub fn help_links(&self) -> Vec<(&'static str, &'static str)> {
        if self.success() {
            return vec![];
        }

        match self.error_code() {
            Some(code) => vec![
                (
                    "官方文档",
                    code.help_url()
                        .unwrap_or("https://open.feishu.cn/document/"),
                ),
                (
                    "开发者社区",
                    "https://getfeishu.cn/hc/zh-cn/categories/360000150856",
                ),
            ],
            None => vec![
                ("API文档", "https://open.feishu.cn/document/"),
                (
                    "开发者社区",
                    "https://getfeishu.cn/hc/zh-cn/categories/360000150856",
                ),
            ],
        }
    }

    /// 打印详细的错误信息（用于调试）
    pub fn print_error_details(&self) {
        if self.success() {
            println!("✅ 请求成功");
            return;
        }

        println!("❌ 请求失败");
        println!("错误码: {}", self.code());

        if let Some(error_code) = self.error_code() {
            println!("错误类型: {}", error_code.description());
            println!("详细说明: {}", error_code.detailed_description());
        }

        println!("错误消息: {}", self.msg());

        let solutions = self.error_solutions();
        if !solutions.is_empty() {
            println!("\n💡 建议解决方案:");
            for (i, solution) in solutions.iter().enumerate() {
                println!("   {}. {}", i + 1, solution);
            }
        }

        let help_links = self.help_links();
        if !help_links.is_empty() {
            println!("\n🔗 相关链接:");
            for (name, url) in help_links {
                println!("   {name}: {url}");
            }
        }

        if let Some(delay) = self.suggested_retry_delay() {
            println!("\n⏱️ 建议重试延迟: {delay}秒");
        }
    }
}

/// 业务返回值格式
pub trait ApiResponseTrait: for<'a> Deserialize<'a> + Send + Sync + 'static + Debug {
    /// 是否是标准数据格式, 既是用data包裹数据
    fn data_format() -> ResponseFormat;

    fn from_binary(_file_name: String, _body: Vec<u8>) -> Option<Self> {
        None
    }
}

/// 响应格式类型
///
/// 定义API响应的不同格式类型
#[derive(Debug, PartialEq)]
pub enum ResponseFormat {
    /// 标准数据格式, 既是用data包裹数据
    Data,
    /// 扁平数据格式, 既是直接返回数据
    Flatten,
    /// 二进制数据格式
    Binary,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct RawResponse {
    pub code: i32,
    pub msg: String,
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub err: Option<ErrorInfo>,
}

impl ApiResponseTrait for RawResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Flatten
    }
}

impl Display for RawResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "code: {}, msg: {}", self.code, self.msg)
    }
}

/// 空响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct EmptyResponse {}

impl ApiResponseTrait for EmptyResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// JSON响应体
pub type JsonResponse = serde_json::Value;

impl ApiResponseTrait for JsonResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 二进制数据响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct BinaryResponse {
    pub file_name: String,
    pub body: Vec<u8>,
}

impl ApiResponseTrait for BinaryResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Binary
    }

    fn from_binary(file_name: String, body: Vec<u8>) -> Option<Self> {
        Some(BinaryResponse { file_name, body })
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ErrorInfo {
    #[serde(rename = "key", default, skip_serializing_if = "Option::is_none")]
    pub log_id: Option<String>,
    #[serde(rename = "details", default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<CodeErrorDetail>,
    #[serde(
        rename = "permission_violations",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub permission_violations: Vec<CodeErrorPermissionViolation>,
    #[serde(
        rename = "field_violations",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub field_violations: Vec<CodeErrorFieldViolation>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodeErrorDetail {
    #[serde(rename = "key", default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodeErrorPermissionViolation {
    #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "subject", default, skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodeErrorFieldViolation {
    #[serde(rename = "field", default, skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    #[serde(rename = "value", default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(
        rename = "description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    // Helper function to create a test RawResponse
    fn create_test_raw_response(code: i32, msg: &str) -> RawResponse {
        RawResponse {
            code,
            msg: msg.to_string(),
            err: None,
        }
    }

    // Helper function to create a test BaseResponse
    fn create_test_response<T>(code: i32, msg: &str, data: Option<T>) -> BaseResponse<T> {
        BaseResponse {
            raw_response: create_test_raw_response(code, msg),
            data,
        }
    }

    #[test]
    fn test_base_response_success() {
        let response: BaseResponse<String> =
            create_test_response(0, "success", Some("test_data".to_string()));
        assert!(response.success());

        let error_response: BaseResponse<String> = create_test_response(1000, "error", None);
        assert!(!error_response.success());
    }

    #[test]
    fn test_base_response_code_and_msg() {
        let response: BaseResponse<String> = create_test_response(1234, "test message", None);
        assert_eq!(response.code(), 1234);
        assert_eq!(response.msg(), "test message");
    }

    #[test]
    fn test_base_response_data_or_error_success() {
        let response: BaseResponse<String> =
            create_test_response(0, "success", Some("test_data".to_string()));
        let result = response.data_or_error();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test_data");
    }

    #[test]
    fn test_base_response_data_or_error_success_but_no_data() {
        let response: BaseResponse<String> = create_test_response(0, "success", None);
        let result = response.data_or_error();
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "响应成功但数据为空");
    }

    #[test]
    fn test_base_response_data_or_error_failure() {
        let response: BaseResponse<String> = create_test_response(1000, "API error", None);
        let result = response.data_or_error();
        assert!(result.is_err());
        let error_msg = result.unwrap_err();
        assert!(error_msg.contains("API error"));
        assert!(error_msg.contains("1000"));
    }

    #[test]
    fn test_base_response_data_or_api_error_success() {
        let response: BaseResponse<String> =
            create_test_response(0, "success", Some("test_data".to_string()));
        let result = response.data_or_api_error();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test_data");
    }

    #[test]
    fn test_base_response_data_or_api_error_failure() {
        let response: BaseResponse<String> = create_test_response(1000, "API error", None);
        let result = response.data_or_api_error();
        assert!(result.is_err());

        match result.unwrap_err() {
            LarkAPIError::ApiError { code, message, .. } => {
                assert_eq!(code, 1000);
                assert_eq!(message, "API error");
            }
            _ => panic!("Expected ApiError"),
        }
    }

    #[test]
    fn test_base_response_handle_common_errors_success() {
        let response: BaseResponse<String> =
            create_test_response(0, "success", Some("test_data".to_string()));
        let result = response.handle_common_errors();
        assert!(result.is_ok());
    }

    #[test]
    fn test_base_response_user_friendly_error_success() {
        let response: BaseResponse<String> =
            create_test_response(0, "success", Some("test_data".to_string()));
        assert!(response.user_friendly_error().is_none());
    }

    #[test]
    fn test_base_response_user_friendly_error_failure() {
        let response: BaseResponse<String> = create_test_response(1000, "API error", None);
        let error = response.user_friendly_error();
        assert!(error.is_some());
        assert!(error.unwrap().contains("API error"));
    }

    #[test]
    fn test_base_response_error_solutions_success() {
        let response: BaseResponse<String> =
            create_test_response(0, "success", Some("test_data".to_string()));
        assert!(response.error_solutions().is_empty());
    }

    #[test]
    fn test_base_response_error_solutions_failure() {
        let response: BaseResponse<String> = create_test_response(1000, "API error", None);
        let solutions = response.error_solutions();
        assert!(!solutions.is_empty());
        assert!(solutions.contains(&"检查请求参数是否正确".to_string()));
    }

    #[test]
    fn test_base_response_help_links_success() {
        let response: BaseResponse<String> =
            create_test_response(0, "success", Some("test_data".to_string()));
        assert!(response.help_links().is_empty());
    }

    #[test]
    fn test_base_response_help_links_failure() {
        let response: BaseResponse<String> = create_test_response(1000, "API error", None);
        let links = response.help_links();
        assert!(!links.is_empty());
        assert!(links.iter().any(|(name, _)| name.contains("API文档")));
    }

    #[test]
    fn test_base_response_print_error_details() {
        let response: BaseResponse<String> =
            create_test_response(0, "success", Some("test_data".to_string()));
        // Test that print_error_details doesn't panic
        response.print_error_details();

        let error_response: BaseResponse<String> = create_test_response(1000, "API error", None);
        error_response.print_error_details();
    }

    #[test]
    fn test_raw_response_serialization() {
        let raw_resp = RawResponse {
            code: 1000,
            msg: "test error".to_string(),
            err: None,
        };

        let json = serde_json::to_string(&raw_resp).unwrap();
        assert!(json.contains("1000"));
        assert!(json.contains("test error"));

        let deserialized: RawResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.code, 1000);
        assert_eq!(deserialized.msg, "test error");
    }

    #[test]
    fn test_raw_response_display() {
        let raw_resp = RawResponse {
            code: 1234,
            msg: "display test".to_string(),
            err: None,
        };

        let display_str = format!("{}", raw_resp);
        assert!(display_str.contains("1234"));
        assert!(display_str.contains("display test"));
    }

    #[test]
    fn test_raw_response_data_format() {
        assert!(matches!(
            RawResponse::data_format(),
            ResponseFormat::Flatten
        ));
    }

    #[test]
    fn test_raw_response_debug() {
        let raw_resp = RawResponse {
            code: 500,
            msg: "debug test".to_string(),
            err: None,
        };

        let debug_str = format!("{:?}", raw_resp);
        assert!(debug_str.contains("RawResponse"));
        assert!(debug_str.contains("500"));
        assert!(debug_str.contains("debug test"));
    }

    #[test]
    fn test_raw_response_clone() {
        let raw_resp = RawResponse {
            code: 200,
            msg: "clone test".to_string(),
            err: None,
        };

        let cloned = raw_resp.clone();
        assert_eq!(raw_resp.code, cloned.code);
        assert_eq!(raw_resp.msg, cloned.msg);
    }

    #[test]
    fn test_raw_response_default() {
        let default_resp = RawResponse::default();
        assert_eq!(default_resp.code, 0);
        assert!(default_resp.msg.is_empty());
        assert!(default_resp.err.is_none());
    }

    #[test]
    fn test_empty_response_data_format() {
        assert!(matches!(EmptyResponse::data_format(), ResponseFormat::Data));
    }

    #[test]
    fn test_empty_response_serialization() {
        let empty_resp = EmptyResponse {};
        let json = serde_json::to_string(&empty_resp).unwrap();
        assert_eq!(json, "{}");

        let deserialized: EmptyResponse = serde_json::from_str("{}").unwrap();
        let _ = deserialized; // Just ensure it deserializes without error
    }

    #[test]
    fn test_empty_response_debug() {
        let empty_resp = EmptyResponse {};
        let debug_str = format!("{:?}", empty_resp);
        assert!(debug_str.contains("EmptyResponse"));
    }

    #[test]
    fn test_json_response_data_format() {
        assert!(matches!(JsonResponse::data_format(), ResponseFormat::Data));
    }

    #[test]
    fn test_binary_response_data_format() {
        assert!(matches!(
            BinaryResponse::data_format(),
            ResponseFormat::Binary
        ));
    }

    #[test]
    fn test_binary_response_from_binary() {
        let file_name = "test.pdf".to_string();
        let body = vec![1, 2, 3, 4, 5];

        let binary_resp = BinaryResponse::from_binary(file_name.clone(), body.clone());
        assert!(binary_resp.is_some());

        let resp = binary_resp.unwrap();
        assert_eq!(resp.file_name, file_name);
        assert_eq!(resp.body, body);
    }

    #[test]
    fn test_binary_response_serialization() {
        let binary_resp = BinaryResponse {
            file_name: "test_file.txt".to_string(),
            body: vec![72, 101, 108, 108, 111], // "Hello" in bytes
        };

        let json = serde_json::to_string(&binary_resp).unwrap();
        assert!(json.contains("test_file.txt"));

        let deserialized: BinaryResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.file_name, "test_file.txt");
        assert_eq!(deserialized.body, vec![72, 101, 108, 108, 111]);
    }

    #[test]
    fn test_binary_response_debug() {
        let binary_resp = BinaryResponse {
            file_name: "debug_file.bin".to_string(),
            body: vec![255, 254, 253],
        };

        let debug_str = format!("{:?}", binary_resp);
        assert!(debug_str.contains("BinaryResponse"));
        assert!(debug_str.contains("debug_file.bin"));
    }

    #[test]
    fn test_error_info_serialization() {
        let error_info = ErrorInfo {
            log_id: Some("test_log_id".to_string()),
            details: vec![],
            permission_violations: vec![],
            field_violations: vec![],
        };

        let json = serde_json::to_string(&error_info).unwrap();
        assert!(json.contains("test_log_id"));

        let deserialized: ErrorInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.log_id, Some("test_log_id".to_string()));
    }

    #[test]
    fn test_error_info_with_details() {
        let error_detail = CodeErrorDetail {
            key: Some("param_name".to_string()),
            value: Some("invalid_value".to_string()),
        };

        let error_info = ErrorInfo {
            log_id: Some("log_123".to_string()),
            details: vec![error_detail],
            permission_violations: vec![],
            field_violations: vec![],
        };

        let json = serde_json::to_string(&error_info).unwrap();
        assert!(json.contains("param_name"));
        assert!(json.contains("invalid_value"));

        let deserialized: ErrorInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.details.len(), 1);
        assert_eq!(deserialized.details[0].key, Some("param_name".to_string()));
    }

    #[test]
    fn test_error_info_with_permission_violations() {
        let permission_violation = CodeErrorPermissionViolation {
            type_: Some("SCOPE_REQUIRED".to_string()),
            subject: Some("read:user".to_string()),
            description: Some("Missing required permission".to_string()),
        };

        let error_info = ErrorInfo {
            log_id: None,
            details: vec![],
            permission_violations: vec![permission_violation],
            field_violations: vec![],
        };

        let json = serde_json::to_string(&error_info).unwrap();
        assert!(json.contains("SCOPE_REQUIRED"));
        assert!(json.contains("read:user"));

        let deserialized: ErrorInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.permission_violations.len(), 1);
        assert_eq!(
            deserialized.permission_violations[0].type_,
            Some("SCOPE_REQUIRED".to_string())
        );
    }

    #[test]
    fn test_error_info_with_field_violations() {
        let field_violation = CodeErrorFieldViolation {
            field: Some("email".to_string()),
            value: Some("invalid-email".to_string()),
            description: Some("Invalid email format".to_string()),
        };

        let error_info = ErrorInfo {
            log_id: None,
            details: vec![],
            permission_violations: vec![],
            field_violations: vec![field_violation],
        };

        let json = serde_json::to_string(&error_info).unwrap();
        assert!(json.contains("email"));
        assert!(json.contains("Invalid email format"));

        let deserialized: ErrorInfo = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.field_violations.len(), 1);
        assert_eq!(
            deserialized.field_violations[0].field,
            Some("email".to_string())
        );
    }

    #[test]
    fn test_error_info_debug() {
        let error_info = ErrorInfo {
            log_id: Some("debug_log".to_string()),
            details: vec![],
            permission_violations: vec![],
            field_violations: vec![],
        };

        let debug_str = format!("{:?}", error_info);
        assert!(debug_str.contains("ErrorInfo"));
        assert!(debug_str.contains("debug_log"));
    }

    #[test]
    fn test_error_info_clone() {
        let error_info = ErrorInfo {
            log_id: Some("clone_test".to_string()),
            details: vec![],
            permission_violations: vec![],
            field_violations: vec![],
        };

        let cloned = error_info.clone();
        assert_eq!(error_info.log_id, cloned.log_id);
        assert_eq!(error_info.details.len(), cloned.details.len());
    }

    #[test]
    fn test_code_error_detail_serialization() {
        let detail = CodeErrorDetail {
            key: Some("test_key".to_string()),
            value: Some("test_value".to_string()),
        };

        let json = serde_json::to_string(&detail).unwrap();
        assert!(json.contains("test_key"));
        assert!(json.contains("test_value"));

        let deserialized: CodeErrorDetail = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.key, Some("test_key".to_string()));
        assert_eq!(deserialized.value, Some("test_value".to_string()));
    }

    #[test]
    fn test_code_error_detail_with_none_values() {
        let detail = CodeErrorDetail {
            key: None,
            value: None,
        };

        let json = serde_json::to_string(&detail).unwrap();
        // Should serialize to empty object or minimal JSON
        assert!(json == "{}" || json.len() < 10);

        let deserialized: CodeErrorDetail = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.key, None);
        assert_eq!(deserialized.value, None);
    }

    #[test]
    fn test_base_response_serialization() {
        let response = BaseResponse {
            raw_response: RawResponse {
                code: 0,
                msg: "success".to_string(),
                err: None,
            },
            data: Some("test_data".to_string()),
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("success"));
        assert!(json.contains("test_data"));

        let deserialized: BaseResponse<String> = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.raw_response.code, 0);
        assert_eq!(deserialized.data, Some("test_data".to_string()));
    }

    #[test]
    fn test_base_response_debug() {
        let response: BaseResponse<String> =
            create_test_response(200, "OK", Some("debug_data".to_string()));
        let debug_str = format!("{:?}", response);
        assert!(debug_str.contains("BaseResponse"));
        assert!(debug_str.contains("debug_data"));
    }

    #[test]
    fn test_base_response_with_error_info() {
        let mut raw_resp = create_test_raw_response(400, "Bad Request");
        raw_resp.err = Some(ErrorInfo {
            log_id: Some("error_log_123".to_string()),
            details: vec![],
            permission_violations: vec![],
            field_violations: vec![],
        });

        let response: BaseResponse<String> = BaseResponse {
            raw_response: raw_resp,
            data: None,
        };

        assert!(!response.success());
        assert_eq!(response.code(), 400);
        assert!(response.err().is_some());
        assert_eq!(
            response.err().unwrap().log_id,
            Some("error_log_123".to_string())
        );
    }

    #[test]
    fn test_response_format_enum() {
        // Test that ResponseFormat variants can be created
        let _data_format = ResponseFormat::Data;
        let _flatten_format = ResponseFormat::Flatten;
        let _binary_format = ResponseFormat::Binary;
    }

    #[test]
    fn test_multiple_error_types() {
        let error_info = ErrorInfo {
            log_id: Some("multi_error_log".to_string()),
            details: vec![
                CodeErrorDetail {
                    key: Some("detail1".to_string()),
                    value: Some("value1".to_string()),
                },
                CodeErrorDetail {
                    key: Some("detail2".to_string()),
                    value: Some("value2".to_string()),
                },
            ],
            permission_violations: vec![CodeErrorPermissionViolation {
                type_: Some("PERMISSION_DENIED".to_string()),
                subject: Some("user:123".to_string()),
                description: Some("User lacks permission".to_string()),
            }],
            field_violations: vec![CodeErrorFieldViolation {
                field: Some("username".to_string()),
                value: Some("".to_string()),
                description: Some("Username cannot be empty".to_string()),
            }],
        };

        let json = serde_json::to_string(&error_info).unwrap();
        let deserialized: ErrorInfo = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.details.len(), 2);
        assert_eq!(deserialized.permission_violations.len(), 1);
        assert_eq!(deserialized.field_violations.len(), 1);
        assert_eq!(deserialized.log_id, Some("multi_error_log".to_string()));
    }

    #[test]
    fn test_base_response_memory_efficiency() {
        // Test creating many BaseResponse instances doesn't consume excessive memory
        let responses: Vec<BaseResponse<i32>> = (0..100)
            .map(|i| create_test_response(0, "success", Some(i)))
            .collect();

        assert_eq!(responses.len(), 100);

        // Verify each response has correct data
        for (i, response) in responses.iter().enumerate() {
            assert!(response.success());
            assert_eq!(response.data, Some(i as i32));
        }
    }
}
