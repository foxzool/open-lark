pub mod admin;
pub mod app_badge;
pub mod app_usage;
pub mod application;
pub mod application_feedback;
pub mod appstore_paid_info;
pub mod scope;

use crate::core::config::Config;

/// Application API v6版本服务
pub struct V6 {
    /// 应用信息管理
    pub application: application::ApplicationService,
    /// 应用权限管理
    pub scope: scope::ScopeService,
    /// 应用管理
    pub admin: admin::AdminService,
    /// 应用商店信息
    pub appstore_paid_info: appstore_paid_info::AppstorePaidInfoService,
    /// 应用使用情况
    pub app_usage: app_usage::AppUsageService,
    /// 应用反馈
    pub application_feedback: application_feedback::ApplicationFeedbackService,
    /// 应用红点
    pub app_badge: app_badge::AppBadgeService,
}

impl V6 {
    pub fn new(config: Config) -> Self {
        Self {
            application: application::ApplicationService::new(config.clone()),
            scope: scope::ScopeService::new(config.clone()),
            admin: admin::AdminService::new(config.clone()),
            appstore_paid_info: appstore_paid_info::AppstorePaidInfoService::new(config.clone()),
            app_usage: app_usage::AppUsageService::new(config.clone()),
            application_feedback: application_feedback::ApplicationFeedbackService::new(
                config.clone(),
            ),
            app_badge: app_badge::AppBadgeService::new(config),
        }
    }
}
