//! performance 服务端点常量定义

// ==================== 绩效服务端点 ====================

/// 指标明细导入
pub const PERFORMANCE_V1_METRIC_DETAIL_IMPORT: &str =
    "/open-apis/performance/v1/metric_details/import";

/// 指标明细查询
pub const PERFORMANCE_V1_METRIC_DETAIL_QUERY: &str =
    "/open-apis/performance/v1/metric_details/query";

/// 评估数据明细查询
pub const PERFORMANCE_V1_REVIEW_DATA_DETAILS_QUERY: &str =
    "/open-apis/performance/v1/review_data/details/query";

/// 评估数据查询
pub const PERFORMANCE_V1_REVIEW_DATA_QUERY: &str = "/open-apis/performance/v1/review_data/query";

/// 阶段任务分页查询
pub const PERFORMANCE_V1_STAGE_TASK_FIND_BY_PAGE: &str =
    "/open-apis/performance/v1/stage_tasks/find_by_page";

/// 阶段任务用户列表查询
pub const PERFORMANCE_V1_STAGE_TASK_FIND_BY_USER_LIST: &str =
    "/open-apis/performance/v1/stage_tasks/find_by_user_list";
