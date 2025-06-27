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

use crate::core::config::Config;

/// Mail API v1版本服务
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
}
