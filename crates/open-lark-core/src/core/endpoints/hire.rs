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

/// 附件管理
pub const HIRE_V1_ATTACHMENTS: &str = "/open-apis/hire/v1/attachments";
pub const HIRE_V1_ATTACHMENT_GET: &str = "/open-apis/hire/v1/attachments/{attachment_id}";
pub const HIRE_V1_ATTACHMENT_UPLOAD: &str = "/open-apis/hire/v1/attachments/upload";
pub const HIRE_V1_ATTACHMENT_DOWNLOAD: &str =
    "/open-apis/hire/v1/attachments/{attachment_id}/download";
pub const HIRE_V1_ATTACHMENT_PREVIEW: &str =
    "/open-apis/hire/v1/attachments/{attachment_id}/preview";
pub const HIRE_V1_ATTACHMENT_STATISTICS: &str = "/open-apis/hire/v1/attachments/statistics";
pub const HIRE_V1_ATTACHMENTS_BATCH_DELETE: &str = "/open-apis/hire/v1/attachments/batch_delete";
pub const HIRE_V1_ATTACHMENTS_BATCH_DOWNLOAD: &str =
    "/open-apis/hire/v1/attachments/batch_download";

/// 职位管理
pub const HIRE_V1_JOBS: &str = "/open-apis/hire/v1/jobs";
pub const HIRE_V1_JOB_GET_DETAIL: &str = "/open-apis/hire/v1/jobs/{job_id}";
pub const HIRE_V1_JOB_COMBINED_CREATE: &str = "/open-apis/hire/v1/jobs/combined_create";
pub const HIRE_V1_JOB_COMBINED_UPDATE: &str = "/open-apis/hire/v1/jobs/{job_id}/combined_update";
pub const HIRE_V1_JOB_OPEN: &str = "/open-apis/hire/v1/jobs/{job_id}/open";
pub const HIRE_V1_JOB_CLOSE: &str = "/open-apis/hire/v1/jobs/{job_id}/close";
pub const HIRE_V1_JOB_PROCESSES: &str = "/open-apis/hire/v1/job_processes";
pub const HIRE_V1_JOB_PROCESS_GET: &str = "/open-apis/hire/v1/job_processes/{job_process_id}";
pub const HIRE_V1_JOB_REQUIREMENTS: &str = "/open-apis/hire/v1/job_requirements";
pub const HIRE_V1_JOB_REQUIREMENT_GET: &str =
    "/open-apis/hire/v1/job_requirements/{job_requirement_id}";

/// 人才库管理
pub const HIRE_V1_TALENT_POOLS: &str = "/open-apis/hire/v1/talent_pools";
pub const HIRE_V1_TALENT_POOL_GET: &str = "/open-apis/hire/v1/talent_pools/{talent_pool_id}";
pub const HIRE_V1_TALENT_POOL_TALENTS: &str =
    "/open-apis/hire/v1/talent_pools/{talent_pool_id}/talents";
pub const HIRE_V1_TALENT_POOL_TALENT_GET: &str =
    "/open-apis/hire/v1/talent_pools/{talent_pool_id}/talents/{talent_id}";

/// 标签管理
pub const HIRE_V1_TALENT_TAGS: &str = "/open-apis/hire/v1/talent_tags";

/// 机构管理
pub const HIRE_V1_AGENCIES: &str = "/open-apis/hire/v1/agencies";
pub const HIRE_V1_AGENCIES_CONSULTANTS: &str = "/open-apis/hire/v1/agencies/consultants";
pub const HIRE_V1_AGENCY_CONSULTANTS: &str = "/open-apis/hire/v1/agencies/{agency_id}/consultants";
pub const HIRE_V1_AGENCY_RECOMMENDATIONS: &str = "/open-apis/hire/v1/agency_recommendations";
pub const HIRE_V1_AGENCY_RECOMMENDATION_CONFIRM: &str =
    "/open-apis/hire/v1/agency_recommendations/{recommendation_id}/confirm";
pub const HIRE_V1_AGENCY_RECOMMENDATION_REJECT: &str =
    "/open-apis/hire/v1/agency_recommendations/{recommendation_id}/reject";

