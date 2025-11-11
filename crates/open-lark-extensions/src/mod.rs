// Extensions 服务模块

pub mod mdm;
pub mod calendar;
pub mod mail;
pub mod helpdesk;
pub mod minutes;
pub mod moments;
pub mod elearning;
pub mod lingo;
pub mod apass;
pub mod acs;
pub mod human_authentication;
pub mod trust_party;
pub mod workplace;
pub mod report;
pub mod verification;

/// Extensions 服务包
///
/// 提供飞书平台的扩展功能服务，包括：
/// - 📱 **移动设备管理**: 企业设备管理和安全策略
/// - 📅 **日历服务**: 会议安排和日程管理
/// - 📧 **邮件服务**: 企业邮件和通信管理
/// - 🛟 **帮助台**: 客户支持和工单系统
/// - 📝 **会议纪要**: 会议记录和文档管理
/// - 📸 **动态时刻**: 企业社交和动态分享
///
/// 为企业提供丰富的扩展功能，支持各种业务场景和集成需求。

#[derive(Debug)]
pub struct ExtensionsService {
    config: openlark_core::config::Config,
}

impl ExtensionsService {
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self { config }
    }
}

impl openlark_core::trait_system::Service for ExtensionsService {
    fn config(&self) -> &openlark_core::config::Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "extensions"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}
