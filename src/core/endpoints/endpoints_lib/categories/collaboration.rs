//! 协作工具服务端点

/// 协作工具服务端点
pub struct Collaboration;

impl Collaboration {
    /// / 获取审批定义 (需要使用 EndpointBuilder::replace_param 替换 {approval_code})
    pub const APPROVAL_V4_APPROVALS: &'static str = "/open-apis/approval/v4/approvals";
    /// ===== 外部审批端点 =====
    pub const APPROVAL_V4_APPROVAL_GET: &'static str =
        "/open-apis/approval/v4/approvals/{approval_code}";
    /// / 获取外部审批 (需要使用 EndpointBuilder::replace_param 替换 {approval_code})
    pub const APPROVAL_V4_EXTERNAL_APPROVALS: &'static str =
        "/open-apis/approval/v4/external_approvals";
    /// ===== 文件上传端点 =====
    pub const APPROVAL_V4_EXTERNAL_APPROVAL_GET: &'static str =
        "/open-apis/approval/v4/external_approvals/{approval_code}";
    /// / 获取审批实例列表
    pub const APPROVAL_V4_INSTANCES: &'static str = "/open-apis/approval/v4/instances";
    /// / 获取审批实例 (需要使用 EndpointBuilder::replace_param 替换 {instance_code})
    pub const APPROVAL_V4_INSTANCES_LIST: &'static str = "/open-apis/approval/v4/instances";
    /// / 取消审批实例 (需要使用 EndpointBuilder::replace_param 替换 {instance_code})
    pub const APPROVAL_V4_INSTANCE_GET: &'static str =
        "/open-apis/approval/v4/instances/{instance_code}";
    /// / 抄送审批实例 (需要使用 EndpointBuilder::replace_param 替换 {instance_code})
    pub const APPROVAL_V4_INSTANCE_CANCEL: &'static str =
        "/open-apis/approval/v4/instances/{instance_code}/cancel";
    /// / 预览审批实例
    pub const APPROVAL_V4_INSTANCE_CC: &'static str =
        "/open-apis/approval/v4/instances/{instance_code}/cc";
    /// ===== 实例评论端点 =====
    pub const APPROVAL_V4_INSTANCE_PREVIEW: &'static str =
        "/open-apis/approval/v4/instances/preview";
    /// / 删除实例评论 (需要使用 EndpointBuilder::replace_param 替换 {instance_code} 和 {comment_id})
    pub const APPROVAL_V4_INSTANCE_COMMENTS_CREATE: &'static str =
        "/open-apis/approval/v4/instances/{instance_code}/comments";
    /// / 获取实例评论列表 (需要使用 EndpointBuilder::replace_param 替换 {instance_code})
    pub const APPROVAL_V4_INSTANCE_COMMENT_DELETE: &'static str =
        "/open-apis/approval/v4/instances/{instance_code}/comments/{comment_id}";
    /// / 回复实例评论 (需要使用 EndpointBuilder::replace_param 替换 {instance_code})
    pub const APPROVAL_V4_INSTANCE_COMMENTS_LIST: &'static str =
        "/open-apis/approval/v4/instances/{instance_code}/comments";
    /// ===== 外部实例端点 =====
    pub const APPROVAL_V4_INSTANCE_COMMENTS_REPLY: &'static str =
        "/open-apis/approval/v4/instances/{instance_code}/comments";
    /// / 校验外部实例 (需要使用 EndpointBuilder::replace_param 替换 {instance_code})
    pub const APPROVAL_V4_EXTERNAL_INSTANCES: &'static str =
        "/open-apis/approval/v4/external_instances";
    /// ===== 外部任务端点 =====
    pub const APPROVAL_V4_EXTERNAL_INSTANCE_CHECK: &'static str =
        "/open-apis/approval/v4/external_instances/{instance_code}/check";
    /// ===== 消息管理端点 =====
    pub const APPROVAL_V4_EXTERNAL_TASKS: &'static str = "/open-apis/approval/v4/external_tasks";
    /// / 搜索审批任务
    pub const APPROVAL_V4_INSTANCES_SEARCH: &'static str =
        "/open-apis/approval/v4/instances/search";
    /// / 搜索抄送实例
    pub const APPROVAL_V4_TASKS_SEARCH: &'static str = "/open-apis/approval/v4/tasks/search";
    /// / 搜索审批定义
    pub const APPROVAL_V4_INSTANCES_SEARCH_CC: &'static str =
        "/open-apis/approval/v4/instances/search_cc";
    /// / 查询任务
    pub const APPROVAL_V4_APPROVALS_SEARCH: &'static str =
        "/open-apis/approval/v4/approvals/search";
    /// ===== 任务处理端点 =====
    pub const APPROVAL_V4_TASKS_QUERY: &'static str = "/open-apis/approval/v4/tasks/query";
    /// / 拒绝任务 (需要使用 EndpointBuilder::replace_param 替换 {task_id})
    pub const APPROVAL_V4_TASK_APPROVE: &'static str =
        "/open-apis/approval/v4/tasks/{task_id}/approve";
    /// / 转交任务 (需要使用 EndpointBuilder::replace_param 替换 {task_id})
    pub const APPROVAL_V4_TASK_REJECT: &'static str =
        "/open-apis/approval/v4/tasks/{task_id}/reject";
    /// / 指定回退任务 (需要使用 EndpointBuilder::replace_param 替换 {task_id})
    pub const APPROVAL_V4_TASK_TRANSFER: &'static str =
        "/open-apis/approval/v4/tasks/{task_id}/transfer";
    /// / 加签任务 (需要使用 EndpointBuilder::replace_param 替换 {task_id})
    pub const APPROVAL_V4_TASK_SPECIFIED_ROLLBACK: &'static str =
        "/open-apis/approval/v4/tasks/{task_id}/specified_rollback";
    /// / 重新提交任务 (需要使用 EndpointBuilder::replace_param 替换 {task_id})
    pub const APPROVAL_V4_TASK_ADD_SIGN: &'static str =
        "/open-apis/approval/v4/tasks/{task_id}/add_sign";
    /// ==================== 身份验证服务端点 ====================
    pub const APPROVAL_V4_TASK_RESUBMIT: &'static str =
        "/open-apis/approval/v4/tasks/{task_id}/resubmit";
    /// / 获取日历
    pub const CALENDAR_CREATE: &'static str = "/open-apis/calendar/v4/calendars";
    /// / 更新日历
    pub const CALENDAR_GET: &'static str = "/open-apis/calendar/v4/calendars/{calendar_id}";
    /// / 删除日历
    pub const CALENDAR_UPDATE: &'static str = "/open-apis/calendar/v4/calendars/{calendar_id}";
    /// / 获取日历列表
    pub const CALENDAR_DELETE: &'static str = "/open-apis/calendar/v4/calendars/{calendar_id}";
    /// / 设置主日历
    pub const CALENDAR_LIST: &'static str = "/open-apis/calendar/v4/calendars";
    /// / 搜索日历
    pub const CALENDAR_PRIMARY: &'static str =
        "/open-apis/calendar/v4/calendars/{calendar_id}/primary";
    /// 日程管理
    pub const CALENDAR_SEARCH: &'static str =
        "/open-apis/calendar/v4/calendars/{calendar_id}/search";
    /// / 获取日程
    pub const CALENDAR_EVENT_CREATE: &'static str =
        "/open-apis/calendar/v4/calendars/{calendar_id}/events";
    /// / 更新日程
    pub const CALENDAR_EVENT_GET: &'static str =
        "/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}";
    /// / 删除日程
    pub const CALENDAR_EVENT_UPDATE: &'static str =
        "/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}";
    /// / 获取日程列表
    pub const CALENDAR_EVENT_DELETE: &'static str =
        "/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}";
    /// / 回复日程邀请
    pub const CALENDAR_EVENT_LIST: &'static str =
        "/open-apis/calendar/v4/calendars/{calendar_id}/events";
    /// / 搜索日程
    pub const CALENDAR_EVENT_REPLY: &'static str =
        "/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}/reply";
    /// ==================== AI Document AI 相关端点 ====================
    pub const CALENDAR_EVENT_SEARCH: &'static str =
        "/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}/search";
    /// / 任务附件管理
    pub const TASK_V2_ATTACHMENTS_UPLOAD: &'static str = "/open-apis/task/v2/attachments/upload";
    /// / 任务分组管理
    pub const TASK_V2_ATTACHMENTS: &'static str = "/open-apis/task/v2/attachments";
    /// / 任务管理
    pub const TASK_V2_SECTIONS: &'static str = "/open-apis/task/v2/sections";
    /// / 任务自定义字段管理
    pub const TASK_V2_TASKS: &'static str = "/open-apis/task/v2/tasks";
    /// / 任务清单管理
    pub const TASK_V2_CUSTOM_FIELDS: &'static str = "/open-apis/task/v2/custom_fields";
    /// 参数化端点常量
    pub const TASK_V2_TASKLISTS: &'static str = "/open-apis/task/v2/tasklists";
    /// / 获取/更新/删除特定自定义字段
    pub const TASK_V2_ATTACHMENT_GET: &'static str =
        "/open-apis/task/v2/attachments/{attachment_guid}";
    /// / 添加自定义字段选项
    pub const TASK_V2_CUSTOM_FIELD_GET: &'static str =
        "/open-apis/task/v2/custom_fields/{custom_field_guid}";
    /// / 移除自定义字段选项
    pub const TASK_V2_CUSTOM_FIELD_ADD: &'static str =
        "/open-apis/task/v2/custom_fields/{custom_field_guid}/add";
    /// / 自定义字段选项管理
    pub const TASK_V2_CUSTOM_FIELD_REMOVE: &'static str =
        "/open-apis/task/v2/custom_fields/{custom_field_guid}/remove";
    /// / 获取/更新/删除特定自定义字段选项
    pub const TASK_V2_CUSTOM_FIELD_OPTIONS: &'static str =
        "/open-apis/task/v2/custom_fields/{custom_field_guid}/options";
    /// / 获取/更新/删除特定分组
    pub const TASK_V2_CUSTOM_FIELD_OPTION_GET: &'static str =
        "/open-apis/task/v2/custom_fields/{custom_field_guid}/options/{option_guid}";
    /// / 分组任务管理
    pub const TASK_V2_SECTION_GET: &'static str = "/open-apis/task/v2/sections/{section_guid}";
    /// / 获取/更新/删除特定任务清单
    pub const TASK_V2_SECTION_TASKS: &'static str =
        "/open-apis/task/v2/sections/{section_guid}/tasks";
    /// / 任务清单添加成员
    pub const TASK_V2_TASKLIST_GET: &'static str = "/open-apis/task/v2/tasklists/{tasklist_guid}";
    /// / 任务清单移除成员
    pub const TASK_V2_TASKLIST_ADD_MEMBERS: &'static str =
        "/open-apis/task/v2/tasklists/{tasklist_guid}/add_members";
    /// / 任务清单任务管理
    pub const TASK_V2_TASKLIST_REMOVE_MEMBERS: &'static str =
        "/open-apis/task/v2/tasklists/{tasklist_guid}/remove_members";
    /// / 任务清单活动订阅
    pub const TASK_V2_TASKLIST_TASKS: &'static str =
        "/open-apis/task/v2/tasklists/{tasklist_guid}/tasks";
    /// / 获取/更新/删除特定活动订阅
    pub const TASK_V2_TASKLIST_ACTIVITY_SUBSCRIPTIONS: &'static str =
        "/open-apis/task/v2/tasklists/{tasklist_guid}/activity_subscriptions";
    /// / 获取/更新/删除特定任务
    pub const TASK_V2_TASKLIST_ACTIVITY_SUBSCRIPTION_GET: &'static str = "/open-apis/task/v2/tasklists/{tasklist_guid}/activity_subscriptions/{activity_subscription_guid}";
    /// / 任务添加成员
    pub const TASK_V2_TASK_GET: &'static str = "/open-apis/task/v2/tasks/{task_guid}";
    /// / 任务移除成员
    pub const TASK_V2_TASK_ADD_MEMBERS: &'static str =
        "/open-apis/task/v2/tasks/{task_guid}/add_members";
    /// / 任务添加提醒
    pub const TASK_V2_TASK_REMOVE_MEMBERS: &'static str =
        "/open-apis/task/v2/tasks/{task_guid}/remove_members";
    /// / 任务移除提醒
    pub const TASK_V2_TASK_ADD_REMINDERS: &'static str =
        "/open-apis/task/v2/tasks/{task_guid}/add_reminders";
    /// / 任务加入清单
    pub const TASK_V2_TASK_REMOVE_REMINDERS: &'static str =
        "/open-apis/task/v2/tasks/{task_guid}/remove_reminders";
    /// / 任务添加依赖
    pub const TASK_V2_TASK_ADD_TASKLIST: &'static str =
        "/open-apis/task/v2/tasks/{task_guid}/add_tasklist";
    /// / 任务移除依赖
    pub const TASK_V2_TASK_ADD_DEPENDENCIES: &'static str =
        "/open-apis/task/v2/tasks/{task_guid}/add_dependencies";
    /// / 任务评论管理
    pub const TASK_V2_TASK_REMOVE_DEPENDENCIES: &'static str =
        "/open-apis/task/v2/tasks/{task_guid}/remove_dependencies";
    /// / 获取/更新/删除特定任务评论
    pub const TASK_V2_TASK_COMMENTS: &'static str = "/open-apis/task/v2/tasks/{task_guid}/comments";
    /// / 任务子任务管理
    pub const TASK_V2_TASK_COMMENT_GET: &'static str =
        "/open-apis/task/v2/tasks/{task_guid}/comments/{comment_id}";
    /// ==================== Payroll 薪酬管理相关端点 ====================
    pub const TASK_V2_TASK_SUBTASKS: &'static str = "/open-apis/task/v2/tasks/{task_guid}/subtasks";
    /// / 同意用户任务
    pub const APASS_V1_FLOW_USER_TASK_QUERY: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/flow/user_task/query";
    /// / 拒绝用户任务
    pub const APASS_V1_FLOW_USER_TASK_AGREE: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/flow/user_task/{task_id}/agree";
    /// / 转发用户任务
    pub const APASS_V1_FLOW_USER_TASK_REJECT: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/flow/user_task/{task_id}/reject";
    /// / 添加用户任务处理人
    pub const APASS_V1_FLOW_USER_TASK_TRANSFER: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/flow/user_task/{task_id}/transfer";
    /// / 抄送用户任务
    pub const APASS_V1_FLOW_USER_TASK_ADD_ASSIGNEE: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/flow/user_task/{task_id}/add_assignee";
    /// / 催办用户任务
    pub const APASS_V1_FLOW_USER_TASK_CC: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/flow/user_task/{task_id}/cc";
    /// / 取消用户任务
    pub const APASS_V1_FLOW_USER_TASK_EXPEDITING: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/flow/user_task/{task_id}/expediting";
    /// / 查询用户任务回退点
    pub const APASS_V1_FLOW_USER_TASK_CANCEL: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/flow/user_task/{task_id}/cancel";
    /// / 回退用户任务
    pub const APASS_V1_FLOW_USER_TASK_ROLLBACK_POINTS: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/flow/user_task/{task_id}/rollback_points";
    /// / 获取用户任务群聊
    pub const APASS_V1_FLOW_USER_TASK_ROLLBACK: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/flow/user_task/{task_id}/rollback";
    /// / 阶段任务分页查询
    pub const PERFORMANCE_V1_STAGE_TASK_FIND_BY_USER_LIST: &'static str =
        "/open-apis/performance/v1/stage_task/find_by_user_list";
    /// / 指标详情查询
    pub const PERFORMANCE_V1_STAGE_TASK_FIND_BY_PAGE: &'static str =
        "/open-apis/performance/v1/stage_task/find_by_page";
    /// / 日历详情操作
    pub const CALENDAR_V4_CALENDARS: &'static str = "/open-apis/calendar/v4/calendars";
}
