//! apass 服务端点常量定义

/// 审计日志列表
pub const APASS_V1_AUDIT_LOG_LIST: &str = "/open-apis/apass/v1/audit_logs";

/// 审计日志详情
pub const APASS_V1_AUDIT_LOG_GET: &str = "/open-apis/apass/v1/audit_logs/{audit_log_id}";

/// 审计日志数据变更记录
pub const APASS_V1_AUDIT_LOG_DATA_CHANGE_LOGS: &str =
    "/open-apis/apass/v1/audit_logs/{audit_log_id}/data_change_logs";

/// 审计日志数据变更记录详情
pub const APASS_V1_AUDIT_LOG_DATA_CHANGE_LOG_GET: &str =
    "/open-apis/apass/v1/audit_logs/{audit_log_id}/data_change_logs/{data_change_log_id}";

/// 环境变量
pub const APASS_V1_ENVIRONMENT_VARIABLES: &str = "/open-apis/apass/v1/environment_variables";

/// 环境变量操作
pub const APASS_V1_ENVIRONMENT_VARIABLE_OPERATION: &str =
    "/open-apis/apass/v1/environment_variables/{env_var_id}";

/// 流程
pub const APASS_V1_FLOWS: &str = "/open-apis/apass/v1/flows";

/// 流程操作
pub const APASS_V1_FLOW_OPERATION: &str = "/open-apis/apass/v1/flows/{flow_id}";

/// 函数
pub const APASS_V1_FUNCTIONS: &str = "/open-apis/apass/v1/functions";

/// 函数操作
pub const APASS_V1_FUNCTION_OPERATION: &str = "/open-apis/apass/v1/functions/{function_id}";

/// 对象
pub const APASS_V1_OBJECTS: &str = "/open-apis/apass/v1/objects";

/// 对象操作
pub const APASS_V1_OBJECT_OPERATION: &str = "/open-apis/apass/v1/objects/{object_id}";

/// 权限
pub const APASS_V1_PERMISSIONS: &str = "/open-apis/apass/v1/permissions";

/// 权限操作
pub const APASS_V1_PERMISSION_OPERATION: &str = "/open-apis/apass/v1/permissions/{permission_id}";

/// 席位
pub const APASS_V1_SEATS: &str = "/open-apis/apass/v1/seats";

/// 席位操作
pub const APASS_V1_SEAT_OPERATION: &str = "/open-apis/apass/v1/seats/{seat_id}";

/// 席位分配列表
pub const APASS_V1_SEAT_ASSIGNMENT_LIST: &str = "/open-apis/apass/v1/seats/assignments";

/// 席位活动列表
pub const APASS_V1_SEAT_ACTIVITY_LIST: &str = "/open-apis/apass/v1/seats/activities";

/// 审计日志审计事件
pub const APASS_V1_AUDIT_LOG_AUDIT_EVENTS: &str = "/open-apis/apass/v1/audit_logs/audit_events";

/// 环境变量查询
pub const APASS_V1_ENVIRONMENT_VARIABLE_QUERY: &str =
    "/open-apis/apass/v1/environment_variables/query";

/// 环境变量获取
pub const APASS_V1_ENVIRONMENT_VARIABLE_GET: &str =
    "/open-apis/apass/v1/environment_variables/{env_var_id}";

/// 流程执行
pub const APASS_V1_FLOW_EXECUTE: &str = "/open-apis/apass/v1/flows/{flow_id}/execute";

/// 流程用户任务查询
pub const APASS_V1_FLOW_USER_TASK_QUERY: &str = "/open-apis/apass/v1/flows/user_tasks/query";

/// 流程用户任务同意
pub const APASS_V1_FLOW_USER_TASK_AGREE: &str =
    "/open-apis/apass/v1/flows/user_tasks/{task_id}/agree";

/// 流程用户任务拒绝
pub const APASS_V1_FLOW_USER_TASK_REJECT: &str =
    "/open-apis/apass/v1/flows/user_tasks/{task_id}/reject";

/// 流程用户任务转交
pub const APASS_V1_FLOW_USER_TASK_TRANSFER: &str =
    "/open-apis/apass/v1/flows/user_tasks/{task_id}/transfer";

/// 流程用户任务添加处理人
pub const APASS_V1_FLOW_USER_TASK_ADD_ASSIGNEE: &str =
    "/open-apis/apass/v1/flows/user_tasks/{task_id}/add_assignee";

