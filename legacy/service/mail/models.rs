use serde::{Deserialize, Serialize};

/// 用户ID类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UserIdType {
    #[serde(rename = "open_id")]
    OpenId,
    #[serde(rename = "user_id")]
    UserId,
    #[serde(rename = "union_id")]
    UnionId,
}

impl UserIdType {
    pub fn as_str(&self) -> &str {
        match self {
            UserIdType::OpenId => "open_id",
            UserIdType::UserId => "user_id",
            UserIdType::UnionId => "union_id",
        }
    }
}

/// 邮箱文件夹
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Folder {
    /// 文件夹ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_id: Option<String>,
    /// 文件夹名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_name: Option<String>,
    /// 上级文件夹ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_folder_id: Option<String>,
    /// 文件夹类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_type: Option<FolderType>,
    /// 文件夹路径
    #[serde(skip_serializing_if = "Option::is_none")]
    pub folder_path: Option<String>,
}

/// 文件夹类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FolderType {
    /// 系统文件夹
    #[serde(rename = "system")]
    System,
    /// 自定义文件夹
    #[serde(rename = "custom")]
    Custom,
}

/// 邮件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// 邮件ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// 主题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject: Option<String>,
    /// 发件人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<MailAddress>,
    /// 收件人
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<Vec<MailAddress>>,
    /// 抄送
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cc: Option<Vec<MailAddress>>,
    /// 密送
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bcc: Option<Vec<MailAddress>>,
    /// 邮件内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<MailBody>,
    /// 发送时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sent_time: Option<i64>,
    /// 是否已读
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_read: Option<bool>,
    /// 邮件状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<MessageStatus>,
    /// 附件列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<Attachment>>,
}

/// 邮件地址
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailAddress {
    /// 邮箱地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 显示名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// 邮件内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailBody {
    /// 文本内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    /// HTML内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub html: Option<String>,
}

/// 邮件状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageStatus {
    /// 草稿
    #[serde(rename = "draft")]
    Draft,
    /// 已发送
    #[serde(rename = "sent")]
    Sent,
    /// 已接收
    #[serde(rename = "received")]
    Received,
    /// 已删除
    #[serde(rename = "deleted")]
    Deleted,
}

/// 邮件附件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    /// 附件ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachment_id: Option<String>,
    /// 附件名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 附件大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// 附件类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// 下载URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
}

/// 收信规则
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rule {
    /// 规则ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    /// 规则名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_name: Option<String>,
    /// 是否启用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// 条件
    #[serde(skip_serializing_if = "Option::is_none")]
    pub conditions: Option<Vec<RuleCondition>>,
    /// 动作
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<RuleAction>>,
    /// 优先级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
}

/// 规则条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleCondition {
    /// 字段
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field: Option<String>,
    /// 操作符
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// 值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

/// 规则动作
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleAction {
    /// 动作类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_type: Option<String>,
    /// 目标文件夹ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_folder_id: Option<String>,
    /// 标记为已读
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_as_read: Option<bool>,
    /// 转发到
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forward_to: Option<String>,
    /// 参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
}

/// 邮箱联系人
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contact {
    /// 联系人ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    /// 联系人姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 邮箱地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 备注
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
}

/// 邮件组
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailGroup {
    /// 邮件组ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailgroup_id: Option<String>,
    /// 邮件组名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 邮件组邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 是否允许外部发送
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_external_send: Option<bool>,
    /// 权限设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission: Option<MailGroupPermission>,
}

/// 邮件组权限
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailGroupPermission {
    /// 加入权限
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_permission: Option<JoinPermission>,
    /// 发送权限
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_permission: Option<SendPermission>,
}

/// 加入权限
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum JoinPermission {
    /// 所有人
    #[serde(rename = "all")]
    All,
    /// 仅管理员
    #[serde(rename = "admin_only")]
    AdminOnly,
    /// 邀请制
    #[serde(rename = "invite_only")]
    InviteOnly,
}

