//! 任务 API 端点定义（类型安全枚举系统）
//!
//! 本模块提供基于枚举的 API 端点定义，用于生产代码中的类型安全调用。

/// 任务 API V1 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum TaskApiV1 {
    /// 创建任务
    TaskCreate,
}

impl TaskApiV1 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            TaskApiV1::TaskCreate => "/open-apis/task/v1/tasks".to_string(),
        }
    }
}

/// 任务 API V2 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum TaskApiV2 {
    /// 创建任务
    TaskCreate,
    /// 获取任务详情
    TaskGet(String),
    /// 更新任务
    TaskUpdate(String),
    /// 删除任务
    TaskDelete(String),
    /// 完成任务
    TaskComplete(String),
    /// 取消完成任务
    TaskUncomplete(String),
    /// 获取任务列表
    TaskList,
    /// 创建任务清单
    TasklistCreate,
    /// 获取任务清单详情
    TasklistGet(String),
    /// 更新任务清单
    TasklistUpdate(String),
    /// 删除任务清单
    TasklistDelete(String),
    /// 获取任务清单列表
    TasklistList,
    /// 创建分组
    SectionCreate(String),
    /// 获取分组详情
    SectionGet(String, String),
    /// 更新分组
    SectionUpdate(String, String),
    /// 删除分组
    SectionDelete(String, String),
    /// 获取分组列表
    SectionList(String),
    /// 创建自定义字段
    CustomFieldCreate(String),
    /// 获取自定义字段详情
    CustomFieldGet(String, String),
    /// 更新自定义字段
    CustomFieldUpdate(String, String),
    /// 删除自定义字段
    CustomFieldDelete(String, String),
    /// 获取自定义字段列表
    CustomFieldList(String),
    /// 创建评论
    CommentCreate(String),
    /// 获取评论详情
    CommentGet(String, String),
    /// 更新评论
    CommentUpdate(String, String),
    /// 删除评论
    CommentDelete(String, String),
    /// 获取评论列表
    CommentList(String),
    /// 上传附件
    AttachmentUpload(String),
    /// 删除附件
    AttachmentDelete(String, String),

    // 子任务相关
    /// 创建子任务
    SubtaskCreate(String),
    /// 获取子任务列表
    SubtaskList(String),

    // 任务相关扩展
    /// 添加任务到任务清单
    TaskAddTasklist(String),
    /// 从任务清单移除任务
    TaskRemoveTasklist(String),
    /// 获取任务所属任务清单列表
    TaskGetTasklists(String),
    /// 添加任务成员
    TaskAddMembers(String),
    /// 移除任务成员
    TaskRemoveMembers(String),
    /// 添加任务提醒
    TaskAddReminders(String),
    /// 移除任务提醒
    TaskRemoveReminders(String),
    /// 添加任务依赖
    TaskAddDependencies(String),
    /// 移除任务依赖
    TaskRemoveDependencies(String),

    // 任务清单相关扩展
    /// 获取任务清单中的任务列表
    TasklistGetTasks(String),
    /// 添加任务清单成员
    TasklistAddMembers(String),
    /// 移除任务清单成员
    TasklistRemoveMembers(String),

    // 分组相关扩展
    /// 获取分组中的任务列表
    SectionGetTasks(String),

    // 活动订阅相关
    /// 创建活动订阅
    ActivitySubscriptionCreate(String),
    /// 获取活动订阅详情
    ActivitySubscriptionGet(String, String),
    /// 更新活动订阅
    ActivitySubscriptionUpdate(String, String),
    /// 删除活动订阅
    ActivitySubscriptionDelete(String, String),
    /// 获取活动订阅列表
    ActivitySubscriptionList(String),

    // 自定义字段选项相关
    /// 创建自定义字段选项
    CustomFieldOptionCreate(String),
    /// 更新自定义字段选项
    CustomFieldOptionUpdate(String, String),

    // 附件相关扩展
    /// 获取附件详情
    AttachmentGet(String),
    /// 获取附件列表
    AttachmentList,

    // 自定义字段 add/remove
    /// 添加自定义字段到任务清单
    CustomFieldAdd(String),
    /// 从任务清单移除自定义字段
    CustomFieldRemove(String),
}

