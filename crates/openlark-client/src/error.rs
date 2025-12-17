//! OpenLark Client 错误类型定义
//!
//! 基于 openlark-core 的现代化错误处理系统
//! 直接使用 CoreError，提供类型安全和用户友好的错误管理

use crate::registry::RegistryError;
use openlark_core::error::{
    ApiError, CoreError, ErrorCategory, ErrorCode, ErrorContext, ErrorTrait, ErrorType,
};

/// 🚨 OpenLark 客户端错误类型
///
/// 直接类型别名，充分利用 CoreError 的强大功能
pub type Error = CoreError;

/// 📦 客户端结果类型别名
pub type Result<T> = std::result::Result<T, Error>;

// ============================================================================
// 便利错误创建函数（重新导出核心函数）
// ============================================================================

/// 创建网络错误
pub fn network_error(message: impl Into<String>) -> Error {
    openlark_core::error::network_error(message)
}

/// 创建认证错误
pub fn authentication_error(message: impl Into<String>) -> Error {
    openlark_core::error::authentication_error(message)
}

/// 创建访问令牌格式/内容无效错误
pub fn token_invalid_error(detail: impl Into<String>) -> Error {
    openlark_core::error::token_invalid_error(detail)
}

/// 创建访问令牌过期错误（飞书通用码 99991677）
pub fn token_expired_error(detail: impl Into<String>) -> Error {
    openlark_core::error::token_expired_error(detail)
}

/// 创建缺少权限 scope 的错误
pub fn permission_missing_error(scopes: &[impl AsRef<str>]) -> Error {
    openlark_core::error::permission_missing_error(scopes)
}

/// 创建 SSO 令牌无效错误
pub fn sso_token_invalid_error(detail: impl Into<String>) -> Error {
    openlark_core::error::sso_token_invalid_error(detail)
}

/// 创建身份标识非法错误
pub fn user_identity_invalid_error(desc: impl Into<String>) -> Error {
    openlark_core::error::user_identity_invalid_error(desc)
}

/// 基于飞书通用 `code` 的统一错误映射（客户端自定义解析时可复用）
pub fn from_feishu_response(
    code: i32,
    endpoint: impl Into<String>,
    message: impl Into<String>,
    request_id: Option<String>,
) -> Error {
    let mapped = ErrorCode::from_feishu_code(code).unwrap_or_else(|| ErrorCode::from_code(code));

    let mut ctx = ErrorContext::new();
    ctx.add_context("feishu_code", code.to_string());
    if let Some(req) = request_id {
        ctx.set_request_id(req);
    }

    let status = mapped
        .http_status()
        .unwrap_or_else(|| match mapped.category() {
            ErrorCategory::RateLimit => 429,
            ErrorCategory::Authentication
            | ErrorCategory::Permission
            | ErrorCategory::Parameter => 400,
            ErrorCategory::Resource => 404,
            _ => 500,
        });

    CoreError::Api(Box::new(ApiError {
        status,
        endpoint: endpoint.into().into(),
        message: message.into(),
        source: None,
        code: mapped,
        ctx: Box::new(ctx),
    }))
}

/// 创建API错误
pub fn api_error(
    status: u16,
    endpoint: impl Into<String>,
    message: impl Into<String>,
    request_id: Option<String>,
) -> Error {
    openlark_core::error::api_error(status, endpoint, message, request_id)
}

/// 创建验证错误
pub fn validation_error(field: impl Into<String>, message: impl Into<String>) -> Error {
    openlark_core::error::validation_error(field, message)
}

/// 创建配置错误
pub fn configuration_error(message: impl Into<String>) -> Error {
    openlark_core::error::configuration_error(message)
}

/// 创建序列化错误
pub fn serialization_error(message: impl Into<String>) -> Error {
    openlark_core::error::serialization_error(message, None::<serde_json::Error>)
}

/// 创建业务逻辑错误
pub fn business_error(_code: impl Into<String>, message: impl Into<String>) -> Error {
    openlark_core::error::business_error(message)
}

/// 创建超时错误
pub fn timeout_error(operation: impl Into<String>) -> Error {
    use std::time::Duration;
    openlark_core::error::timeout_error(Duration::from_secs(30), Some(operation.into()))
}

/// 创建限流错误
pub fn rate_limit_error(retry_after: Option<u64>) -> Error {
    use std::time::Duration;
    openlark_core::error::rate_limit_error(
        100,
        Duration::from_secs(60),
        retry_after.map(|s| Duration::from_secs(s)),
    )
}

