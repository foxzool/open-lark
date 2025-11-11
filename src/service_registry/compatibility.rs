//! ServiceRegistry 兼容性处理机制
//!
//! 提供向后兼容性、版本检查和迁移支持功能

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use super::{error::ServiceError, metadata::ServiceMetadata, ServiceRegistry};
use config::Config;

/// 兼容性配置
#[derive(Debug, Clone)]
pub struct CompatibilityConfig {
    /// 是否启用严格模式
    pub strict_mode: bool,
    /// 是否允许自动降级
    pub allow_auto_downgrade: bool,
    /// 最大兼容性检查深度
    pub max_check_depth: usize,
    /// 兼容性检查超时
    pub check_timeout: std::time::Duration,
}

impl Default for CompatibilityConfig {
    fn default() -> Self {
        Self {
            strict_mode: false,
            allow_auto_downgrade: true,
            max_check_depth: 10,
            check_timeout: std::time::Duration::from_secs(30),
        }
    }
}

/// 服务版本信息
#[derive(Debug, Clone, PartialEq)]
pub struct ServiceVersion {
    /// 主版本号
    pub major: u32,
    /// 次版本号
    pub minor: u32,
    /// 修订版本号
    pub patch: u32,
    /// 预发布标识
    pub pre_release: Option<String>,
}

impl ServiceVersion {
    /// 创建新版本
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self {
            major,
            minor,
            patch,
            pre_release: None,
        }
    }

    /// 创建预发布版本
    pub fn pre_release(major: u32, minor: u32, patch: u32, pre: String) -> Self {
        Self {
            major,
            minor,
            patch,
            pre_release: Some(pre),
        }
    }

    /// 从字符串解析版本
    pub fn from_string(version: &str) -> Result<Self, ServiceError> {
        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() < 3 {
            return Err(ServiceError::invalid_configuration(
                "version",
                "invalid version format, expected major.minor.patch",
            ));
        }

        let major = parts[0]
            .parse()
            .map_err(|_| ServiceError::invalid_configuration("version", "invalid major version"))?;
        let minor = parts[1]
            .parse()
            .map_err(|_| ServiceError::invalid_configuration("version", "invalid minor version"))?;

        // 处理修订版本和预发布标识
        let patch_part = parts[2];
        let (patch_str, pre_release) = if let Some((patch, pre)) = patch_part.split_once('-') {
            (patch, Some(pre.to_string()))
        } else {
            (patch_part, None)
        };

        let patch = patch_str
            .parse()
            .map_err(|_| ServiceError::invalid_configuration("version", "invalid patch version"))?;

        Ok(Self {
            major,
            minor,
            patch,
            pre_release,
        })
    }

    /// 转换为字符串
    pub fn to_string(&self) -> String {
        let mut result = format!("{}.{}.{}", self.major, self.minor, self.patch);
        if let Some(ref pre) = self.pre_release {
            result.push_str("-");
            result.push_str(pre);
        }
        result
    }

    /// 检查版本兼容性
    pub fn is_compatible_with(&self, other: &ServiceVersion, strict: bool) -> bool {
        if strict {
            self == other
        } else {
            // 兼容性规则：主版本必须相同，次版本可以向后兼容
            if self.major != other.major {
                return false;
            }

            // 次版本检查：当前版本应 >= 要求的版本
            if self.minor < other.minor {
                return false;
            }

            // 如果次版本相同，检查修订版本
            if self.minor == other.minor && self.patch < other.patch {
                return false;
            }

            true
        }
    }
}

/// 兼容性检查结果
#[derive(Debug, Clone)]
pub struct CompatibilityResult {
    /// 是否兼容
    pub is_compatible: bool,
    /// 兼容性级别
    pub compatibility_level: CompatibilityLevel,
    /// 发现的问题
    pub issues: Vec<CompatibilityIssue>,
    /// 建议的解决方案
    pub recommendations: Vec<String>,
}

/// 兼容性级别
#[derive(Debug, Clone, PartialEq)]
pub enum CompatibilityLevel {
    /// 完全兼容
    Full,
    /// 大部分兼容，有小问题
    Mostly,
    /// 部分兼容，需要适配
    Partial,
    /// 不兼容
    Incompatible,
}