impl TaskApiV2 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            // 任务相关
            TaskApiV2::TaskCreate => "/open-apis/task/v2/tasks".to_string(),
            TaskApiV2::TaskGet(task_guid) => {
                format!("/open-apis/task/v2/tasks/{}", task_guid)
            }
            TaskApiV2::TaskUpdate(task_guid) => {
                format!("/open-apis/task/v2/tasks/{}", task_guid)
            }
            TaskApiV2::TaskDelete(task_guid) => {
                format!("/open-apis/task/v2/tasks/{}", task_guid)
            }
            TaskApiV2::TaskComplete(task_guid) => {
                format!("/open-apis/task/v2/tasks/{}/complete", task_guid)
            }
            TaskApiV2::TaskUncomplete(task_guid) => {
                format!("/open-apis/task/v2/tasks/{}/uncomplete", task_guid)
            }
            TaskApiV2::TaskList => "/open-apis/task/v2/tasks".to_string(),

            // 任务清单相关
            TaskApiV2::TasklistCreate => "/open-apis/task/v2/tasklists".to_string(),
            TaskApiV2::TasklistGet(tasklist_guid) => {
                format!("/open-apis/task/v2/tasklists/{}", tasklist_guid)
            }
            TaskApiV2::TasklistUpdate(tasklist_guid) => {
                format!("/open-apis/task/v2/tasklists/{}", tasklist_guid)
            }
            TaskApiV2::TasklistDelete(tasklist_guid) => {
                format!("/open-apis/task/v2/tasklists/{}", tasklist_guid)
            }
            TaskApiV2::TasklistList => "/open-apis/task/v2/tasklists".to_string(),

            // 分组相关
            TaskApiV2::SectionCreate(tasklist_guid) => {
                format!("/open-apis/task/v2/tasklists/{}/sections", tasklist_guid)
            }
            TaskApiV2::SectionGet(tasklist_guid, section_guid) => {
                format!(
                    "/open-apis/task/v2/tasklists/{}/sections/{}",
                    tasklist_guid, section_guid
                )
            }
            TaskApiV2::SectionUpdate(tasklist_guid, section_guid) => {
                format!(
                    "/open-apis/task/v2/tasklists/{}/sections/{}",
                    tasklist_guid, section_guid
                )
            }
            TaskApiV2::SectionDelete(tasklist_guid, section_guid) => {
                format!(
                    "/open-apis/task/v2/tasklists/{}/sections/{}",
                    tasklist_guid, section_guid
                )
            }
            TaskApiV2::SectionList(tasklist_guid) => {
                format!("/open-apis/task/v2/tasklists/{}/sections", tasklist_guid)
            }

            // 自定义字段相关
            TaskApiV2::CustomFieldCreate(tasklist_guid) => {
                format!(
                    "/open-apis/task/v2/tasklists/{}/custom_fields",
                    tasklist_guid
                )
            }
            TaskApiV2::CustomFieldGet(tasklist_guid, field_guid) => {
                format!(
                    "/open-apis/task/v2/tasklists/{}/custom_fields/{}",
                    tasklist_guid, field_guid
                )
            }
            TaskApiV2::CustomFieldUpdate(tasklist_guid, field_guid) => {
                format!(
                    "/open-apis/task/v2/tasklists/{}/custom_fields/{}",
                    tasklist_guid, field_guid
                )
            }
            TaskApiV2::CustomFieldDelete(tasklist_guid, field_guid) => {
                format!(
                    "/open-apis/task/v2/tasklists/{}/custom_fields/{}",
                    tasklist_guid, field_guid
                )
            }
            TaskApiV2::CustomFieldList(tasklist_guid) => {
                format!(
                    "/open-apis/task/v2/tasklists/{}/custom_fields",
                    tasklist_guid
                )
            }

            // 评论相关
            TaskApiV2::CommentCreate(task_guid) => {
                format!("/open-apis/task/v2/tasks/{}/comments", task_guid)
            }
            TaskApiV2::CommentGet(task_guid, comment_guid) => {
                format!(
                    "/open-apis/task/v2/tasks/{}/comments/{}",
                    task_guid, comment_guid
                )
            }
            TaskApiV2::CommentUpdate(task_guid, comment_guid) => {
                format!(
                    "/open-apis/task/v2/tasks/{}/comments/{}",
                    task_guid, comment_guid
                )
            }
            TaskApiV2::CommentDelete(task_guid, comment_guid) => {
                format!(
                    "/open-apis/task/v2/tasks/{}/comments/{}",
                    task_guid, comment_guid
                )
            }
            TaskApiV2::CommentList(task_guid) => {
                format!("/open-apis/task/v2/tasks/{}/comments", task_guid)
            }

            // 附件相关
            TaskApiV2::AttachmentUpload(task_guid) => {
                format!("/open-apis/task/v2/tasks/{}/attachments", task_guid)
            }
            TaskApiV2::AttachmentDelete(task_guid, attachment_guid) => {
                format!(
                    "/open-apis/task/v2/tasks/{}/attachments/{}",
                    task_guid, attachment_guid
                )
            }

            // 子任务相关
            TaskApiV2::SubtaskCreate(task_guid) => {
                format!("/open-apis/task/v2/tasks/{}/subtasks", task_guid)
            }
            TaskApiV2::SubtaskList(task_guid) => {
                format!("/open-apis/task/v2/tasks/{}/subtasks", task_guid)
            }

            // 任务相关扩展
            TaskApiV2::TaskAddTasklist(task_guid) => {
                format!("/open-apis/task/v2/tasks/{}/add_tasklist", task_guid)
            }
            TaskApiV2::TaskRemoveTasklist(task_guid) => {
                format!("/open-apis/task/v2/tasks/{}/remove_tasklist", task_guid)
            }
            TaskApiV2::TaskGetTasklists(task_guid) => {
                format!("/open-apis/task/v2/tasks/{}/tasklists", task_guid)
            }
            TaskApiV2::TaskAddMembers(task_guid) => {
                format!("/open-apis/task/v2/tasks/{}/add_members", task_guid)
            }
            TaskApiV2::TaskRemoveMembers(task_guid) => {
                format!("/open-apis/task/v2/tasks/{}/remove_members", task_guid)
            }
            TaskApiV2::TaskAddReminders(task_guid) => {
                format!("/open-apis/task/v2/tasks/{}/add_reminders", task_guid)
            }
            TaskApiV2::TaskRemoveReminders(task_guid) => {
                format!("/open-apis/task/v2/tasks/{}/remove_reminders", task_guid)
            }
            TaskApiV2::TaskAddDependencies(task_guid) => {
                format!("/open-apis/task/v2/tasks/{}/add_dependencies", task_guid)
            }
            TaskApiV2::TaskRemoveDependencies(task_guid) => {
                format!("/open-apis/task/v2/tasks/{}/remove_dependencies", task_guid)
            }

            // 任务清单相关扩展
            TaskApiV2::TasklistGetTasks(tasklist_guid) => {
                format!("/open-apis/task/v2/tasklists/{}/tasks", tasklist_guid)
            }
            TaskApiV2::TasklistAddMembers(tasklist_guid) => {
                format!("/open-apis/task/v2/tasklists/{}/add_members", tasklist_guid)
            }
            TaskApiV2::TasklistRemoveMembers(tasklist_guid) => {
                format!(
                    "/open-apis/task/v2/tasklists/{}/remove_members",
                    tasklist_guid
                )
            }

            // 分组相关扩展
            TaskApiV2::SectionGetTasks(section_guid) => {
                format!("/open-apis/task/v2/sections/{}/tasks", section_guid)
            }

            // 活动订阅相关
            TaskApiV2::ActivitySubscriptionCreate(tasklist_guid) => {
                format!(
                    "/open-apis/task/v2/tasklists/{}/activity_subscriptions",
                    tasklist_guid
                )
            }
            TaskApiV2::ActivitySubscriptionGet(tasklist_guid, subscription_guid) => {
                format!(
                    "/open-apis/task/v2/tasklists/{}/activity_subscriptions/{}",
                    tasklist_guid, subscription_guid
                )
            }
            TaskApiV2::ActivitySubscriptionUpdate(tasklist_guid, subscription_guid) => {
                format!(
                    "/open-apis/task/v2/tasklists/{}/activity_subscriptions/{}",
                    tasklist_guid, subscription_guid
                )
            }
            TaskApiV2::ActivitySubscriptionDelete(tasklist_guid, subscription_guid) => {
                format!(
                    "/open-apis/task/v2/tasklists/{}/activity_subscriptions/{}",
                    tasklist_guid, subscription_guid
                )
            }
            TaskApiV2::ActivitySubscriptionList(tasklist_guid) => {
                format!(
                    "/open-apis/task/v2/tasklists/{}/activity_subscriptions",
                    tasklist_guid
                )
            }

            // 自定义字段选项相关
            TaskApiV2::CustomFieldOptionCreate(custom_field_guid) => {
                format!(
                    "/open-apis/task/v2/custom_fields/{}/options",
                    custom_field_guid
                )
            }
            TaskApiV2::CustomFieldOptionUpdate(custom_field_guid, option_guid) => {
                format!(
                    "/open-apis/task/v2/custom_fields/{}/options/{}",
                    custom_field_guid, option_guid
                )
            }

            // 附件相关扩展
            TaskApiV2::AttachmentGet(attachment_guid) => {
                format!("/open-apis/task/v2/attachments/{}", attachment_guid)
            }
            TaskApiV2::AttachmentList => "/open-apis/task/v2/attachments".to_string(),

            // 自定义字段 add/remove
            TaskApiV2::CustomFieldAdd(custom_field_guid) => {
                format!("/open-apis/task/v2/custom_fields/{}/add", custom_field_guid)
            }
            TaskApiV2::CustomFieldRemove(custom_field_guid) => {
                format!(
                    "/open-apis/task/v2/custom_fields/{}/remove",
                    custom_field_guid
                )
            }
        }
    }
}
