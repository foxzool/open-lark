//! attendance 服务端点常量定义
//!
//! 考勤相关 API 端点常量，包括：
//! - 考勤组管理
//! - 班次管理
//! - 用户任务管理
//! - 统计数据管理
//! - 设置和配置管理

/// 考勤组管理
pub const ATTENDANCE_V1_GROUPS: &str = "/open-apis/attendance/v1/groups";
pub const ATTENDANCE_V1_GROUP_GET: &str = "/open-apis/attendance/v1/groups/{group_id}";
pub const ATTENDANCE_V1_GROUP_DELETE: &str = "/open-apis/attendance/v1/groups/{group_id}";
pub const ATTENDANCE_V1_GROUPS_SEARCH: &str = "/open-apis/attendance/v1/groups/search";
pub const ATTENDANCE_V1_GROUP_USERS: &str = "/open-apis/attendance/v1/groups/{group_id}/users";

/// 班次管理
pub const ATTENDANCE_V1_SHIFTS: &str = "/open-apis/attendance/v1/shifts";
pub const ATTENDANCE_V1_SHIFT_GET: &str = "/open-apis/attendance/v1/shifts/{shift_id}";
pub const ATTENDANCE_V1_SHIFT_DELETE: &str = "/open-apis/attendance/v1/shifts/{shift_id}";
pub const ATTENDANCE_V1_SHIFTS_QUERY: &str = "/open-apis/attendance/v1/shifts/query";

/// 用户任务管理
pub const ATTENDANCE_V1_USER_TASKS_BATCH_CREATE: &str =
    "/open-apis/attendance/v1/user_tasks/batch_create";
pub const ATTENDANCE_V1_USER_TASKS_QUERY: &str = "/open-apis/attendance/v1/user_tasks/query";
pub const ATTENDANCE_V1_USER_TASKS_BATCH_DELETE: &str =
    "/open-apis/attendance/v1/user_tasks/batch_del";
pub const ATTENDANCE_V1_USER_TASK_GET: &str = "/open-apis/attendance/v1/user_tasks/{user_task_id}";
pub const ATTENDANCE_V1_USER_TASK_RESULTS_QUERY: &str =
    "/open-apis/attendance/v1/user_task_results/query";

/// 用户补卡
pub const ATTENDANCE_V1_USER_TASK_REMEDYS: &str = "/open-apis/attendance/v1/user_task_remedys";
pub const ATTENDANCE_V1_USER_TASK_REMEDYS_QUERY_USER_ALLOWED_REMEDYS: &str =
    "/open-apis/attendance/v1/user_task_remedys/query_user_allowed_remedys";

/// 日常班次
pub const ATTENDANCE_V1_USER_DAILY_SHIFTS_QUERY: &str =
    "/open-apis/attendance/v1/user_daily_shifts/query";
pub const ATTENDANCE_V1_USER_DAILY_SHIFTS_BATCH_CREATE: &str =
    "/open-apis/attendance/v1/user_daily_shifts/batch_create";
pub const ATTENDANCE_V1_USER_DAILY_SHIFTS_BATCH_CREATE_TEMP: &str =
    "/open-apis/attendance/v1/user_daily_shifts/batch_create_temp";

/// 统计数据
pub const ATTENDANCE_V1_USER_STATS_DATAS_QUERY: &str =
    "/open-apis/attendance/v1/user_stats_datas/query";
pub const ATTENDANCE_V1_USER_STATS_DATAS_UPDATE: &str =
    "/open-apis/attendance/v1/user_stats_datas/update";
pub const ATTENDANCE_V1_USER_STATS_DATAS_QUERY_FIELDS: &str =
    "/open-apis/attendance/v1/user_stats_datas/query_fields";
pub const ATTENDANCE_V1_USER_STATS_DATAS_QUERY_DATA: &str =
    "/open-apis/attendance/v1/user_stats_datas/query_data";

/// 用户设置
pub const ATTENDANCE_V1_USER_SETTINGS_QUERY: &str = "/open-apis/attendance/v1/user_settings/query";
pub const ATTENDANCE_V1_USER_SETTINGS_MODIFY: &str =
    "/open-apis/attendance/v1/user_settings/modify";
pub const ATTENDANCE_V1_USER_SETTINGS_UPLOAD: &str =
    "/open-apis/attendance/v1/user_settings/upload";
pub const ATTENDANCE_V1_USER_SETTINGS_DOWNLOAD: &str =
    "/open-apis/attendance/v1/user_settings/download";

/// 审批流程
pub const ATTENDANCE_V1_USER_APPROVALS: &str = "/open-apis/attendance/v1/user_approvals";
pub const ATTENDANCE_V1_USER_APPROVAL_PROCESS: &str =
    "/open-apis/attendance/v1/user_approval_process";

/// 请假记录
pub const ATTENDANCE_V1_LEAVE_ACCRUAL_RECORD_GET: &str =
    "/open-apis/attendance/v1/leave_accrual_records/{leave_id}";
pub const ATTENDANCE_V1_LEAVE_EMPLOY_EXPIRE_RECORDS: &str =
    "/open-apis/attendance/v1/leave_employ_expire_records";

/// 归档规则
pub const ATTENDANCE_V1_ARCHIVE_RULES: &str = "/open-apis/attendance/v1/archive_rules";
pub const ATTENDANCE_V1_ARCHIVE_RULE_DEL_REPORT: &str =
    "/open-apis/attendance/v1/archive_rules/del_report";
pub const ATTENDANCE_V1_ARCHIVE_RULE_UPLOAD_REPORT: &str =
    "/open-apis/attendance/v1/archive_rules/upload_report";
pub const ATTENDANCE_V1_ARCHIVE_RULE_USER_STATS_FIELDS: &str =
    "/open-apis/attendance/v1/archive_rules/user_stats_fields";
