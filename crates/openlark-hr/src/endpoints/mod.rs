//! OpenLark HR 服务端点定义
//!
//! 此模块包含人力资源相关的所有API端点常量，从 openlark-core 迁移而来。
//! 包含考勤管理、核心人力资源、OKR、薪资管理、绩效管理等完整功能。
//!
//! # 服务模块包含
//!
//! - **attendance**: 考勤管理（考勤组、班次、用户任务、统计数据）
//! - **corehr**: 核心人力资源（员工、职位、部门等基础数据）
//! - **okr**: 目标与关键成果管理
//! - **payroll**: 薪资管理
//! - **performance**: 绩效管理
//!
//! # 使用示例
//!
//! ```rust
//! use openlark_hr::endpoints::*;
//!
//! // 考勤相关端点
//! let groups_endpoint = ATTENDANCE_V1_GROUPS;
//! let stats_endpoint = ATTENDANCE_V1_USER_STATS_DATAS_QUERY;
//!
//! // 核心HR端点
//! let employees_endpoint = COREHR_V1_EMPLOYEES;
//!
//! // OKR端点
//! let okr_period_endpoint = OKR_V1_PERIODS;
//! ```

// 导入核心端点（auth, application等基础端点）
pub use openlark_core::endpoints::{apass, application, auth, platform_integration};

// ===== 考勤管理端点 (Attendance) =====

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

/// 设置和配置
pub const ATTENDANCE_V1_SETTINGS_APPROVAL_CHAINS: &str =
    "/open-apis/attendance/v1/settings/approval_chains";
pub const ATTENDANCE_V1_SETTINGS_FIELD_APPROVALS: &str =
    "/open-apis/attendance/v1/settings/field_approvals";
pub const ATTENDANCE_V1_SETTINGS_HOLIDAY_POLICIES: &str =
    "/open-apis/attendance/v1/settings/holiday_policies";
pub const ATTENDANCE_V1_SETTINGS_LEAVE_POLICIES: &str =
    "/open-apis/attendance/v1/settings/leave_policies";

/// 考勤报告
pub const ATTENDANCE_V1_REPORTS_DAILY_RECORD: &str =
    "/open-apis/attendance/v1/reports/daily_record";
pub const ATTENDANCE_V1_REPORTS_USER_LEAVE_QUOTA: &str =
    "/open-apis/attendance/v1/reports/user_leave_quota";
pub const ATTENDANCE_V1_REPORTS_LEAVE_RECORD: &str =
    "/open-apis/attendance/v1/reports/leave_record";
pub const ATTENDANCE_V1_REPORTS_OVERTIME_RECORD: &str =
    "/open-apis/attendance/v1/reports/overtime_record";

// ===== 核心人力资源端点 (CoreHR) =====

/// 员工管理
pub const COREHR_V1_EMPLOYEES: &str = "/open-apis/corehr/v1/employees";
pub const COREHR_V1_EMPLOYEE_GET: &str = "/open-apis/corehr/v1/employees/{employee_id}";
pub const COREHR_V1_EMPLOYEES_SEARCH: &str = "/open-apis/corehr/v1/employees/search";
pub const COREHR_V1_EMPLOYEES_ADVANCED_SEARCH: &str =
    "/open-apis/corehr/v1/employees/advanced_search";

/// 岗位管理
pub const COREHR_V1_JOBS: &str = "/open-apis/corehr/v1/jobs";
pub const COREHR_V1_JOB_GET: &str = "/open-apis/corehr/v1/jobs/{job_id}";
pub const COREHR_V1_JOBS_SEARCH: &str = "/open-apis/corehr/v1/jobs/search";

/// 部门管理
pub const COREHR_V1_DEPARTMENTS: &str = "/open-apis/corehr/v1/departments";
pub const COREHR_V1_DEPARTMENT_GET: &str = "/open-apis/corehr/v1/departments/{department_id}";
pub const COREHR_V1_DEPARTMENTS_SEARCH: &str = "/open-apis/corehr/v1/departments/search";

/// 雇佣合同
pub const COREHR_V1_EMPLOYMENT_CONTRACTS: &str = "/open-apis/corehr/v1/employment_contracts";
pub const COREHR_V1_EMPLOYMENT_CONTRACT_GET: &str =
    "/open-apis/corehr/v1/employment_contracts/{contract_id}";

