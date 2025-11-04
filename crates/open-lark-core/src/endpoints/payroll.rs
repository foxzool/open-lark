//! payroll 服务端点常量定义

// ==================== 薪酬服务端点 ====================

/// 薪酬科目列表
pub const PAYROLL_V1_ACCT_ITEMS: &str = "/open-apis/payroll/v1/acct_items";

/// 成本分摊计划列表
pub const PAYROLL_V1_COST_ALLOCATION_PLANS: &str = "/open-apis/payroll/v1/cost_allocation_plans";

/// 成本分摊报告列表
pub const PAYROLL_V1_COST_ALLOCATION_REPORTS: &str =
    "/open-apis/payroll/v1/cost_allocation_reports";

/// 数据源列表
pub const PAYROLL_V1_DATASOURCES: &str = "/open-apis/payroll/v1/datasources";

/// 数据源记录查询
pub const PAYROLL_V1_DATASOURCE_RECORDS_QUERY: &str =
    "/open-apis/payroll/v1/datasource_records/query";

/// 数据源记录保存
pub const PAYROLL_V1_DATASOURCE_RECORDS_SAVE: &str =
    "/open-apis/payroll/v1/datasource_records/save";

/// 薪酬组列表
pub const PAYROLL_V1_PAYGROUPS: &str = "/open-apis/payroll/v1/paygroups";

/// 薪酬活动列表
pub const PAYROLL_V1_PAYMENT_ACTIVITIES: &str = "/open-apis/payroll/v1/payment_activities";

/// 薪酬活动归档
pub const PAYROLL_V1_PAYMENT_ACTIVITY_ARCHIVE: &str =
    "/open-apis/payroll/v1/payment_activities/{payment_activity_id}/archive";

/// 薪酬明细
pub const PAYROLL_V1_PAYMENT_DETAILS: &str = "/open-apis/payroll/v1/payment_details";

/// 薪酬明细查询
pub const PAYROLL_V1_PAYMENT_DETAILS_QUERY: &str = "/open-apis/payroll/v1/payment_details/query";
