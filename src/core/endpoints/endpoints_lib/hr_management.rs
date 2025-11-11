//! 人力资源管理服务端点
//!
//! 包含招聘、考勤、绩效等相关的API端点。

/// 人力资源管理相关端点
pub struct HrManagement;

impl HrManagement {
    // ==================== 招聘管理 ====================

    /// 申请管理
    pub const APPLICATIONS: &'static str = "/open-apis/hire/v1/applications";

    /// 面试管理
    pub const INTERVIEWS: &'static str = "/open-apis/hire/v1/interviews";

    /// Offer管理
    pub const OFFERS: &'static str = "/open-apis/hire/v1/offers";

    /// 人才管理
    pub const TALENTS: &'static str = "/open-apis/hire/v1/talents";

    /// 职位管理
    pub const JOBS: &'static str = "/open-apis/hire/v1/jobs";

    // ==================== 考勤管理 ====================

    /// 考勤记录
    pub const ATTENDANCE_RECORDS: &'static str = "/open-apis/attendance/v1/records";

    /// 考勤统计
    pub const ATTENDANCE_STATS: &'static str = "/open-apis/attendance/v1/stats";

    // ==================== 绩效管理 ====================

    /// 绩效评估
    pub const PERFORMANCE_REVIEWS: &'static str = "/open-apis/performance/v1/reviews";

    /// 绩效目标
    pub const PERFORMANCE_GOALS: &'static str = "/open-apis/performance/v1/goals";
}

// 向后兼容性别名
#[allow(dead_code)]
pub mod legacy {
    pub const HIRE_V1_APPLICATIONS: &str = HrManagement::APPLICATIONS;
    pub const HIRE_V1_INTERVIEWS: &str = HrManagement::INTERVIEWS;
    pub const HIRE_V1_OFFERS: &str = HrManagement::OFFERS;
    pub const HIRE_V1_TALENTS: &str = HrManagement::TALENTS;
    pub const HIRE_V1_JOBS: &str = HrManagement::JOBS;
}