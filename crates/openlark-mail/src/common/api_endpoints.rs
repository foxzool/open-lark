//! 邮件 API 端点定义（类型安全枚举系统）

/// 邮件 API V1 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum MailApiV1 {
    // MailGroup APIs
    MailGroupCreate,
    MailGroupGet(String),
    MailGroupUpdate(String),
    MailGroupDelete(String),
    MailGroupList,
    MailGroupPatch(String),
    // MailGroup Alias APIs
    MailGroupAliasCreate(String),
    MailGroupAliasDelete(String, String),
    MailGroupAliasList(String),
    // MailGroup Manager APIs
    MailGroupManagerBatchCreate(String),
    MailGroupManagerBatchDelete(String),
    MailGroupManagerList(String),
    // MailGroup Member APIs
    MailGroupMemberCreate(String),
    MailGroupMemberDelete(String, String),
    MailGroupMemberGet(String, String),
    MailGroupMemberList(String),
    MailGroupMemberBatchCreate(String),
    MailGroupMemberBatchDelete(String),
    // MailGroup Permission Member APIs
    MailGroupPermissionMemberCreate(String),
    MailGroupPermissionMemberDelete(String, String),
    MailGroupPermissionMemberGet(String, String),
    MailGroupPermissionMemberList(String),
    MailGroupPermissionMemberBatchCreate(String),
    MailGroupPermissionMemberBatchDelete(String),
    // Public Mailbox APIs
    PublicMailboxCreate,
    PublicMailboxGet(String),
    PublicMailboxUpdate(String),
    PublicMailboxDelete(String),
    PublicMailboxList,
    PublicMailboxPatch(String),
    PublicMailboxRemoveToRecycleBin(String),
    // Public Mailbox Alias APIs
    PublicMailboxAliasCreate(String),
    PublicMailboxAliasDelete(String, String),
    PublicMailboxAliasList(String),
    // Public Mailbox Member APIs
    PublicMailboxMemberCreate(String),
    PublicMailboxMemberDelete(String, String),
    PublicMailboxMemberGet(String, String),
    PublicMailboxMemberList(String),
    PublicMailboxMemberBatchCreate(String),
    PublicMailboxMemberBatchDelete(String),
    PublicMailboxMemberClear(String),
    // User APIs
    UserQuery,
    // User Mailbox APIs
    UserMailboxDelete(String),
    // User Mailbox Alias APIs
    UserMailboxAliasCreate(String),
    UserMailboxAliasDelete(String, String),
    UserMailboxAliasList(String),
    // User Mailbox Event APIs
    UserMailboxEventSubscribe(String),
    UserMailboxEventUnsubscribe(String),
    UserMailboxEventSubscription(String),
    // User Mailbox Folder APIs
    UserMailboxFolderCreate(String),
    UserMailboxFolderDelete(String),
    UserMailboxFolderList(String),
    UserMailboxFolderPatch(String),
    // User Mailbox Mail Contact APIs
    UserMailboxMailContactCreate(String),
    UserMailboxMailContactDelete(String),
    UserMailboxMailContactList(String),
    UserMailboxMailContactPatch(String),
    // User Mailbox Message APIs
    UserMailboxMessageGet(String),
    UserMailboxMessageGetByCard(String),
    UserMailboxMessageList(String),
    UserMailboxMessageSend(String),
    UserMailboxMessageAttachmentDownloadUrl(String),
    // User Mailbox Rule APIs
    UserMailboxRuleCreate(String),
    UserMailboxRuleDelete(String),
    UserMailboxRuleList(String),
    UserMailboxRuleReorder(String),
    UserMailboxRuleUpdate(String),
}