/// 发送权限
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SendPermission {
    /// 所有人
    #[serde(rename = "all")]
    All,
    /// 仅成员
    #[serde(rename = "members_only")]
    MembersOnly,
    /// 仅管理员
    #[serde(rename = "admin_only")]
    AdminOnly,
}

/// 邮件组成员
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailGroupMember {
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 成员类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<MemberType>,
}

/// 成员类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MemberType {
    /// 普通成员
    #[serde(rename = "member")]
    Member,
    /// 管理员
    #[serde(rename = "admin")]
    Admin,
}

/// 邮件组别名
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MailGroupAlias {
    /// 别名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
}

/// 公共邮箱
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicMailbox {
    /// 公共邮箱ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_mailbox_id: Option<String>,
    /// 公共邮箱名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 公共邮箱邮箱地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PublicMailboxStatus>,
}

/// 公共邮箱状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PublicMailboxStatus {
    /// 正常
    #[serde(rename = "active")]
    Active,
    /// 已删除
    #[serde(rename = "deleted")]
    Deleted,
    /// 在回收站
    #[serde(rename = "in_recycle_bin")]
    InRecycleBin,
}

/// 公共邮箱成员
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublicMailboxMember {
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 权限类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_type: Option<PublicMailboxPermissionType>,
}

/// 公共邮箱权限类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PublicMailboxPermissionType {
    /// 管理员
    #[serde(rename = "admin")]
    Admin,
    /// 成员
    #[serde(rename = "member")]
    Member,
}

/// 用户邮箱别名
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMailboxAlias {
    /// 别名ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_id: Option<String>,
    /// 别名邮箱地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias_email: Option<String>,
    /// 是否为主邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_primary: Option<bool>,
}

/// 邮箱地址状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddressStatus {
    /// 邮箱地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<EmailStatus>,
    /// 邮箱类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailbox_type: Option<MailboxType>,
}

/// 邮箱状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmailStatus {
    /// 存在
    #[serde(rename = "exists")]
    Exists,
    /// 不存在
    #[serde(rename = "not_exists")]
    NotExists,
    /// 已删除
    #[serde(rename = "deleted")]
    Deleted,
}

/// 邮箱类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MailboxType {
    /// 用户邮箱
    #[serde(rename = "user")]
    User,
    /// 邮件组
    #[serde(rename = "group")]
    Group,
    /// 公共邮箱
    #[serde(rename = "public")]
    Public,
}

/// 事件订阅状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionStatus {
    /// 是否已订阅
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_subscribed: Option<bool>,
    /// 订阅时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_time: Option<i64>,
}

/// 邮箱订阅信息
#[derive(Debug, Serialize, Deserialize)]
pub struct MailboxSubscription {
    /// 订阅ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<String>,
    /// 邮箱ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailbox_id: Option<String>,
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Webhook URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webhook_url: Option<String>,
    /// 事件类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_types: Option<Vec<String>>,
    /// 订阅类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_type: Option<String>,
    /// 订阅状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 是否已订阅
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_subscribed: Option<bool>,
    /// 订阅时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe_time: Option<i64>,
    /// 订阅时间字符串
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_user_id_type_as_str() {
        assert_eq!(UserIdType::OpenId.as_str(), "open_id");
        assert_eq!(UserIdType::UserId.as_str(), "user_id");
        assert_eq!(UserIdType::UnionId.as_str(), "union_id");
    }

    #[test]
    fn test_user_id_type_serialization() {
        let open_id = UserIdType::OpenId;
        let json = serde_json::to_string(&open_id).unwrap();
        assert_eq!(json, "\"open_id\"");

        let user_id = UserIdType::UserId;
        let json = serde_json::to_string(&user_id).unwrap();
        assert_eq!(json, "\"user_id\"");

        let union_id = UserIdType::UnionId;
        let json = serde_json::to_string(&union_id).unwrap();
        assert_eq!(json, "\"union_id\"");
    }

    #[test]
    fn test_folder_type_serialization() {
        let system = FolderType::System;
        let json = serde_json::to_string(&system).unwrap();
        assert_eq!(json, "\"system\"");

        let custom = FolderType::Custom;
        let json = serde_json::to_string(&custom).unwrap();
        assert_eq!(json, "\"custom\"");
    }

