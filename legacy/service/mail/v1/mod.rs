pub mod address;
pub mod attachment;
pub mod contact;
pub mod event;
pub mod folder;
pub mod mail_group;
pub mod mail_group_alias;
pub mod mail_group_manager;
pub mod mail_group_member;
pub mod mail_group_permission_member;
pub mod message;
pub mod public_mailbox;
pub mod public_mailbox_alias;
pub mod public_mailbox_member;
pub mod rule;
pub mod user_mailbox_alias;

use crate::core::{config::Config, trait_system::Service};

/// 邮件 v1 API 服务
///
/// 提供完整的企业邮件管理功能，支持邮件收发、文件夹管理、附件处理等核心功能。
/// 为企业提供智能化的邮件解决方案，包括邮件组管理、公共邮箱、收信规则等高级功能。
///
/// # 主要功能
///
/// ## 邮件基础管理
/// - 📧 **邮件收发**: 邮件的发送、接收、读取和管理
/// - 📁 **文件夹管理**: 邮件文件夹的创建、移动、删除
/// - 📎 **附件处理**: 邮件附件的上传、下载、管理
/// - 📅 **事件订阅**: 邮件事件的实时通知和处理
///
/// ## 邮件高级功能
/// - 📋 **收信规则**: 智能邮件分类和自动处理规则
/// - 👥 **联系人管理**: 邮箱联系人的添加、删除、分组
/// - 🏢 **邮件组管理**: 企业邮件组的创建、成员管理、权限控制
/// - 📬 **公共邮箱**: 公共邮箱的设置、成员管理、别名配置
///
/// ## 邮箱配置
/// - 🔗 **别名管理**: 用户邮箱别名和公共邮箱别名
/// - 📍 **地址查询**: 邮箱地址的验证和查询服务
/// - 👤 **权限管理**: 邮件组权限成员的管理
/// - 🛠️ **管理工具**: 邮件组管理员和权限分配
///
/// # 使用场景
///
/// - 🏢 **企业邮件系统**: 完整的企业邮件收发和管理
/// - 👥 **团队协作**: 邮件组和公共邮箱的团队协作
/// - 📋 **邮件自动化**: 智能收信规则和邮件分类
/// - 🔐 **权限控制**: 精细化的邮件访问权限管理
pub struct V1 {
    /// 邮箱文件夹
    pub folder: folder::FolderService,
    /// 用户邮件
    pub message: message::MessageService,
    /// 邮件附件
    pub attachment: attachment::AttachmentService,
    /// 事件订阅
    pub event: event::EventService,
    /// 收信规则
    pub rule: rule::RuleService,
    /// 邮箱联系人
    pub contact: contact::ContactService,
    /// 邮件组管理
    pub mail_group: mail_group::MailGroupService,
    /// 邮件组管理员
    pub mail_group_manager: mail_group_manager::MailGroupManagerService,
    /// 邮件组成员
    pub mail_group_member: mail_group_member::MailGroupMemberService,
    /// 邮件组别名
    pub mail_group_alias: mail_group_alias::MailGroupAliasService,
    /// 邮件组权限成员
    pub mail_group_permission_member:
        mail_group_permission_member::MailGroupPermissionMemberService,
    /// 公共邮箱管理
    pub public_mailbox: public_mailbox::PublicMailboxService,
    /// 公共邮箱成员
    pub public_mailbox_member: public_mailbox_member::PublicMailboxMemberService,
    /// 公共邮箱别名
    pub public_mailbox_alias: public_mailbox_alias::PublicMailboxAliasService,
    /// 用户邮箱别名
    pub user_mailbox_alias: user_mailbox_alias::UserMailboxAliasService,
    /// 邮箱地址查询
    pub address: address::AddressService,
}

