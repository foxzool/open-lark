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

/// 任务实体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    /// 任务GUID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    /// 任务标题
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// 任务描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 截止时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due: Option<TaskDue>,
    /// 开始时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<TaskStart>,
    /// 完成时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    /// 任务成员列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<TaskMember>>,
    /// 重复规则
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repeat_rule: Option<String>,
    /// 自定义完成配置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_complete: Option<TaskCustomComplete>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// 状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 工作流状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workflow_state: Option<String>,
    /// 任务来源
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<i32>,
    /// URL链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// 任务截止时间
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDue {
    /// 截止时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// 是否为全天任务
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_all_day: Option<bool>,
}

/// 任务开始时间
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStart {
    /// 开始时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    /// 是否为全天任务
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_all_day: Option<bool>,
}

/// 任务成员
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskMember {
    /// 成员ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 成员类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// 角色
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

/// 任务自定义完成配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCustomComplete {
    /// 完成模式
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<String>,
    /// 完成设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub complete_setting: Option<TaskCompleteSetting>,
}

/// 任务完成设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCompleteSetting {
    /// 子任务完成数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtask_count: Option<i32>,
}

/// 任务列表
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tasklist {
    /// 清单GUID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    /// 清单名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 创建者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<TaskMember>,
    /// 拥有者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<TaskMember>,
    /// 成员列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<TaskMember>>,
    /// 清单URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// 评论
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Comment {
    /// 评论ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 评论内容
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
    /// 父评论ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// 创建者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creator: Option<TaskMember>,
    /// 回复列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<Vec<Comment>>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// 附件
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Attachment {
    /// 附件GUID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    /// 文件名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 文件大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// 文件类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// 文件URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// 上传者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploader: Option<TaskMember>,
    /// 上传时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uploaded_at: Option<String>,
}

/// 自定义分组
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Section {
    /// 分组GUID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    /// 分组名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 是否是默认分组
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// 自定义字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomField {
    /// 字段GUID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    /// 字段名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 字段类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// 设置
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setting: Option<CustomFieldSetting>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// 自定义字段设置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFieldSetting {
    /// 选项列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<CustomFieldOption>>,
}

/// 自定义字段选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFieldOption {
    /// 选项GUID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    /// 选项名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 颜色索引
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color_index: Option<i32>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

/// 提醒时间
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reminder {
    /// 提醒ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// 相对触发时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub relative_fire_minute: Option<i32>,
}

/// 依赖关系
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    /// 依赖类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    /// 依赖任务GUID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_guid: Option<String>,
}

/// 清单活动订阅
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActivitySubscription {
    /// 订阅GUID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guid: Option<String>,
    /// 订阅名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 订阅者
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribers: Option<Vec<TaskMember>>,
    /// 包含已完成任务
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_completed: Option<bool>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
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
    fn test_task_serialization() {
        let task = Task {
            guid: Some("task123".to_string()),
            summary: Some("Test Task".to_string()),
            description: Some("This is a test task".to_string()),
            due: Some(TaskDue {
                timestamp: Some("2023-12-31T23:59:59Z".to_string()),
                is_all_day: Some(false),
            }),
            start: Some(TaskStart {
                timestamp: Some("2023-01-01T00:00:00Z".to_string()),
                is_all_day: Some(false),
            }),
            completed_at: None,
            members: Some(vec![TaskMember {
                id: Some("user123".to_string()),
                type_: Some("user".to_string()),
                role: Some("assignee".to_string()),
            }]),
            repeat_rule: None,
            custom_complete: None,
            created_at: Some("2023-01-01T00:00:00Z".to_string()),
            updated_at: Some("2023-01-01T01:00:00Z".to_string()),
            status: Some("pending".to_string()),
            workflow_state: Some("active".to_string()),
            source: Some(1),
            url: Some("https://example.com/task/123".to_string()),
        };

        let serialized = serde_json::to_string(&task).unwrap();
        let deserialized: Task = serde_json::from_str(&serialized).unwrap();

        assert_eq!(task.guid, deserialized.guid);
        assert_eq!(task.summary, deserialized.summary);
        assert_eq!(task.description, deserialized.description);
        assert_eq!(task.status, deserialized.status);
    }

    #[test]
    fn test_task_due_serialization() {
        let due = TaskDue {
            timestamp: Some("2023-12-31T23:59:59Z".to_string()),
            is_all_day: Some(true),
        };

        let serialized = serde_json::to_string(&due).unwrap();
        let deserialized: TaskDue = serde_json::from_str(&serialized).unwrap();

        assert_eq!(due.timestamp, deserialized.timestamp);
        assert_eq!(due.is_all_day, deserialized.is_all_day);
    }

    #[test]
    fn test_task_member_serialization() {
        let member = TaskMember {
            id: Some("user123".to_string()),
            type_: Some("user".to_string()),
            role: Some("assignee".to_string()),
        };

        let serialized = serde_json::to_string(&member).unwrap();
        let deserialized: TaskMember = serde_json::from_str(&serialized).unwrap();

        assert_eq!(member.id, deserialized.id);
        assert_eq!(member.type_, deserialized.type_);
        assert_eq!(member.role, deserialized.role);
    }

    #[test]
    fn test_activity_subscription_serialization() {
        let subscription = ActivitySubscription {
            guid: Some("sub123".to_string()),
            name: Some("Task Updates".to_string()),
            subscribers: Some(vec![TaskMember {
                id: Some("user123".to_string()),
                type_: Some("user".to_string()),
                role: Some("subscriber".to_string()),
            }]),
            include_completed: Some(true),
            created_at: Some("2023-01-01T00:00:00Z".to_string()),
            updated_at: Some("2023-01-01T01:00:00Z".to_string()),
        };

        let serialized = serde_json::to_string(&subscription).unwrap();
        let deserialized: ActivitySubscription = serde_json::from_str(&serialized).unwrap();

        assert_eq!(subscription.guid, deserialized.guid);
        assert_eq!(subscription.name, deserialized.name);
        assert_eq!(
            subscription.include_completed,
            deserialized.include_completed
        );
    }

    #[test]
    fn test_models_with_none_values() {
        let task = Task {
            guid: None,
            summary: None,
            description: None,
            due: None,
            start: None,
            completed_at: None,
            members: None,
            repeat_rule: None,
            custom_complete: None,
            created_at: None,
            updated_at: None,
            status: None,
            workflow_state: None,
            source: None,
            url: None,
        };

        let serialized = serde_json::to_string(&task).unwrap();
        assert_eq!(serialized, "{}");

        let deserialized: Task = serde_json::from_str("{}").unwrap();
        assert_eq!(deserialized.guid, None);
        assert_eq!(deserialized.summary, None);
    }

    #[test]
    fn test_debug_trait_for_models() {
        let task = Task {
            guid: Some("task123".to_string()),
            summary: Some("Test".to_string()),
            description: None,
            due: None,
            start: None,
            completed_at: None,
            members: None,
            repeat_rule: None,
            custom_complete: None,
            created_at: None,
            updated_at: None,
            status: None,
            workflow_state: None,
            source: None,
            url: None,
        };

        let debug_output = format!("{:?}", task);
        assert!(debug_output.contains("Task"));
        assert!(debug_output.contains("task123"));
    }

    #[test]
    fn test_clone_trait_for_models() {
        let original_member = TaskMember {
            id: Some("user123".to_string()),
            type_: Some("user".to_string()),
            role: Some("assignee".to_string()),
        };

        let cloned_member = original_member.clone();
        assert_eq!(original_member.id, cloned_member.id);
        assert_eq!(original_member.type_, cloned_member.type_);
        assert_eq!(original_member.role, cloned_member.role);
    }
}
