//! Task服务数据模型
//!
//! 定义任务管理相关的数据结构，包括任务、任务清单、自定义字段等核心实体。

use serde::{Deserialize, Serialize};

/// 用户ID类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserIdType {
    /// 开放平台用户ID
    OpenId,
    /// 用户ID
    UserId,
    /// 联合ID
    UnionId,
}

/// 任务实体
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
    pub status: Option<TaskStatus>,
    /// 优先级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<TaskPriority>,
    /// 任务清单ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasklist_id: Option<String>,
    /// 父任务ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_guid: Option<String>,
    /// 自定义字段值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<TaskCustomFieldValue>>,
    /// 附件列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<TaskAttachment>>,
    /// 评论列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Vec<TaskComment>>,
    /// 依赖任务
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<TaskDependency>>,
    /// 标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/// 任务截止时间
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDue {
    /// 日期字符串 (YYYY-MM-DD)
    pub date: Option<String>,
    /// 时间戳字符串
    pub timestamp: Option<String>,
}

/// 任务开始时间
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStart {
    /// 日期字符串 (YYYY-MM-DD)
    pub date: Option<String>,
    /// 时间戳字符串
    pub timestamp: Option<String>,
}

/// 任务状态
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    /// 未开始
    NotStarted,
    /// 进行中
    InProgress,
    /// 已完成
    Completed,
    /// 已取消
    Cancelled,
}

/// 任务优先级
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskPriority {
    /// 低
    Low,
    /// 中
    Medium,
    /// 高
    High,
    /// 紧急
    Urgent,
}

/// 任务成员
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskMember {
    /// 成员ID
    pub id: String,
    /// 成员类型
    pub r#type: String,
    /// 成员名称
    pub name: Option<String>,
    /// 头像URL
    pub avatar: Option<String>,
}

/// 自定义完成配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCustomComplete {
    /// 需要完成的子任务数量
    pub subtask_count: Option<i32>,
    /// 是否需要所有子任务完成
    pub require_all: Option<bool>,
}

/// 任务清单
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaskList {
    /// 清单ID
    pub guid: Option<String>,
    /// 清单名称
    pub title: Option<String>,
    /// 清单描述
    pub description: Option<String>,
    /// 创建时间
    pub created_at: Option<String>,
    /// 更新时间
    pub updated_at: Option<String>,
    /// 成员列表
    pub members: Option<Vec<TaskMember>>,
    /// 任务数量
    pub task_count: Option<i32>,
    /// 已完成任务数量
    pub completed_task_count: Option<i32>,
}

/// 自定义字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomField {
    /// 字段ID
    pub id: Option<String>,
    /// 字段名称
    pub name: Option<String>,
    /// 字段类型
    pub r#type: Option<CustomFieldType>,
    /// 是否必填
    pub required: Option<bool>,
    /// 字段选项（用于选择类型字段）
    pub options: Option<Vec<CustomFieldOption>>,
}

/// 自定义字段类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CustomFieldType {
    /// 文本
    Text,
    /// 数字
    Number,
    /// 日期
    Date,
    /// 单选
    SingleSelect,
    /// 多选
    MultiSelect,
    /// 布尔值
    Boolean,
    /// URL
    Url,
}

/// 自定义字段选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFieldOption {
    /// 选项ID
    pub id: Option<String>,
    /// 选项名称
    pub name: Option<String>,
    /// 选项值
    pub value: Option<String>,
}

/// 任务自定义字段值
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskCustomFieldValue {
    /// 字段ID
    pub field_id: Option<String>,
    /// 字段值
    pub value: Option<serde_json::Value>,
}

/// 任务附件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskAttachment {
    /// 附件ID
    pub id: Option<String>,
    /// 附件名称
    pub name: Option<String>,
    /// 附件URL
    pub url: Option<String>,
    /// 文件大小（字节）
    pub size: Option<i64>,
    /// 文件类型
    pub mime_type: Option<String>,
    /// 上传时间
    pub created_at: Option<String>,
    /// 上传者信息
    pub uploader: Option<TaskMember>,
}

/// 任务评论
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskComment {
    /// 评论ID
    pub id: Option<String>,
    /// 评论内容
    pub content: Option<String>,
    /// 评论者
    pub author: Option<TaskMember>,
    /// 创建时间
    pub created_at: Option<String>,
    /// 更新时间
    pub updated_at: Option<String>,
    /// 父评论ID（用于回复）
    pub parent_id: Option<String>,
    /// 子评论列表
    pub replies: Option<Vec<TaskComment>>,
}

/// 任务依赖
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDependency {
    /// 依赖任务ID
    pub task_guid: Option<String>,
    /// 依赖类型
    pub r#type: Option<DependencyType>,
}

/// 依赖类型
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DependencyType {
    /// 完成后开始
    FinishToStart,
    /// 开始后开始
    StartToStart,
    /// 完成后完成
    FinishToFinish,
    /// 开始后完成
    StartToFinish,
}

// ==================== 请求和响应模型 ====================