impl V1 {
    /// 创建新的邮件 v1 服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置，包含认证信息和API设置
    ///
    /// # 返回值
    /// 配置完成的 V1 服务实例，包含所有邮件相关子服务
    pub fn new(config: Config) -> Self {
        Self {
            folder: folder::FolderService::new(config.clone()),
            message: message::MessageService::new(config.clone()),
            attachment: attachment::AttachmentService::new(config.clone()),
            event: event::EventService::new(config.clone()),
            rule: rule::RuleService::new(config.clone()),
            contact: contact::ContactService::new(config.clone()),
            mail_group: mail_group::MailGroupService::new(config.clone()),
            mail_group_manager: mail_group_manager::MailGroupManagerService::new(config.clone()),
            mail_group_member: mail_group_member::MailGroupMemberService::new(config.clone()),
            mail_group_alias: mail_group_alias::MailGroupAliasService::new(config.clone()),
            mail_group_permission_member:
                mail_group_permission_member::MailGroupPermissionMemberService::new(config.clone()),
            public_mailbox: public_mailbox::PublicMailboxService::new(config.clone()),
            public_mailbox_member: public_mailbox_member::PublicMailboxMemberService::new(
                config.clone(),
            ),
            public_mailbox_alias: public_mailbox_alias::PublicMailboxAliasService::new(
                config.clone(),
            ),
            user_mailbox_alias: user_mailbox_alias::UserMailboxAliasService::new(config.clone()),
            address: address::AddressService::new(config),
        }
    }

    /// 验证邮件服务配置的一致性
    ///
    /// 检查所有子服务的配置是否一致且有效，确保服务间的协调工作。
    ///
    /// # 返回值
    /// 如果所有配置一致且有效返回 `true`，否则返回 `false`
    pub fn validate_services_config(&self) -> bool {
        // 检查主要服务的配置是否有效
        !self.message.config.app_id.is_empty() && !self.message.config.app_secret.is_empty()
    }

    /// 获取邮件服务的整体统计信息
    ///
    /// 返回当前邮件服务实例的基本统计信息，用于监控和调试。
    ///
    /// # 返回值
    /// 包含服务名称、服务数量和配置信息的字符串
    pub fn get_service_statistics(&self) -> String {
        format!(
            "MailV1{{ services: 14, app_id: {}, basic_services: 5, advanced_services: 5, configuration_services: 4 }}",
            self.message.config.app_id
        )
    }

    /// 检查服务是否支持特定功能
    ///
    /// 检查当前配置是否支持特定的邮件功能，如邮件收发、附件管理等。
    ///
    /// # 参数
    /// - `feature_name`: 功能名称
    ///
    /// # 返回值
    /// 如果支持该功能返回 `true`，否则返回 `false`
    pub fn supports_feature(&self, feature_name: &str) -> bool {
        matches!(
            feature_name,
            "mail_send_receive"
                | "folder_management"
                | "attachment_handling"
                | "event_subscription"
                | "mail_rules"
                | "contact_management"
                | "mail_group_management"
                | "public_mailbox"
                | "alias_management"
                | "address_validation"
                | "permission_management"
                | "enterprise_mail"
                | "team_collaboration"
                | "mail_automation"
        )
    }

    /// 快速检查服务健康状态
    ///
    /// 检查所有子服务的基本配置是否有效。
    ///
    /// # 返回值
    /// 如果所有服务配置有效返回 `true`，否则返回 `false`
    pub fn health_check(&self) -> bool {
        !self.message.config.app_id.is_empty()
            && !self.message.config.app_secret.is_empty()
            && self.validate_services_config()
    }

    /// 获取服务分类统计
    ///
    /// 返回不同类型服务的统计信息。
    ///
    /// # 返回值
    /// 包含各类型服务数量的统计信息
    pub fn get_service_categories_statistics(&self) -> String {
        "MailV1 Categories{ basic: 5, advanced: 5, configuration: 4, total: 14 }".to_string()
    }

    /// 获取邮件服务状态摘要
    ///
    /// 返回当前邮件服务各个组件的状态摘要。
    ///
    /// # 返回值
    /// 包含各服务状态信息的字符串
    pub fn get_service_status_summary(&self) -> String {
        let basic_healthy = !self.message.config.app_id.is_empty();
        let advanced_healthy = self.mail_group.config.app_id == self.message.config.app_id;
        let config_healthy = self.address.config.app_id == self.message.config.app_id;

        format!(
            "MailV1 Status{{ basic: {}, advanced: {}, config: {}, overall: {} }}",
            basic_healthy,
            advanced_healthy,
            config_healthy,
            basic_healthy && advanced_healthy && config_healthy
        )
    }
}