/// 创建服务不可用错误
pub fn service_unavailable_error(service: impl Into<String>) -> Error {
    use std::time::Duration;
    openlark_core::error::service_unavailable_error(service, Some(Duration::from_secs(60)))
}

/// 创建内部错误
pub fn internal_error(message: impl Into<String>) -> Error {
    openlark_core::error::api_error(500, "internal", message, None::<String>)
}

/// 创建注册表错误
pub fn registry_error(err: RegistryError) -> Error {
    internal_error(format!("服务注册表错误: {}", err))
}

// ============================================================================
// 错误扩展功能
// ============================================================================

pub trait ClientErrorExt {
    /// 获取错误建议
    fn suggestion(&self) -> &'static str;

    /// 获取错误恢复步骤
    fn recovery_steps(&self) -> Vec<&'static str>;
}

impl ClientErrorExt for Error {
    fn suggestion(&self) -> &'static str {
        match self.error_type() {
            ErrorType::Network => "检查网络连接，确认防火墙设置",
            ErrorType::Authentication => "验证应用凭据，检查令牌有效性",
            ErrorType::Api => "检查API参数，确认请求格式正确",
            ErrorType::Validation => "验证输入参数格式和范围",
            ErrorType::Configuration => "检查应用配置文件和环境变量",
            ErrorType::Serialization => "确认数据格式正确，检查JSON结构",
            ErrorType::Business => "确认业务逻辑条件，检查相关权限",
            ErrorType::Timeout => "增加超时时间或优化请求内容",
            ErrorType::RateLimit => "稍后重试，考虑降低请求频率",
            ErrorType::ServiceUnavailable => "稍后重试，检查服务状态",
            ErrorType::Internal => "联系技术支持，提供错误详情",
        }
    }

    fn recovery_steps(&self) -> Vec<&'static str> {
        match self.error_type() {
            ErrorType::Network => vec![
                "检查网络连接状态",
                "确认代理设置正确",
                "验证防火墙规则",
                "尝试切换网络环境",
            ],
            ErrorType::Authentication => vec![
                "验证应用ID和密钥正确性",
                "检查令牌是否过期",
                "确认应用权限配置",
                "重新生成访问令牌",
            ],
            ErrorType::Api => vec![
                "检查请求参数格式",
                "确认API端点正确",
                "验证请求体结构",
                "查阅API文档",
            ],
            ErrorType::Validation => vec![
                "检查必填字段",
                "验证数据格式和范围",
                "确认字段类型正确",
                "参考输入示例",
            ],
            ErrorType::Configuration => vec![
                "检查环境变量设置",
                "验证配置文件格式",
                "确认应用权限配置",
                "重新加载配置",
            ],
            ErrorType::Serialization => vec![
                "检查JSON格式正确性",
                "验证数据结构匹配",
                "确认字段类型一致",
                "使用在线JSON验证工具",
            ],
            ErrorType::Business => vec![
                "检查业务规则约束",
                "确认用户权限充分",
                "验证数据完整性",
                "联系业务负责人",
            ],
            ErrorType::Timeout => vec![
                "增加请求超时时间",
                "优化网络环境",
                "减少请求数据量",
                "考虑分批处理",
            ],
            ErrorType::RateLimit => vec![
                "等待后重试",
                "降低请求频率",
                "实施退避策略",
                "联系技术支持提高限额",
            ],
            ErrorType::ServiceUnavailable => vec![
                "稍后重试请求",
                "检查服务状态页面",
                "切换到备用方案",
                "联系技术支持",
            ],
            ErrorType::Internal => vec![
                "记录详细错误信息",
                "检查系统日志",
                "重启相关服务",
                "联系技术支持",
            ],
        }
    }
}

// ============================================================================
// 类型转换
// ============================================================================

// 注意: reqwest::Error -> CoreError 转换已在 openlark-core 中实现
// 这里不需要重复实现，直接使用 CoreError 的转换机制

// 注意: 不能为外部类型实现 From，因为这些类型由 CoreError 定义在 openlark-core 中
// 请使用对应的函数来进行错误转换

// 从注册表错误转换
impl From<RegistryError> for Error {
    fn from(err: RegistryError) -> Self {
        registry_error(err)
    }
}

impl From<crate::registry::feature_flags::FeatureFlagError> for Error {
    fn from(err: crate::registry::feature_flags::FeatureFlagError) -> Self {
        registry_error(RegistryError::FeatureFlagError(err))
    }
}

impl From<crate::registry::dependency_resolver::DependencyError> for Error {
    fn from(err: crate::registry::dependency_resolver::DependencyError) -> Self {
        registry_error(RegistryError::DependencyError(err))
    }
}