    #[test]
    fn test_folder_complete() {
        let folder = Folder {
            folder_id: Some("folder123".to_string()),
            folder_name: Some("Inbox".to_string()),
            parent_folder_id: Some("root".to_string()),
            folder_type: Some(FolderType::System),
            folder_path: Some("/inbox".to_string()),
        };
        let json = serde_json::to_string(&folder).unwrap();
        assert!(json.contains("folder123"));
        assert!(json.contains("Inbox"));
        assert!(json.contains("system"));
    }

    #[test]
    fn test_folder_optional_fields() {
        let folder = Folder {
            folder_id: Some("folder456".to_string()),
            folder_name: None,
            parent_folder_id: None,
            folder_type: None,
            folder_path: None,
        };
        let json = serde_json::to_string(&folder).unwrap();
        assert!(json.contains("folder456"));
        assert!(!json.contains("folder_name"));
        assert!(!json.contains("parent_folder_id"));
    }

    #[test]
    fn test_mail_address_complete() {
        let address = MailAddress {
            name: Some("John Doe".to_string()),
            email: Some("john.doe@example.com".to_string()),
        };
        let json = serde_json::to_string(&address).unwrap();
        assert!(json.contains("John Doe"));
        assert!(json.contains("john.doe@example.com"));
    }

    #[test]
    fn test_mail_body_types() {
        let html_body = MailBody {
            text: None,
            html: Some("<p>HTML content</p>".to_string()),
        };
        let json = serde_json::to_string(&html_body).unwrap();
        assert!(json.contains("html"));
        assert!(json.contains("<p>HTML content</p>"));

        let text_body = MailBody {
            text: Some("Plain text content".to_string()),
            html: None,
        };
        let json = serde_json::to_string(&text_body).unwrap();
        assert!(json.contains("text"));
        assert!(json.contains("Plain text content"));
    }

    #[test]
    fn test_message_status_serialization() {
        let draft = MessageStatus::Draft;
        let json = serde_json::to_string(&draft).unwrap();
        assert_eq!(json, "\"draft\"");

        let sent = MessageStatus::Sent;
        let json = serde_json::to_string(&sent).unwrap();
        assert_eq!(json, "\"sent\"");

        let deleted = MessageStatus::Deleted;
        let json = serde_json::to_string(&deleted).unwrap();
        assert_eq!(json, "\"deleted\"");

        let received = MessageStatus::Received;
        let json = serde_json::to_string(&received).unwrap();
        assert_eq!(json, "\"received\"");
    }

    #[test]
    fn test_message_with_recipients() {
        let message = Message {
            message_id: Some("msg123".to_string()),
            subject: Some("Test Subject".to_string()),
            from: Some(MailAddress {
                name: Some("Sender Name".to_string()),
                email: Some("sender@example.com".to_string()),
            }),
            to: Some(vec![
                MailAddress {
                    name: Some("Recipient 1".to_string()),
                    email: Some("recipient1@example.com".to_string()),
                },
                MailAddress {
                    name: Some("Recipient 2".to_string()),
                    email: Some("recipient2@example.com".to_string()),
                },
            ]),
            cc: Some(vec![]),
            bcc: None,
            body: Some(MailBody {
                text: None,
                html: Some("<p>Email body content</p>".to_string()),
            }),
            sent_time: Some(1640995200),
            is_read: Some(false),
            status: Some(MessageStatus::Draft),
            attachments: Some(vec![]),
        };
        let json = serde_json::to_string(&message).unwrap();
        assert!(json.contains("msg123"));
        assert!(json.contains("Test Subject"));
        assert!(json.contains("sender@example.com"));
        assert!(json.contains("recipient1@example.com"));
    }

    #[test]
    fn test_attachment_with_size() {
        let attachment = Attachment {
            attachment_id: Some("att789".to_string()),
            name: Some("document.pdf".to_string()),
            content_type: Some("application/pdf".to_string()),
            size: Some(1024000),
            download_url: Some("https://example.com/download/att789".to_string()),
        };
        let json = serde_json::to_string(&attachment).unwrap();
        assert!(json.contains("att789"));
        assert!(json.contains("document.pdf"));
        assert!(json.contains("application/pdf"));
        assert!(json.contains("1024000"));
    }