/// 工作地点
pub const COREHR_V1_WORK_LOCATIONS: &str = "/open-apis/corehr/v1/work_locations";
pub const COREHR_V1_WORK_LOCATION_GET: &str =
    "/open-apis/corehr/v1/work_locations/{work_location_id}";

// ===== OKR 端点 =====

/// OKR 周期管理
pub const OKR_V1_PERIODS: &str = "/open-apis/okr/v1/periods";
pub const OKR_V1_PERIOD_GET: &str = "/open-apis/okr/v1/periods/{period_id}";
pub const OKR_V1_PERIODS_SEARCH: &str = "/open-apis/okr/v1/periods/search";

/// OKR 目标管理
pub const OKR_V1_OBJECTIVES: &str = "/open-apis/okr/v1/objectives";
pub const OKR_V1_OBJECTIVE_GET: &str = "/open-apis/okr/v1/objectives/{objective_id}";
pub const OKR_V1_OBJECTIVE_DELETE: &str = "/open-apis/okr/v1/objectives/{objective_id}";
pub const OKR_V1_OBJECTIVES_SEARCH: &str = "/open-apis/okr/v1/objectives/search";

/// OKR 关键结果
pub const OKR_V1_KEY_RESULTS: &str = "/open-apis/okr/v1/key_results";
pub const OKR_V1_KEY_RESULT_GET: &str = "/open-apis/okr/v1/key_results/{key_result_id}";
pub const OKR_V1_KEY_RESULTS_SEARCH: &str = "/open-apis/okr/v1/key_results/search";

/// OKR 进度更新
pub const OKR_V1_PROGRESS: &str = "/open-apis/okr/v1/progress";
pub const OKR_V1_PROGRESS_BATCH_UPDATE: &str = "/open-apis/okr/v1/progress/batch_update";

// ===== 薪资管理端点 (Payroll) =====

/// 薪资组管理
pub const PAYROLL_V1_SALARY_GROUPS: &str = "/open-apis/payroll/v1/salary_groups";
pub const PAYROLL_V1_SALARY_GROUP_GET: &str =
    "/open-apis/payroll/v1/salary_groups/{salary_group_id}";
pub const PAYROLL_V1_SALARY_GROUPS_SEARCH: &str = "/open-apis/payroll/v1/salary_groups/search";

/// 薪资调整
pub const PAYROLL_V1_SALARY_ADJUSTMENTS: &str = "/open-apis/payroll/v1/salary_adjustments";
pub const PAYROLL_V1_SALARY_ADJUSTMENT_GET: &str =
    "/open-apis/payroll/v1/salary_adjustments/{adjustment_id}";
pub const PAYROLL_V1_SALARY_ADJUSTMENTS_SEARCH: &str =
    "/open-apis/payroll/v1/salary_adjustments/search";

/// 工资单
pub const PAYROLL_V1_PAYROLLS: &str = "/open-apis/payroll/v1/payrolls";
pub const PAYROLL_V1_PAYROLL_GET: &str = "/open-apis/payroll/v1/payrolls/{payroll_id}";
pub const PAYROLL_V1_PAYROLLS_SEARCH: &str = "/open-apis/payroll/v1/payrolls/search";

/// 薪资条目
pub const PAYROLL_V1_SALARY_ITEMS: &str = "/open-apis/payroll/v1/salary_items";
pub const PAYROLL_V1_SALARY_ITEM_GET: &str = "/open-apis/payroll/v1/salary_items/{salary_item_id}";
pub const PAYROLL_V1_SALARY_ITEMS_SEARCH: &str = "/open-apis/payroll/v1/salary_items/search";

// ===== 绩效管理端点 (Performance) =====

/// 绩效周期
pub const PERFORMANCE_V1_PERIODS: &str = "/open-apis/performance/v1/periods";
pub const PERFORMANCE_V1_PERIOD_GET: &str = "/open-apis/performance/v1/periods/{period_id}";
pub const PERFORMANCE_V1_PERIODS_SEARCH: &str = "/open-apis/performance/v1/periods/search";

/// 绩效评估
pub const PERFORMANCE_V1_APPRAISALS: &str = "/open-apis/performance/v1/appraisals";
pub const PERFORMANCE_V1_APPRAISAL_GET: &str =
    "/open-apis/performance/v1/appraisals/{appraisal_id}";
