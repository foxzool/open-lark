//! approval 服务端点常量定义

/// 审批列表
pub const APPROVAL_V4_APPROVALS: &str = "/open-apis/approval/v4/approvals";

/// 审批详情
pub const APPROVAL_V4_APPROVAL_GET: &str = "/open-apis/approval/v4/approvals/{approval_id}";

/// 外部审批列表
pub const APPROVAL_V4_EXTERNAL_APPROVALS: &str = "/open-apis/approval/v4/external_approvals";

/// 外部审批详情
pub const APPROVAL_V4_EXTERNAL_APPROVAL_GET: &str =
    "/open-apis/approval/v4/external_approvals/{approval_id}";

/// 外部实例列表
pub const APPROVAL_V4_EXTERNAL_INSTANCES: &str = "/open-apis/approval/v4/external_instances";

/// 外部实例检查
pub const APPROVAL_V4_EXTERNAL_INSTANCE_CHECK: &str =
    "/open-apis/approval/v4/external_instances/{instance_id}/check";

/// 外部任务列表
pub const APPROVAL_V4_EXTERNAL_TASKS: &str = "/open-apis/approval/v4/external_tasks";

/// 文件上传
pub const APPROVAL_V4_FILE_UPLOAD: &str = "/open-apis/approval/v4/files/upload";

/// 审批实例列表
pub const APPROVAL_V4_INSTANCES: &str = "/open-apis/approval/v4/instances";

/// 审批实例列表查询
pub const APPROVAL_V4_INSTANCES_LIST: &str = "/open-apis/approval/v4/instances/list";

/// 审批实例搜索
pub const APPROVAL_V4_INSTANCES_SEARCH: &str = "/open-apis/approval/v4/instances/search";

/// 审批实例抄送搜索
pub const APPROVAL_V4_INSTANCES_SEARCH_CC: &str = "/open-apis/approval/v4/instances/search_cc";

/// 审批实例取消
pub const APPROVAL_V4_INSTANCE_CANCEL: &str =
    "/open-apis/approval/v4/instances/{instance_id}/cancel";

/// 审批实例抄送
pub const APPROVAL_V4_INSTANCE_CC: &str = "/open-apis/approval/v4/instances/{instance_id}/cc";

/// 审批实例预览
pub const APPROVAL_V4_INSTANCE_PREVIEW: &str = "/open-apis/approval/v4/instances/preview";

/// 审批实例详情
pub const APPROVAL_V4_INSTANCE_GET: &str = "/open-apis/approval/v4/instances/{instance_id}";

/// 审批实例评论列表
pub const APPROVAL_V4_INSTANCE_COMMENTS: &str =
    "/open-apis/approval/v4/instances/{instance_id}/comments";

/// 审批实例评论创建
pub const APPROVAL_V4_INSTANCE_COMMENTS_CREATE: &str =
    "/open-apis/approval/v4/instances/{instance_id}/comments/create";

/// 审批实例评论列表查询
pub const APPROVAL_V4_INSTANCE_COMMENTS_LIST: &str =
    "/open-apis/approval/v4/instances/{instance_id}/comments/list";

/// 审批实例评论回复
pub const APPROVAL_V4_INSTANCE_COMMENTS_REPLY: &str =
    "/open-apis/approval/v4/instances/{instance_id}/comments/reply";

/// 审批实例评论删除
pub const APPROVAL_V4_INSTANCE_COMMENT_DELETE: &str =
    "/open-apis/approval/v4/instances/{instance_id}/comments/{comment_id}/delete";

/// 审批实例评论操作
pub const APPROVAL_V4_INSTANCE_COMMENT_OPERATION: &str =
    "/open-apis/approval/v4/instances/{instance_id}/comments/{comment_id}";

/// 审批消息
pub const APPROVAL_V4_MESSAGES: &str = "/open-apis/approval/v4/messages";

/// 审批消息更新
pub const APPROVAL_V4_MESSAGE_PATCH: &str = "/open-apis/approval/v4/messages/{message_id}";

/// 审批搜索
pub const APPROVAL_V4_SEARCH: &str = "/open-apis/approval/v4/search";

/// 审批列表搜索
pub const APPROVAL_V4_APPROVALS_SEARCH: &str = "/open-apis/approval/v4/approvals/search";

/// 审批任务列表
pub const APPROVAL_V4_TASKS: &str = "/open-apis/approval/v4/tasks";

/// 审批任务操作
pub const APPROVAL_V4_TASK_OPERATION: &str = "/open-apis/approval/v4/tasks/{task_id}";

/// 审批任务搜索
pub const APPROVAL_V4_TASKS_SEARCH: &str = "/open-apis/approval/v4/tasks/search";

/// 审批任务查询
pub const APPROVAL_V4_TASKS_QUERY: &str = "/open-apis/approval/v4/tasks/query";

/// 审批任务同意
pub const APPROVAL_V4_TASK_APPROVE: &str = "/open-apis/approval/v4/tasks/{task_id}/approve";

/// 审批任务拒绝
pub const APPROVAL_V4_TASK_REJECT: &str = "/open-apis/approval/v4/tasks/{task_id}/reject";

/// 审批任务转交
pub const APPROVAL_V4_TASK_TRANSFER: &str = "/open-apis/approval/v4/tasks/{task_id}/transfer";

/// 审批任务指定回退
pub const APPROVAL_V4_TASK_SPECIFIED_ROLLBACK: &str =
    "/open-apis/approval/v4/tasks/{task_id}/specified_rollback";

/// 审批任务回退
pub const APPROVAL_V4_TASK_ROLLBACK: &str = "/open-apis/approval/v4/tasks/{task_id}/rollback";

/// 审批任务加签
pub const APPROVAL_V4_TASK_ADD_SIGN: &str = "/open-apis/approval/v4/tasks/{task_id}/add_sign";

/// 审批任务重新发起
pub const APPROVAL_V4_TASK_RESUBMIT: &str = "/open-apis/approval/v4/tasks/{task_id}/resubmit";