/// 背景调查
pub const HIRE_V1_BACKGROUND_CHECK_ORDERS: &str = "/open-apis/hire/v1/background_check_orders";
pub const HIRE_V1_BACKGROUND_CHECK_ORDER_GET: &str =
    "/open-apis/hire/v1/background_check_orders/{order_id}";
pub const HIRE_V1_BACKGROUND_CHECK_ORDER_CANCEL: &str =
    "/open-apis/hire/v1/background_check_orders/{order_id}/cancel";
pub const HIRE_V1_BACKGROUND_CHECK_ORDER_REPORT: &str =
    "/open-apis/hire/v1/background_check_orders/{order_id}/report";
pub const HIRE_V1_BACKGROUND_CHECK_ORDERS_BATCH: &str =
    "/open-apis/hire/v1/background_check_orders/batch";
pub const HIRE_V1_BACKGROUND_CHECK_PACKAGES: &str = "/open-apis/hire/v1/background_check_packages";

/// 在线考试
pub const HIRE_V1_EXAM_PAPERS: &str = "/open-apis/hire/v1/exam_papers";
pub const HIRE_V1_EXAM_ARRANGEMENTS: &str = "/open-apis/hire/v1/exam_arrangements";
pub const HIRE_V1_EXAM_RECORDS: &str = "/open-apis/hire/v1/exam_records";
pub const HIRE_V1_EXAM_RECORD_GET: &str = "/open-apis/hire/v1/exam_records/{exam_record_id}";
pub const HIRE_V1_EXAM_RECORD_CANCEL: &str =
    "/open-apis/hire/v1/exam_records/{exam_record_id}/cancel";
pub const HIRE_V1_EXAM_RECORD_RESCHEDULE: &str =
    "/open-apis/hire/v1/exam_records/{exam_record_id}/reschedule";
pub const HIRE_V1_EXAM_STATISTICS: &str = "/open-apis/hire/v1/exam_statistics";
pub const HIRE_V1_EXAM_SUBMISSIONS: &str = "/open-apis/hire/v1/exam_submissions";

/// 外部系统
pub const HIRE_V1_EXTERNAL_SYSTEMS: &str = "/open-apis/hire/v1/external_systems";
pub const HIRE_V1_EXTERNAL_SYSTEMS_CANDIDATES: &str =
    "/open-apis/hire/v1/external_systems/candidates";
pub const HIRE_V1_EXTERNAL_SYSTEMS_CANDIDATES_CONVERT: &str =
    "/open-apis/hire/v1/external_systems/candidates/convert";
pub const HIRE_V1_EXTERNAL_SYSTEMS_CANDIDATES_IMPORT: &str =
    "/open-apis/hire/v1/external_systems/candidates/import";
pub const HIRE_V1_EXTERNAL_SYSTEMS_SYNC_RECORDS: &str =
    "/open-apis/hire/v1/external_systems/sync_records";
pub const HIRE_V1_EXTERNAL_SYSTEMS_SYNC_TASKS: &str =
    "/open-apis/hire/v1/external_systems/sync_tasks";
pub const HIRE_V1_EXTERNAL_SYSTEMS_TEST_CONNECTION: &str =
    "/open-apis/hire/v1/external_systems/test_connection";

/// 面试设置
pub const HIRE_V1_INTERVIEW_SETTINGS: &str = "/open-apis/hire/v1/interview_settings";
pub const HIRE_V1_INTERVIEW_SETTING_GET: &str =
    "/open-apis/hire/v1/interview_settings/{interview_setting_id}";

/// 地点管理
pub const HIRE_V1_LOCATIONS: &str = "/open-apis/hire/v1/locations";
pub const HIRE_V1_LOCATIONS_QUERY: &str = "/open-apis/hire/v1/locations/query";

/// Offer 设置
pub const HIRE_V1_OFFER_SETTINGS: &str = "/open-apis/hire/v1/offer_settings";
pub const HIRE_V1_OFFER_SETTING_GET: &str = "/open-apis/hire/v1/offer_settings/{offer_setting_id}";

