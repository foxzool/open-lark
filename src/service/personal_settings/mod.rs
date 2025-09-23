//! 个人设置（Personal Settings）服务
//!
//! 提供飞书个人设置的完整功能集，支持系统状态、个人偏好、通知设置、
//! 隐私配置等个人级设置管理能力。是用户个性化体验的核心服务。
//!
//! # 核心功能
//!
//! ## 系统状态管理
//! - 📊 用户在线状态查询和设置
//! - 🔔 状态消息自定义
//! - ⏰ 自动状态切换规则
//! - 📱 多设备状态同步
//! - 🎯 状态可见性控制
//!
//! ## 个人偏好设置
//! - 🎨 界面主题和外观设置
//! - 🌍 语言和地区偏好
//! - ⏰ 时区和时间格式
//! - 📱 移动端个性化配置
//! - 🔧 功能模块启用禁用
//!
//! ## 通知设置
//! - 🔔 消息通知偏好设置
//! - 📧 邮件通知配置
//! - 📱 移动推送设置
//! - ⏰ 免打扰时间段
//! - 🎯 通知分类和优先级
//!
//! ## 隐私配置
//! - 🔐 个人信息可见性
//! - 👥 联系人权限设置
//! - 📊 数据分享偏好
//! - 🛡️ 安全和隐私控制
//! - 🔍 搜索可见性配置
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
//! // 获取个人设置服务
//! let settings = &client.personal_settings;
//!
//! // 查询系统状态
//! // let status_request = GetSystemStatusRequest::builder()
//! //     .user_id("user_123")
//! //     .build();
//! // let status = settings.v1.system_status.get(status_request, None).await?;
//!
//! // 更新系统状态
//! // let update_request = UpdateSystemStatusRequest::builder()
//! //     .user_id("user_123")
//! //     .status("busy")
//! //     .status_text("专注工作中")
//! //     .build();
//! // settings.v1.system_status.update(update_request, None).await?;
//!
//! // 设置免打扰模式
//! // let dnd_request = SetDoNotDisturbRequest::builder()
//! //     .user_id("user_123")
//! //     .start_time("09:00")
//! //     .end_time("18:00")
//! //     .build();
//! // settings.v1.do_not_disturb.set(dnd_request, None).await?;
//! ```
//!
//! # API版本
//!
//! 当前支持v1版本，提供基础的个人设置功能：
//! - 系统状态管理
//! - 个人偏好配置
//! - 通知设置管理
//! - 隐私权限控制
//!
//! # 个人设置特性
//!
//! - 🎨 丰富的个性化选项
//! - 🔐 细粒度隐私控制
//! - 📱 跨平台设置同步
//! - 🤖 智能推荐和建议
//! - ⚡ 实时设置生效
//!
//! # 用户体验
//!
//! - 🎯 精准的个性化推荐
//! - 🔔 智能通知管理
//! - 🌍 本地化用户体验
//! - 📊 使用习惯分析
//! - 🔄 设置备份和恢复

/// 数据模型定义
pub mod models;
/// 个人设置服务 v1 版本
pub mod v1;

use crate::core::config::Config;

/// 个人设置服务
///
/// 个人级设置管理的统一入口，提供系统状态、个人偏好、
/// 通知设置、隐私配置等完整的个人设置管理能力。
///
/// # 服务架构
///
/// - **v1**: 个人设置API v1版本，提供基础功能集
/// - **models**: 数据模型和结构定义
///
/// # 核心特性
///
/// - 📊 全面的状态管理功能
/// - 🎨 丰富的个性化选项
/// - 🔔 智能的通知管理
/// - 🔐 细致的隐私控制
/// - 📱 无缝的跨平台同步
///
/// # 适用场景
///
/// - 用户个性化配置
/// - 工作状态管理
/// - 通知偏好设置
/// - 隐私权限控制
/// - 用户体验优化
///
/// # 最佳实践
///
/// - 合理设置通知偏好
/// - 保护个人隐私信息
/// - 定期更新状态信息
/// - 优化个性化体验
/// - 建立良好使用习惯
pub struct PersonalSettingsService {
    /// v1版本API服务
    pub v1: v1::V1,
}