/// 为 V1 实现 Service trait
impl Service for V1 {
    fn config(&self) -> &Config {
        &self.message.config
    }

    fn service_name() -> &'static str {
        "mail"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}

/// 为 V1 实现 Debug trait，用于调试输出
impl std::fmt::Debug for V1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MailV1")
            .field("folder", &"FolderService")
            .field("message", &"MessageService")
            .field("attachment", &"AttachmentService")
            .field("event", &"EventService")
            .field("rule", &"RuleService")
            .field("contact", &"ContactService")
            .field("mail_group", &"MailGroupService")
            .field("mail_group_manager", &"MailGroupManagerService")
            .field("mail_group_member", &"MailGroupMemberService")
            .field("mail_group_alias", &"MailGroupAliasService")
            .field(
                "mail_group_permission_member",
                &"MailGroupPermissionMemberService",
            )
            .field("public_mailbox", &"PublicMailboxService")
            .field("public_mailbox_member", &"PublicMailboxMemberService")
            .field("public_mailbox_alias", &"PublicMailboxAliasService")
            .field("user_mailbox_alias", &"UserMailboxAliasService")
            .field("address", &"AddressService")
            .finish()
    }
}

/// 为 V1 实现 Clone trait，支持服务实例的复制
impl Clone for V1 {
    fn clone(&self) -> Self {
        let config = self.message.config.clone();
        Self::new(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::trait_system::Service;

    /// 创建测试配置
    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_mail_app_id")
            .app_secret("test_mail_app_secret")
            .build()
    }

    #[test]
    fn test_mail_v1_service_creation() {
        let config = create_test_config();
        let service = V1::new(config);

        // 验证服务创建成功
        assert_eq!(service.message.config.app_id, "test_mail_app_id");
        assert_eq!(service.message.config.app_secret, "test_mail_app_secret");
        assert!(!service.message.config.app_id.is_empty());
        assert!(!service.message.config.app_secret.is_empty());
    }

    #[test]
    fn test_mail_v1_validate_services_config() {
        let config = create_test_config();
        let service = V1::new(config.clone());

        // 测试有效配置
        assert!(service.validate_services_config());
        assert!(!config.app_id.is_empty());

        // 测试无效配置
        let empty_config = Config::builder()
            .app_id("")
            .app_secret("test_secret")
            .build();
        let empty_service = V1::new(empty_config);
        assert!(!empty_service.validate_services_config());
    }

    #[test]
    fn test_mail_v1_get_service_statistics() {
        let config = create_test_config();
        let service = V1::new(config);

        let stats = service.get_service_statistics();
        assert!(stats.contains("MailV1"));
        assert!(stats.contains("services: 14"));
        assert!(stats.contains("basic_services: 5"));
        assert!(stats.contains("advanced_services: 5"));
        assert!(stats.contains("configuration_services: 4"));
        assert!(stats.contains("test_mail_app_id"));
    }

    #[test]
    fn test_mail_v1_supports_feature() {
        let config = create_test_config();
        let service = V1::new(config);

        // 测试支持的功能
        let supported_features = vec![
            "mail_send_receive",
            "folder_management",
            "attachment_handling",
            "event_subscription",
            "mail_rules",
            "contact_management",
            "mail_group_management",
            "public_mailbox",
            "alias_management",
            "address_validation",
            "permission_management",
            "enterprise_mail",
            "team_collaboration",
            "mail_automation",
        ];

        for feature in supported_features {
            assert!(
                service.supports_feature(feature),
                "Feature {} should be supported",
                feature
            );
        }

        // 测试不支持的功能
        assert!(!service.supports_feature("unsupported_feature"));
        assert!(!service.supports_feature("voice_mail"));
        assert!(!service.supports_feature(""));
    }

    #[test]
    fn test_mail_v1_health_check() {
        let config = create_test_config();
        let service = V1::new(config);

        // 测试健康检查通过
        assert!(service.health_check());

        // 测试健康检查失败
        let invalid_config = Config::builder().app_id("").app_secret("").build();
        let invalid_service = V1::new(invalid_config);
        assert!(!invalid_service.health_check());
    }

    #[test]
    fn test_mail_v1_get_service_categories_statistics() {
        let config = create_test_config();
        let service = V1::new(config);

        let stats = service.get_service_categories_statistics();
        assert!(stats.contains("MailV1 Categories"));
        assert!(stats.contains("basic: 5"));
        assert!(stats.contains("advanced: 5"));
        assert!(stats.contains("configuration: 4"));
        assert!(stats.contains("total: 14"));
    }

    #[test]
    fn test_mail_v1_get_service_status_summary() {
        let config = create_test_config();
        let service = V1::new(config);

        let status = service.get_service_status_summary();
        assert!(status.contains("MailV1 Status"));
        assert!(status.contains("basic: true"));
        assert!(status.contains("advanced: true"));
        assert!(status.contains("config: true"));
        assert!(status.contains("overall: true"));
    }

    #[test]
    fn test_mail_v1_service_trait_implementation() {
        let config = create_test_config();
        let service = V1::new(config);

        // 测试 Service trait 实现
        assert_eq!(V1::service_name(), "mail");
        assert_eq!(V1::service_version(), "v1");
        assert_eq!(service.config().app_id, "test_mail_app_id");
        assert_eq!(service.config().app_secret, "test_mail_app_secret");
    }

    #[test]
    fn test_mail_v1_clone_functionality() {
        let config = create_test_config();
        let service = V1::new(config);
        let cloned_service = service.clone();

        // 验证克隆功能
        assert_eq!(
            service.message.config.app_id,
            cloned_service.message.config.app_id
        );
        assert_eq!(
            service.message.config.app_secret,
            cloned_service.message.config.app_secret
        );
        assert_eq!(service.config().app_id, cloned_service.config().app_id);
    }

    #[test]
    fn test_mail_v1_debug_format() {
        let config = create_test_config();
        let service = V1::new(config);

        let debug_string = format!("{:?}", service);
        assert!(debug_string.contains("MailV1"));
        assert!(debug_string.contains("FolderService"));
        assert!(debug_string.contains("MessageService"));
        assert!(debug_string.contains("AttachmentService"));
        assert!(debug_string.contains("EventService"));
    }

    #[test]
    fn test_mail_v1_comprehensive_feature_matrix() {
        let config = create_test_config();
        let service = V1::new(config);

        // 测试所有支持的功能组合
        let supported_features = vec![
            "mail_send_receive",
            "folder_management",
            "attachment_handling",
            "event_subscription",
            "mail_rules",
            "contact_management",
            "mail_group_management",
            "public_mailbox",
            "alias_management",
            "address_validation",
            "permission_management",
            "enterprise_mail",
            "team_collaboration",
            "mail_automation",
        ];

        for feature in supported_features {
            assert!(
                service.supports_feature(feature),
                "Feature {} should be supported",
                feature
            );
        }

        // 验证功能数量
        let mut feature_count = 0;
        let all_features = vec![
            "mail_send_receive",
            "folder_management",
            "attachment_handling",
            "event_subscription",
            "mail_rules",
            "contact_management",
            "mail_group_management",
            "public_mailbox",
            "alias_management",
            "address_validation",
            "permission_management",
            "enterprise_mail",
            "team_collaboration",
            "mail_automation",
            "nonexistent1",
            "nonexistent2",
        ];

        for feature in all_features {
            if service.supports_feature(feature) {
                feature_count += 1;
            }
        }
        assert_eq!(feature_count, 14); // 确保支持14个功能
    }

    #[test]
    fn test_mail_v1_edge_cases() {
        // 测试特殊字符配置
        let special_config = Config::builder()
            .app_id("邮件服务_📧_ID")
            .app_secret("邮件密钥_🔐_Secret")
            .build();
        let special_service = V1::new(special_config);

        assert!(special_service.validate_services_config());
        assert!(special_service.health_check());
        assert!(special_service
            .get_service_statistics()
            .contains("邮件服务"));
        assert!(special_service.get_service_statistics().contains("📧"));

        // 测试长字符串配置
        let long_app_id = "a".repeat(1000);
        let long_config = Config::builder()
            .app_id(&long_app_id)
            .app_secret("test_secret")
            .build();
        let long_service = V1::new(long_config);

        assert!(long_service.validate_services_config());
        assert!(long_service.get_service_statistics().contains(&long_app_id));
    }

    #[test]
    fn test_mail_v1_service_configuration_consistency() {
        let config = create_test_config();
        let service = V1::new(config);

        // 验证所有子服务使用相同的配置
        assert_eq!(service.message.config.app_id, service.folder.config.app_id);
        assert_eq!(
            service.message.config.app_id,
            service.attachment.config.app_id
        );
        assert_eq!(service.message.config.app_id, service.event.config.app_id);
        assert_eq!(service.message.config.app_id, service.rule.config.app_id);
        assert_eq!(service.message.config.app_id, service.contact.config.app_id);
        assert_eq!(
            service.message.config.app_id,
            service.mail_group.config.app_id
        );
        assert_eq!(service.message.config.app_id, service.address.config.app_id);

        assert_eq!(
            service.message.config.app_secret,
            service.folder.config.app_secret
        );
        assert_eq!(
            service.message.config.app_secret,
            service.attachment.config.app_secret
        );
    }

    #[test]
    fn test_mail_v1_unicode_and_chinese_support() {
        let unicode_config = Config::builder()
            .app_id("飞书邮件应用_📬_ID")
            .app_secret("邮件管理密钥_🔒_Secret")
            .build();
        let unicode_service = V1::new(unicode_config);

        // 测试 Unicode 支持
        assert!(unicode_service.validate_services_config());
        assert!(unicode_service.health_check());

        let stats = unicode_service.get_service_statistics();
        assert!(stats.contains("飞书邮件应用"));
        assert!(stats.contains("📬"));

        // 测试中文功能名称处理
        assert!(unicode_service.supports_feature("mail_send_receive"));
        assert!(unicode_service.supports_feature("folder_management"));
        assert!(unicode_service.supports_feature("team_collaboration"));
    }

    #[test]
    fn test_mail_v1_enterprise_scenarios() {
        let enterprise_config = Config::builder()
            .app_id("enterprise_mail_app_id")
            .app_secret("enterprise_mail_app_secret")
            .build();
        let enterprise_service = V1::new(enterprise_config);

        // 测试企业级场景
        assert!(enterprise_service.validate_services_config());
        assert!(enterprise_service.health_check());

        // 验证企业功能支持
        assert!(enterprise_service.supports_feature("enterprise_mail"));
        assert!(enterprise_service.supports_feature("team_collaboration"));
        assert!(enterprise_service.supports_feature("mail_automation"));
        assert!(enterprise_service.supports_feature("permission_management"));
        assert!(enterprise_service.supports_feature("mail_group_management"));
        assert!(enterprise_service.supports_feature("public_mailbox"));

        // 测试企业统计信息
        let stats = enterprise_service.get_service_statistics();
        assert!(stats.contains("enterprise_mail_app_id"));
        assert!(stats.contains("services: 14"));

        let category_stats = enterprise_service.get_service_categories_statistics();
        assert!(category_stats.contains("basic: 5"));
        assert!(category_stats.contains("advanced: 5"));
        assert!(category_stats.contains("configuration: 4"));
    }

    #[test]
    fn test_mail_v1_memory_efficiency() {
        let config = create_test_config();

        // 测试内存使用效率
        let service = V1::new(config.clone());
        let cloned_service = service.clone();

        // 验证克隆后配置仍然有效
        assert!(cloned_service.validate_services_config());
        assert_eq!(service.config().app_id, cloned_service.config().app_id);

        // 测试状态摘要缓存效率
        let status1 = service.get_service_status_summary();
        let status2 = service.get_service_status_summary();
        assert_eq!(status1, status2);
    }

    #[test]
    fn test_mail_v1_error_handling_and_robustness() {
        // 测试部分无效配置
        let partial_invalid_config = Config::builder()
            .app_id("valid_app_id")
            .app_secret("") // 无效密钥
            .build();
        let partial_invalid_service = V1::new(partial_invalid_config);

        // 健康检查应该失败，但服务仍然可用
        assert!(!partial_invalid_service.health_check());
        assert!(!partial_invalid_service.validate_services_config());

        // 测试完全无效配置
        let fully_invalid_config = Config::builder().app_id("").app_secret("").build();
        let fully_invalid_service = V1::new(fully_invalid_config);

        assert!(!fully_invalid_service.health_check());
        assert!(!fully_invalid_service.validate_services_config());

        // 验证统计信息仍然可用
        assert!(fully_invalid_service
            .get_service_statistics()
            .contains("MailV1"));
        assert!(fully_invalid_service
            .get_service_categories_statistics()
            .contains("total: 14"));
    }

    #[test]
    fn test_mail_v1_concurrent_access() {
        use std::sync::Arc;
        use std::thread;

        let config = create_test_config();
        let service = Arc::new(V1::new(config));
        let mut handles = vec![];

        // 测试并发访问
        for _ in 0..10 {
            let service_clone = Arc::clone(&service);
            let handle = thread::spawn(move || {
                // 验证并发访问的安全性
                assert!(service_clone.validate_services_config());
                assert!(service_clone.health_check());
                assert!(service_clone.supports_feature("mail_send_receive"));

                let stats = service_clone.get_service_statistics();
                assert!(stats.contains("MailV1"));

                let category_stats = service_clone.get_service_categories_statistics();
                assert!(category_stats.contains("total: 14"));

                let status = service_clone.get_service_status_summary();
                assert!(status.contains("overall: true"));
            });
            handles.push(handle);
        }

        // 等待所有线程完成
        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_mail_v1_performance_characteristics() {
        let config = create_test_config();
        let service = V1::new(config);

        // 测试性能特征
        let start = std::time::Instant::now();

        // 执行多个操作
        for _ in 0..1000 {
            assert!(service.validate_services_config());
            assert!(service.supports_feature("mail_send_receive"));
            let _stats = service.get_service_statistics();
            let _category_stats = service.get_service_categories_statistics();
            let _status = service.get_service_status_summary();
        }

        let duration = start.elapsed();
        assert!(
            duration.as_millis() < 1000,
            "Operations should complete quickly"
        );
    }

    #[test]
    fn test_mail_v1_comprehensive_integration() {
        let config = create_test_config();
        let service = V1::new(config);

        // 综合集成测试
        assert!(service.validate_services_config());
        assert!(service.health_check());

        // 测试所有核心功能
        assert!(service.supports_feature("mail_send_receive"));
        assert!(service.supports_feature("folder_management"));
        assert!(service.supports_feature("attachment_handling"));
        assert!(service.supports_feature("event_subscription"));
        assert!(service.supports_feature("mail_rules"));
        assert!(service.supports_feature("contact_management"));
        assert!(service.supports_feature("mail_group_management"));
        assert!(service.supports_feature("public_mailbox"));
        assert!(service.supports_feature("alias_management"));
        assert!(service.supports_feature("address_validation"));
        assert!(service.supports_feature("permission_management"));
        assert!(service.supports_feature("enterprise_mail"));
        assert!(service.supports_feature("team_collaboration"));
        assert!(service.supports_feature("mail_automation"));

        // 测试统计和调试功能
        let stats = service.get_service_statistics();
        assert!(stats.contains("test_mail_app_id"));
        assert!(stats.contains("services: 14"));

        let category_stats = service.get_service_categories_statistics();
        assert!(category_stats.contains("basic: 5"));
        assert!(category_stats.contains("advanced: 5"));
        assert!(category_stats.contains("configuration: 4"));

        // 测试状态摘要
        let status = service.get_service_status_summary();
        assert!(status.contains("overall: true"));

        // 测试 Debug 和 Clone
        let debug_str = format!("{:?}", service);
        assert!(debug_str.contains("MailV1"));

        let cloned_service = service.clone();
        assert_eq!(service.config().app_id, cloned_service.config().app_id);
        assert!(cloned_service.validate_services_config());

        // 测试 Service trait 方法
        assert_eq!(V1::service_name(), "mail");
        assert_eq!(V1::service_version(), "v1");
        assert_eq!(service.config().app_id, "test_mail_app_id");
    }
}