// ============================================================================
// 便利函数
// ============================================================================

/// 🔧 从 openlark-core SDKResult 转换为客户端 Result 的便利函数
///
/// 这个函数现在只是类型转换，因为我们直接使用 CoreError
///
/// # 示例
///
/// ```rust
/// use openlark_client::error::from_sdk_result;
///
/// let core_result: openlark_core::SDKResult<String> = Ok("success".to_string());
/// let client_result = from_sdk_result(core_result);
/// assert!(client_result.is_ok());
/// ```
pub fn from_sdk_result<T>(result: openlark_core::SDKResult<T>) -> Result<T> {
    result
}

/// 🔧 创建带有上下文的错误的便利函数
pub fn with_context<T>(
    result: Result<T>,
    context_key: impl Into<String>,
    context_value: impl Into<String>,
) -> Result<T> {
    result.map_err(|err| {
        let key = context_key.into();
        let val = context_value.into();

        match err {
            CoreError::Network(mut net) => {
                net.ctx.add_context(key, val);
                CoreError::Network(net)
            }
            CoreError::Authentication {
                message,
                code,
                mut ctx,
            } => {
                ctx.add_context(key, val);
                CoreError::Authentication { message, code, ctx }
            }
            CoreError::Api(mut api) => {
                api.ctx.add_context(key, val);
                CoreError::Api(api)
            }
            CoreError::Validation {
                field,
                message,
                code,
                mut ctx,
            } => {
                ctx.add_context(key, val);
                CoreError::Validation {
                    field,
                    message,
                    code,
                    ctx,
                }
            }
            CoreError::Configuration {
                message,
                code,
                mut ctx,
            } => {
                ctx.add_context(key, val);
                CoreError::Configuration { message, code, ctx }
            }
            CoreError::Serialization {
                message,
                source,
                code,
                mut ctx,
            } => {
                ctx.add_context(key, val);
                CoreError::Serialization {
                    message,
                    source,
                    code,
                    ctx,
                }
            }
            CoreError::Business {
                code,
                message,
                mut ctx,
            } => {
                ctx.add_context(key, val);
                CoreError::Business { code, message, ctx }
            }
            CoreError::Timeout {
                duration,
                operation,
                mut ctx,
            } => {
                ctx.add_context(key, val);
                CoreError::Timeout {
                    duration,
                    operation,
                    ctx,
                }
            }
            CoreError::RateLimit {
                limit,
                window,
                reset_after,
                code,
                mut ctx,
            } => {
                ctx.add_context(key, val);
                CoreError::RateLimit {
                    limit,
                    window,
                    reset_after,
                    code,
                    ctx,
                }
            }
            CoreError::ServiceUnavailable {
                service,
                retry_after,
                code,
                mut ctx,
            } => {
                ctx.add_context(key, val);
                CoreError::ServiceUnavailable {
                    service,
                    retry_after,
                    code,
                    ctx,
                }
            }
            CoreError::Internal {
                code,
                message,
                source,
                mut ctx,
            } => {
                ctx.add_context(key, val);
                CoreError::Internal {
                    code,
                    message,
                    source,
                    ctx,
                }
            }
            _ => err,
        }
    })
}

