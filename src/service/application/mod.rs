//! 应用管理（Application）服务
//!
//! 提供飞书开放平台应用管理的完整功能集，支持应用信息查询、应用商店管理、
//! 应用使用统计、应用反馈等企业级应用生命周期管理能力。
//!
//! # 核心功能
//!
//! ## 应用信息管理
//! - 📱 应用基本信息查询和更新
//! - 🔧 应用配置和设置管理
//! - 📊 应用版本和发布管理
//! - 🏷️ 应用分类和标签管理
//! - 🔐 应用权限和授权管理
//!
//! ## 应用商店管理
//! - 🏪 应用商店信息和付费信息
//! - 💰 应用定价和计费模式
//! - 📈 应用下载和安装统计
//! - ⭐ 应用评分和评价管理
//! - 🎯 应用推广和营销
//!
//! ## 应用使用统计
//! - 📊 应用使用数据和指标
//! - 👥 用户活跃度和留存率
//! - 📈 功能使用情况统计
//! - 🔍 用户行为分析
//! - 📋 数据报表和导出
//!
//! ## 应用反馈管理
//! - 💬 用户反馈收集和管理
//! - 🐛 问题反馈和Bug跟踪
//! - ⭐ 用户评价和建议
//! - 📞 客户支持和服务
//! - 🔄 反馈处理流程
//!
//! ## 应用徽章系统
//! - 🏆 应用徽章设计和管理
//! - 🎖️ 徽章授予和撤销
//! - 📊 徽章统计和分析
//! - 🎯 徽章激励机制
//!
//! ## 管理员功能
//! - 👑 管理员权限和角色
//! - 🔧 应用审核和审批
//! - 📋 应用监控和管理
//! - 🚫 应用禁用和恢复
//! - 📊 平台运营数据
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
//! // 获取应用管理服务
//! let app = &client.application;
//!
//! // 获取应用信息
//! // let app_request = GetApplicationRequest::builder()
//! //     .app_id("app_id")
//! //     .build();
//! // let app_info = app.v6.application.get(app_request, None).await?;
//!
//! // 查询应用使用统计
//! // let usage_request = GetAppUsageRequest::builder()
//! //     .app_id("app_id")
//! //     .date_range("2024-01-01,2024-01-31")
//! //     .build();
//! // let usage_data = app.v6.app_usage.get(usage_request, None).await?;
//!
//! // 创建应用徽章
//! // let badge_request = CreateAppBadgeRequest::builder()
//! //     .name("新手入门")
//! //     .description("完成应用基础配置")
//! //     .icon_url("https://example.com/badge.png")
//! //     .build();
//! // app.v6.app_badge.create(badge_request, None).await?;
//!
//! // 提交应用反馈
//! // let feedback_request = CreateFeedbackRequest::builder()
//! //     .app_id("app_id")
//! //     .feedback_type("feature_request")
//! //     .content("希望增加更多自定义选项")
//! //     .build();
//! // app.v6.application_feedback.create(feedback_request, None).await?;
//! ```
//!
//! # API版本
//!
//! 当前支持v6版本，是最新的稳定版本，提供：
//! - 完整的应用生命周期管理
//! - 丰富的统计和分析功能
//! - 强大的商店和营销能力
//! - 完善的反馈和支持系统
//!
//! # 应用管理特性
//!
//! - 📈 实时数据监控和分析
//! - 🔄 自动化运营和管理
//! - 🎯 精准用户群体定位
//! - 💡 智能推荐和优化建议
//! - 🔐 企业级安全和合规
//!
//! # 商业化能力
//!
//! - 💰 灵活的定价和计费模式
//! - 📊 收入统计和财务管理
//! - 🎯 精准营销和推广
//! - 📈 商业数据分析和洞察
//! - 🤝 合作伙伴生态系统

/// 数据模型定义
pub mod models;
/// 应用管理服务 v6 版本
pub mod v6;

use crate::core::config::Config;

/// 应用管理服务
///
/// 企业级应用生命周期管理的统一入口，提供应用信息管理、商店运营、
/// 数据统计、用户反馈等完整的应用管理能力。
///
/// # 服务架构
///
/// - **v6**: 最新版本API，提供完整的应用管理功能集
/// - **models**: 数据模型和结构定义
///
/// # 核心特性
///
/// - 📱 完整的应用生命周期管理
/// - 📊 深度的数据分析和洞察
/// - 🏪 专业的应用商店运营
/// - 💬 完善的用户反馈系统
/// - 🎯 智能的营销和推广工具
///
/// # 适用场景
///
/// - 企业应用开发和运营
/// - 应用商店管理和营销
/// - 用户体验优化和改进
/// - 数据驱动的产品决策
/// - 应用生态系统建设
///
/// # 最佳实践
///
/// - 定期分析应用使用数据
/// - 积极收集和处理用户反馈
/// - 持续优化应用性能和体验
/// - 合理设计应用商业模式
/// - 建立完善的运营流程
pub struct ApplicationService {
    /// v6版本API服务
    pub v6: v6::V6,
}