pub const PERFORMANCE_V1_APPRAISALS_SEARCH: &str = "/open-apis/performance/v1/appraisals/search";

/// 绩效反馈
pub const PERFORMANCE_V1_FEEDBACKS: &str = "/open-apis/performance/v1/feedbacks";
pub const PERFORMANCE_V1_FEEDBACK_GET: &str = "/open-apis/performance/v1/feedbacks/{feedback_id}";
pub const PERFORMANCE_V1_FEEDBACKS_SEARCH: &str = "/open-apis/performance/v1/feedbacks/search";

/// 绩效目标
pub const PERFORMANCE_V1_GOALS: &str = "/open-apis/performance/v1/goals";
pub const PERFORMANCE_V1_GOAL_GET: &str = "/open-apis/performance/v1/goals/{goal_id}";
pub const PERFORMANCE_V1_GOALS_SEARCH: &str = "/open-apis/performance/v1/goals/search";

// ===== 兼容性别名 =====

// 为保持向后兼容性，提供一些简短的别名
pub const GROUPS: &str = ATTENDANCE_V1_GROUPS;
pub const EMPLOYEES: &str = COREHR_V1_EMPLOYEES;
pub const OKRS: &str = OKR_V1_OBJECTIVES;
pub const SALARY_GROUPS: &str = PAYROLL_V1_SALARY_GROUPS;
pub const APPRAISALS: &str = PERFORMANCE_V1_APPRAISALS;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attendance_endpoints() {
        // 验证考勤端点结构
        assert!(ATTENDANCE_V1_GROUPS.starts_with("/open-apis/attendance/v1/"));
        assert!(ATTENDANCE_V1_GROUP_GET.contains("{group_id}"));
        assert!(ATTENDANCE_V1_USER_TASKS_BATCH_CREATE.contains("batch_create"));
        assert!(ATTENDANCE_V1_REPORTS_DAILY_RECORD.contains("reports"));
    }

    #[test]
    fn test_corehr_endpoints() {
        // 验证核心HR端点
        assert!(COREHR_V1_EMPLOYEES.starts_with("/open-apis/corehr/v1/"));
        assert!(COREHR_V1_EMPLOYEE_GET.contains("{employee_id}"));
        assert!(COREHR_V1_EMPLOYEES_SEARCH.contains("search"));
        assert!(COREHR_V1_DEPARTMENTS.starts_with("/open-apis/corehr/v1/"));
    }

    #[test]
    fn test_okr_endpoints() {
        // 验证OKR端点
        assert!(OKR_V1_PERIODS.starts_with("/open-apis/okr/v1/"));
        assert!(OKR_V1_OBJECTIVES.contains("objectives"));
        assert!(OKR_V1_KEY_RESULTS.contains("key_results"));
        assert!(OKR_V1_PROGRESS.contains("progress"));
    }

    #[test]
    fn test_payroll_endpoints() {
        // 验证薪资端点
        assert!(PAYROLL_V1_SALARY_GROUPS.starts_with("/open-apis/payroll/v1/"));
        assert!(PAYROLL_V1_PAYROLLS.contains("payrolls"));
        assert!(PAYROLL_V1_SALARY_ADJUSTMENTS.contains("salary_adjustments"));
    }

    #[test]
    fn test_performance_endpoints() {
        // 验证绩效端点
        assert!(PERFORMANCE_V1_PERIODS.starts_with("/open-apis/performance/v1/"));
        assert!(PERFORMANCE_V1_APPRAISALS.contains("appraisals"));
        assert!(PERFORMANCE_V1_FEEDBACKS.contains("feedbacks"));
        assert!(PERFORMANCE_V1_GOALS.contains("goals"));
    }

    #[test]
    fn test_backward_compatibility() {
        // 验证兼容性别名
        assert_eq!(GROUPS, ATTENDANCE_V1_GROUPS);
        assert_eq!(EMPLOYEES, COREHR_V1_EMPLOYEES);
        assert_eq!(OKRS, OKR_V1_OBJECTIVES);
        assert_eq!(SALARY_GROUPS, PAYROLL_V1_SALARY_GROUPS);
        assert_eq!(APPRAISALS, PERFORMANCE_V1_APPRAISALS);
    }
} // Endpoints and EndpointBuilder are now available directly from openlark_core::endpoints
