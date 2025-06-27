pub mod badge;
pub mod badge_grant;
pub mod data_report;
pub mod models;
pub mod password;

use crate::core::config::Config;
use badge::BadgeService;
use badge_grant::BadgeGrantService;
use data_report::DataReportService;
use password::PasswordService;

/// 管理后台服务
///
/// 提供飞书管理后台相关的API功能，包括：
/// - 登录密码管理
/// - 数据报表管理  
/// - 企业勋章管理
/// - 勋章授予名单管理
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
    pub fn new(config: Config) -> Self {
        Self {
            password: PasswordService::new(config.clone()),
            data_report: DataReportService::new(config.clone()),
            badge: BadgeService::new(config.clone()),
            badge_grant: BadgeGrantService::new(config),
        }
    }
}
