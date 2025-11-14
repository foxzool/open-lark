//! Mail 邮件服务端点常量定义

// ===== 邮件组管理 =====

/// 邮件组基础操作
pub const MAIL_V1_MAILGROUPS: &str = "/open-apis/mail/v1/mailgroups";

/// 邮件组详情操作 (需要使用 EndpointBuilder::replace_param 替换 {mailgroup_id})
pub const MAIL_V1_MAILGROUP: &str = "/open-apis/mail/v1/mailgroups/{mailgroup_id}";

// ===== 邮件组管理员 =====

/// 批量创建邮件组管理员 (需要使用 EndpointBuilder::replace_param 替换 {mailgroup_id})
pub const MAIL_V1_MAILGROUP_MANAGERS_BATCH_CREATE: &str =
    "/open-apis/mail/v1/mailgroups/{mailgroup_id}/managers/batch_create";

/// 批量删除邮件组管理员 (需要使用 EndpointBuilder::replace_param 替换 {mailgroup_id})
pub const MAIL_V1_MAILGROUP_MANAGERS_BATCH_DELETE: &str =
    "/open-apis/mail/v1/mailgroups/{mailgroup_id}/managers/batch_delete";

/// 获取邮件组管理员列表 (需要使用 EndpointBuilder::replace_param 替换 {mailgroup_id})
pub const MAIL_V1_MAILGROUP_MANAGERS: &str =
    "/open-apis/mail/v1/mailgroups/{mailgroup_id}/managers";

// ===== 用户邮箱事件 =====

/// 订阅用户邮箱事件 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id})
pub const MAIL_V1_USER_MAILBOX_EVENTS_SUBSCRIBE: &str =
    "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/events/subscribe";

/// 获取用户邮箱事件订阅状态 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id})
pub const MAIL_V1_USER_MAILBOX_EVENTS_SUBSCRIPTION: &str =
    "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/events/subscription";

/// 取消订阅用户邮箱事件 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id})
pub const MAIL_V1_USER_MAILBOX_EVENTS_UNSUBSCRIBE: &str =
    "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/events/unsubscribe";

// ===== 用户邮箱文件夹 =====

/// 用户邮箱文件夹操作 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id})
pub const MAIL_V1_USER_MAILBOX_FOLDERS: &str =
    "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/folders";

/// 用户邮箱文件夹详情操作 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id} 和 {folder_id})
pub const MAIL_V1_USER_MAILBOX_FOLDER: &str =
    "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/folders/{folder_id}";

// ===== 用户邮箱消息 =====

/// 用户邮箱消息操作 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id})
pub const MAIL_V1_USER_MAILBOX_MESSAGES: &str =
    "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/messages";

/// 用户邮箱消息详情 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id} 和 {message_id})
pub const MAIL_V1_USER_MAILBOX_MESSAGE: &str =
    "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/messages/{message_id}";

/// 通过卡片获取邮件消息 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id})
pub const MAIL_V1_USER_MAILBOX_MESSAGES_GET_BY_CARD: &str =
    "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/messages/get_by_card";

// ===== 用户邮箱规则 =====

/// 用户邮箱规则操作 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id})
pub const MAIL_V1_USER_MAILBOX_RULES: &str =
    "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/rules";

/// 用户邮箱规则详情 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id} 和 {rule_id})
pub const MAIL_V1_USER_MAILBOX_RULE: &str =
    "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/rules/{rule_id}";

/// 重新排序用户邮箱规则 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id})
pub const MAIL_V1_USER_MAILBOX_RULES_REORDER: &str =
    "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/rules/reorder";

// ===== 用户邮箱联系人 =====

/// 用户邮箱联系人操作 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id})
pub const MAIL_V1_USER_MAILBOX_MAIL_CONTACTS: &str =
    "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/mail_contacts";

/// 用户邮箱联系人详情 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id} 和 {contact_id})
pub const MAIL_V1_USER_MAILBOX_MAIL_CONTACT: &str =
    "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/mail_contacts/{contact_id}";

// ===== 用户邮箱附件 =====

/// 获取邮件附件下载链接 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id}, {message_id} 和 {attachment_id})
pub const MAIL_V1_USER_MAILBOX_MESSAGE_ATTACHMENT_DOWNLOAD_URL: &str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/messages/{message_id}/attachments/{attachment_id}/download_url";