/// 兼容性问题
#[derive(Debug, Clone)]
pub struct CompatibilityIssue {
    /// 问题类型
    pub issue_type: CompatibilityIssueType,
    /// 严重程度
    pub severity: IssueSeverity,
    /// 描述
    pub description: String,
    /// 影响的服务
    pub affected_services: Vec<String>,
}

/// 兼容性问题类型
#[derive(Debug, Clone)]
pub enum CompatibilityIssueType {
    /// 版本不匹配
    VersionMismatch,
    /// API 变更
    ApiChange,
    /// 配置格式变更
    ConfigFormatChange,
    /// 依赖缺失
    DependencyMissing,
    /// 功能特性不匹配
    FeatureMismatch,
    /// 性能降级
    PerformanceDegradation,
}

/// 问题严重程度
#[derive(Debug, Clone, PartialEq)]
pub enum IssueSeverity {
    /// 信息级别
    Info,
    /// 警告级别
    Warning,
    /// 错误级别
    Error,
    /// 严重错误
    Critical,
}

/// 兼容性检查器
#[derive(Clone)]
pub struct CompatibilityChecker {
    config: CompatibilityConfig,
    version_cache: Arc<RwLock<HashMap<String, ServiceVersion>>>,
}

impl CompatibilityChecker {
    /// 创建新的兼容性检查器
    pub fn new(config: CompatibilityConfig) -> Self {
        Self {
            config,
            version_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// 检查服务兼容性
    pub fn check_service_compatibility(
        &self,
        service_name: &str,
        current_version: &ServiceVersion,
        registry: &ServiceRegistry,
    ) -> Result<CompatibilityResult, ServiceError> {
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();

        // 获取注册的服务信息
        let registered_services = registry.discover_services();

        // 检查服务是否存在
        if !registered_services.contains(&service_name) {
            issues.push(CompatibilityIssue {
                issue_type: CompatibilityIssueType::DependencyMissing,
                severity: IssueSeverity::Error,
                description: format!("Service '{}' is not registered", service_name),
                affected_services: vec![service_name.to_string()],
            });

            return Ok(CompatibilityResult {
                is_compatible: false,
                compatibility_level: CompatibilityLevel::Incompatible,
                issues,
                recommendations: vec![format!(
                    "Register service '{}' before checking compatibility",
                    service_name
                )],
            });
        }

        // 获取服务信息进行版本比较
        if let Some(service_info) = registry.get_service_info(service_name) {
            let registered_version = ServiceVersion::from_string(&service_info.version)?;

            if !current_version.is_compatible_with(&registered_version, self.config.strict_mode) {
                let severity = if current_version.major != registered_version.major {
                    IssueSeverity::Critical
                } else {
                    IssueSeverity::Warning
                };

                issues.push(CompatibilityIssue {
                    issue_type: CompatibilityIssueType::VersionMismatch,
                    severity,
                    description: format!(
                        "Version mismatch: current={}, registered={}",
                        current_version.to_string(),
                        registered_version.to_string()
                    ),
                    affected_services: vec![service_name.to_string()],
                });

                if self.config.allow_auto_downgrade {
                    recommendations.push(format!(
                        "Consider downgrading service '{}' to version {}",
                        service_name,
                        registered_version.to_string()
                    ));
                } else {
                    recommendations.push(format!(
                        "Update service '{}' to a compatible version",
                        service_name
                    ));
                }
            }
        }

        // 确定兼容性级别
        let compatibility_level = self.determine_compatibility_level(&issues);
        let is_compatible = !matches!(compatibility_level, CompatibilityLevel::Incompatible);

        Ok(CompatibilityResult {
            is_compatible,
            compatibility_level,
            issues,
            recommendations,
        })
    }

    /// 检查配置兼容性
    pub fn check_config_compatibility(
        &self,
        config: &Config,
        registry: &ServiceRegistry,
    ) -> Result<CompatibilityResult, ServiceError> {
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();

        // 检查配置的基本字段
        if config.app_id.is_empty() {
            issues.push(CompatibilityIssue {
                issue_type: CompatibilityIssueType::ConfigFormatChange,
                severity: IssueSeverity::Error,
                description: "App ID is empty".to_string(),
                affected_services: vec!["all".to_string()],
            });
        }

        if config.app_secret.is_empty() {
            issues.push(CompatibilityIssue {
                issue_type: CompatibilityIssueType::ConfigFormatChange,
                severity: IssueSeverity::Error,
                description: "App secret is empty".to_string(),
                affected_services: vec!["all".to_string()],
            });
        }

        // 检查 URL 格式
        if !config.base_url.starts_with("http://") && !config.base_url.starts_with("https://") {
            issues.push(CompatibilityIssue {
                issue_type: CompatibilityIssueType::ConfigFormatChange,
                severity: IssueSeverity::Warning,
                description: format!("Invalid base URL format: {}", config.base_url),
                affected_services: vec!["all".to_string()],
            });

            recommendations
                .push("Update base URL to include protocol (http:// or https://)".to_string());
        }

        // 确定兼容性级别
        let compatibility_level = self.determine_compatibility_level(&issues);
        let is_compatible = !matches!(compatibility_level, CompatibilityLevel::Incompatible);

        Ok(CompatibilityResult {
            is_compatible,
            compatibility_level,
            issues,
            recommendations,
        })
    }

    /// 批量兼容性检查
    pub fn batch_compatibility_check(
        &self,
        services: &HashMap<String, ServiceVersion>,
        registry: &ServiceRegistry,
    ) -> Result<Vec<(String, CompatibilityResult)>, ServiceError> {
        let mut results = Vec::new();

        for (service_name, version) in services {
            let result = self.check_service_compatibility(service_name, version, registry)?;
            results.push((service_name.clone(), result));
        }

        Ok(results)
    }

    /// 确定兼容性级别
    fn determine_compatibility_level(&self, issues: &[CompatibilityIssue]) -> CompatibilityLevel {
        if issues.is_empty() {
            return CompatibilityLevel::Full;
        }

        let has_critical = issues
            .iter()
            .any(|i| matches!(i.severity, IssueSeverity::Critical));
        let has_error = issues
            .iter()
            .any(|i| matches!(i.severity, IssueSeverity::Error));
        let has_warning = issues
            .iter()
            .any(|i| matches!(i.severity, IssueSeverity::Warning));

        if has_critical || has_error {
            CompatibilityLevel::Incompatible
        } else if has_warning {
            CompatibilityLevel::Partial
        } else {
            CompatibilityLevel::Mostly
        }
    }
}

/// 兼容性处理器
#[derive(Clone)]
pub struct CompatibilityHandler {
    checker: CompatibilityChecker,
    registry: Arc<ServiceRegistry>,
}

impl CompatibilityHandler {
    /// 创建新的兼容性处理器
    pub fn new(registry: Arc<ServiceRegistry>, config: CompatibilityConfig) -> Self {
        Self {
            checker: CompatibilityChecker::new(config),
            registry,
        }
    }

    /// 处理服务注册时的兼容性检查
    pub fn handle_service_registration(
        &self,
        service_name: &str,
        version: &str,
    ) -> Result<(), ServiceError> {
        let service_version = ServiceVersion::from_string(version)?;
        let result = self.checker.check_service_compatibility(
            service_name,
            &service_version,
            &self.registry,
        )?;

        if !result.is_compatible {
            let error_msg = format!(
                "Service '{}' version {} is not compatible: {}",
                service_name,
                version,
                result
                    .issues
                    .first()
                    .map(|i| &i.description)
                    .unwrap_or(&"Unknown issue".to_string())
            );
            return Err(ServiceError::validation_error(&error_msg));
        }

        // 记录兼容性警告
        if !matches!(result.compatibility_level, CompatibilityLevel::Full) {
            for issue in &result.issues {
                match issue.severity {
                    IssueSeverity::Warning => {
                        log::warn!(
                            "Compatibility warning for service '{}': {}",
                            service_name,
                            issue.description
                        );
                    }
                    IssueSeverity::Info => {
                        log::info!(
                            "Compatibility info for service '{}': {}",
                            service_name,
                            issue.description
                        );
                    }
                    _ => {}
                }
            }
        }

        Ok(())
    }

    /// 获取兼容性报告
    pub fn generate_compatibility_report(&self) -> CompatibilityReport {
        let registered_services = self.registry.discover_services();
        let mut service_results = HashMap::new();

        for service_name in registered_services {
            if let Some(service_info) = self.registry.get_service_info(service_name) {
                if let Ok(version) = ServiceVersion::from_string(&service_info.version) {
                    if let Ok(result) = self.checker.check_service_compatibility(
                        service_name,
                        &version,
                        &self.registry,
                    ) {
                        service_results.insert(service_name.to_string(), result);
                    }
                }
            }
        }

        CompatibilityReport {
            generated_at: std::time::SystemTime::now(),
            total_services: service_results.len(),
            compatible_services: service_results.values().filter(|r| r.is_compatible).count(),
            service_results,
        }
    }
}

/// 兼容性报告
#[derive(Debug, Clone)]
pub struct CompatibilityReport {
    /// 生成时间
    pub generated_at: std::time::SystemTime,
    /// 总服务数
    pub total_services: usize,
    /// 兼容服务数
    pub compatible_services: usize,
    /// 服务检查结果
    pub service_results: HashMap<String, CompatibilityResult>,
}

impl CompatibilityReport {
    /// 打印报告摘要
    pub fn print_summary(&self) {
        println!("📊 ServiceRegistry 兼容性报告");
        println!("==============================");
        println!("生成时间: {:?}", self.generated_at);
        println!("总服务数: {}", self.total_services);
        println!("兼容服务数: {}", self.compatible_services);

        let compatibility_rate = if self.total_services > 0 {
            (self.compatible_services as f64 / self.total_services as f64) * 100.0
        } else {
            0.0
        };

        println!("兼容率: {:.1}%", compatibility_rate);
        println!();

        // 打印有问题的服务
        let mut has_issues = false;
        for (service_name, result) in &self.service_results {
            if !result.issues.is_empty() {
                if !has_issues {
                    println!("⚠️  发现兼容性问题:");
                    has_issues = true;
                }

                println!("  服务: {}", service_name);
                for issue in &result.issues {
                    let severity_icon = match issue.severity {
                        IssueSeverity::Critical => "🔴",
                        IssueSeverity::Error => "❌",
                        IssueSeverity::Warning => "⚠️",
                        IssueSeverity::Info => "ℹ️",
                    };
                    println!(
                        "    {} {}: {}",
                        severity_icon,
                        format!("{:?}", issue.issue_type),
                        issue.description
                    );
                }

                if !result.recommendations.is_empty() {
                    println!("    💡 建议:");
                    for rec in &result.recommendations {
                        println!("      - {}", rec);
                    }
                }
                println!();
            }
        }

        if !has_issues {
            println!("✅ 所有服务都完全兼容！");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct MockService {
        name: &'static str,
    }

    impl MockService {
        fn new(name: &'static str) -> Self {
            Self { name }
        }
    }

    #[test]
    fn test_service_version_parsing() {
        let version = ServiceVersion::from_string("1.2.3").unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
        assert_eq!(version.to_string(), "1.2.3");
    }

    #[test]
    fn test_service_version_pre_release() {
        let version = ServiceVersion::from_string("1.2.3-beta").unwrap();
        assert_eq!(version.major, 1);
        assert_eq!(version.minor, 2);
        assert_eq!(version.patch, 3);
        assert_eq!(version.pre_release, Some("beta".to_string()));
        assert_eq!(version.to_string(), "1.2.3-beta");
    }

    #[test]
    fn test_version_compatibility() {
        let v1 = ServiceVersion::new(1, 2, 3);
        let v2 = ServiceVersion::new(1, 2, 4);
        let v3 = ServiceVersion::new(1, 3, 0);
        let v4 = ServiceVersion::new(2, 0, 0);

        // 非严格模式
        assert!(v2.is_compatible_with(&v1, false)); // 向后兼容
        assert!(v3.is_compatible_with(&v1, false)); // 次版本向后兼容
        assert!(!v1.is_compatible_with(&v2, false)); // 不能降级
        assert!(!v4.is_compatible_with(&v1, false)); // 主版本不同

        // 严格模式
        assert!(!v2.is_compatible_with(&v1, true)); // 版本不同
        assert!(v1.is_compatible_with(&v1, true)); // 完全相同
    }

    #[test]
    fn test_compatibility_checker() {
        let config = CompatibilityConfig::default();
        let checker = CompatibilityChecker::new(config);
        let registry = ServiceRegistry::new();

        let version = ServiceVersion::new(1, 0, 0);
        let result = checker.check_service_compatibility("test-service", &version, &registry);

        // 服务不存在，应该返回不兼容
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(!result.is_compatible);
        assert_eq!(result.compatibility_level, CompatibilityLevel::Incompatible);
    }
}
