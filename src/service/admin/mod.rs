//! 管理后台（Admin）服务
//!
//! 提供飞书管理后台的完整功能集，支持登录密码管理、数据报表、
//! 企业勋章、授予管理等企业级后台管理能力。是企业管理和运营的核心工具。
//!
//! # 核心功能
//!
//! ## 登录密码管理
//! - 🔐 企业员工登录密码设置
//! - 🔄 密码重置和强制更新
//! - 📊 密码安全策略管理
//! - 🛡️ 密码复杂度和有效期控制
//! - 📋 密码变更历史记录
//!
//! ## 数据报表管理
//! - 📈 企业数据统计和分析
//! - 📊 用户活跃度报表
//! - 💼 应用使用情况统计
//! - 📅 定期报表生成和推送
//! - 📋 自定义报表配置
//!
//! ## 企业勋章管理
//! - 🏆 企业勋章设计和创建
//! - 🎨 勋章样式和等级设置
//! - 📊 勋章统计和分析
//! - 🎯 勋章规则和条件配置
//! - 🔄 勋章更新和维护
//!
//! ## 勋章授予管理
//! - 👥 勋章授予名单管理
//! - 🎖️ 批量勋章授予操作
//! - 📋 授予记录查询和统计
//! - 🔔 授予通知和推送
//! - 📊 授予效果分析
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
//! // 获取管理后台服务
//! let admin = &client.admin;
//!
//! // 重置用户密码
//! // let password_request = ResetPasswordRequest::builder()
//! //     .user_id("user_id")
//! //     .new_password("new_password")
//! //     .build();
//! // admin.password.reset(password_request, None).await?;
//!
//! // 获取数据报表
//! // let report_request = GetDataReportRequest::builder()
//! //     .report_type("user_activity")
//! //     .start_date("2024-07-01")
//! //     .end_date("2024-07-31")
//! //     .build();
//! // let report = admin.data_report.get(report_request, None).await?;
//!
//! // 创建企业勋章
//! // let badge_request = CreateBadgeRequest::builder()
//! //     .name("年度优秀员工")
//! //     .description("表彰年度表现优秀的员工")
//! //     .icon_url("https://example.com/badge.png")
//! //     .build();
//! // admin.badge.create(badge_request, None).await?;
//!
//! // 授予勋章
//! // let grant_request = GrantBadgeRequest::builder()
//! //     .badge_id("badge_id")
//! //     .user_ids(vec!["user1", "user2"])
//! //     .reason("2024年度优秀表现")
//! //     .build();
//! // admin.badge_grant.grant(grant_request, None).await?;
//! ```
//!
//! # 管理后台特性
//!
//! - 👑 超级管理员权限控制
//! - 📊 实时数据监控和分析
//! - 🔐 企业级安全管理
//! - 🎯 精细化权限控制
//! - 📱 移动端管理支持
//!
//! # 运营管理
//!
//! - 📈 数据驱动的决策支持
//! - 🎯 用户行为分析洞察
//! - 🏆 激励体系设计和管理
//! - 📋 操作审计和合规
//! - 🔄 自动化运营流程

/// 勋章管理功能
pub mod badge;
/// 勋章授予管理功能
pub mod badge_grant;
/// 数据报表功能
pub mod data_report;
/// 数据模型定义
pub mod models;
/// 密码管理功能
pub mod password;

use crate::core::config::Config;
use badge::BadgeService;
use badge_grant::BadgeGrantService;
use data_report::DataReportService;
use password::PasswordService;

/// 管理后台服务
///
/// 企业级后台管理的统一入口，提供登录密码管理、数据报表、
/// 企业勋章、授予管理等完整的后台管理能力。
///
/// # 服务架构
///
/// - **password**: 登录密码管理服务
/// - **data_report**: 数据报表管理服务
/// - **badge**: 企业勋章管理服务
/// - **badge_grant**: 勋章授予管理服务
/// - **models**: 数据模型和结构定义
///
/// # 核心特性
///
/// - 🔐 全面的安全管理能力
/// - 📊 丰富的数据分析功能
/// - 🏆 完善的激励体系管理
/// - 👑 细粒度权限控制
/// - 📱 多端管理支持
///
/// # 适用场景
///
/// - 企业安全策略管理
/// - 数据统计和分析
/// - 员工激励体系建设
/// - 后台运营管理
/// - 合规审计支持
///
/// # 最佳实践
///
/// - 定期更新安全策略
/// - 合理设计激励机制
/// - 及时分析运营数据
/// - 保护用户隐私信息
/// - 建立完善的审计流程
pub struct AdminService {
    /// 登录密码管理服务
    pub password: PasswordService,
    /// 数据报表管理服务
    pub data_report: DataReportService,
    /// 勋章管理服务
    pub badge: BadgeService,
    /// 勋章授予名单管理服务
    pub badge_grant: BadgeGrantService,
}

