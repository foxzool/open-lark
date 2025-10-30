//! 管理后台（Admin）服务
//!
//! 提供飞书管理后台的核心功能，包括密码管理、数据报表、企业勋章等。
//!
//! # 核心功能
//!
//! - 🔐 企业员工登录密码管理
//! - 📊 企业数据统计和分析
//! - 🏆 企业勋章设计和授予
//! - 👑 超级管理员权限控制

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
use badge::SimpleService as BadgeService;
use badge_grant::SimpleService as BadgeGrantService;
use data_report::SimpleService as DataReportService;
use password::SimpleService as PasswordService;

/// 管理后台服务
///
/// 企业级后台管理的统一入口，提供登录密码管理、数据报表、
/// 企业勋章、授予管理等完整的后台管理能力。
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
    pub fn new(config: Config) -> Self {
        Self {
            password: PasswordService::new(config.clone()),
            data_report: DataReportService::new(config.clone()),
            badge: BadgeService::new(config.clone()),
            badge_grant: BadgeGrantService::new(config),
        }
    }

    /// 验证管理后台配置
    pub fn validate_admin_config(&self) -> bool {
        !self.password.config.app_id.is_empty()
            && !self.password.config.app_secret.is_empty()
    }

    /// 获取管理后台服务统计信息
    pub fn get_admin_service_statistics(&self) -> String {
        format!(
            "AdminService{{ services: 4, app_id: {}, password: true, data_report: true, badge: true, badge_grant: true }}",
            self.password.config.app_id
        )
    }

    /// 健康检查
    pub fn health_check(&self) -> bool {
        self.validate_admin_config()
    }
}

use crate::core::trait_system::Service;

impl Service for AdminService {
    fn config(&self) -> &Config {
        &self.password.config
    }

    fn service_name() -> &'static str
    where
        Self: Sized,
    {
        "AdminService"
    }
}

impl Clone for AdminService {
    fn clone(&self) -> Self {
        Self {
            password: PasswordService::new(self.password.config.clone()),
            data_report: DataReportService::new(self.data_report.config.clone()),
            badge: BadgeService::new(self.badge.config.clone()),
            badge_grant: BadgeGrantService::new(self.badge_grant.config.clone()),
        }
    }
}

impl std::fmt::Debug for AdminService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AdminService")
            .field("service_name", &Self::service_name())
            .field("app_id", &self.password.config.app_id)
            .field("password", &"PasswordService")
            .field("data_report", &"DataReportService")
            .field("badge", &"BadgeService")
            .field("badge_grant", &"BadgeGrantService")
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    /// 创建测试配置
    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_admin_app_id")
            .app_secret("test_admin_app_secret")
            .build()
    }

    #[test]
    fn test_admin_service_creation() {
        let config = create_test_config();
        let service = AdminService::new(config.clone());

        assert_eq!(service.password.config.app_id, config.app_id);
        assert_eq!(service.data_report.config.app_id, config.app_id);
        assert_eq!(service.badge.config.app_id, config.app_id);
    }

    #[test]
    fn test_admin_service_with_custom_config() {
        let config = Config::builder()
            .app_id("custom_admin_app")
            .app_secret("custom_secret")
            .req_timeout(Duration::from_secs(180))
            .build();

        let service = AdminService::new(config);
        assert_eq!(service.password.config.app_id, "custom_admin_app");
    }

    #[test]
    fn test_admin_service_config_independence() {
        let config1 = Config::builder().app_id("admin_app_1").build();
        let config2 = Config::builder().app_id("admin_app_2").build();

        let service1 = AdminService::new(config1);
        let service2 = AdminService::new(config2);

        assert_eq!(service1.password.config.app_id, "admin_app_1");
        assert_eq!(service2.password.config.app_id, "admin_app_2");
        assert_ne!(service1.password.config.app_id, service2.password.config.app_id);
    }

    #[test]
    fn test_admin_service_validate_admin_config() {
        let config = create_test_config();
        let service = AdminService::new(config);

        assert!(service.validate_admin_config());
    }

    #[test]
    fn test_admin_service_get_admin_service_statistics() {
        let config = create_test_config();
        let service = AdminService::new(config);

        let stats = service.get_admin_service_statistics();
        assert!(stats.contains("AdminService"));
        assert!(stats.contains("test_admin_app_id"));
        assert!(stats.contains("services: 4"));
    }

    #[test]
    fn test_admin_service_health_check() {
        let config = create_test_config();
        let service = AdminService::new(config);

        assert!(service.health_check());
    }

    #[test]
    fn test_admin_service_clone() {
        let config = create_test_config();
        let service = AdminService::new(config.clone());
        let cloned_service = service.clone();

        assert_eq!(service.password.config.app_id, cloned_service.password.config.app_id);
    }

    #[test]
    fn test_admin_service_debug_format() {
        let config = create_test_config();
        let service = AdminService::new(config);
        let debug_string = format!("{:?}", service);

        assert!(debug_string.contains("AdminService"));
        assert!(debug_string.contains("test_admin_app_id"));
    }
}