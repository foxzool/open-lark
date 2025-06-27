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

/// 客服信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    /// 客服ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// 客服邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_email: Option<String>,
    /// 客服姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_name: Option<String>,
    /// 客服头像
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,
    /// 客服状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<AgentStatus>,
}

/// 客服状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentStatus {
    /// 在线
    #[serde(rename = "online")]
    Online,
    /// 离线
    #[serde(rename = "offline")]
    Offline,
    /// 忙碌
    #[serde(rename = "busy")]
    Busy,
    /// 离开
    #[serde(rename = "away")]
    Away,
}

/// 客服工作日程
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSchedule {
    /// 日程ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_id: Option<String>,
    /// 客服ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// 开始时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    /// 结束时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// 重复模式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_type: Option<RepeatType>,
}

/// 重复模式
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RepeatType {
    /// 不重复
    #[serde(rename = "none")]
    None,
    /// 每日
    #[serde(rename = "daily")]
    Daily,
    /// 每周
    #[serde(rename = "weekly")]
    Weekly,
    /// 每月
    #[serde(rename = "monthly")]
    Monthly,
}

/// 客服技能
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentSkill {
    /// 技能ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_id: Option<String>,
    /// 技能名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill_name: Option<String>,
    /// 技能描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 技能级别
    #[serde(skip_serializing_if = "Option::is_none")]
    pub level: Option<i32>,
}

/// 工单信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ticket {
    /// 工单ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket_id: Option<String>,
    /// 工单标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 工单描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 工单状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TicketStatus>,
    /// 优先级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<TicketPriority>,
    /// 创建者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<String>,
    /// 分配的客服
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// 工单状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TicketStatus {
    /// 待处理
    #[serde(rename = "pending")]
    Pending,
    /// 处理中
    #[serde(rename = "processing")]
    Processing,
    /// 已解决
    #[serde(rename = "solved")]
    Solved,
    /// 已关闭
    #[serde(rename = "closed")]
    Closed,
}

/// 工单优先级
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TicketPriority {
    /// 低
    #[serde(rename = "low")]
    Low,
    /// 中
    #[serde(rename = "medium")]
    Medium,
    /// 高
    #[serde(rename = "high")]
    High,
    /// 紧急
    #[serde(rename = "urgent")]
    Urgent,
}

/// 工单消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TicketMessage {
    /// 消息ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_id: Option<String>,
    /// 工单ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ticket_id: Option<String>,
    /// 消息内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 消息类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_type: Option<MessageType>,
    /// 发送者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender: Option<String>,
    /// 发送时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
}

/// 消息类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageType {
    /// 文本
    #[serde(rename = "text")]
    Text,
    /// 图片
    #[serde(rename = "image")]
    Image,
    /// 文件
    #[serde(rename = "file")]
    File,
    /// 卡片
    #[serde(rename = "card")]
    Card,
}

/// 知识库条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Faq {
    /// FAQ ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub faq_id: Option<String>,
    /// 标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    /// 标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<FaqStatus>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// FAQ状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FaqStatus {
    /// 草稿
    #[serde(rename = "draft")]
    Draft,
    /// 已发布
    #[serde(rename = "published")]
    Published,
    /// 已归档
    #[serde(rename = "archived")]
    Archived,
}

/// 知识库分类
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Category {
    /// 分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category_id: Option<String>,
    /// 分类名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 父分类ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// 排序
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<i32>,
}

/// 推送信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    /// 推送ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_id: Option<String>,
    /// 标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 目标用户
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_users: Option<Vec<String>>,
    /// 推送状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<NotificationStatus>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// 计划发送时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_at: Option<String>,
}

/// 推送状态
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationStatus {
    /// 草稿
    #[serde(rename = "draft")]
    Draft,
    /// 待审核
    #[serde(rename = "pending_approval")]
    PendingApproval,
    /// 已审核
    #[serde(rename = "approved")]
    Approved,
    /// 已发送
    #[serde(rename = "sent")]
    Sent,
    /// 已取消
    #[serde(rename = "cancelled")]
    Cancelled,
}

/// 自定义字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomizedField {
    /// 字段ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_id: Option<String>,
    /// 字段名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_name: Option<String>,
    /// 字段类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub field_type: Option<FieldType>,
    /// 是否必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// 默认值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    /// 选项列表（用于单选、多选字段）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
}

/// 字段类型
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldType {
    /// 文本
    #[serde(rename = "text")]
    Text,
    /// 数字
    #[serde(rename = "number")]
    Number,
    /// 日期
    #[serde(rename = "date")]
    Date,
    /// 单选
    #[serde(rename = "single_select")]
    SingleSelect,
    /// 多选
    #[serde(rename = "multi_select")]
    MultiSelect,
    /// 文本域
    #[serde(rename = "textarea")]
    Textarea,
}