impl AdminService {
    /// 创建新的管理后台服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置，包含认证信息和API设置
    ///
    /// # 返回值
    /// 配置完成的管理后台服务实例
    pub fn new(config: Config) -> Self {
        Self {
            password: PasswordService::new(config.clone()),
            data_report: DataReportService::new(config.clone()),
            badge: BadgeService::new(config.clone()),
            badge_grant: BadgeGrantService::new(config),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_admin_service_creation() {
        let config = Config::default();
        let service = AdminService::new(config.clone());

        assert_eq!(service.password.config.app_id, config.app_id);
        assert_eq!(service.data_report.config.app_id, config.app_id);
        assert_eq!(service.badge.config.app_id, config.app_id);
        assert_eq!(service.badge_grant.config.app_id, config.app_id);
    }

    #[test]
    fn test_admin_service_with_custom_config() {
        let config = Config::builder()
            .app_id("admin_test_app")
            .app_secret("admin_test_secret")
            .req_timeout(Duration::from_secs(120))
            .build();

        let service = AdminService::new(config.clone());

        assert_eq!(service.password.config.app_id, "admin_test_app");
        assert_eq!(service.password.config.app_secret, "admin_test_secret");
        assert_eq!(
            service.password.config.req_timeout,
            Some(Duration::from_secs(120))
        );

        assert_eq!(service.data_report.config.app_id, "admin_test_app");
        assert_eq!(
            service.data_report.config.req_timeout,
            Some(Duration::from_secs(120))
        );

        assert_eq!(service.badge.config.app_id, "admin_test_app");
        assert_eq!(service.badge_grant.config.app_id, "admin_test_app");
    }

    #[test]
    fn test_admin_service_config_independence() {
        let config1 = Config::builder().app_id("admin_app_1").build();

        let config2 = Config::builder().app_id("admin_app_2").build();

        let service1 = AdminService::new(config1);
        let service2 = AdminService::new(config2);

        assert_eq!(service1.password.config.app_id, "admin_app_1");
        assert_eq!(service2.password.config.app_id, "admin_app_2");
        assert_ne!(
            service1.password.config.app_id,
            service2.password.config.app_id
        );

        assert_eq!(service1.data_report.config.app_id, "admin_app_1");
        assert_eq!(service2.data_report.config.app_id, "admin_app_2");
    }

    #[test]
    fn test_admin_service_all_sub_services_accessible() {
        let config = Config::default();
        let service = AdminService::new(config.clone());

        assert_eq!(service.password.config.app_id, config.app_id);
        assert_eq!(service.data_report.config.app_id, config.app_id);
        assert_eq!(service.badge.config.app_id, config.app_id);
        assert_eq!(service.badge_grant.config.app_id, config.app_id);
    }

    #[test]
    fn test_admin_service_config_cloning() {
        let config = Config::builder()
            .app_id("clone_test_app")
            .app_secret("clone_test_secret")
            .build();

        let service = AdminService::new(config.clone());

        let services_configs = [
            &service.password.config,
            &service.data_report.config,
            &service.badge.config,
            &service.badge_grant.config,
        ];

        for service_config in &services_configs {
            assert_eq!(service_config.app_id, "clone_test_app");
            assert_eq!(service_config.app_secret, "clone_test_secret");
        }
    }

    #[test]
    fn test_admin_service_timeout_propagation() {
        let config = Config::builder()
            .req_timeout(Duration::from_secs(180))
            .build();

        let service = AdminService::new(config);

        assert_eq!(
            service.password.config.req_timeout,
            Some(Duration::from_secs(180))
        );
        assert_eq!(
            service.data_report.config.req_timeout,
            Some(Duration::from_secs(180))
        );
        assert_eq!(
            service.badge.config.req_timeout,
            Some(Duration::from_secs(180))
        );
        assert_eq!(
            service.badge_grant.config.req_timeout,
            Some(Duration::from_secs(180))
        );
    }

    #[test]
    fn test_admin_service_multiple_instances() {
        let config = Config::default();

        let service1 = AdminService::new(config.clone());
        let service2 = AdminService::new(config.clone());

        assert_eq!(
            service1.password.config.app_id,
            service2.password.config.app_id
        );
        assert_eq!(
            service1.data_report.config.app_id,
            service2.data_report.config.app_id
        );
        assert_eq!(service1.badge.config.app_id, service2.badge.config.app_id);
        assert_eq!(
            service1.badge_grant.config.app_id,
            service2.badge_grant.config.app_id
        );
    }

    #[test]
    fn test_admin_service_config_consistency() {
        let config = Config::builder()
            .app_id("consistency_test")
            .app_secret("consistency_secret")
            .req_timeout(Duration::from_secs(200))
            .build();

        let service = AdminService::new(config);

        let configs = [
            &service.password.config,
            &service.data_report.config,
            &service.badge.config,
            &service.badge_grant.config,
        ];

        for config in &configs {
            assert_eq!(config.app_id, "consistency_test");
            assert_eq!(config.app_secret, "consistency_secret");
            assert_eq!(config.req_timeout, Some(Duration::from_secs(200)));
        }

        for i in 1..configs.len() {
            assert_eq!(configs[0].app_id, configs[i].app_id);
            assert_eq!(configs[0].app_secret, configs[i].app_secret);
            assert_eq!(configs[0].req_timeout, configs[i].req_timeout);
        }
    }
}
