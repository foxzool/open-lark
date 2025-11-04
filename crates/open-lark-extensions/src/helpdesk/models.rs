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
        let serialized = serde_json::to_string(&open_id).unwrap();
        assert_eq!(serialized, "\"open_id\"");

        let user_id = UserIdType::UserId;
        let serialized = serde_json::to_string(&user_id).unwrap();
        assert_eq!(serialized, "\"user_id\"");

        let union_id = UserIdType::UnionId;
        let serialized = serde_json::to_string(&union_id).unwrap();
        assert_eq!(serialized, "\"union_id\"");
    }

    #[test]
    fn test_user_id_type_deserialization() {
        let deserialized: UserIdType = serde_json::from_str("\"open_id\"").unwrap();
        assert!(matches!(deserialized, UserIdType::OpenId));

        let deserialized: UserIdType = serde_json::from_str("\"user_id\"").unwrap();
        assert!(matches!(deserialized, UserIdType::UserId));

        let deserialized: UserIdType = serde_json::from_str("\"union_id\"").unwrap();
        assert!(matches!(deserialized, UserIdType::UnionId));
    }

    #[test]
    fn test_agent_status_serialization() {
        assert_eq!(
            serde_json::to_string(&AgentStatus::Online).unwrap(),
            "\"online\""
        );
        assert_eq!(
            serde_json::to_string(&AgentStatus::Offline).unwrap(),
            "\"offline\""
        );
        assert_eq!(
            serde_json::to_string(&AgentStatus::Busy).unwrap(),
            "\"busy\""
        );
        assert_eq!(
            serde_json::to_string(&AgentStatus::Away).unwrap(),
            "\"away\""
        );
    }

    #[test]
    fn test_repeat_type_serialization() {
        assert_eq!(
            serde_json::to_string(&RepeatType::None).unwrap(),
            "\"none\""
        );
        assert_eq!(
            serde_json::to_string(&RepeatType::Daily).unwrap(),
            "\"daily\""
        );
        assert_eq!(
            serde_json::to_string(&RepeatType::Weekly).unwrap(),
            "\"weekly\""
        );
        assert_eq!(
            serde_json::to_string(&RepeatType::Monthly).unwrap(),
            "\"monthly\""
        );
    }

    #[test]
    fn test_ticket_status_serialization() {
        assert_eq!(
            serde_json::to_string(&TicketStatus::Pending).unwrap(),
            "\"pending\""
        );
        assert_eq!(
            serde_json::to_string(&TicketStatus::Processing).unwrap(),
            "\"processing\""
        );
        assert_eq!(
            serde_json::to_string(&TicketStatus::Solved).unwrap(),
            "\"solved\""
        );
        assert_eq!(
            serde_json::to_string(&TicketStatus::Closed).unwrap(),
            "\"closed\""
        );
    }

    #[test]
    fn test_ticket_priority_serialization() {
        assert_eq!(
            serde_json::to_string(&TicketPriority::Low).unwrap(),
            "\"low\""
        );
        assert_eq!(
            serde_json::to_string(&TicketPriority::Medium).unwrap(),
            "\"medium\""
        );
        assert_eq!(
            serde_json::to_string(&TicketPriority::High).unwrap(),
            "\"high\""
        );
        assert_eq!(
            serde_json::to_string(&TicketPriority::Urgent).unwrap(),
            "\"urgent\""
        );
    }

    #[test]
    fn test_message_type_serialization() {
        assert_eq!(
            serde_json::to_string(&MessageType::Text).unwrap(),
            "\"text\""
        );
        assert_eq!(
            serde_json::to_string(&MessageType::Image).unwrap(),
            "\"image\""
        );
        assert_eq!(
            serde_json::to_string(&MessageType::File).unwrap(),
            "\"file\""
        );
        assert_eq!(
            serde_json::to_string(&MessageType::Card).unwrap(),
            "\"card\""
        );
    }

    #[test]
    fn test_faq_status_serialization() {
        assert_eq!(
            serde_json::to_string(&FaqStatus::Draft).unwrap(),
            "\"draft\""
        );
        assert_eq!(
            serde_json::to_string(&FaqStatus::Published).unwrap(),
            "\"published\""
        );
        assert_eq!(
            serde_json::to_string(&FaqStatus::Archived).unwrap(),
            "\"archived\""
        );
    }

    #[test]
    fn test_notification_status_serialization() {
        assert_eq!(
            serde_json::to_string(&NotificationStatus::Draft).unwrap(),
            "\"draft\""
        );
        assert_eq!(
            serde_json::to_string(&NotificationStatus::PendingApproval).unwrap(),
            "\"pending_approval\""
        );
        assert_eq!(
            serde_json::to_string(&NotificationStatus::Approved).unwrap(),
            "\"approved\""
        );
        assert_eq!(
            serde_json::to_string(&NotificationStatus::Sent).unwrap(),
            "\"sent\""
        );
        assert_eq!(
            serde_json::to_string(&NotificationStatus::Cancelled).unwrap(),
            "\"cancelled\""
        );
    }

    #[test]
    fn test_field_type_serialization() {
        assert_eq!(serde_json::to_string(&FieldType::Text).unwrap(), "\"text\"");
        assert_eq!(
            serde_json::to_string(&FieldType::Number).unwrap(),
            "\"number\""
        );
        assert_eq!(serde_json::to_string(&FieldType::Date).unwrap(), "\"date\"");
        assert_eq!(
            serde_json::to_string(&FieldType::SingleSelect).unwrap(),
            "\"single_select\""
        );
        assert_eq!(
            serde_json::to_string(&FieldType::MultiSelect).unwrap(),
            "\"multi_select\""
        );
        assert_eq!(
            serde_json::to_string(&FieldType::Textarea).unwrap(),
            "\"textarea\""
        );
    }

    #[test]
    fn test_agent_model_serialization() {
        let agent = Agent {
            agent_id: Some("agent123".to_string()),
            agent_email: Some("agent@example.com".to_string()),
            agent_name: Some("John Doe".to_string()),
            avatar_url: Some("https://example.com/avatar.jpg".to_string()),
            status: Some(AgentStatus::Online),
        };

        let serialized = serde_json::to_string(&agent).unwrap();
        let deserialized: Agent = serde_json::from_str(&serialized).unwrap();

        assert_eq!(agent.agent_id, deserialized.agent_id);
        assert_eq!(agent.agent_email, deserialized.agent_email);
        assert_eq!(agent.agent_name, deserialized.agent_name);
        assert_eq!(agent.avatar_url, deserialized.avatar_url);
    }

    #[test]
    fn test_ticket_model_serialization() {
        let ticket = Ticket {
            ticket_id: Some("ticket123".to_string()),
            title: Some("Test Ticket".to_string()),
            description: Some("This is a test ticket".to_string()),
            status: Some(TicketStatus::Pending),
            priority: Some(TicketPriority::High),
            creator: Some("user123".to_string()),
            assignee: Some("agent123".to_string()),
            created_at: Some("2023-01-01T00:00:00Z".to_string()),
            updated_at: Some("2023-01-01T01:00:00Z".to_string()),
        };

        let serialized = serde_json::to_string(&ticket).unwrap();
        let deserialized: Ticket = serde_json::from_str(&serialized).unwrap();

        assert_eq!(ticket.ticket_id, deserialized.ticket_id);
        assert_eq!(ticket.title, deserialized.title);
        assert_eq!(ticket.description, deserialized.description);
        assert_eq!(ticket.creator, deserialized.creator);
        assert_eq!(ticket.assignee, deserialized.assignee);
    }

    #[test]
    fn test_ticket_message_model_serialization() {
        let message = TicketMessage {
            message_id: Some("msg123".to_string()),
            ticket_id: Some("ticket123".to_string()),
            content: Some("Hello, how can I help you?".to_string()),
            message_type: Some(MessageType::Text),
            sender: Some("agent123".to_string()),
            created_at: Some("2023-01-01T00:00:00Z".to_string()),
        };

        let serialized = serde_json::to_string(&message).unwrap();
        let deserialized: TicketMessage = serde_json::from_str(&serialized).unwrap();

        assert_eq!(message.message_id, deserialized.message_id);
        assert_eq!(message.ticket_id, deserialized.ticket_id);
        assert_eq!(message.content, deserialized.content);
        assert_eq!(message.sender, deserialized.sender);
        assert_eq!(message.created_at, deserialized.created_at);
    }

    #[test]
    fn test_faq_model_serialization() {
        let faq = Faq {
            faq_id: Some("faq123".to_string()),
            title: Some("How to reset password?".to_string()),
            content: Some("To reset your password, click on...".to_string()),
            category_id: Some("cat123".to_string()),
            tags: Some(vec!["password".to_string(), "security".to_string()]),
            status: Some(FaqStatus::Published),
            created_at: Some("2023-01-01T00:00:00Z".to_string()),
            updated_at: Some("2023-01-01T01:00:00Z".to_string()),
        };

        let serialized = serde_json::to_string(&faq).unwrap();
        let deserialized: Faq = serde_json::from_str(&serialized).unwrap();

        assert_eq!(faq.faq_id, deserialized.faq_id);
        assert_eq!(faq.title, deserialized.title);
        assert_eq!(faq.content, deserialized.content);
        assert_eq!(faq.category_id, deserialized.category_id);
        assert_eq!(faq.tags, deserialized.tags);
    }

    #[test]
    fn test_customized_field_model_serialization() {
        let field = CustomizedField {
            field_id: Some("field123".to_string()),
            field_name: Some("Priority Level".to_string()),
            field_type: Some(FieldType::SingleSelect),
            required: Some(true),
            default_value: Some("Medium".to_string()),
            options: Some(vec![
                "Low".to_string(),
                "Medium".to_string(),
                "High".to_string(),
            ]),
        };

        let serialized = serde_json::to_string(&field).unwrap();
        let deserialized: CustomizedField = serde_json::from_str(&serialized).unwrap();

        assert_eq!(field.field_id, deserialized.field_id);
        assert_eq!(field.field_name, deserialized.field_name);
        assert_eq!(field.required, deserialized.required);
        assert_eq!(field.default_value, deserialized.default_value);
        assert_eq!(field.options, deserialized.options);
    }

    #[test]
    fn test_notification_model_serialization() {
        let notification = Notification {
            notification_id: Some("notif123".to_string()),
            title: Some("System Maintenance".to_string()),
            content: Some("The system will be under maintenance...".to_string()),
            target_users: Some(vec!["user1".to_string(), "user2".to_string()]),
            status: Some(NotificationStatus::Sent),
            created_at: Some("2023-01-01T00:00:00Z".to_string()),
            scheduled_at: Some("2023-01-01T09:00:00Z".to_string()),
        };

        let serialized = serde_json::to_string(&notification).unwrap();
        let deserialized: Notification = serde_json::from_str(&serialized).unwrap();

        assert_eq!(notification.notification_id, deserialized.notification_id);
        assert_eq!(notification.title, deserialized.title);
        assert_eq!(notification.content, deserialized.content);
        assert_eq!(notification.target_users, deserialized.target_users);
    }

    #[test]
    fn test_models_with_none_values() {
        let ticket = Ticket {
            ticket_id: None,
            title: None,
            description: None,
            status: None,
            priority: None,
            creator: None,
            assignee: None,
            created_at: None,
            updated_at: None,
        };

        let serialized = serde_json::to_string(&ticket).unwrap();
        assert_eq!(serialized, "{}");

        let deserialized: Ticket = serde_json::from_str("{}").unwrap();
        assert_eq!(deserialized.ticket_id, None);
        assert_eq!(deserialized.title, None);
    }

    #[test]
    fn test_agent_schedule_model() {
        let schedule = AgentSchedule {
            schedule_id: Some("schedule123".to_string()),
            agent_id: Some("agent123".to_string()),
            start_time: Some("09:00".to_string()),
            end_time: Some("17:00".to_string()),
            repeat_type: Some(RepeatType::Daily),
        };

        let serialized = serde_json::to_string(&schedule).unwrap();
        let deserialized: AgentSchedule = serde_json::from_str(&serialized).unwrap();

        assert_eq!(schedule.schedule_id, deserialized.schedule_id);
        assert_eq!(schedule.agent_id, deserialized.agent_id);
        assert_eq!(schedule.start_time, deserialized.start_time);
        assert_eq!(schedule.end_time, deserialized.end_time);
    }

    #[test]
    fn test_agent_skill_model() {
        let skill = AgentSkill {
            skill_id: Some("skill123".to_string()),
            skill_name: Some("Customer Service".to_string()),
            description: Some("Excellent customer service skills".to_string()),
            level: Some(5),
        };

        let serialized = serde_json::to_string(&skill).unwrap();
        let deserialized: AgentSkill = serde_json::from_str(&serialized).unwrap();

        assert_eq!(skill.skill_id, deserialized.skill_id);
        assert_eq!(skill.skill_name, deserialized.skill_name);
        assert_eq!(skill.description, deserialized.description);
        assert_eq!(skill.level, deserialized.level);
    }

    #[test]
    fn test_category_model() {
        let category = Category {
            category_id: Some("cat123".to_string()),
            name: Some("Technical Support".to_string()),
            description: Some("Technical issues and questions".to_string()),
            parent_id: Some("parent123".to_string()),
            sort_order: Some(1),
        };

        let serialized = serde_json::to_string(&category).unwrap();
        let deserialized: Category = serde_json::from_str(&serialized).unwrap();

        assert_eq!(category.category_id, deserialized.category_id);
        assert_eq!(category.name, deserialized.name);
        assert_eq!(category.description, deserialized.description);
        assert_eq!(category.parent_id, deserialized.parent_id);
        assert_eq!(category.sort_order, deserialized.sort_order);
    }
}