/// 入职管理
pub const HIRE_V1_ONBOARDINGS: &str = "/open-apis/hire/v1/onboardings";
pub const HIRE_V1_ONBOARDING_PROGRESS: &str = "/open-apis/hire/v1/onboarding_progress";

/// 内推管理
pub const HIRE_V1_REFERRALS: &str = "/open-apis/hire/v1/referrals";
pub const HIRE_V1_REFERRAL_GET: &str = "/open-apis/hire/v1/referrals/{referral_id}";
pub const HIRE_V1_REFERRAL_ACCOUNTS: &str = "/open-apis/hire/v1/referral_accounts";
pub const HIRE_V1_REFERRAL_ACCOUNT_GET: &str = "/open-apis/hire/v1/referral_accounts/{account_id}";
pub const HIRE_V1_REFERRAL_GRANT_REWARD: &str =
    "/open-apis/hire/v1/referrals/{referral_id}/grant_reward";
pub const HIRE_V1_REFERRAL_REWARD_SETTINGS: &str = "/open-apis/hire/v1/referral_reward_settings";

/// 内推账户和提现管理（补充）
pub const HIRE_REFERRAL_ACCOUNT_BALANCE: &str = "/open-apis/hire/v1/referral_accounts/balance";
pub const HIRE_REFERRAL_ACCOUNT_ENABLE: &str = "/open-apis/hire/v1/referral_accounts/enable";
pub const HIRE_REFERRAL_ACCOUNT_DISABLE: &str = "/open-apis/hire/v1/referral_accounts/disable";
pub const HIRE_REFERRAL_INCOME_RECORDS: &str = "/open-apis/hire/v1/referral_income_records";
pub const HIRE_REFERRAL_STATISTICS: &str = "/open-apis/hire/v1/referral_statistics";
pub const HIRE_REFERRAL_WITHDRAWALS: &str = "/open-apis/hire/v1/referral_withdrawals";
pub const HIRE_REFERRAL_WITHDRAWAL_APPROVE: &str =
    "/open-apis/hire/v1/referral_withdrawals/approve";

/// 登记表单
pub const HIRE_V1_REGISTRATION_FORMS: &str = "/open-apis/hire/v1/registration_forms";

/// 角色管理
pub const HIRE_V1_ROLES: &str = "/open-apis/hire/v1/roles";
pub const HIRE_V1_ROLE_GET: &str = "/open-apis/hire/v1/roles/{role_id}";
pub const HIRE_V1_USER_ROLES: &str = "/open-apis/hire/v1/user_roles";

/// 题目管理
pub const HIRE_V1_SUBJECTS: &str = "/open-apis/hire/v1/subjects";
pub const HIRE_V1_SUBJECT_GET: &str = "/open-apis/hire/v1/subjects/{subject_id}";
pub const HIRE_V1_SUBJECT_ENABLE: &str = "/open-apis/hire/v1/subjects/{subject_id}/enable";
pub const HIRE_V1_SUBJECT_DISABLE: &str = "/open-apis/hire/v1/subjects/{subject_id}/disable";

/// 官网管理
pub const HIRE_V1_WEBSITE_APPLICATIONS: &str = "/open-apis/hire/v1/website/applications";
pub const HIRE_V1_WEBSITE_APPLICATION_CONVERT: &str =
    "/open-apis/hire/v1/website/applications/{application_id}/convert";
pub const HIRE_V1_WEBSITE_CONFIGURATION: &str = "/open-apis/hire/v1/website/configuration";
pub const HIRE_V1_WEBSITE_JOBS: &str = "/open-apis/hire/v1/website/jobs";
pub const HIRE_V1_WEBSITE_JOBS_PUBLISH: &str = "/open-apis/hire/v1/website/jobs/publish";
pub const HIRE_V1_WEBSITE_JOB_UNPUBLISH: &str =
    "/open-apis/hire/v1/website/jobs/{job_id}/unpublish";
pub const HIRE_V1_WEBSITE_STATISTICS: &str = "/open-apis/hire/v1/website/statistics";