/// 🔧 创建带有操作上下文的错误的便利函数
pub fn with_operation_context<T>(
    result: Result<T>,
    operation: impl Into<String>,
    component: impl Into<String>,
) -> Result<T> {
    result.map_err(|err| {
        let op = operation.into();
        let comp = component.into();

        // 直接解构 CoreError，修改上下文后重新构建，保留所有其他信息
        match err {
            CoreError::Network(mut net) => {
                net.ctx
                    .set_operation(op.clone())
                    .set_component(comp.clone());
                net.ctx
                    .add_context("operation", op)
                    .add_context("component", comp);
                CoreError::Network(net)
            }
            CoreError::Authentication {
                message,
                code,
                mut ctx,
            } => {
                ctx.set_operation(op.clone()).set_component(comp.clone());
                ctx.add_context("operation", op)
                    .add_context("component", comp);
                CoreError::Authentication { message, code, ctx }
            }
            CoreError::Api(mut api) => {
                api.ctx
                    .set_operation(op.clone())
                    .set_component(comp.clone());
                api.ctx
                    .add_context("operation", op)
                    .add_context("component", comp);
                CoreError::Api(api)
            }
            CoreError::Validation {
                field,
                message,
                code,
                mut ctx,
            } => {
                ctx.set_operation(op.clone()).set_component(comp.clone());
                ctx.add_context("operation", op)
                    .add_context("component", comp);
                CoreError::Validation {
                    field,
                    message,
                    code,
                    ctx,
                }
            }
            CoreError::Configuration {
                message,
                code,
                mut ctx,
            } => {
                ctx.set_operation(op.clone()).set_component(comp.clone());
                ctx.add_context("operation", op)
                    .add_context("component", comp);
                CoreError::Configuration { message, code, ctx }
            }
            CoreError::Serialization {
                message,
                source,
                code,
                mut ctx,
            } => {
                ctx.set_operation(op.clone()).set_component(comp.clone());
                ctx.add_context("operation", op)
                    .add_context("component", comp);
                CoreError::Serialization {
                    message,
                    source,
                    code,
                    ctx,
                }
            }
            CoreError::Business {
                code,
                message,
                mut ctx,
            } => {
                ctx.set_operation(op.clone()).set_component(comp.clone());
                ctx.add_context("operation", op)
                    .add_context("component", comp);
                CoreError::Business { code, message, ctx }
            }
            CoreError::Timeout {
                duration,
                operation: _, // 覆盖操作名
                mut ctx,
            } => {
                ctx.set_operation(op.clone()).set_component(comp.clone());
                ctx.add_context("operation", op.clone())
                    .add_context("component", comp);
                CoreError::Timeout {
                    duration,
                    operation: Some(op), // 确保更新 Enum 字段
                    ctx,
                }
            }
            CoreError::RateLimit {
                limit,
                window,
                reset_after,
                code,
                mut ctx,
            } => {
                ctx.set_operation(op.clone()).set_component(comp.clone());
                ctx.add_context("operation", op)
                    .add_context("component", comp);
                CoreError::RateLimit {
                    limit,
                    window,
                    reset_after,
                    code,
                    ctx,
                }
            }
            CoreError::ServiceUnavailable {
                service,
                retry_after,
                code,
                mut ctx,
            } => {
                ctx.set_operation(op.clone()).set_component(comp.clone());
                ctx.add_context("operation", op)
                    .add_context("component", comp);
                CoreError::ServiceUnavailable {
                    service,
                    retry_after,
                    code,
                    ctx,
                }
            }
            CoreError::Internal {
                code,
                message,
                source,
                mut ctx,
            } => {
                ctx.set_operation(op.clone()).set_component(comp.clone());
                ctx.add_context("operation", op)
                    .add_context("component", comp);
                CoreError::Internal {
                    code,
                    message,
                    source,
                    ctx,
                }
            }
            _ => err,
        }
    })
}

// ============================================================================
// 错误分析和报告
// ============================================================================

/// 错误分析器，提供详细的错误信息和恢复建议
#[derive(Debug)]
pub struct ErrorAnalyzer<'a> {
    error: &'a Error,
}

impl<'a> ErrorAnalyzer<'a> {
    /// 创建错误分析器
    pub fn new(error: &'a Error) -> Self {
        Self { error }
    }

    /// 获取详细的错误报告
    pub fn detailed_report(&self) -> String {
        let mut report = String::new();

        report.push_str("🚨 错误分析报告\n");
        report.push_str("================\n\n");

        // 基本信息
        report.push_str("📋 基本信息:\n");
        report.push_str(&format!("  错误类型: {:?}\n", self.error.error_type()));
        report.push_str(&format!("  错误代码: {:?}\n", self.error.error_code()));
        report.push_str(&format!("  严重程度: {:?}\n", self.error.severity()));
        report.push_str(&format!("  可重试: {}\n", self.error.is_retryable()));

        if let Some(request_id) = self.error.context().request_id() {
            report.push_str(&format!("  请求ID: {}\n", request_id));
        }

        report.push_str("\n");

        // 错误消息
        report.push_str("💬 错误消息:\n");
        report.push_str(&format!("  技术消息: {}\n", self.error.to_string()));
        report.push_str(&format!(
            "  用户消息: {}\n",
            self.error.user_message().unwrap_or("未知错误")
        ));

        report.push_str("\n");

        // 建议和恢复步骤
        report.push_str("💡 建议:\n");
        report.push_str(&format!("  {}\n", self.error.suggestion()));

        report.push_str("\n🔧 恢复步骤:\n");
        for (i, step) in self.error.recovery_steps().iter().enumerate() {
            report.push_str(&format!("  {}. {}\n", i + 1, step));
        }

        report.push_str("\n");

        // 上下文信息
        if self.error.context().context_len() > 0 {
            report.push_str("📊 上下文信息:\n");
            for (key, value) in self.error.context().all_context() {
                report.push_str(&format!("  {}: {}\n", key, value));
            }
            report.push_str("\n");
        }

        // 时间戳
        if let Some(timestamp) = self.error.context().timestamp() {
            report.push_str(&format!(
                "⏰ 发生时间: {}\n",
                timestamp.format("%Y-%m-%d %H:%M:%S UTC")
            ));
        }

        report
    }