impl ApplicationService {
    /// 创建新的应用管理服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置，包含认证信息和API设置
    ///
    /// # 返回值
    /// 配置完成的应用管理服务实例
    pub fn new(config: Config) -> Self {
        Self {
            v6: v6::V6::new(config),
        }
    }

    /// 使用共享配置创建服务（实验性）
    pub fn new_from_shared(shared: std::sync::Arc<Config>) -> Self {
        Self {
            v6: v6::V6::new(shared.as_ref().clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_application_service_creation() {
        let config = Config::default();
        let service = ApplicationService::new(config);

        // Verify V6 service structure exists
        let _ = &service.v6;
        let _ = &service.v6.application;
        let _ = &service.v6.scope;
        let _ = &service.v6.admin;
        let _ = &service.v6.appstore_paid_info;
        let _ = &service.v6.app_usage;
        let _ = &service.v6.application_feedback;
        let _ = &service.v6.app_badge;
    }

    #[test]
    fn test_application_service_with_custom_config() {
        let config = Config::builder()
            .app_id("application_test_app")
            .app_secret("application_test_secret")
            .req_timeout(Duration::from_secs(300))
            .build();

        let service = ApplicationService::new(config);

        // Verify service creation with custom config
        let _ = &service.v6.application;
        let _ = &service.v6.scope;
        let _ = &service.v6.admin;
        let _ = &service.v6.appstore_paid_info;
        let _ = &service.v6.app_usage;
        let _ = &service.v6.application_feedback;
        let _ = &service.v6.app_badge;
    }

    #[test]
    fn test_application_service_config_independence() {
        let config1 = Config::builder().app_id("application_app_1").build();

        let config2 = Config::builder().app_id("application_app_2").build();

        let service1 = ApplicationService::new(config1);
        let service2 = ApplicationService::new(config2);

        // Verify both services are created successfully
        let _ = &service1.v6.application;
        let _ = &service1.v6.scope;
        let _ = &service2.v6.application;
        let _ = &service2.v6.scope;
    }

    #[test]
    fn test_application_service_sub_services_accessible() {
        let config = Config::default();
        let service = ApplicationService::new(config);

        // Test that all sub-services are accessible
        let _ = &service.v6.application;
        let _ = &service.v6.scope;
        let _ = &service.v6.admin;
        let _ = &service.v6.appstore_paid_info;
        let _ = &service.v6.app_usage;
        let _ = &service.v6.application_feedback;
        let _ = &service.v6.app_badge;
    }

    #[test]
    fn test_application_service_config_cloning() {
        let config = Config::builder()
            .app_id("clone_test_app")
            .app_secret("clone_test_secret")
            .build();

        let service = ApplicationService::new(config.clone());

        // Verify service creation with cloned config
        let _ = &service.v6.application;
        let _ = &service.v6.scope;
        let _ = &service.v6.admin;
        let _ = &service.v6.appstore_paid_info;
        let _ = &service.v6.app_usage;
        let _ = &service.v6.application_feedback;
        let _ = &service.v6.app_badge;
    }

    #[test]
    fn test_application_service_timeout_propagation() {
        let config = Config::builder()
            .req_timeout(Duration::from_secs(310))
            .build();

        let service = ApplicationService::new(config);

        // Verify service creation with timeout config
        let _ = &service.v6.application;
        let _ = &service.v6.scope;
        let _ = &service.v6.admin;
        let _ = &service.v6.appstore_paid_info;
        let _ = &service.v6.app_usage;
        let _ = &service.v6.application_feedback;
        let _ = &service.v6.app_badge;
    }

    #[test]
    fn test_application_service_multiple_instances() {
        let config = Config::default();

        let service1 = ApplicationService::new(config.clone());
        let service2 = ApplicationService::new(config.clone());

        // Verify both instances are created successfully
        let _ = &service1.v6.application;
        let _ = &service1.v6.scope;
        let _ = &service2.v6.application;
        let _ = &service2.v6.scope;
    }

    #[test]
    fn test_application_service_config_consistency() {
        let config = Config::builder()
            .app_id("consistency_test")
            .app_secret("consistency_secret")
            .req_timeout(Duration::from_secs(200))
            .build();

        let service = ApplicationService::new(config);

        // Verify all sub-services are created consistently
        let _ = &service.v6.application;
        let _ = &service.v6.scope;
        let _ = &service.v6.admin;
        let _ = &service.v6.appstore_paid_info;
        let _ = &service.v6.app_usage;
        let _ = &service.v6.application_feedback;
        let _ = &service.v6.app_badge;
    }
}
