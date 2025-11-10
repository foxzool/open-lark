//! Task v2 任务管理服务端点常量定义

// ===== 基础管理端点 =====

/// 任务附件上传
pub const TASK_V2_ATTACHMENTS_UPLOAD: &str = "/open-apis/task/v2/attachments/upload";

/// 任务附件管理
pub const TASK_V2_ATTACHMENTS: &str = "/open-apis/task/v2/attachments";

/// 任务分组管理
pub const TASK_V2_SECTIONS: &str = "/open-apis/task/v2/sections";

/// 任务管理
pub const TASK_V2_TASKS: &str = "/open-apis/task/v2/tasks";

/// 任务自定义字段管理
pub const TASK_V2_CUSTOM_FIELDS: &str = "/open-apis/task/v2/custom_fields";

/// 任务清单管理
pub const TASK_V2_TASKLISTS: &str = "/open-apis/task/v2/tasklists";

// ===== 参数化端点常量 =====

/// 获取/更新/删除特定附件
pub const TASK_V2_ATTACHMENT_GET: &str = "/open-apis/task/v2/attachments/{attachment_guid}";

/// 获取/更新/删除特定自定义字段
pub const TASK_V2_CUSTOM_FIELD_GET: &str = "/open-apis/task/v2/custom_fields/{custom_field_guid}";

/// 添加自定义字段选项
pub const TASK_V2_CUSTOM_FIELD_ADD: &str = "/open-apis/task/v2/custom_fields/{custom_field_guid}/add";

/// 移除自定义字段选项
pub const TASK_V2_CUSTOM_FIELD_REMOVE: &str = "/open-apis/task/v2/custom_fields/{custom_field_guid}/remove";

/// 自定义字段选项管理
pub const TASK_V2_CUSTOM_FIELD_OPTIONS: &str = "/open-apis/task/v2/custom_fields/{custom_field_guid}/options";

/// 获取/更新/删除特定自定义字段选项
pub const TASK_V2_CUSTOM_FIELD_OPTION_GET: &str = "/open-apis/task/v2/custom_fields/{custom_field_guid}/options/{option_guid}";

/// 获取/更新/删除特定分组
pub const TASK_V2_SECTION_GET: &str = "/open-apis/task/v2/sections/{section_guid}";

/// 分组任务管理
pub const TASK_V2_SECTION_TASKS: &str = "/open-apis/task/v2/sections/{section_guid}/tasks";

/// 获取/更新/删除特定任务清单
pub const TASK_V2_TASKLIST_GET: &str = "/open-apis/task/v2/tasklists/{tasklist_guid}";

/// 任务清单添加成员
pub const TASK_V2_TASKLIST_ADD_MEMBERS: &str = "/open-apis/task/v2/tasklists/{tasklist_guid}/add_members";

/// 任务清单移除成员
pub const TASK_V2_TASKLIST_REMOVE_MEMBERS: &str = "/open-apis/task/v2/tasklists/{tasklist_guid}/remove_members";

/// 任务清单任务管理
pub const TASK_V2_TASKLIST_TASKS: &str = "/open-apis/task/v2/tasklists/{tasklist_guid}/tasks";

/// 任务清单活动订阅
pub const TASK_V2_TASKLIST_ACTIVITY_SUBSCRIPTIONS: &str = "/open-apis/task/v2/tasklists/{tasklist_guid}/activity_subscriptions";

/// 获取/更新/删除特定活动订阅
pub const TASK_V2_TASKLIST_ACTIVITY_SUBSCRIPTION_GET: &str = "/open-apis/task/v2/tasklists/{tasklist_guid}/activity_subscriptions/{activity_subscription_guid}";

/// 获取/更新/删除特定任务
pub const TASK_V2_TASK_GET: &str = "/open-apis/task/v2/tasks/{task_guid}";

/// 任务添加成员
pub const TASK_V2_TASK_ADD_MEMBERS: &str = "/open-apis/task/v2/tasks/{task_guid}/add_members";

/// 任务移除成员
pub const TASK_V2_TASK_REMOVE_MEMBERS: &str = "/open-apis/task/v2/tasks/{task_guid}/remove_members";

/// 任务添加提醒
pub const TASK_V2_TASK_ADD_REMINDERS: &str = "/open-apis/task/v2/tasks/{task_guid}/add_reminders";

/// 任务移除提醒
pub const TASK_V2_TASK_REMOVE_REMINDERS: &str = "/open-apis/task/v2/tasks/{task_guid}/remove_reminders";

/// 任务加入清单
pub const TASK_V2_TASK_ADD_TASKLIST: &str = "/open-apis/task/v2/tasks/{task_guid}/add_tasklist";

/// 任务添加依赖
pub const TASK_V2_TASK_ADD_DEPENDENCIES: &str = "/open-apis/task/v2/tasks/{task_guid}/add_dependencies";

/// 任务移除依赖
pub const TASK_V2_TASK_REMOVE_DEPENDENCIES: &str = "/open-apis/task/v2/tasks/{task_guid}/remove_dependencies";

/// 任务评论管理
pub const TASK_V2_TASK_COMMENTS: &str = "/open-apis/task/v2/tasks/{task_guid}/comments";

/// 获取/更新/删除特定任务评论
pub const TASK_V2_TASK_COMMENT_GET: &str = "/open-apis/task/v2/tasks/{task_guid}/comments/{comment_id}";

/// 任务子任务管理
pub const TASK_V2_TASK_SUBTASKS: &str = "/open-apis/task/v2/tasks/{task_guid}/subtasks";