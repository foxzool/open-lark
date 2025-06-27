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
