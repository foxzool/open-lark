//! 企业信息（Tenant）服务
//!
//! 提供飞书企业信息管理的完整功能集，支持企业基本信息查询、产品配置、
//! 管理员设置、企业权限等企业级管理能力。是企业管理和配置的核心服务。
//!
//! # 核心功能
//!
//! ## 企业信息管理
//! - 🏢 企业基本信息查询和更新
//! - 🆔 企业ID和域名管理
//! - 📊 企业规模和类型信息
//! - 🌍 企业地理位置和时区
//! - 📞 企业联系方式管理
//!
//! ## 产品配置管理
//! - 📦 企业产品授权和配置
//! - 💰 产品计费和订阅管理
//! - 🎯 功能模块启用和禁用
//! - 📈 产品使用情况统计
//! - 🔄 产品升级和变更
//!
//! ## 管理员权限
//! - 👑 超级管理员设置和管理
//! - 🔐 管理员权限分配和控制
//! - 📋 管理员操作日志记录
//! - 🛡️ 安全策略和访问控制
//! - 👥 管理员角色和职责定义
//!
//! ## 企业设置
//! - ⚙️ 企业级功能设置和配置
//! - 🔒 安全策略和合规设置
//! - 📧 通知和消息推送配置
//! - 🎨 企业品牌和外观定制
//! - 🔗 第三方集成和对接
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
//! // 获取企业信息服务
//! let tenant = &client.tenant;
//!
//! // 查询企业基本信息
//! // let tenant_request = GetTenantInfoRequest::builder()
//! //     .build();
//! // let tenant_info = tenant.v2.tenant.get(tenant_request, None).await?;
//!
//! // 查询产品配置信息
//! // let product_request = GetProductAssignInfoRequest::builder()
//! //     .build();
//! // let product_info = tenant.v2.tenant_product_assign_info.get(product_request, None).await?;
//!
//! // 查询管理员列表
//! // let admin_request = ListAdminRequest::builder()
//! //     .page_size(20)
//! //     .build();
//! // let admins = tenant.v2.admin.list(admin_request, None).await?;
//! ```
//!
//! # API版本
//!
//! 当前支持v2版本，是最新的稳定版本，提供：
//! - 完整的企业信息管理功能
//! - 详细的产品配置管理
//! - 全面的权限和安全控制
//! - 丰富的企业设置选项
//!
//! # 企业管理特性
//!
//! - 🏢 多租户架构支持
//! - 🔐 企业级安全保障
//! - 📊 实时数据同步更新
//! - 🌍 全球化和本地化支持
//! - 🔗 灵活的集成能力
//!
//! # 管理功能
//!
//! - 👑 分级管理权限体系
//! - 📋 详细的操作审计日志
//! - 🛡️ 多层次安全防护
//! - ⚙️ 灵活的配置管理
//! - 📈 企业运营数据分析

/// 数据模型定义
pub mod models;
/// 企业信息服务 v2 版本
pub mod v2;

use crate::core::config::Config;

/// 企业信息服务
///
/// 企业级信息管理的统一入口，提供企业基本信息、产品配置、
/// 管理员权限、企业设置等完整的企业管理能力。
///
/// # 服务架构
///
/// - **v2**: 最新版本API，提供完整的企业管理功能集
/// - **models**: 数据模型和结构定义
///
/// # 核心特性
///
/// - 🏢 全面的企业信息管理
/// - 📦 灵活的产品配置管理
/// - 👑 完善的权限管理体系
/// - ⚙️ 丰富的企业设置选项
/// - 🔐 企业级安全保障
///
/// # 适用场景
///
/// - 企业基础信息管理
/// - 产品授权和配置
/// - 管理员权限分配
/// - 企业安全策略设置
/// - 企业级功能配置
///
/// # 最佳实践
///
/// - 定期更新企业信息
/// - 合理分配管理员权限
/// - 设置合适的安全策略
/// - 监控产品使用情况
/// - 保护企业敏感信息
pub struct TenantService {
    /// v2版本API服务
    pub v2: v2::V2,
}

impl TenantService {
    /// 创建新的企业信息服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置，包含认证信息和API设置
    ///
    /// # 返回值
    /// 配置完成的企业信息服务实例
    pub fn new(config: Config) -> Self {
        Self {
            v2: v2::V2::new(config),
        }
    }

