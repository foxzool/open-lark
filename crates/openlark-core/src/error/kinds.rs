//! Error Kinds Definition
//!
//! 定义所有可能的错误种类，提供分类化的错误处理。

use std::borrow::Cow;

use super::{
    codes::ErrorCode,
    traits::{ErrorSeverity, ErrorType},
};

/// 错误种类枚举
///
/// 将所有可能的错误按层次分类，便于统一处理和分析。
/// 每个错误种类都有默认的处理策略和用户消息。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorKind {
    // 传输层错误 - 底层技术问题
    /// 网络相关错误
    Network,

    /// 序列化/反序列化错误
    Serialization,

    /// 认证相关错误
    Authentication,

    // 业务层错误 - 应用逻辑问题
    /// 权限相关错误
    Permission,

    /// 数据验证错误
    Validation,

    /// 业务逻辑错误
    Business,

    // 系统层错误 - 系统性问题
    /// 配置相关错误
    Configuration,

    /// 内部系统错误
    Internal,

    /// 限流相关错误
    RateLimit,
}

impl ErrorKind {
    /// 获取默认的用户友好消息
    pub fn default_user_message(&self) -> Cow<'static, str> {
        match self {
            Self::Network => Cow::Borrowed("网络连接失败，请检查网络设置"),
            Self::Serialization => Cow::Borrowed("数据格式错误，请检查输入"),
            Self::Authentication => Cow::Borrowed("身份验证失败，请重新登录"),
            Self::Permission => Cow::Borrowed("权限不足，请联系管理员"),
            Self::Validation => Cow::Borrowed("输入参数不正确，请检查后重试"),
            Self::Business => Cow::Borrowed("操作失败，请稍后重试或联系支持"),
            Self::Configuration => Cow::Borrowed("系统配置错误，请联系技术支持"),
            Self::Internal => Cow::Borrowed("系统内部错误，请联系技术支持"),
            Self::RateLimit => Cow::Borrowed("请求过于频繁，请稍后重试"),
        }
    }

    /// 获取建议的恢复动作
    pub fn recovery_action(&self) -> &'static str {
        match self {
            Self::Network => "检查网络连接后重试",
            Self::Serialization => "验证数据格式后重试",
            Self::Authentication => "重新进行身份验证",
            Self::Permission => "联系管理员获取权限",
            Self::Validation => "修正输入参数后重试",
            Self::Business => "检查业务规则后重试",
            Self::Configuration => "修复配置后重启服务",
            Self::Internal => "联系技术支持",
            Self::RateLimit => "等待后重试",
        }
    }

    /// 获取默认的严重程度
    pub fn default_severity(&self) -> ErrorSeverity {
        match self {
            Self::Network => ErrorSeverity::Error,
            Self::Serialization => ErrorSeverity::Warning,
            Self::Authentication => ErrorSeverity::Error,
            Self::Permission => ErrorSeverity::Warning,
            Self::Validation => ErrorSeverity::Warning,
            Self::Business => ErrorSeverity::Error,
            Self::Configuration => ErrorSeverity::Critical,
            Self::Internal => ErrorSeverity::Critical,
            Self::RateLimit => ErrorSeverity::Warning,
        }
    }

    /// 判断是否可重试
    pub fn is_retryable(&self) -> bool {
        matches!(self, Self::Network | Self::RateLimit)
    }

    /// 转换为对应的ErrorType
    pub fn to_error_type(&self) -> ErrorType {
        match self {
            Self::Network => ErrorType::Network,
            Self::Serialization => ErrorType::Serialization,
            Self::Authentication => ErrorType::Authentication,
            Self::Permission => ErrorType::Authentication, // 权限错误归类为认证错误
            Self::Validation => ErrorType::Validation,
            Self::Business => ErrorType::Business,
            Self::Configuration => ErrorType::Configuration,
            Self::Internal => ErrorType::Internal,
            Self::RateLimit => ErrorType::RateLimit,
        }
    }

    /// 获取错误种类的描述
    pub fn description(&self) -> &'static str {
        match self {
            Self::Network => "网络连接相关错误",
            Self::Serialization => "数据序列化相关错误",
            Self::Authentication => "身份验证相关错误",
            Self::Permission => "权限控制相关错误",
            Self::Validation => "数据验证相关错误",
            Self::Business => "业务逻辑相关错误",
            Self::Configuration => "系统配置相关错误",
            Self::Internal => "内部系统错误",
            Self::RateLimit => "请求频率限制错误",
        }
    }

    /// 判断是否为传输层错误
    pub fn is_transport_layer(&self) -> bool {
        matches!(
            self,
            Self::Network | Self::Serialization | Self::Authentication
        )
    }

    /// 判断是否为业务层错误
    pub fn is_business_layer(&self) -> bool {
        matches!(self, Self::Permission | Self::Validation | Self::Business)
    }

    /// 判断是否为系统层错误
    pub fn is_system_layer(&self) -> bool {
        matches!(self, Self::Configuration | Self::Internal | Self::RateLimit)
    }

    /// 判断是否需要用户干预
    pub fn requires_user_intervention(&self) -> bool {
        matches!(
            self,
            Self::Permission | Self::Configuration | Self::Internal
        )
    }

    /// 获取相关的错误码（如果有默认关联）
    pub fn default_error_code(&self) -> Option<ErrorCode> {
        match self {
            Self::Network => Some(ErrorCode::NetworkConnectionFailed),
            Self::Serialization => Some(ErrorCode::SerializationError),
            Self::Authentication => Some(ErrorCode::AuthenticationFailed),
            Self::Permission => Some(ErrorCode::PermissionDenied),
            Self::Validation => Some(ErrorCode::ValidationError),
            Self::Business => Some(ErrorCode::BusinessError),
            Self::Configuration => Some(ErrorCode::ConfigurationError),
            Self::Internal => Some(ErrorCode::InternalError),
            Self::RateLimit => Some(ErrorCode::RateLimitExceeded),
        }
    }
}

