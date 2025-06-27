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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Serialize, Deserialize)]
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
