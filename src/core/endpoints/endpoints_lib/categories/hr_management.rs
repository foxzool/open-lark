//! 人力资源管理服务端点

/// 人力资源管理服务端点
pub struct HrManagement;

impl HrManagement {
    /// / 面试管理
    pub const HIRE_V1_APPLICATION_EVALUATIONS: &'static str = "/open-apis/hire/v1/applications/{application_id}/evaluations";
    /// / Offer 管理
    pub const HIRE_V1_INTERVIEW_ARRANGEMENTS: &'static str = "/open-apis/hire/v1/interview_arrangements";
    /// / 人才管理
    pub const HIRE_V1_OFFER_WITHDRAW: &'static str = "/open-apis/hire/v1/offers/{offer_id}/withdraw";
    /// / 人才库管理
    pub const HIRE_V1_TALENTS_BATCH_IMPORT: &'static str = "/open-apis/hire/v1/talents/batch_import";
    /// ===== 招聘配置端点 =====
    pub const HIRE_V1_TALENT_POOL_TALENT_GET: &'static str = "/open-apis/hire/v1/talent_pools/{pool_id}/talents/{talent_id}";
    /// / 招聘流程
    pub const HIRE_V1_JOB_OPEN: &'static str = "/open-apis/hire/v1/jobs/{job_id}/open";
    /// / 地点管理
    pub const HIRE_V1_JOB_PROCESS_GET: &'static str = "/open-apis/hire/v1/job_processes/{process_id}";
    /// / Offer 设置
    pub const HIRE_V1_LOCATIONS_QUERY: &'static str = "/open-apis/hire/v1/locations/query";
    /// / 科目管理
    pub const HIRE_V1_OFFER_SETTING_GET: &'static str = "/open-apis/hire/v1/offer_settings/{settings_id}";
    /// / 职位要求
    pub const HIRE_V1_SUBJECT_DISABLE: &'static str = "/open-apis/hire/v1/subjects/{subject_id}/disable";
    /// / 权限管理
    pub const HIRE_V1_JOB_REQUIREMENT_GET: &'static str = "/open-apis/hire/v1/job_requirements/{requirement_id}";
    /// / 面试设置
    pub const HIRE_V1_USER_ROLES: &'static str = "/open-apis/hire/v1/users/{user_id}/roles";
    /// / 应用配置
    pub const HIRE_V1_INTERVIEW_SETTING_GET: &'static str = "/open-apis/hire/v1/interview_settings/{settings_id}";
    /// ===== 获取候选人端点 =====
    pub const HIRE_V1_REGISTRATION_FORMS: &'static str = "/open-apis/hire/v1/registration_forms";
    /// / 外部系统
    pub const HIRE_V1_AGENCY_RECOMMENDATIONS: &'static str = "/open-apis/hire/v1/agency_recommendations";
    /// / 内推渠道
    pub const HIRE_V1_EXTERNAL_SYSTEMS_CANDIDATES: &'static str = "/open-apis/hire/v1/external_systems/candidates";
    /// / 内推账户相关端点
    pub const HIRE_V1_REFERRAL_REWARD_SETTINGS: &'static str = "/open-apis/hire/v1/referral_reward_settings";
    /// / 网站渠道
    pub const HIRE_REFERRAL_STATISTICS: &'static str = "/open-apis/hire/v1/referral_statistics";
    /// ===== 生态对接端点 =====
    pub const HIRE_V1_WEBSITE_APPLICATION_CONVERT: &'static str = "/open-apis/hire/v1/website/applications/{website_application_id}/convert";
    /// / 考试管理
    pub const HIRE_V1_BACKGROUND_CHECK_ORDERS_BATCH: &'static str = "/open-apis/hire/v1/background_check_orders/batch";
    /// ===== 其他模块端点 =====
    pub const HIRE_V1_EXAM_STATISTICS: &'static str = "/open-apis/hire/v1/exam_statistics";
    /// / 入职管理
    pub const HIRE_V1_ATTACHMENT_STATISTICS: &'static str = "/open-apis/hire/v1/attachment_statistics";
    /// / 获取入职进度详情 (需要使用 EndpointBuilder::replace_param 替换 {onboarding_id} 和 {progress_id})
    pub const HIRE_V1_ONBOARDINGS: &'static str = "/open-apis/hire/v1/onboardings";
    /// 动态路径常量 - 用于 agency, external_system, referral 等模块
    pub const HIRE_V1_ONBOARDING_PROGRESS: &'static str = "/open-apis/hire/v1/onboardings/{onboarding_id}/progress/{progress_id}";
    /// / 获取收藏的推荐规则
    pub const HIRE_V1_EXTERNAL_SYSTEMS_TEST_CONNECTION: &'static str = "/open-apis/hire/v1/external_systems/{system_config_id}/test_connection";
    /// / 考勤组详情查询 (需要使用 EndpointBuilder::replace_param 替换 {group_id})
    pub const ATTENDANCE_V1_GROUPS: &'static str = "/open-apis/attendance/v1/groups";
    /// / 考勤组删除 (需要使用 EndpointBuilder::replace_param 替换 {group_id})
    pub const ATTENDANCE_V1_GROUP_GET: &'static str = "/open-apis/attendance/v1/groups/{group_id}";
    /// / 考勤组搜索
    pub const ATTENDANCE_V1_GROUP_DELETE: &'static str = "/open-apis/attendance/v1/groups/{group_id}";
    /// / 考勤组用户列表 (需要使用 EndpointBuilder::replace_param 替换 {group_id})
    pub const ATTENDANCE_V1_GROUPS_SEARCH: &'static str = "/open-apis/attendance/v1/groups/search";
    /// 班次管理
    pub const ATTENDANCE_V1_GROUP_USERS: &'static str = "/open-apis/attendance/v1/groups/{group_id}/users";
    /// / 班次详情查询 (需要使用 EndpointBuilder::replace_param 替换 {shift_id})
    pub const ATTENDANCE_V1_SHIFTS: &'static str = "/open-apis/attendance/v1/shifts";
    /// / 班次删除 (需要使用 EndpointBuilder::replace_param 替换 {shift_id})
    pub const ATTENDANCE_V1_SHIFT_GET: &'static str = "/open-apis/attendance/v1/shifts/{shift_id}";
    /// / 班次查询
    pub const ATTENDANCE_V1_SHIFT_DELETE: &'static str = "/open-apis/attendance/v1/shifts/{shift_id}";
    /// 用户任务管理
    pub const ATTENDANCE_V1_SHIFTS_QUERY: &'static str = "/open-apis/attendance/v1/shifts/query";
    /// / 用户任务查询
    pub const ATTENDANCE_V1_USER_TASKS_BATCH_CREATE: &'static str = "/open-apis/attendance/v1/user_tasks/batch_create";
    /// / 用户任务批量删除
    pub const ATTENDANCE_V1_USER_TASKS_QUERY: &'static str = "/open-apis/attendance/v1/user_tasks/query";
    /// / 用户任务结果查询
    pub const ATTENDANCE_V1_USER_TASKS_BATCH_DELETE: &'static str = "/open-apis/attendance/v1/user_tasks/batch_del";
    /// 用户班表管理
    pub const ATTENDANCE_V1_USER_TASK_RESULTS_QUERY: &'static str = "/open-apis/attendance/v1/user_task_results/query";
    /// / 用户班表查询
    pub const ATTENDANCE_V1_USER_DAILY_SHIFTS_BATCH_CREATE: &'static str = "/open-apis/attendance/v1/user_daily_shifts/batch_create";
    /// 用户统计数据管理
    pub const ATTENDANCE_V1_USER_DAILY_SHIFTS_QUERY: &'static str = "/open-apis/attendance/v1/user_daily_shifts/query";
    /// / 用户统计数据字段查询
    pub const ATTENDANCE_V1_USER_STATS_DATAS_QUERY: &'static str = "/open-apis/attendance/v1/user_stats_datas/query";
    /// / 用户统计数据数据查询
    pub const ATTENDANCE_V1_USER_STATS_DATAS_QUERY_FIELDS: &'static str = "/open-apis/attendance/v1/user_stats_datas/query_fields";
    /// / 用户统计数据更新
    pub const ATTENDANCE_V1_USER_STATS_DATAS_QUERY_DATA: &'static str = "/open-apis/attendance/v1/user_stats_datas/query_data";
    /// 用户设置管理
    pub const ATTENDANCE_V1_USER_STATS_DATAS_UPDATE: &'static str = "/open-apis/attendance/v1/user_stats_datas/update";
    /// 用户审批管理
    pub const ATTENDANCE_V1_USER_SETTINGS_QUERY: &'static str = "/open-apis/attendance/v1/user_settings/query";
    /// 用户任务补救管理
    pub const ATTENDANCE_V1_USER_APPROVALS: &'static str = "/open-apis/attendance/v1/user_approvals";
    /// 假期管理
    pub const ATTENDANCE_V1_USER_TASK_REMEDYS: &'static str = "/open-apis/attendance/v1/user_task_remedys";
    /// 归档规则管理
    pub const ATTENDANCE_V1_LEAVE_EMPLOY_EXPIRE_RECORDS: &'static str = "/open-apis/attendance/v1/leave_employ_expire_records";
    /// / 归档规则用户统计字段 (需要使用 EndpointBuilder::replace_param 替换 {archive_rule_id})
    pub const ATTENDANCE_V1_ARCHIVE_RULES: &'static str = "/open-apis/attendance/v1/archive_rules";
    /// / 归档规则上传报表 (需要使用 EndpointBuilder::replace_param 替换 {archive_rule_id})
    pub const ATTENDANCE_V1_ARCHIVE_RULE_USER_STATS_FIELDS: &'static str = "/open-apis/attendance/v1/archive_rules/{archive_rule_id}/user_stats_fields";
    /// / 归档规则删除报表 (需要使用 EndpointBuilder::replace_param 替换 {archive_rule_id})
    pub const ATTENDANCE_V1_ARCHIVE_RULE_UPLOAD_REPORT: &'static str = "/open-apis/attendance/v1/archive_rules/{archive_rule_id}/upload_report";
    /// 用户设置管理
    pub const ATTENDANCE_V1_ARCHIVE_RULE_DEL_REPORT: &'static str = "/open-apis/attendance/v1/archive_rules/{archive_rule_id}/del_report";
    /// / 用户设置上传 (需要使用 EndpointBuilder::replace_param 替换 {user_id})
    pub const ATTENDANCE_V1_USER_SETTINGS_MODIFY: &'static str = "/open-apis/attendance/v1/user_settings/{user_id}/modify";
    /// / 用户设置下载 (需要使用 EndpointBuilder::replace_param 替换 {user_id})
    pub const ATTENDANCE_V1_USER_SETTINGS_UPLOAD: &'static str = "/open-apis/attendance/v1/user_settings/{user_id}/upload";
    /// 用户审批管理
    pub const ATTENDANCE_V1_USER_SETTINGS_DOWNLOAD: &'static str = "/open-apis/attendance/v1/user_settings/{user_id}/download";
    /// 用户任务补救管理
    pub const ATTENDANCE_V1_USER_APPROVAL_PROCESS: &'static str = "/open-apis/attendance/v1/user_approvals/{approval_id}/process";
    /// 用户班表管理
    pub const ATTENDANCE_V1_USER_TASK_REMEDYS_QUERY_USER_ALLOWED_REMEDYS: &'static str = "/open-apis/attendance/v1/user_task_remedys/query_user_allowed_remedys";
    /// 用户任务管理
    pub const ATTENDANCE_V1_USER_DAILY_SHIFTS_BATCH_CREATE_TEMP: &'static str = "/open-apis/attendance/v1/user_daily_shifts/batch_create_temp";
    /// 假期管理
    pub const ATTENDANCE_V1_USER_TASK_GET: &'static str = "/open-apis/attendance/v1/user_tasks/{user_id}/get";
    /// ==================== 客服工具服务端点 ====================
    pub const ATTENDANCE_V1_LEAVE_ACCRUAL_RECORD_GET: &'static str = "/open-apis/attendance/v1/leave_accrual_records/{leave_accrual_record_id}";
}