impl std::fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.description())
    }
}

/// 错误种类特征
pub trait ErrorKindExt {
    /// 判断是否为网络相关错误
    fn is_network_error(&self) -> bool;

    /// 判断是否为认证相关错误
    fn is_auth_error(&self) -> bool;

    /// 判断是否为权限相关错误
    fn is_permission_error(&self) -> bool;

    /// 判断是否为可重试错误
    fn is_retryable(&self) -> bool;

    /// 判断是否为严重错误
    fn is_critical(&self) -> bool;
}

impl ErrorKindExt for ErrorKind {
    fn is_network_error(&self) -> bool {
        matches!(self, Self::Network)
    }

    fn is_auth_error(&self) -> bool {
        matches!(self, Self::Authentication)
    }

    fn is_permission_error(&self) -> bool {
        matches!(self, Self::Permission)
    }

    fn is_retryable(&self) -> bool {
        self.is_retryable()
    }

    fn is_critical(&self) -> bool {
        matches!(self, Self::Configuration | Self::Internal)
    }
}

/// 错误种类分析工具
pub struct ErrorKindAnalyzer;

impl ErrorKindAnalyzer {
    /// 分析错误模式的建议
    pub fn analyze_pattern(errors: &[ErrorKind]) -> ErrorAnalysis {
        let mut kind_counts = std::collections::HashMap::new();
        let mut retryable_count = 0;
        let mut critical_count = 0;

        for &kind in errors {
            *kind_counts.entry(kind).or_insert(0) += 1;
            if kind.is_retryable() {
                retryable_count += 1;
            }
            if kind.is_critical() {
                critical_count += 1;
            }
        }

        ErrorAnalysis {
            total_errors: errors.len(),
            kind_distribution: kind_counts,
            retryable_percentage: (retryable_count as f64 / errors.len() as f64) * 100.0,
            critical_percentage: (critical_count as f64 / errors.len() as f64) * 100.0,
            most_common_kind: errors
                .iter()
                .cloned()
                .fold(std::collections::HashMap::new(), |mut acc, kind| {
                    *acc.entry(kind).or_insert(0) += 1;
                    acc
                })
                .into_iter()
                .max_by_key(|(_, count)| *count)
                .map(|(kind, _)| kind),
        }
    }