    #[test]
    fn test_attachment_inline() {
        let attachment = Attachment {
            attachment_id: Some("att456".to_string()),
            name: Some("image.png".to_string()),
            content_type: Some("image/png".to_string()),
            size: Some(52000),
            download_url: None,
        };
        let json = serde_json::to_string(&attachment).unwrap();
        assert!(json.contains("att456"));
        assert!(json.contains("image.png"));
        assert!(json.contains("image/png"));
        assert!(!json.contains("download_url"));
    }

    #[test]
    fn test_rule_with_conditions() {
        let rule = Rule {
            rule_id: Some("rule123".to_string()),
            rule_name: Some("Important Emails".to_string()),
            enabled: Some(true),
            conditions: Some(vec![
                RuleCondition {
                    field: Some("from".to_string()),
                    operator: Some("contains".to_string()),
                    value: Some("boss@company.com".to_string()),
                },
                RuleCondition {
                    field: Some("subject".to_string()),
                    operator: Some("contains".to_string()),
                    value: Some("urgent".to_string()),
                },
            ]),
            actions: Some(vec![RuleAction {
                action_type: Some("move_to_folder".to_string()),
                target_folder_id: Some("important".to_string()),
                mark_as_read: Some(false),
                forward_to: None,
                parameters: None,
            }]),
            priority: Some(1),
        };
        let json = serde_json::to_string(&rule).unwrap();
        assert!(json.contains("rule123"));
        assert!(json.contains("Important Emails"));
        assert!(json.contains("boss@company.com"));
        assert!(json.contains("move_to_folder"));
    }

    #[test]
    fn test_rule_action_forward() {
        let action = RuleAction {
            action_type: Some("forward".to_string()),
            target_folder_id: None,
            mark_as_read: Some(true),
            forward_to: Some("assistant@company.com".to_string()),
            parameters: None,
        };
        let json = serde_json::to_string(&action).unwrap();
        assert!(json.contains("forward"));
        assert!(json.contains("assistant@company.com"));
        assert!(!json.contains("target_folder_id"));
    }

    #[test]
    fn test_mailbox_subscription() {
        let subscription = MailboxSubscription {
            subscription_id: Some("sub789".to_string()),
            mailbox_id: Some("mailbox123".to_string()),
            user_id: Some("user123".to_string()),
            webhook_url: Some("https://webhook.example.com/mail".to_string()),
            event_types: Some(vec![
                "message_received".to_string(),
                "message_sent".to_string(),
            ]),
            subscription_type: Some("webhook".to_string()),
            status: Some("active".to_string()),
            is_subscribed: Some(true),
            subscribe_time: Some(1640995200),
            subscription_time: Some("2024-01-01T10:00:00Z".to_string()),
            update_time: Some("2024-01-01T10:00:00Z".to_string()),
        };
        let json = serde_json::to_string(&subscription).unwrap();
        assert!(json.contains("sub789"));
        assert!(json.contains("mailbox123"));
        assert!(json.contains("https://webhook.example.com/mail"));
        assert!(json.contains("message_received"));
    }

    #[test]
    fn test_mailbox_subscription_unsubscribed() {
        let subscription = MailboxSubscription {
            subscription_id: Some("sub456".to_string()),
            mailbox_id: Some("mailbox456".to_string()),
            user_id: None,
            webhook_url: None,
            event_types: None,
            subscription_type: Some("inactive".to_string()),
            status: Some("inactive".to_string()),
            is_subscribed: Some(false),
            subscribe_time: None,
            subscription_time: None,
            update_time: None,
        };
        let json = serde_json::to_string(&subscription).unwrap();
        assert!(json.contains("sub456"));
        assert!(json.contains("false"));
        assert!(!json.contains("webhook_url"));
        assert!(!json.contains("subscribe_time"));
    }
}
