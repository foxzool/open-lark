//! 认证信息（Verification）服务
//!
//! 提供飞书认证信息的完整功能集，支持身份验证、权限校验、认证管理、
//! 安全审计等企业级身份认证能力。是系统安全和权限控制的核心基础。
//!
//! # 核心功能
//!
//! ## 身份验证
//! - 🔐 用户身份认证验证
//! - 📱 多因素认证支持
//! - 🔑 令牌验证和管理
//! - 📊 认证状态查询
//! - ⏰ 认证有效期控制
//!
//! ## 权限校验
//! - 👑 用户权限验证
//! - 🔒 资源访问控制
//! - 📋 权限范围查询
//! - 🎯 细粒度权限管理
//! - 🛡️ 安全策略执行
//!
//! ## 认证管理
//! - 📝 认证信息管理
//! - 🔄 认证状态更新
//! - 📊 认证历史记录
//! - 🔔 认证异常通知
//! - 🔧 认证配置管理
//!
//! ## 安全审计
//! - 📋 认证日志记录
//! - 🔍 安全事件监控
//! - 📊 安全数据分析
//! - 🚨 异常行为检测
//! - 📈 安全趋势分析
//!
//! # 使用示例
//!
//! ```rust
//! use open_lark::prelude::*;
//!
//! let client = LarkClient::builder("app_id", "app_secret")
//!     .with_app_type(AppType::SelfBuild)
//!     .build();
//!
//! // 获取认证信息服务
//! let verification = &client.verification;
//!
//! // 验证用户身份
//! // let verify_request = VerifyUserRequest::builder()
//! //     .user_id("user_123")
//! //     .token("auth_token")
//! //     .build();
//! // let verify_result = verification.v1.user.verify(verify_request, None).await?;
//!
//! // 检查权限
//! // let permission_request = CheckPermissionRequest::builder()
//! //     .user_id("user_123")
//! //     .resource("document_123")
//! //     .action("read")
//! //     .build();
//! // let has_permission = verification.v1.permission.check(permission_request, None).await?;
//!
//! // 获取认证信息
//! // let info_request = GetVerificationInfoRequest::builder()
//! //     .user_id("user_123")
//! //     .build();
//! // let info = verification.v1.info.get(info_request, None).await?;
//!
//! // 记录安全事件
//! // let audit_request = LogSecurityEventRequest::builder()
//! //     .user_id("user_123")
//! //     .event_type("login_attempt")
//! //     .details("用户尝试登录")
//! //     .build();
//! // verification.v1.audit.log(audit_request, None).await?;
//! ```
//!
//! # API版本
//!
//! 当前支持v1版本，提供基础的认证功能：
//! - 身份验证和权限校验
//! - 认证信息管理
//! - 安全审计和监控
//! - 认证配置管理
//!
//! # 安全特性
//!
//! - 🔐 多层次安全防护
//! - 🛡️ 实时威胁检测
//! - 📊 智能风险评估
//! - 🔔 安全事件通知
//! - 📱 多设备认证同步
//!
//! # 合规支持
//!
//! - 📋 合规审计要求
//! - 🔍 详细操作日志
//! - 📊 安全报告生成
//! - 🛡️ 数据保护机制
//! - 📈 合规状态监控

/// 数据模型定义
pub mod models;
/// 认证信息服务 v1 版本
pub mod v1;

use crate::core::config::Config;

/// 认证信息服务
///
/// 企业级身份认证的统一入口，提供身份验证、权限校验、
/// 认证管理、安全审计等完整的身份认证功能。
///
/// # 服务架构
///
/// - **v1**: 认证信息API v1版本，提供基础功能集
/// - **models**: 数据模型和结构定义
///
/// # 核心特性
///
/// - 🔐 全面的身份验证功能
/// - 👑 细粒度权限控制系统
/// - 📊 完善的认证管理机制
/// - 🛡️ 实时安全审计能力
/// - 📱 多平台认证支持
///
/// # 适用场景
///
/// - 用户身份验证
/// - 系统权限控制
/// - 安全审计监控
/// - 合规性检查
/// - 风险管理控制
///
/// # 最佳实践
///
/// - 启用多因素认证
/// - 定期审查权限设置
/// - 监控异常认证行为
/// - 保护认证凭据安全
/// - 建立安全审计流程
pub struct VerificationService {
    /// v1版本API服务
    pub v1: v1::V1,
}

impl VerificationService {
    /// 创建新的认证信息服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置，包含认证信息和API设置
    ///
    /// # 返回值
    /// 配置完成的认证信息服务实例
    pub fn new(config: Config) -> Self {
        Self {
            v1: v1::V1::new(config),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_verification_service_creation() {
        let config = Config::default();
        let service = VerificationService::new(config.clone());

        assert_eq!(service.v1.config.app_id, config.app_id);
        assert_eq!(service.v1.config.app_secret, config.app_secret);
    }

    #[test]
    fn test_verification_service_with_custom_config() {
        let config = Config::builder()
            .app_id("verification_test_app")
            .app_secret("verification_test_secret")
            .req_timeout(Duration::from_secs(45))
            .build();

        let service = VerificationService::new(config.clone());

        assert_eq!(service.v1.config.app_id, "verification_test_app");
        assert_eq!(service.v1.config.app_secret, "verification_test_secret");
        assert_eq!(service.v1.config.req_timeout, Some(Duration::from_secs(45)));
    }

    #[test]
    fn test_verification_service_debug_trait() {
        let config = Config::default();
        let service = VerificationService::new(config);

        let debug_output = format!("{:?}", service.v1.config);
        assert!(debug_output.contains("Config"));
    }

    #[test]
    fn test_verification_service_config_independence() {
        let config1 = Config::builder().app_id("app_1").build();

        let config2 = Config::builder().app_id("app_2").build();

        let service1 = VerificationService::new(config1);
        let service2 = VerificationService::new(config2);

        assert_eq!(service1.v1.config.app_id, "app_1");
        assert_eq!(service2.v1.config.app_id, "app_2");
        assert_ne!(service1.v1.config.app_id, service2.v1.config.app_id);
    }

    #[test]
    fn test_verification_service_v1_access() {
        let config = Config::default();
        let service = VerificationService::new(config.clone());

        // Test that we can access the v1 service
        assert_eq!(service.v1.config.app_id, config.app_id);
    }

    #[test]
    fn test_verification_service_multiple_instances() {
        let config = Config::default();

        let service1 = VerificationService::new(config.clone());
        let service2 = VerificationService::new(config.clone());

        assert_eq!(service1.v1.config.app_id, service2.v1.config.app_id);
        assert_eq!(service1.v1.config.app_secret, service2.v1.config.app_secret);
    }
}