    /// 获取错误处理建议
    pub fn handling_recommendations(kind: ErrorKind) -> Vec<&'static str> {
        match kind {
            ErrorKind::Network => vec![
                "检查网络连接状态",
                "验证服务器可达性",
                "考虑使用连接池",
                "实现指数退避重试",
            ],
            ErrorKind::Serialization => vec![
                "验证数据格式",
                "检查Schema兼容性",
                "更新序列化版本",
                "添加数据校验",
            ],
            ErrorKind::Authentication => vec![
                "检查访问令牌有效性",
                "实现自动令牌刷新",
                "验证凭据配置",
                "考虑使用OAuth2流程",
            ],
            ErrorKind::Permission => vec![
                "检查应用权限配置",
                "验证用户权限范围",
                "联系管理员授权",
                "更新权限申请",
            ],
            ErrorKind::Validation => vec![
                "添加输入数据校验",
                "提供明确的错误提示",
                "使用Schema验证",
                "实现客户端校验",
            ],
            ErrorKind::Business => vec![
                "检查业务逻辑规则",
                "验证操作前置条件",
                "提供降级方案",
                "实现业务流程重试",
            ],
            ErrorKind::Configuration => vec![
                "检查配置文件格式",
                "验证必需参数",
                "更新配置版本",
                "实现配置热重载",
            ],
            ErrorKind::Internal => vec![
                "记录详细错误日志",
                "监控系统健康状态",
                "实现熔断机制",
                "联系技术支持",
            ],
            ErrorKind::RateLimit => vec![
                "实现请求限流",
                "使用缓存减少请求",
                "优化API调用频率",
                "考虑分布式限流",
            ],
        }
    }
}

/// 错误分析结果
#[derive(Debug, Clone)]
pub struct ErrorAnalysis {
    /// 总错误数
    pub total_errors: usize,
    /// 错误种类分布
    pub kind_distribution: std::collections::HashMap<ErrorKind, usize>,
    /// 可重试错误百分比
    pub retryable_percentage: f64,
    /// 严重错误百分比
    pub critical_percentage: f64,
    /// 最常见的错误种类
    pub most_common_kind: Option<ErrorKind>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_kind_properties() {
        assert!(ErrorKind::Network.is_retryable());
        assert!(ErrorKind::Network.is_transport_layer());
        assert!(!ErrorKind::Network.is_business_layer());
        assert!(!ErrorKind::Network.requires_user_intervention());

        assert!(!ErrorKind::Permission.is_retryable());
        assert!(ErrorKind::Permission.is_business_layer());
        assert!(ErrorKind::Permission.requires_user_intervention());
        assert!(ErrorKind::Permission.is_permission_error());

        assert!(!ErrorKind::Internal.is_retryable());
        assert!(ErrorKind::Internal.is_system_layer());
        assert!(ErrorKind::Internal.requires_user_intervention());
        assert!(ErrorKind::Internal.is_critical());
    }

    #[test]
    fn test_error_kind_messages() {
        let network_msg = ErrorKind::Network.default_user_message();
        assert!(network_msg.contains("网络"));

        let auth_msg = ErrorKind::Authentication.default_user_message();
        assert!(auth_msg.contains("身份验证"));
    }

    #[test]
    fn test_error_type_conversion() {
        assert_eq!(ErrorKind::Network.to_error_type(), ErrorType::Network);
        assert_eq!(
            ErrorKind::Authentication.to_error_type(),
            ErrorType::Authentication
        );
        assert_eq!(
            ErrorKind::Permission.to_error_type(),
            ErrorType::Authentication
        );
        assert_eq!(ErrorKind::Validation.to_error_type(), ErrorType::Validation);
    }

    #[test]
    fn test_error_kind_analyzer() {
        let errors = vec![
            ErrorKind::Network,
            ErrorKind::Network,
            ErrorKind::Authentication,
            ErrorKind::Permission,
        ];

        let analysis = ErrorKindAnalyzer::analyze_pattern(&errors);
        assert_eq!(analysis.total_errors, 4);
        assert_eq!(analysis.retryable_percentage, 50.0); // 两个Network可重试
        assert_eq!(analysis.most_common_kind, Some(ErrorKind::Network));

        let recommendations = ErrorKindAnalyzer::handling_recommendations(ErrorKind::Network);
        assert!(!recommendations.is_empty());
        assert!(recommendations.iter().any(|r| r.contains("网络")));
    }

    #[test]
    fn test_error_kind_display() {
        assert_eq!(ErrorKind::Network.to_string(), "网络连接相关错误");
        assert_eq!(ErrorKind::Authentication.to_string(), "身份验证相关错误");
    }
}