impl PersonalSettingsService {
    /// 创建新的个人设置服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置，包含认证信息和API设置
    ///
    /// # 返回值
    /// 配置完成的个人设置服务实例
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
    fn test_personal_settings_service_creation() {
        let config = Config::default();
        let service = PersonalSettingsService::new(config.clone());

        assert_eq!(service.v1.system_status.config.app_id, config.app_id);
        assert_eq!(
            service.v1.system_status.config.app_secret,
            config.app_secret
        );
    }

    #[test]
    fn test_personal_settings_service_with_custom_config() {
        let config = Config::builder()
            .app_id("personal_settings_test_app")
            .app_secret("personal_settings_test_secret")
            .req_timeout(Duration::from_secs(140))
            .build();

        let service = PersonalSettingsService::new(config.clone());

        assert_eq!(
            service.v1.system_status.config.app_id,
            "personal_settings_test_app"
        );
        assert_eq!(
            service.v1.system_status.config.app_secret,
            "personal_settings_test_secret"
        );
        assert_eq!(
            service.v1.system_status.config.req_timeout,
            Some(Duration::from_secs(140))
        );
    }

    #[test]
    fn test_personal_settings_service_config_independence() {
        let config1 = Config::builder().app_id("personal_settings_app_1").build();

        let config2 = Config::builder().app_id("personal_settings_app_2").build();

        let service1 = PersonalSettingsService::new(config1);
        let service2 = PersonalSettingsService::new(config2);

        assert_eq!(
            service1.v1.system_status.config.app_id,
            "personal_settings_app_1"
        );
        assert_eq!(
            service2.v1.system_status.config.app_id,
            "personal_settings_app_2"
        );
        assert_ne!(
            service1.v1.system_status.config.app_id,
            service2.v1.system_status.config.app_id
        );
    }

    #[test]
    fn test_personal_settings_service_sub_services_accessible() {
        let config = Config::default();
        let service = PersonalSettingsService::new(config.clone());

        assert_eq!(service.v1.system_status.config.app_id, config.app_id);
    }

    #[test]
    fn test_personal_settings_service_config_cloning() {
        let config = Config::builder()
            .app_id("clone_test_app")
            .app_secret("clone_test_secret")
            .build();

        let service = PersonalSettingsService::new(config.clone());

        assert_eq!(service.v1.system_status.config.app_id, "clone_test_app");
        assert_eq!(
            service.v1.system_status.config.app_secret,
            "clone_test_secret"
        );
    }

    #[test]
    fn test_personal_settings_service_timeout_propagation() {
        let config = Config::builder()
            .req_timeout(Duration::from_secs(160))
            .build();

        let service = PersonalSettingsService::new(config);

        assert_eq!(
            service.v1.system_status.config.req_timeout,
            Some(Duration::from_secs(160))
        );
    }

    #[test]
    fn test_personal_settings_service_multiple_instances() {
        let config = Config::default();

        let service1 = PersonalSettingsService::new(config.clone());
        let service2 = PersonalSettingsService::new(config.clone());

        assert_eq!(
            service1.v1.system_status.config.app_id,
            service2.v1.system_status.config.app_id
        );
        assert_eq!(
            service1.v1.system_status.config.app_secret,
            service2.v1.system_status.config.app_secret
        );
    }

    #[test]
    fn test_personal_settings_service_config_consistency() {
        let config = Config::builder()
            .app_id("consistency_test")
            .app_secret("consistency_secret")
            .req_timeout(Duration::from_secs(130))
            .build();

        let service = PersonalSettingsService::new(config);

        assert_eq!(service.v1.system_status.config.app_id, "consistency_test");
        assert_eq!(
            service.v1.system_status.config.app_secret,
            "consistency_secret"
        );
        assert_eq!(
            service.v1.system_status.config.req_timeout,
            Some(Duration::from_secs(130))
        );
    }
}