    /// 使用共享配置创建服务（实验性）
    pub fn new_from_shared(shared: std::sync::Arc<Config>) -> Self {
        Self {
            v2: v2::V2::new_from_shared(shared),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_tenant_service_creation() {
        let config = Config::default();
        let service = TenantService::new(config.clone());

        assert_eq!(service.v2.tenant.config.app_id, config.app_id);
        assert_eq!(service.v2.tenant.config.app_secret, config.app_secret);
        assert_eq!(
            service.v2.tenant_product_assign_info.config.app_id,
            config.app_id
        );
        assert_eq!(
            service.v2.tenant_product_assign_info.config.app_secret,
            config.app_secret
        );
    }

    #[test]
    fn test_tenant_service_with_custom_config() {
        let config = Config::builder()
            .app_id("tenant_test_app")
            .app_secret("tenant_test_secret")
            .req_timeout(Duration::from_secs(160))
            .build();

        let service = TenantService::new(config.clone());

        assert_eq!(service.v2.tenant.config.app_id, "tenant_test_app");
        assert_eq!(service.v2.tenant.config.app_secret, "tenant_test_secret");
        assert_eq!(
            service.v2.tenant.config.req_timeout,
            Some(Duration::from_secs(160))
        );
        assert_eq!(
            service.v2.tenant_product_assign_info.config.app_id,
            "tenant_test_app"
        );
        assert_eq!(
            service.v2.tenant_product_assign_info.config.req_timeout,
            Some(Duration::from_secs(160))
        );
    }

    #[test]
    fn test_tenant_service_config_independence() {
        let config1 = Config::builder().app_id("tenant_app_1").build();

        let config2 = Config::builder().app_id("tenant_app_2").build();

        let service1 = TenantService::new(config1);
        let service2 = TenantService::new(config2);

        assert_eq!(service1.v2.tenant.config.app_id, "tenant_app_1");
        assert_eq!(service2.v2.tenant.config.app_id, "tenant_app_2");
        assert_ne!(
            service1.v2.tenant.config.app_id,
            service2.v2.tenant.config.app_id
        );
        assert_ne!(
            service1.v2.tenant_product_assign_info.config.app_id,
            service2.v2.tenant_product_assign_info.config.app_id
        );
    }

    #[test]
    fn test_tenant_service_sub_services_accessible() {
        let config = Config::default();
        let service = TenantService::new(config.clone());

        assert_eq!(service.v2.tenant.config.app_id, config.app_id);
        assert_eq!(
            service.v2.tenant_product_assign_info.config.app_id,
            config.app_id
        );
    }

    #[test]
    fn test_tenant_service_config_cloning() {
        let config = Config::builder()
            .app_id("clone_test_app")
            .app_secret("clone_test_secret")
            .build();

        let service = TenantService::new(config.clone());

        assert_eq!(service.v2.tenant.config.app_id, "clone_test_app");
        assert_eq!(service.v2.tenant.config.app_secret, "clone_test_secret");
        assert_eq!(
            service.v2.tenant_product_assign_info.config.app_secret,
            "clone_test_secret"
        );
        assert_eq!(
            service.v2.tenant_product_assign_info.config.app_id,
            "clone_test_app"
        );
    }

    #[test]
    fn test_tenant_service_timeout_propagation() {
        let config = Config::builder()
            .req_timeout(Duration::from_secs(170))
            .build();

        let service = TenantService::new(config);

        assert_eq!(
            service.v2.tenant.config.req_timeout,
            Some(Duration::from_secs(170))
        );
        assert_eq!(
            service.v2.tenant_product_assign_info.config.req_timeout,
            Some(Duration::from_secs(170))
        );
    }

    #[test]
    fn test_tenant_service_multiple_instances() {
        let config = Config::default();

        let service1 = TenantService::new(config.clone());
        let service2 = TenantService::new(config.clone());

        assert_eq!(
            service1.v2.tenant.config.app_id,
            service2.v2.tenant.config.app_id
        );
        assert_eq!(
            service1.v2.tenant.config.app_secret,
            service2.v2.tenant.config.app_secret
        );
        assert_eq!(
            service1.v2.tenant_product_assign_info.config.app_id,
            service2.v2.tenant_product_assign_info.config.app_id
        );
        assert_eq!(
            service1.v2.tenant_product_assign_info.config.app_secret,
            service2.v2.tenant_product_assign_info.config.app_secret
        );
    }

    #[test]
    fn test_tenant_service_config_consistency() {
        let config = Config::builder()
            .app_id("consistency_test")
            .app_secret("consistency_secret")
            .req_timeout(Duration::from_secs(140))
            .build();

        let service = TenantService::new(config);

        assert_eq!(service.v2.tenant.config.app_id, "consistency_test");
        assert_eq!(service.v2.tenant.config.app_secret, "consistency_secret");
        assert_eq!(
            service.v2.tenant.config.req_timeout,
            Some(Duration::from_secs(140))
        );
        assert_eq!(
            service.v2.tenant_product_assign_info.config.app_id,
            "consistency_test"
        );
        assert_eq!(
            service.v2.tenant_product_assign_info.config.app_secret,
            "consistency_secret"
        );
        assert_eq!(
            service.v2.tenant_product_assign_info.config.req_timeout,
            Some(Duration::from_secs(140))
        );
    }
}
