//! 招聘相关端点常量定义

// ===== 候选人管理端点 =====

/// 申请管理
pub const HIRE_V1_APPLICATIONS: &str = "/open-apis/hire/v1/applications";
pub const HIRE_V1_APPLICATION_GET: &str = "/open-apis/hire/v1/applications/{application_id}";
pub const HIRE_V1_APPLICATION_REJECT: &str =
    "/open-apis/hire/v1/applications/{application_id}/reject";
pub const HIRE_V1_APPLICATION_INTERVIEWS: &str =
    "/open-apis/hire/v1/applications/{application_id}/interviews";
pub const HIRE_V1_APPLICATION_OFFER: &str =
    "/open-apis/hire/v1/applications/{application_id}/offer";
pub const HIRE_V1_APPLICATION_ADVANCE: &str =
    "/open-apis/hire/v1/applications/{application_id}/advance";
pub const HIRE_V1_APPLICATION_EVALUATIONS: &str =
    "/open-apis/hire/v1/applications/{application_id}/evaluations";

/// 面试管理
pub const HIRE_V1_INTERVIEWS: &str = "/open-apis/hire/v1/interviews";
pub const HIRE_V1_INTERVIEW_GET: &str = "/open-apis/hire/v1/interviews/{interview_id}";
pub const HIRE_V1_INTERVIEW_CANCEL: &str = "/open-apis/hire/v1/interviews/{interview_id}/cancel";
pub const HIRE_V1_INTERVIEW_RESCHEDULE: &str =
    "/open-apis/hire/v1/interviews/{interview_id}/reschedule";
pub const HIRE_V1_INTERVIEW_EVALUATIONS: &str = "/open-apis/hire/v1/interview_evaluations";
pub const HIRE_V1_INTERVIEW_EVALUATIONS_BY_ID: &str =
    "/open-apis/hire/v1/interviews/{interview_id}/evaluations";
pub const HIRE_V1_INTERVIEW_ARRANGEMENTS: &str = "/open-apis/hire/v1/interview_arrangements";

/// Offer 管理
pub const HIRE_V1_OFFERS: &str = "/open-apis/hire/v1/offers";
pub const HIRE_V1_OFFER_GET: &str = "/open-apis/hire/v1/offers/{offer_id}";
pub const HIRE_V1_OFFER_SEND: &str = "/open-apis/hire/v1/offers/{offer_id}/send";
pub const HIRE_V1_OFFER_WITHDRAW: &str = "/open-apis/hire/v1/offers/{offer_id}/withdraw";

/// 人才管理
pub const HIRE_V1_TALENTS: &str = "/open-apis/hire/v1/talents";
pub const HIRE_V1_TALENT_GET: &str = "/open-apis/hire/v1/talents/{talent_id}";
pub const HIRE_V1_TALENT_APPLICATIONS: &str = "/open-apis/hire/v1/talents/{talent_id}/applications";
pub const HIRE_V1_TALENTS_BATCH_IMPORT: &str = "/open-apis/hire/v1/talents/batch_import";

/// 机构保护相关
pub const HIRE_AGENCY_PROTECT_LIST: &str = "/open-apis/hire/v1/agency_protects";
pub const HIRE_AGENCY_PROTECT_UPDATE: &str = "/open-apis/hire/v1/agency_protects";

// TODO: 添加其余招聘端点（人才库管理、职位管理、评估管理等）
// 这是一个示例拆分，实际实现中需要包含完整的 245 行招聘端点