impl MailApiV1 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            // MailGroup APIs
            MailApiV1::MailGroupCreate => "/open-apis/mail/v1/mailgroups".to_string(),
            MailApiV1::MailGroupGet(group_id) => {
                format!("/open-apis/mail/v1/mailgroups/{}", group_id)
            }
            MailApiV1::MailGroupUpdate(group_id) => {
                format!("/open-apis/mail/v1/mailgroups/{}", group_id)
            }
            MailApiV1::MailGroupDelete(group_id) => {
                format!("/open-apis/mail/v1/mailgroups/{}", group_id)
            }
            MailApiV1::MailGroupList => "/open-apis/mail/v1/mailgroups".to_string(),
            MailApiV1::MailGroupPatch(group_id) => {
                format!("/open-apis/mail/v1/mailgroups/{}", group_id)
            }
            // MailGroup Alias APIs
            MailApiV1::MailGroupAliasCreate(group_id) => {
                format!("/open-apis/mail/v1/mailgroups/{}/aliases", group_id)
            }
            MailApiV1::MailGroupAliasDelete(group_id, alias_id) => {
                format!(
                    "/open-apis/mail/v1/mailgroups/{}/aliases/{}",
                    group_id, alias_id
                )
            }
            MailApiV1::MailGroupAliasList(group_id) => {
                format!("/open-apis/mail/v1/mailgroups/{}/aliases", group_id)
            }
            // MailGroup Manager APIs
            MailApiV1::MailGroupManagerBatchCreate(group_id) => {
                format!(
                    "/open-apis/mail/v1/mailgroups/{}/managers/batch_create",
                    group_id
                )
            }
            MailApiV1::MailGroupManagerBatchDelete(group_id) => {
                format!(
                    "/open-apis/mail/v1/mailgroups/{}/managers/batch_delete",
                    group_id
                )
            }
            MailApiV1::MailGroupManagerList(group_id) => {
                format!("/open-apis/mail/v1/mailgroups/{}/managers", group_id)
            }
            // MailGroup Member APIs
            MailApiV1::MailGroupMemberCreate(group_id) => {
                format!("/open-apis/mail/v1/mailgroups/{}/members", group_id)
            }
            MailApiV1::MailGroupMemberDelete(group_id, member_id) => {
                format!(
                    "/open-apis/mail/v1/mailgroups/{}/members/{}",
                    group_id, member_id
                )
            }
            MailApiV1::MailGroupMemberGet(group_id, member_id) => {
                format!(
                    "/open-apis/mail/v1/mailgroups/{}/members/{}",
                    group_id, member_id
                )
            }
            MailApiV1::MailGroupMemberList(group_id) => {
                format!("/open-apis/mail/v1/mailgroups/{}/members", group_id)
            }
            MailApiV1::MailGroupMemberBatchCreate(group_id) => {
                format!(
                    "/open-apis/mail/v1/mailgroups/{}/members/batch_create",
                    group_id
                )
            }
            MailApiV1::MailGroupMemberBatchDelete(group_id) => {
                format!(
                    "/open-apis/mail/v1/mailgroups/{}/members/batch_delete",
                    group_id
                )
            }
            // MailGroup Permission Member APIs
            MailApiV1::MailGroupPermissionMemberCreate(group_id) => {
                format!(
                    "/open-apis/mail/v1/mailgroups/{}/permission_members",
                    group_id
                )
            }
            MailApiV1::MailGroupPermissionMemberDelete(group_id, perm_id) => {
                format!(
                    "/open-apis/mail/v1/mailgroups/{}/permission_members/{}",
                    group_id, perm_id
                )
            }
            MailApiV1::MailGroupPermissionMemberGet(group_id, perm_id) => {
                format!(
                    "/open-apis/mail/v1/mailgroups/{}/permission_members/{}",
                    group_id, perm_id
                )
            }
            MailApiV1::MailGroupPermissionMemberList(group_id) => {
                format!(
                    "/open-apis/mail/v1/mailgroups/{}/permission_members",
                    group_id
                )
            }
            MailApiV1::MailGroupPermissionMemberBatchCreate(group_id) => {
                format!(
                    "/open-apis/mail/v1/mailgroups/{}/permission_members/batch_create",
                    group_id
                )
            }
            MailApiV1::MailGroupPermissionMemberBatchDelete(group_id) => {
                format!(
                    "/open-apis/mail/v1/mailgroups/{}/permission_members/batch_delete",
                    group_id
                )
            }
            // Public Mailbox APIs
            MailApiV1::PublicMailboxCreate => "/open-apis/mail/v1/public_mailboxes".to_string(),
            MailApiV1::PublicMailboxGet(mailbox_id) => {
                format!("/open-apis/mail/v1/public_mailboxes/{}", mailbox_id)
            }
            MailApiV1::PublicMailboxUpdate(mailbox_id) => {
                format!("/open-apis/mail/v1/public_mailboxes/{}", mailbox_id)
            }
            MailApiV1::PublicMailboxDelete(mailbox_id) => {
                format!("/open-apis/mail/v1/public_mailboxes/{}", mailbox_id)
            }
            MailApiV1::PublicMailboxList => "/open-apis/mail/v1/public_mailboxes".to_string(),
            MailApiV1::PublicMailboxPatch(mailbox_id) => {
                format!("/open-apis/mail/v1/public_mailboxes/{}", mailbox_id)
            }
            MailApiV1::PublicMailboxRemoveToRecycleBin(mailbox_id) => {
                format!(
                    "/open-apis/mail/v1/public_mailboxes/{}/remove_to_recycle_bin",
                    mailbox_id
                )
            }
            // Public Mailbox Alias APIs
            MailApiV1::PublicMailboxAliasCreate(mailbox_id) => {
                format!("/open-apis/mail/v1/public_mailboxes/{}/aliases", mailbox_id)
            }
            MailApiV1::PublicMailboxAliasDelete(mailbox_id, alias_id) => {
                format!(
                    "/open-apis/mail/v1/public_mailboxes/{}/aliases/{}",
                    mailbox_id, alias_id
                )
            }
            MailApiV1::PublicMailboxAliasList(mailbox_id) => {
                format!("/open-apis/mail/v1/public_mailboxes/{}/aliases", mailbox_id)
            }
            // Public Mailbox Member APIs
            MailApiV1::PublicMailboxMemberCreate(mailbox_id) => {
                format!("/open-apis/mail/v1/public_mailboxes/{}/members", mailbox_id)
            }
            MailApiV1::PublicMailboxMemberDelete(mailbox_id, member_id) => {
                format!(
                    "/open-apis/mail/v1/public_mailboxes/{}/members/{}",
                    mailbox_id, member_id
                )
            }
            MailApiV1::PublicMailboxMemberGet(mailbox_id, member_id) => {
                format!(
                    "/open-apis/mail/v1/public_mailboxes/{}/members/{}",
                    mailbox_id, member_id
                )
            }
            MailApiV1::PublicMailboxMemberList(mailbox_id) => {
                format!("/open-apis/mail/v1/public_mailboxes/{}/members", mailbox_id)
            }
            MailApiV1::PublicMailboxMemberBatchCreate(mailbox_id) => {
                format!(
                    "/open-apis/mail/v1/public_mailboxes/{}/members/batch_create",
                    mailbox_id
                )
            }
            MailApiV1::PublicMailboxMemberBatchDelete(mailbox_id) => {
                format!(
                    "/open-apis/mail/v1/public_mailboxes/{}/members/batch_delete",
                    mailbox_id
                )
            }
            MailApiV1::PublicMailboxMemberClear(mailbox_id) => {
                format!(
                    "/open-apis/mail/v1/public_mailboxes/{}/members/clear",
                    mailbox_id
                )
            }
            // User APIs
            MailApiV1::UserQuery => "/open-apis/mail/v1/user".to_string(),
            // User Mailbox APIs
            MailApiV1::UserMailboxDelete(mailbox_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{}", mailbox_id)
            }
            // User Mailbox Alias APIs
            MailApiV1::UserMailboxAliasCreate(mailbox_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{}/aliases", mailbox_id)
            }
            MailApiV1::UserMailboxAliasDelete(mailbox_id, alias_id) => {
                format!(
                    "/open-apis/mail/v1/user_mailbox/{}/aliases/{}",
                    mailbox_id, alias_id
                )
            }
            MailApiV1::UserMailboxAliasList(mailbox_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{}/aliases", mailbox_id)
            }
            // User Mailbox Event APIs
            MailApiV1::UserMailboxEventSubscribe(mailbox_id) => {
                format!(
                    "/open-apis/mail/v1/user_mailbox/{}/event/subscribe",
                    mailbox_id
                )
            }
            MailApiV1::UserMailboxEventUnsubscribe(mailbox_id) => {
                format!(
                    "/open-apis/mail/v1/user_mailbox/{}/event/unsubscribe",
                    mailbox_id
                )
            }
            MailApiV1::UserMailboxEventSubscription(mailbox_id) => {
                format!(
                    "/open-apis/mail/v1/user_mailbox/{}/event/subscription",
                    mailbox_id
                )
            }
            // User Mailbox Folder APIs
            MailApiV1::UserMailboxFolderCreate(mailbox_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{}/folder", mailbox_id)
            }
            MailApiV1::UserMailboxFolderDelete(mailbox_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{}/folder", mailbox_id)
            }
            MailApiV1::UserMailboxFolderList(mailbox_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{}/folder", mailbox_id)
            }
            MailApiV1::UserMailboxFolderPatch(mailbox_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{}/folder", mailbox_id)
            }
            // User Mailbox Mail Contact APIs
            MailApiV1::UserMailboxMailContactCreate(mailbox_id) => {
                format!(
                    "/open-apis/mail/v1/user_mailbox/{}/mail_contact",
                    mailbox_id
                )
            }
            MailApiV1::UserMailboxMailContactDelete(mailbox_id) => {
                format!(
                    "/open-apis/mail/v1/user_mailbox/{}/mail_contact",
                    mailbox_id
                )
            }
            MailApiV1::UserMailboxMailContactList(mailbox_id) => {
                format!(
                    "/open-apis/mail/v1/user_mailbox/{}/mail_contact",
                    mailbox_id
                )
            }
            MailApiV1::UserMailboxMailContactPatch(mailbox_id) => {
                format!(
                    "/open-apis/mail/v1/user_mailbox/{}/mail_contact",
                    mailbox_id
                )
            }
            // User Mailbox Message APIs
            MailApiV1::UserMailboxMessageGet(mailbox_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{}/message/get", mailbox_id)
            }
            MailApiV1::UserMailboxMessageGetByCard(mailbox_id) => {
                format!(
                    "/open-apis/mail/v1/user_mailbox/{}/message/get_by_card",
                    mailbox_id
                )
            }
            MailApiV1::UserMailboxMessageList(mailbox_id) => {
                format!(
                    "/open-apis/mail/v1/user_mailbox/{}/message/list",
                    mailbox_id
                )
            }
            MailApiV1::UserMailboxMessageSend(mailbox_id) => {
                format!(
                    "/open-apis/mail/v1/user_mailbox/{}/message/send",
                    mailbox_id
                )
            }
            MailApiV1::UserMailboxMessageAttachmentDownloadUrl(mailbox_id) => {
                format!(
                    "/open-apis/mail/v1/user_mailbox/{}/message/attachment/download_url",
                    mailbox_id
                )
            }
            // User Mailbox Rule APIs
            MailApiV1::UserMailboxRuleCreate(mailbox_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{}/rule", mailbox_id)
            }
            MailApiV1::UserMailboxRuleDelete(mailbox_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{}/rule", mailbox_id)
            }
            MailApiV1::UserMailboxRuleList(mailbox_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{}/rule", mailbox_id)
            }
            MailApiV1::UserMailboxRuleReorder(mailbox_id) => {
                format!(
                    "/open-apis/mail/v1/user_mailbox/{}/rule/reorder",
                    mailbox_id
                )
            }
            MailApiV1::UserMailboxRuleUpdate(mailbox_id) => {
                format!("/open-apis/mail/v1/user_mailbox/{}/rule", mailbox_id)
            }
        }
    }
}
