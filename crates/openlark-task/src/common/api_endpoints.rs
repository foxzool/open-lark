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
        }
    }
}