    /// 获取适合日志记录的错误摘要
    pub fn log_summary(&self) -> String {
        format!(
            "Error[{:?}:{:?}] {} - {}",
            self.error.error_type(),
            self.error.error_code(),
            self.error.user_message().unwrap_or("未知错误"),
            if self.error.is_retryable() {
                "(可重试)"
            } else {
                "(不可重试)"
            }
        )
    }

    /// 获取用户友好的错误消息，包含恢复建议
    pub fn user_friendly_with_suggestion(&self) -> String {
        format!(
            "{}\n\n💡 建议: {}\n\n🔧 可以尝试:\n{}",
            self.error.user_message().unwrap_or("未知错误"),
            self.error.suggestion(),
            self.error
                .recovery_steps()
                .iter()
                .enumerate()
                .map(|(i, step)| format!("{}. {}", i + 1, step))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }
}

// 注意: 不能为外部类型 CoreError 定义 inherent impl
// 请使用 ClientErrorExt trait 来获得扩展功能

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_convenience_functions() {
        let network_err = network_error("连接失败");
        assert!(network_err.is_network_error());
        assert!(network_err.is_retryable());

        let auth_err = authentication_error("令牌无效");
        assert!(auth_err.is_auth_error());
        assert!(!auth_err.is_retryable());

        let validation_err = validation_error("email", "邮箱格式不正确");
        assert!(validation_err.is_validation_error());
        assert!(!validation_err.is_retryable());
    }

    #[test]
    fn test_error_analyzer() {
        let error = api_error(404, "/users", "用户不存在", Some("req-123".to_string()));
        let analyzer = ErrorAnalyzer::new(&error);

        let report = analyzer.detailed_report();
        assert!(report.contains("错误分析报告"));
        assert!(report.contains("API错误"));
        assert!(report.contains("req-123"));

        let summary = analyzer.log_summary();
        assert!(summary.contains("Error"));
        assert!(summary.contains("Api"));

        let user_msg = analyzer.user_friendly_with_suggestion();
        assert!(user_msg.contains("建议"));
        assert!(user_msg.contains("可以尝试"));
    }

    #[test]
    fn test_client_error_ext() {
        let error = timeout_error("数据同步");

        assert!(!error.is_network_error());
        assert!(!error.is_auth_error());
        assert!(!error.is_business_error());
        assert!(error.is_retryable());

        let suggestion = error.suggestion();
        assert!(!suggestion.is_empty());

        let steps = error.recovery_steps();
        assert!(!steps.is_empty());
        assert!(steps.contains(&"增加请求超时时间"));
    }

    #[test]
    fn test_error_conversions() {
        // 测试 JSON 错误转换
        let json_err = serde_json::from_str::<serde_json::Value>("invalid json").unwrap_err();
        let error: Error = json_err.into();
        assert!(error.is_serialization_error());

        // 测试 tokio 超时错误转换
        // let timeout_err = tokio::time::error::Elapsed {}; // Private field
        // let error: Error = timeout_err.into();
        // assert!(error.is_timeout_error());
        // assert!(error.is_retryable());
    }

    #[test]
    fn test_context_functions() {
        let result: Result<i32> = Err(validation_error("age", "年龄不能为负数"));

        let contextual_result = with_context(result, "user_id", "12345");
        assert!(contextual_result.is_err());

        let error = contextual_result.unwrap_err();
        // 我们现在使用结构化上下文，验证上下文内容而不是字符串
        // assert!(error.to_string().contains("user_id: 12345"));
        assert_eq!(error.context().get_context("user_id"), Some("12345"));
    }

    #[test]
    fn test_sdk_result_conversion() {
        // 成功情况
        let core_result: openlark_core::SDKResult<String> = Ok("success".to_string());
        let client_result: Result<String> = from_sdk_result(core_result);
        assert!(client_result.is_ok());
        assert_eq!(client_result.unwrap(), "success");

        // 失败情况
        let core_result: openlark_core::SDKResult<String> = Err(network_error("网络错误"));
        let client_result: Result<String> = from_sdk_result(core_result);
        assert!(client_result.is_err());
        assert!(client_result.unwrap_err().is_network_error());
    }
}