/// 创建任务请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTaskRequest {
    /// 任务标题
    pub summary: String,
    /// 任务描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 截止时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due: Option<TaskDue>,
    /// 开始时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<TaskStart>,
    /// 任务状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TaskStatus>,
    /// 优先级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<TaskPriority>,
    /// 任务清单ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasklist_id: Option<String>,
    /// 父任务ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_guid: Option<String>,
    /// 任务成员
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,
    /// 自定义字段值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<TaskCustomFieldValue>>,
    /// 标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/// 更新任务请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTaskRequest {
    /// 任务GUID
    pub task_guid: String,
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
    /// 任务状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TaskStatus>,
    /// 优先级
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<TaskPriority>,
    /// 任务成员
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,
    /// 自定义字段值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<TaskCustomFieldValue>>,
    /// 标签
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

/// 获取任务请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTaskRequest {
    /// 任务GUID
    pub task_guid: String,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<UserIdType>,
}

/// 删除任务请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteTaskRequest {
    /// 任务GUID
    pub task_guid: String,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<UserIdType>,
}

/// 任务列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTasksRequest {
    /// 任务清单ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tasklist_id: Option<String>,
    /// 任务状态过滤
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<TaskStatus>,
    /// 优先级过滤
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<TaskPriority>,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页面标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<UserIdType>,
}

/// 任务响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<Task>,
}

/// 任务列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskListResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<TaskListData>,
}

/// 任务列表数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskListData {
    /// 任务列表
    pub tasks: Vec<Task>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 是否有更多
    pub has_more: Option<bool>,
}

// ==================== 任务清单模型 ====================

/// 创建任务清单请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTaskListRequest {
    /// 清单名称
    pub title: String,
    /// 清单描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 成员列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,
}

/// 更新任务清单请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateTaskListRequest {
    /// 清单ID
    pub tasklist_guid: String,
    /// 清单名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// 清单描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 成员列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<String>>,
}

/// 获取任务清单请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTaskListRequest {
    /// 清单ID
    pub tasklist_guid: String,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<UserIdType>,
}

/// 删除任务清单请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteTaskListRequest {
    /// 清单ID
    pub tasklist_guid: String,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<UserIdType>,
}

/// 任务清单列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListTaskListsRequest {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页面标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<UserIdType>,
}

/// 任务清单响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskListDetailResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<TaskList>,
}

/// 任务清单列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskListsResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<TaskListsData>,
}

/// 任务清单列表数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskListsData {
    /// 任务清单列表
    pub tasklists: Vec<TaskList>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 是否有更多
    pub has_more: Option<bool>,
}

// ==================== 成员管理模型 ====================

/// 添加任务成员请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddTaskMembersRequest {
    /// 任务GUID
    pub task_guid: String,
    /// 成员ID列表
    pub member_ids: Vec<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<UserIdType>,
}

/// 移除任务成员请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveTaskMembersRequest {
    /// 任务GUID
    pub task_guid: String,
    /// 成员ID列表
    pub member_ids: Vec<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<UserIdType>,
}

/// 添加任务清单成员请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddTaskListMembersRequest {
    /// 清单ID
    pub tasklist_guid: String,
    /// 成员ID列表
    pub member_ids: Vec<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<UserIdType>,
}

/// 移除任务清单成员请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveTaskListMembersRequest {
    /// 清单ID
    pub tasklist_guid: String,
    /// 成员ID列表
    pub member_ids: Vec<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<UserIdType>,
}

/// 空响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyResponse {
    pub code: i32,
    pub msg: String,
}

// ==================== 评论模型 ====================

/// 创建评论请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCommentRequest {
    /// 任务GUID
    pub task_guid: String,
    /// 评论内容
    pub content: String,
    /// 父评论ID（用于回复）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<UserIdType>,
}

/// 获取评论列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCommentsRequest {
    /// 任务GUID
    pub task_guid: String,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页面标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<UserIdType>,
}

/// 评论响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<TaskComment>,
}

/// 评论列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentsResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<CommentsData>,
}

/// 评论列表数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentsData {
    /// 评论列表
    pub comments: Vec<TaskComment>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 是否有更多
    pub has_more: Option<bool>,
}

// ==================== 附件模型 ====================

/// 上传附件请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadAttachmentRequest {
    /// 任务GUID
    pub task_guid: String,
    /// 文件名
    pub file_name: String,
    /// 文件大小
    pub file_size: i64,
    /// 文件类型
    pub mime_type: String,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<UserIdType>,
}

/// 获取附件列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAttachmentsRequest {
    /// 任务GUID
    pub task_guid: String,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 页面标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<UserIdType>,
}

/// 附件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<TaskAttachment>,
}

/// 附件列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentsResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<AttachmentsData>,
}

/// 附件列表数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachmentsData {
    /// 附件列表
    pub attachments: Vec<TaskAttachment>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 是否有更多
    pub has_more: Option<bool>,
}

// ==================== 自定义字段模型 ====================

/// 创建自定义字段请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomFieldRequest {
    /// 清单ID
    pub tasklist_guid: String,
    /// 字段名称
    pub name: String,
    /// 字段类型
    pub r#type: CustomFieldType,
    /// 是否必填
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// 字段选项（用于选择类型字段）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<CustomFieldOption>>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<UserIdType>,
}

/// 获取自定义字段列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCustomFieldsRequest {
    /// 清单ID
    pub tasklist_guid: String,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<UserIdType>,
}

/// 自定义字段响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFieldResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<CustomField>,
}

/// 自定义字段列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFieldsResponse {
    pub code: i32,
    pub msg: String,
    pub data: Option<CustomFieldsData>,
}

/// 自定义字段列表数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomFieldsData {
    /// 自定义字段列表
    pub custom_fields: Vec<CustomField>,
}