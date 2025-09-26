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

/// 绩效活动查询
pub const PERFORMANCE_ACTIVITIES_QUERY: &str = "/open-apis/performance/v1/activities/query";

/// 附加信息管理
pub const PERFORMANCE_ADDITIONAL_INFO_DELETE: &str =
    "/open-apis/performance/v1/additional_info/delete";
pub const PERFORMANCE_ADDITIONAL_INFO_IMPORT: &str =
    "/open-apis/performance/v1/additional_info/import";
pub const PERFORMANCE_ADDITIONAL_INFO_QUERY: &str =
    "/open-apis/performance/v1/additional_info/query";

/// 指标管理（补充）
pub const PERFORMANCE_METRICS_QUERY: &str = "/open-apis/performance/v1/metrics/query";
pub const PERFORMANCE_METRIC_FIELDS_QUERY: &str = "/open-apis/performance/v1/metric_fields/query";
pub const PERFORMANCE_METRIC_TAGS: &str = "/open-apis/performance/v1/metric_tags";
pub const PERFORMANCE_METRIC_TEMPLATES_QUERY: &str =
    "/open-apis/performance/v1/metric_templates/query";

/// 评估对象管理
pub const PERFORMANCE_REVIEWEES_QUERY: &str = "/open-apis/performance/v1/reviewees/query";
pub const PERFORMANCE_REVIEW_ITEMS_QUERY: &str = "/open-apis/performance/v1/review_items/query";
pub const PERFORMANCE_REVIEW_TEMPLATES_QUERY: &str =
    "/open-apis/performance/v1/review_templates/query";

/// 绩效周期管理
pub const PERFORMANCE_SEMESTER_LIST: &str = "/open-apis/performance/v1/semesters";

/// 标签和用户组管理
pub const PERFORMANCE_TAG_QUESTIONS_QUERY: &str = "/open-apis/performance/v1/tag_questions/query";
pub const PERFORMANCE_USER_GROUP_WRITE: &str = "/open-apis/performance/v1/user_groups/write";