/// 流程用户任务抄送
pub const APASS_V1_FLOW_USER_TASK_CC: &str = "/open-apis/apass/v1/flows/user_tasks/{task_id}/cc";

/// 流程用户任务催办
pub const APASS_V1_FLOW_USER_TASK_EXPEDITING: &str =
    "/open-apis/apass/v1/flows/user_tasks/{task_id}/expediting";

/// 流程用户任务取消
pub const APASS_V1_FLOW_USER_TASK_CANCEL: &str =
    "/open-apis/apass/v1/flows/user_tasks/{task_id}/cancel";

/// 流程用户任务回滚点
pub const APASS_V1_FLOW_USER_TASK_ROLLBACK_POINTS: &str =
    "/open-apis/apass/v1/flows/user_tasks/{task_id}/rollback_points";

/// 流程用户任务回滚
pub const APASS_V1_FLOW_USER_TASK_ROLLBACK: &str =
    "/open-apis/apass/v1/flows/user_tasks/{task_id}/rollback";

/// 流程用户任务聊天群
pub const APASS_V1_FLOW_USER_TASK_CHAT_GROUP: &str =
    "/open-apis/apass/v1/flows/user_tasks/{task_id}/chat_group";

/// 函数调用
pub const APASS_V1_FUNCTION_INVOKE: &str = "/open-apis/apass/v1/functions/{function_id}/invoke";

/// 对象OQL查询
pub const APASS_V1_OBJECT_OQL: &str = "/open-apis/apass/v1/objects/{object_id}/oql";

/// 对象记录搜索
pub const APASS_V1_OBJECT_RECORD_SEARCH: &str =
    "/open-apis/apass/v1/objects/{object_id}/records/search";

/// 对象记录获取
pub const APASS_V1_OBJECT_RECORD_GET: &str =
    "/open-apis/apass/v1/objects/{object_id}/records/{record_id}";

/// 对象记录更新
pub const APASS_V1_OBJECT_RECORD_UPDATE: &str =
    "/open-apis/apass/v1/objects/{object_id}/records/{record_id}";

/// 对象记录删除
pub const APASS_V1_OBJECT_RECORD_DELETE: &str =
    "/open-apis/apass/v1/objects/{object_id}/records/{record_id}";

/// 对象记录创建
pub const APASS_V1_OBJECT_RECORD_CREATE: &str = "/open-apis/apass/v1/objects/{object_id}/records";

/// 对象记录批量更新
pub const APASS_V1_OBJECT_RECORD_BATCH_UPDATE: &str =
    "/open-apis/apass/v1/objects/{object_id}/records/batch_update";

/// 对象记录批量查询
pub const APASS_V1_OBJECT_RECORD_BATCH_QUERY: &str =
    "/open-apis/apass/v1/objects/{object_id}/records/batch_query";

/// 对象记录批量删除
pub const APASS_V1_OBJECT_RECORD_BATCH_DELETE: &str =
    "/open-apis/apass/v1/objects/{object_id}/records/batch_delete";

/// 对象记录批量创建
pub const APASS_V1_OBJECT_RECORD_BATCH_CREATE: &str =
    "/open-apis/apass/v1/objects/{object_id}/records/batch_create";

/// 权限角色成员批量移除
pub const APASS_V1_PERMISSION_ROLE_MEMBERS_BATCH_REMOVE: &str =
    "/open-apis/apass/v1/permissions/roles/members/batch_remove";

/// 权限角色成员批量创建
pub const APASS_V1_PERMISSION_ROLE_MEMBERS_BATCH_CREATE: &str =
    "/open-apis/apass/v1/permissions/roles/members/batch_create";

/// 权限角色成员获取
pub const APASS_V1_PERMISSION_ROLE_MEMBER_GET: &str =
    "/open-apis/apass/v1/permissions/roles/members/{member_id}";

/// 权限记录成员批量移除
pub const APASS_V1_PERMISSION_RECORD_MEMBERS_BATCH_REMOVE: &str =
    "/open-apis/apass/v1/permissions/records/members/batch_remove";

/// 权限记录成员批量创建
pub const APASS_V1_PERMISSION_RECORD_MEMBERS_BATCH_CREATE: &str =
    "/open-apis/apass/v1/permissions/records/members/batch_create";
