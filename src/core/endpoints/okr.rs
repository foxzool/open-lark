//! okr 服务端点常量定义

// ==================== OKR服务端点 ====================

/// OKR列表
pub const OKR_V1_OKRS: &str = "/open-apis/okr/v1/okrs";

/// OKR批量获取
pub const OKR_V1_OKRS_BATCH_GET: &str = "/open-apis/okr/v1/okrs/batch_get";

/// 周期列表
pub const OKR_V1_PERIODS: &str = "/open-apis/okr/v1/periods";

/// 获取周期
pub const OKR_V1_PERIOD_GET: &str = "/open-apis/okr/v1/periods/{period_id}";

/// 周期规则列表
pub const OKR_V1_PERIOD_RULES: &str = "/open-apis/okr/v1/period_rules";

/// 进展记录列表
pub const OKR_V1_PROGRESS_RECORDS: &str = "/open-apis/okr/v1/progress_records";

/// 进展记录上传
pub const OKR_V1_PROGRESS_RECORDS_UPLOAD: &str = "/open-apis/okr/v1/progress_records/upload";

/// 进展记录操作
pub const OKR_V1_PROGRESS_RECORD_OPERATION: &str =
    "/open-apis/okr/v1/progress_records/{progress_id}";

/// 评审查询
pub const OKR_V1_REVIEWS_QUERY: &str = "/open-apis/okr/v1/reviews/query";
