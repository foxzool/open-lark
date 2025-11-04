//! API端点常量定义模块
//!
//! 本模块集中定义飞书开放平台的所有API端点路径常量，旨在：
//! 1. 减少字符串分配 - 避免每次API调用时重新创建路径字符串
//! 2. 防止拼写错误 - 统一管理所有API路径，编译期检查
//! 3. 便于维护升级 - 集中管理，方便API版本升级和路径变更
//!
//! # 性能优化
//!
//! 使用静态字符串常量可以显著减少内存分配：
//! ```rust
//! use open_lark::core::endpoints::Endpoints;
//!
//! // 优化前：每次调用时动态分配字符串
//! let dynamic = "/open-apis/workplace/v1/workplace_access_data/search".to_string();
//!
//! // 优化后：使用静态常量，必要时再转换为 String
//! let optimized = Endpoints::WORKPLACE_ACCESS_DATA_SEARCH;
//! assert_eq!(optimized, dynamic.as_str());
//! ```
//!
//! # 组织结构
//!
//! API端点按服务模块分组：
//! - `workplace` - 工作台相关API
//! - `vc` - 视频会议相关API  
//! - `im` - 即时消息相关API
//! - `drive` - 云盘相关API
//! - 等等...

/// 飞书API端点路径常量定义
///
/// 所有API端点的静态字符串常量，按服务分组组织。
/// 使用模块结构提供更好的命名空间和组织结构。
pub struct Endpoints;

impl Endpoints {
    // ==================== 工作台服务端点 ====================

    /// 搜索工作台访问数据
    pub const WORKPLACE_ACCESS_DATA_SEARCH: &'static str =
        "/open-apis/workplace/v1/workplace_access_data/search";

    /// 搜索自定义工作台访问数据
    pub const WORKPLACE_CUSTOM_ACCESS_DATA_SEARCH: &'static str =
        "/open-apis/workplace/v1/custom_workplace_access_data/search";

    /// 搜索工作台小部件访问数据
    pub const WORKPLACE_WIDGET_ACCESS_DATA_SEARCH: &'static str =
        "/open-apis/workplace/v1/custom_workplace_widget_access_data/search";

    // ==================== 招聘相关端点 ====================

    // ===== 候选人管理端点 =====

    /// 申请管理
    pub const HIRE_V1_APPLICATIONS: &'static str = "/open-apis/hire/v1/applications";
    pub const HIRE_V1_APPLICATION_GET: &'static str =
        "/open-apis/hire/v1/applications/{application_id}";
    pub const HIRE_V1_APPLICATION_REJECT: &'static str =
        "/open-apis/hire/v1/applications/{application_id}/reject";
    pub const HIRE_V1_APPLICATION_INTERVIEWS: &'static str =
        "/open-apis/hire/v1/applications/{application_id}/interviews";
    pub const HIRE_V1_APPLICATION_OFFER: &'static str =
        "/open-apis/hire/v1/applications/{application_id}/offer";
    pub const HIRE_V1_APPLICATION_ADVANCE: &'static str =
        "/open-apis/hire/v1/applications/{application_id}/advance";
    pub const HIRE_V1_APPLICATION_EVALUATIONS: &'static str =
        "/open-apis/hire/v1/applications/{application_id}/evaluations";

    /// 面试管理
    pub const HIRE_V1_INTERVIEWS: &'static str = "/open-apis/hire/v1/interviews";
    pub const HIRE_V1_INTERVIEW_GET: &'static str = "/open-apis/hire/v1/interviews/{interview_id}";
    pub const HIRE_V1_INTERVIEW_CANCEL: &'static str =
        "/open-apis/hire/v1/interviews/{interview_id}/cancel";
    pub const HIRE_V1_INTERVIEW_RESCHEDULE: &'static str =
        "/open-apis/hire/v1/interviews/{interview_id}/reschedule";
    pub const HIRE_V1_INTERVIEW_EVALUATIONS: &'static str =
        "/open-apis/hire/v1/interview_evaluations";
    pub const HIRE_V1_INTERVIEW_EVALUATIONS_BY_ID: &'static str =
        "/open-apis/hire/v1/interviews/{interview_id}/evaluations";
    pub const HIRE_V1_INTERVIEW_ARRANGEMENTS: &'static str =
        "/open-apis/hire/v1/interview_arrangements";

    /// Offer 管理
    pub const HIRE_V1_OFFERS: &'static str = "/open-apis/hire/v1/offers";
    pub const HIRE_V1_OFFER_GET: &'static str = "/open-apis/hire/v1/offers/{offer_id}";
    pub const HIRE_V1_OFFER_SEND: &'static str = "/open-apis/hire/v1/offers/{offer_id}/send";
    pub const HIRE_V1_OFFER_WITHDRAW: &'static str =
        "/open-apis/hire/v1/offers/{offer_id}/withdraw";

    /// 人才管理
    pub const HIRE_V1_TALENTS: &'static str = "/open-apis/hire/v1/talents";
    pub const HIRE_V1_TALENT_GET: &'static str = "/open-apis/hire/v1/talents/{talent_id}";
    pub const HIRE_V1_TALENT_APPLICATIONS: &'static str =
        "/open-apis/hire/v1/talents/{talent_id}/applications";
    pub const HIRE_V1_TALENTS_BATCH_IMPORT: &'static str =
        "/open-apis/hire/v1/talents/batch_import";

    /// 人才库管理
    pub const HIRE_V1_TALENT_POOLS: &'static str = "/open-apis/hire/v1/talent_pools";
    pub const HIRE_V1_TALENT_POOL_GET: &'static str = "/open-apis/hire/v1/talent_pools/{pool_id}";
    pub const HIRE_V1_TALENT_POOL_TALENTS: &'static str =
        "/open-apis/hire/v1/talent_pools/{pool_id}/talents";
    pub const HIRE_V1_TALENT_POOL_TALENT_GET: &'static str =
        "/open-apis/hire/v1/talent_pools/{pool_id}/talents/{talent_id}";

    // ===== 招聘配置端点 =====

    /// 职位管理
    pub const HIRE_V1_JOBS: &'static str = "/open-apis/hire/v1/jobs";
    pub const HIRE_V1_JOB_COMBINED_CREATE: &'static str = "/open-apis/hire/v1/jobs/combined_create";
    pub const HIRE_V1_JOB_COMBINED_UPDATE: &'static str =
        "/open-apis/hire/v1/jobs/{job_id}/combined_update";
    pub const HIRE_V1_JOB_GET_DETAIL: &'static str = "/open-apis/hire/v1/jobs/{job_id}/get_detail";
    pub const HIRE_V1_JOB_CLOSE: &'static str = "/open-apis/hire/v1/jobs/{job_id}/close";
    pub const HIRE_V1_JOB_OPEN: &'static str = "/open-apis/hire/v1/jobs/{job_id}/open";

    /// 招聘流程
    pub const HIRE_V1_JOB_PROCESSES: &'static str = "/open-apis/hire/v1/job_processes";
    pub const HIRE_V1_JOB_PROCESS_GET: &'static str =
        "/open-apis/hire/v1/job_processes/{process_id}";

    /// 地点管理
    pub const HIRE_V1_LOCATIONS: &'static str = "/open-apis/hire/v1/locations";
    pub const HIRE_V1_LOCATIONS_QUERY: &'static str = "/open-apis/hire/v1/locations/query";

    /// Offer 设置
    pub const HIRE_V1_OFFER_SETTINGS: &'static str = "/open-apis/hire/v1/offer_settings";
    pub const HIRE_V1_OFFER_SETTING_GET: &'static str =
        "/open-apis/hire/v1/offer_settings/{settings_id}";

    /// 科目管理
    pub const HIRE_V1_SUBJECTS: &'static str = "/open-apis/hire/v1/subjects";
    pub const HIRE_V1_SUBJECT_GET: &'static str = "/open-apis/hire/v1/subjects/{subject_id}";
    pub const HIRE_V1_SUBJECT_ENABLE: &'static str =
        "/open-apis/hire/v1/subjects/{subject_id}/enable";
    pub const HIRE_V1_SUBJECT_DISABLE: &'static str =
        "/open-apis/hire/v1/subjects/{subject_id}/disable";

    /// 职位要求
    pub const HIRE_V1_JOB_REQUIREMENTS: &'static str = "/open-apis/hire/v1/job_requirements";
    pub const HIRE_V1_JOB_REQUIREMENT_GET: &'static str =
        "/open-apis/hire/v1/job_requirements/{requirement_id}";

    /// 权限管理
    pub const HIRE_V1_ROLES: &'static str = "/open-apis/hire/v1/roles";
    pub const HIRE_V1_ROLE_GET: &'static str = "/open-apis/hire/v1/roles/{role_id}";
    pub const HIRE_V1_USER_ROLES: &'static str = "/open-apis/hire/v1/users/{user_id}/roles";

    /// 面试设置
    pub const HIRE_V1_INTERVIEW_SETTINGS: &'static str = "/open-apis/hire/v1/interview_settings";
    pub const HIRE_V1_INTERVIEW_SETTING_GET: &'static str =
        "/open-apis/hire/v1/interview_settings/{settings_id}";

    /// 应用配置
    pub const HIRE_V1_TALENT_TAGS: &'static str = "/open-apis/hire/v1/talent_tags";
    pub const HIRE_V1_REGISTRATION_FORMS: &'static str = "/open-apis/hire/v1/registration_forms";

    // ===== 获取候选人端点 =====

    /// 代理渠道
    pub const HIRE_V1_AGENCIES: &'static str = "/open-apis/hire/v1/agencies";
    pub const HIRE_V1_AGENCY_CONSULTANTS: &'static str = "/open-apis/hire/v1/agency_consultants";
    pub const HIRE_V1_AGENCY_CONSULTANTS_BY_ID: &'static str =
        "/open-apis/hire/v1/agencies/{agency_id}/consultants";
    pub const HIRE_V1_AGENCY_RECOMMENDATIONS: &'static str =
        "/open-apis/hire/v1/agency_recommendations";

    /// 外部系统
    pub const HIRE_V1_EXTERNAL_SYSTEMS: &'static str = "/open-apis/hire/v1/external_systems";
    pub const HIRE_V1_EXTERNAL_SYSTEMS_SYNC_TASKS: &'static str =
        "/open-apis/hire/v1/external_systems/sync_tasks";
    pub const HIRE_V1_EXTERNAL_SYSTEMS_SYNC_RECORDS: &'static str =
        "/open-apis/hire/v1/external_systems/sync_records";
    pub const HIRE_V1_EXTERNAL_SYSTEMS_CANDIDATES_IMPORT: &'static str =
        "/open-apis/hire/v1/external_systems/candidates/import";
    pub const HIRE_V1_EXTERNAL_SYSTEMS_CANDIDATES: &'static str =
        "/open-apis/hire/v1/external_systems/candidates";

    /// 内推渠道
    pub const HIRE_V1_REFERRALS: &'static str = "/open-apis/hire/v1/referrals";
    pub const HIRE_V1_REFERRAL_GET: &'static str = "/open-apis/hire/v1/referrals/{referral_id}";
    pub const HIRE_V1_REFERRAL_GRANT_REWARD: &'static str =
        "/open-apis/hire/v1/referrals/{referral_id}/grant_reward";
    pub const HIRE_V1_REFERRAL_REWARD_SETTINGS: &'static str =
        "/open-apis/hire/v1/referral_reward_settings";

    /// 内推账户相关端点
    pub const HIRE_V1_REFERRAL_ACCOUNTS: &'static str = "/open-apis/hire/v1/referral_accounts";
    pub const HIRE_V1_REFERRAL_ACCOUNT_GET: &'static str =
        "/open-apis/hire/v1/referral_accounts/{user_id}";
    pub const HIRE_REFERRAL_ACCOUNT_BALANCE: &'static str =
        "/open-apis/hire/v1/referral_accounts/{user_id}/balance";
    pub const HIRE_REFERRAL_ACCOUNT_ENABLE: &'static str =
        "/open-apis/hire/v1/referral_accounts/{user_id}/enable";
    pub const HIRE_REFERRAL_ACCOUNT_DISABLE: &'static str =
        "/open-apis/hire/v1/referral_accounts/{user_id}/disable";
    pub const HIRE_REFERRAL_INCOME_RECORDS: &'static str =
        "/open-apis/hire/v1/referral_income_records";
    pub const HIRE_REFERRAL_WITHDRAWALS: &'static str = "/open-apis/hire/v1/referral_withdrawals";
    pub const HIRE_REFERRAL_WITHDRAWAL_APPROVE: &'static str =
        "/open-apis/hire/v1/referral_withdrawals/{withdrawal_id}/approve";
    pub const HIRE_REFERRAL_STATISTICS: &'static str = "/open-apis/hire/v1/referral_statistics";

    /// 网站渠道
    pub const HIRE_V1_WEBSITE_JOBS: &'static str = "/open-apis/hire/v1/website/jobs";
    pub const HIRE_V1_WEBSITE_JOBS_PUBLISH: &'static str =
        "/open-apis/hire/v1/website/jobs/publish";
    pub const HIRE_V1_WEBSITE_JOB_UNPUBLISH: &'static str =
        "/open-apis/hire/v1/website/jobs/{job_id}/unpublish";
    pub const HIRE_V1_WEBSITE_APPLICATIONS: &'static str =
        "/open-apis/hire/v1/website/applications";
    pub const HIRE_V1_WEBSITE_CONFIGURATION: &'static str =
        "/open-apis/hire/v1/website/configuration";
    pub const HIRE_V1_WEBSITE_STATISTICS: &'static str = "/open-apis/hire/v1/website/statistics";
    pub const HIRE_V1_WEBSITE_APPLICATION_CONVERT: &'static str =
        "/open-apis/hire/v1/website/applications/{website_application_id}/convert";

    // ===== 生态对接端点 =====

    /// 背景调查
    pub const HIRE_V1_BACKGROUND_CHECK_PACKAGES: &'static str =
        "/open-apis/hire/v1/background_check_packages";
    pub const HIRE_V1_BACKGROUND_CHECK_ORDERS: &'static str =
        "/open-apis/hire/v1/background_check_orders";
    pub const HIRE_V1_BACKGROUND_CHECK_ORDER_GET: &'static str =
        "/open-apis/hire/v1/background_check_orders/{order_id}";
    pub const HIRE_V1_BACKGROUND_CHECK_ORDER_CANCEL: &'static str =
        "/open-apis/hire/v1/background_check_orders/{order_id}/cancel";
    pub const HIRE_V1_BACKGROUND_CHECK_ORDER_REPORT: &'static str =
        "/open-apis/hire/v1/background_check_orders/{order_id}/report";
    pub const HIRE_V1_BACKGROUND_CHECK_ORDERS_BATCH: &'static str =
        "/open-apis/hire/v1/background_check_orders/batch";

    /// 考试管理
    pub const HIRE_V1_EXAM_PAPERS: &'static str = "/open-apis/hire/v1/exam_papers";
    pub const HIRE_V1_EXAM_ARRANGEMENTS: &'static str = "/open-apis/hire/v1/exam_arrangements";
    pub const HIRE_V1_EXAM_RECORDS: &'static str = "/open-apis/hire/v1/exam_records";
    pub const HIRE_V1_EXAM_RECORD_GET: &'static str = "/open-apis/hire/v1/exam_records/{record_id}";
    pub const HIRE_V1_EXAM_RECORD_CANCEL: &'static str =
        "/open-apis/hire/v1/exam_records/{record_id}/cancel";
    pub const HIRE_V1_EXAM_RECORD_RESCHEDULE: &'static str =
        "/open-apis/hire/v1/exam_records/{record_id}/reschedule";
    pub const HIRE_V1_EXAM_SUBMISSIONS: &'static str = "/open-apis/hire/v1/exam_submissions";
    pub const HIRE_V1_EXAM_STATISTICS: &'static str = "/open-apis/hire/v1/exam_statistics";

    // ===== 其他模块端点 =====

    /// 附件管理
    pub const HIRE_V1_ATTACHMENTS: &'static str = "/open-apis/hire/v1/attachments";
    pub const HIRE_V1_ATTACHMENT_GET: &'static str =
        "/open-apis/hire/v1/attachments/{attachment_id}";
    pub const HIRE_V1_ATTACHMENT_UPLOAD: &'static str = "/open-apis/hire/v1/attachments/upload";
    pub const HIRE_V1_ATTACHMENT_DOWNLOAD: &'static str =
        "/open-apis/hire/v1/attachments/{attachment_id}/download";
    pub const HIRE_V1_ATTACHMENT_PREVIEW: &'static str =
        "/open-apis/hire/v1/attachments/{attachment_id}/preview";
    pub const HIRE_V1_ATTACHMENTS_BATCH_DOWNLOAD: &'static str =
        "/open-apis/hire/v1/attachments/batch_download";
    pub const HIRE_V1_ATTACHMENTS_BATCH_DELETE: &'static str =
        "/open-apis/hire/v1/attachments/batch_delete";
    pub const HIRE_V1_ATTACHMENT_STATISTICS: &'static str =
        "/open-apis/hire/v1/attachment_statistics";

    /// 入职管理
    pub const HIRE_V1_ONBOARDINGS: &'static str = "/open-apis/hire/v1/onboardings";

    /// 获取入职进度详情 (需要使用 EndpointBuilder::replace_param 替换 {onboarding_id} 和 {progress_id})
    pub const HIRE_V1_ONBOARDING_PROGRESS: &'static str =
        "/open-apis/hire/v1/onboardings/{onboarding_id}/progress/{progress_id}";

    // 动态路径常量 - 用于 agency, external_system, referral 等模块
    pub const HIRE_V1_AGENCIES_CONSULTANTS: &'static str =
        "/open-apis/hire/v1/agencies/{agency_id}/consultants";
    pub const HIRE_V1_AGENCY_RECOMMENDATION_CONFIRM: &'static str =
        "/open-apis/hire/v1/agency_recommendations/{recommendation_id}/confirm";
    pub const HIRE_V1_AGENCY_RECOMMENDATION_REJECT: &'static str =
        "/open-apis/hire/v1/agency_recommendations/{recommendation_id}/reject";
    pub const HIRE_V1_EXTERNAL_SYSTEMS_CANDIDATES_CONVERT: &'static str =
        "/open-apis/hire/v1/external_systems/candidates/{external_candidate_id}/convert";
    pub const HIRE_V1_EXTERNAL_SYSTEMS_TEST_CONNECTION: &'static str =
        "/open-apis/hire/v1/external_systems/{system_config_id}/test_connection";

    /// 获取收藏的推荐规则
    pub const WORKPLACE_APP_RECOMMEND_FAVOURITE: &'static str =
        "/open-apis/workplace/v1/app_recommend_rule/favourite";

    /// 获取推荐的推荐规则
    pub const WORKPLACE_APP_RECOMMEND_RECOMMEND: &'static str =
        "/open-apis/workplace/v1/app_recommend_rule/recommend";

    /// 获取推荐规则列表
    pub const WORKPLACE_APP_RECOMMEND_LIST: &'static str =
        "/open-apis/workplace/v1/app_recommend_rule/list";

    // ==================== 视频会议服务端点 ====================
    // 注意：VC相关端点已迁移至 core::endpoints::vc 模块

    // ==================== Lingo 知识管理服务端点 ====================

    // 分类管理
    /// 获取词典分类
    pub const LINGO_CLASSIFICATION_LIST: &'static str = "/open-apis/lingo/v1/classifications";

    // 草稿管理
    /// 创建草稿
    pub const LINGO_DRAFT_CREATE: &'static str = "/open-apis/lingo/v1/drafts";

    /// 更新草稿 (需要使用 EndpointBuilder::replace_param 替换 {draft_id})
    pub const LINGO_DRAFT_UPDATE: &'static str = "/open-apis/lingo/v1/drafts/{draft_id}";

    // 词条管理
    /// 创建词条
    pub const LINGO_ENTITY_CREATE: &'static str = "/open-apis/lingo/v1/entities";

    /// 获取词条详情 (需要使用 EndpointBuilder::replace_param 替换 {entity_id})
    pub const LINGO_ENTITY_GET: &'static str = "/open-apis/lingo/v1/entities/{entity_id}";

    /// 更新词条 (需要使用 EndpointBuilder::replace_param 替换 {entity_id})
    pub const LINGO_ENTITY_UPDATE: &'static str = "/open-apis/lingo/v1/entities/{entity_id}";

    /// 搜索词条
    pub const LINGO_ENTITY_SEARCH: &'static str = "/open-apis/lingo/v1/entities/search";

    /// 词条匹配
    pub const LINGO_ENTITY_MATCH: &'static str = "/open-apis/lingo/v1/entities/match";

    /// 提取可能的词条
    pub const LINGO_ENTITY_HIGHLIGHT: &'static str = "/open-apis/lingo/v1/entities/highlight";

    // 文件管理
    /// 上传文件
    pub const LINGO_FILE_UPLOAD: &'static str = "/open-apis/lingo/v1/file/upload";

    /// 下载文件 (需要使用 EndpointBuilder::replace_param 替换 {file_token})
    pub const LINGO_FILE_DOWNLOAD: &'static str = "/open-apis/lingo/v1/file/download/{file_token}";

    // 知识库管理
    /// 获取知识库列表
    pub const LINGO_REPO_LIST: &'static str = "/open-apis/lingo/v1/repos";

    // ==================== 租户管理服务端点 ====================

    // 租户信息
    // 注意：租户相关端点已迁移至 core::endpoints::tenant 模块

    // ==================== 即时消息服务端点 ====================

    // 消息管理 - v1版本
    /// 发送消息
    pub const IM_V1_SEND_MESSAGE: &'static str = "/open-apis/im/v1/messages";

    /// 获取消息详情
    pub const IM_V1_GET_MESSAGE: &'static str = "/open-apis/im/v1/messages/{message_id}";

    /// 更新消息
    pub const IM_V1_UPDATE_MESSAGE: &'static str = "/open-apis/im/v1/messages/{message_id}";

    /// 删除消息
    pub const IM_V1_DELETE_MESSAGE: &'static str = "/open-apis/im/v1/messages/{message_id}";

    /// 消息已读回执
    pub const IM_V1_READ_MESSAGE: &'static str =
        "/open-apis/im/v1/messages/{message_id}/read_users";

    /// 获取消息列表
    pub const IM_V1_LIST_MESSAGE: &'static str = "/open-apis/im/v1/messages";

    // 聊天管理
    /// 创建聊天
    pub const IM_CHAT_CREATE: &'static str = "/open-apis/im/v1/chats";

    /// 获取聊天信息
    pub const IM_CHAT_GET: &'static str = "/open-apis/im/v1/chats/{chat_id}";

    /// 更新聊天信息
    pub const IM_CHAT_UPDATE: &'static str = "/open-apis/im/v1/chats/{chat_id}";

    /// 解散聊天
    pub const IM_CHAT_DELETE: &'static str = "/open-apis/im/v1/chats/{chat_id}";

    /// 获取聊天成员列表
    pub const IM_CHAT_MEMBERS: &'static str = "/open-apis/im/v1/chats/{chat_id}/members";

    /// 将用户或机器人拉入聊天
    pub const IM_CHAT_ADD_MEMBERS: &'static str = "/open-apis/im/v1/chats/{chat_id}/members";

    /// 将用户或机器人移出聊天
    pub const IM_CHAT_REMOVE_MEMBERS: &'static str =
        "/open-apis/im/v1/chats/{chat_id}/members/batch_delete";

    // 回复消息
    /// 回复消息
    pub const IM_V1_REPLY_MESSAGE: &'static str = "/open-apis/im/v1/messages/{message_id}/reply";

    // 消息表情回应
    /// 添加消息表情回应
    pub const IM_V1_MESSAGE_REACTIONS: &'static str =
        "/open-apis/im/v1/messages/{message_id}/reactions";

    /// 删除消息表情回应
    pub const IM_V1_DELETE_MESSAGE_REACTION: &'static str =
        "/open-apis/im/v1/messages/{message_id}/reactions/{reaction_id}";

    // 批量消息
    /// 批量发送消息
    pub const IM_V1_BATCH_MESSAGES: &'static str = "/open-apis/im/v1/batch_messages";

    /// 批量撤回消息
    pub const IM_V1_DELETE_BATCH_MESSAGE: &'static str =
        "/open-apis/im/v1/batch_messages/{batch_message_id}";

    /// 查询批量发送消息进度
    pub const IM_V1_BATCH_MESSAGE_PROGRESS: &'static str =
        "/open-apis/im/v1/batch_messages/{batch_message_id}/get_progress";

    /// 查询批量发送消息已读状态
    pub const IM_V1_BATCH_MESSAGE_READ_USER: &'static str =
        "/open-apis/im/v1/batch_messages/{batch_message_id}/read_user";

    // 紧急消息/消息加急
    /// 应用内加急
    pub const IM_V1_MESSAGE_URGENT_APP: &'static str =
        "/open-apis/im/v1/messages/{message_id}/urgent_app";

    /// 短信加急
    pub const IM_V1_MESSAGE_URGENT_SMS: &'static str =
        "/open-apis/im/v1/messages/{message_id}/urgent_sms";

    /// 电话加急
    pub const IM_V1_MESSAGE_URGENT_PHONE: &'static str =
        "/open-apis/im/v1/messages/{message_id}/urgent_phone";

    // 延时更新卡片
    /// 延时更新卡片
    pub const IM_V1_MESSAGE_DELAY_UPDATE: &'static str =
        "/open-apis/im/v1/messages/{message_id}/delay_update";

    // Pin 消息
    /// 创建Pin消息
    pub const IM_V1_PINS: &'static str = "/open-apis/im/v1/pins";

    /// 删除Pin消息
    pub const IM_V1_DELETE_PIN: &'static str = "/open-apis/im/v1/pins/{pin_id}";

    // 文件和图片
    /// 上传文件
    pub const IM_V1_FILES: &'static str = "/open-apis/im/v1/files";

    /// 下载文件
    pub const IM_V1_DOWNLOAD_FILE: &'static str = "/open-apis/im/v1/files/{file_key}";

    /// 上传图片
    pub const IM_V1_IMAGES: &'static str = "/open-apis/im/v1/images";

    /// 下载图片
    pub const IM_V1_DOWNLOAD_IMAGE: &'static str = "/open-apis/im/v1/images/{image_key}";

    // URL预览
    /// 批量更新消息URL预览
    pub const IM_V1_MESSAGE_URL_PREVIEW_BATCH_UPDATE: &'static str =
        "/open-apis/im/v1/messages/{message_id}/url_preview/batch_update";

    // 基础消息操作
    /// 获取消息详情 (需要使用 EndpointBuilder::replace_param 替换 {message_id})
    pub const IM_V1_MESSAGE_GET: &'static str = "/open-apis/im/v1/messages/{message_id}";

    /// 更新消息内容 (需要使用 EndpointBuilder::replace_param 替换 {message_id})
    pub const IM_V1_MESSAGE_PATCH: &'static str = "/open-apis/im/v1/messages/{message_id}";

    /// 删除消息 (需要使用 EndpointBuilder::replace_param 替换 {message_id})
    pub const IM_V1_MESSAGE_DELETE: &'static str = "/open-apis/im/v1/messages/{message_id}";

    // V2 API 端点
    // App feed card
    /// 应用信息流卡片
    pub const IM_V2_APP_FEED_CARD: &'static str = "/open-apis/im/v2/app_feed_card";

    /// 获取应用信息流卡片
    pub const IM_V2_GET_APP_FEED_CARD: &'static str = "/open-apis/im/v2/app_feed_card/{card_id}";

    /// 删除应用信息流卡片
    pub const IM_V2_DELETE_APP_FEED_CARD: &'static str = "/open-apis/im/v2/app_feed_card/{card_id}";

    // Groups bots
    /// 群机器人时间敏感性设置
    pub const IM_V2_GROUPS_BOTS_TIME_SENSITIVE: &'static str =
        "/open-apis/im/v2/groups-bots/bot_time_sentive";

    /// 更新群机器人消息
    pub const IM_V2_GROUPS_BOTS_UPDATE: &'static str =
        "/open-apis/im/v2/groups-bots/{message_id}/update";

    /// 批量更新群机器人设置
    pub const IM_V2_GROUPS_BOTS_PATCH: &'static str = "/open-apis/im/v2/groups-bots/patch";

    // ==================== 云盘服务端点 ====================

    // 文件管理 - v1版本
    /// 获取文件元信息
    pub const DRIVE_V1_GET_META: &'static str = "/open-apis/drive/v1/metas/{token}";

    /// 新建文件夹
    pub const DRIVE_V1_CREATE_FOLDER: &'static str = "/open-apis/drive/v1/files/create_folder";

    /// 获取文件夹中的文件清单
    pub const DRIVE_V1_LIST_FILES: &'static str = "/open-apis/drive/v1/files";

    /// 复制文件或文件夹
    pub const DRIVE_V1_COPY: &'static str = "/open-apis/drive/v1/files/{file_token}/copy";

    /// 移动文件或文件夹
    pub const DRIVE_V1_MOVE: &'static str = "/open-apis/drive/v1/files/{file_token}/move";

    /// 删除文件或文件夹
    pub const DRIVE_V1_DELETE: &'static str = "/open-apis/drive/v1/files/{file_token}";

    /// 获取文件下载链接
    pub const DRIVE_V1_DOWNLOAD: &'static str = "/open-apis/drive/v1/files/{file_token}/download";

    /// 分片上传文件-预上传
    pub const DRIVE_V1_UPLOAD_PREPARE: &'static str = "/open-apis/drive/v1/files/upload_prepare";

    /// 分片上传文件-分片上传
    pub const DRIVE_V1_UPLOAD_PART: &'static str = "/open-apis/drive/v1/files/upload_part";

    /// 分片上传文件-完成上传
    pub const DRIVE_V1_UPLOAD_FINISH: &'static str = "/open-apis/drive/v1/files/upload_finish";

    // ==================== 审批服务端点 ====================

    // ===== 审批管理端点 =====

    /// 创建审批定义
    pub const APPROVAL_V4_APPROVALS: &'static str = "/open-apis/approval/v4/approvals";

    /// 获取审批定义 (需要使用 EndpointBuilder::replace_param 替换 {approval_code})
    pub const APPROVAL_V4_APPROVAL_GET: &'static str =
        "/open-apis/approval/v4/approvals/{approval_code}";

    // ===== 外部审批端点 =====

    /// 创建外部审批
    pub const APPROVAL_V4_EXTERNAL_APPROVALS: &'static str =
        "/open-apis/approval/v4/external_approvals";

    /// 获取外部审批 (需要使用 EndpointBuilder::replace_param 替换 {approval_code})
    pub const APPROVAL_V4_EXTERNAL_APPROVAL_GET: &'static str =
        "/open-apis/approval/v4/external_approvals/{approval_code}";

    // ===== 文件上传端点 =====

    /// 上传文件
    pub const APPROVAL_V4_FILE_UPLOAD: &'static str = "/open-apis/approval/v4/files/upload";

    // ===== 实例管理端点 =====

    /// 创建审批实例
    pub const APPROVAL_V4_INSTANCES: &'static str = "/open-apis/approval/v4/instances";

    /// 获取审批实例列表
    pub const APPROVAL_V4_INSTANCES_LIST: &'static str = "/open-apis/approval/v4/instances";

    /// 获取审批实例 (需要使用 EndpointBuilder::replace_param 替换 {instance_code})
    pub const APPROVAL_V4_INSTANCE_GET: &'static str =
        "/open-apis/approval/v4/instances/{instance_code}";

    /// 取消审批实例 (需要使用 EndpointBuilder::replace_param 替换 {instance_code})
    pub const APPROVAL_V4_INSTANCE_CANCEL: &'static str =
        "/open-apis/approval/v4/instances/{instance_code}/cancel";

    /// 抄送审批实例 (需要使用 EndpointBuilder::replace_param 替换 {instance_code})
    pub const APPROVAL_V4_INSTANCE_CC: &'static str =
        "/open-apis/approval/v4/instances/{instance_code}/cc";

    /// 预览审批实例
    pub const APPROVAL_V4_INSTANCE_PREVIEW: &'static str =
        "/open-apis/approval/v4/instances/preview";

    // ===== 实例评论端点 =====

    /// 创建实例评论 (需要使用 EndpointBuilder::replace_param 替换 {instance_code})
    pub const APPROVAL_V4_INSTANCE_COMMENTS_CREATE: &'static str =
        "/open-apis/approval/v4/instances/{instance_code}/comments";

    /// 删除实例评论 (需要使用 EndpointBuilder::replace_param 替换 {instance_code} 和 {comment_id})
    pub const APPROVAL_V4_INSTANCE_COMMENT_DELETE: &'static str =
        "/open-apis/approval/v4/instances/{instance_code}/comments/{comment_id}";

    /// 获取实例评论列表 (需要使用 EndpointBuilder::replace_param 替换 {instance_code})
    pub const APPROVAL_V4_INSTANCE_COMMENTS_LIST: &'static str =
        "/open-apis/approval/v4/instances/{instance_code}/comments";

    /// 回复实例评论 (需要使用 EndpointBuilder::replace_param 替换 {instance_code})
    pub const APPROVAL_V4_INSTANCE_COMMENTS_REPLY: &'static str =
        "/open-apis/approval/v4/instances/{instance_code}/comments";

    // ===== 外部实例端点 =====

    /// 创建外部实例
    pub const APPROVAL_V4_EXTERNAL_INSTANCES: &'static str =
        "/open-apis/approval/v4/external_instances";

    /// 校验外部实例 (需要使用 EndpointBuilder::replace_param 替换 {instance_code})
    pub const APPROVAL_V4_EXTERNAL_INSTANCE_CHECK: &'static str =
        "/open-apis/approval/v4/external_instances/{instance_code}/check";

    // ===== 外部任务端点 =====

    /// 创建外部任务
    pub const APPROVAL_V4_EXTERNAL_TASKS: &'static str = "/open-apis/approval/v4/external_tasks";

    // ===== 消息管理端点 =====

    /// 发送消息
    pub const APPROVAL_V4_MESSAGES: &'static str = "/open-apis/approval/v4/messages";

    /// 更新消息 (需要使用 EndpointBuilder::replace_param 替换 {message_id})
    pub const APPROVAL_V4_MESSAGE_PATCH: &'static str =
        "/open-apis/approval/v4/messages/{message_id}";

    // ===== 搜索端点 =====

    /// 搜索审批实例
    pub const APPROVAL_V4_INSTANCES_SEARCH: &'static str =
        "/open-apis/approval/v4/instances/search";

    /// 搜索审批任务
    pub const APPROVAL_V4_TASKS_SEARCH: &'static str = "/open-apis/approval/v4/tasks/search";

    /// 搜索抄送实例
    pub const APPROVAL_V4_INSTANCES_SEARCH_CC: &'static str =
        "/open-apis/approval/v4/instances/search_cc";

    /// 搜索审批定义
    pub const APPROVAL_V4_APPROVALS_SEARCH: &'static str =
        "/open-apis/approval/v4/approvals/search";

    /// 查询任务
    pub const APPROVAL_V4_TASKS_QUERY: &'static str = "/open-apis/approval/v4/tasks/query";

    // ===== 任务处理端点 =====

    /// 同意任务 (需要使用 EndpointBuilder::replace_param 替换 {task_id})
    pub const APPROVAL_V4_TASK_APPROVE: &'static str =
        "/open-apis/approval/v4/tasks/{task_id}/approve";

    /// 拒绝任务 (需要使用 EndpointBuilder::replace_param 替换 {task_id})
    pub const APPROVAL_V4_TASK_REJECT: &'static str =
        "/open-apis/approval/v4/tasks/{task_id}/reject";

    /// 转交任务 (需要使用 EndpointBuilder::replace_param 替换 {task_id})
    pub const APPROVAL_V4_TASK_TRANSFER: &'static str =
        "/open-apis/approval/v4/tasks/{task_id}/transfer";

    /// 指定回退任务 (需要使用 EndpointBuilder::replace_param 替换 {task_id})
    pub const APPROVAL_V4_TASK_SPECIFIED_ROLLBACK: &'static str =
        "/open-apis/approval/v4/tasks/{task_id}/specified_rollback";

    /// 加签任务 (需要使用 EndpointBuilder::replace_param 替换 {task_id})
    pub const APPROVAL_V4_TASK_ADD_SIGN: &'static str =
        "/open-apis/approval/v4/tasks/{task_id}/add_sign";

    /// 重新提交任务 (需要使用 EndpointBuilder::replace_param 替换 {task_id})
    pub const APPROVAL_V4_TASK_RESUBMIT: &'static str =
        "/open-apis/approval/v4/tasks/{task_id}/resubmit";

    // ==================== 身份验证服务端点 ====================

    /// 获取访问令牌
    pub const AUTH_ACCESS_TOKEN: &'static str = "/open-apis/auth/v3/app_access_token";

    /// 获取用户访问令牌
    pub const AUTH_USER_ACCESS_TOKEN: &'static str = "/open-apis/auth/v3/user_access_token";

    /// 刷新用户访问令牌
    pub const AUTH_REFRESH_USER_TOKEN: &'static str =
        "/open-apis/auth/v3/refresh_user_access_token";

    /// 获取登录预授权码
    pub const AUTH_LOGIN_PRE_AUTH: &'static str = "/open-apis/authen/v1/oidc/access_token";

    /// 获取用户信息
    pub const AUTH_USER_INFO: &'static str = "/open-apis/authen/v1/user_info";

    // ==================== 联系人服务端点 ====================

    // 用户管理
    /// 获取用户信息
    pub const CONTACT_USER_GET: &'static str = "/open-apis/contact/v3/users/{user_id}";

    /// 获取用户列表
    pub const CONTACT_USER_LIST: &'static str = "/open-apis/contact/v3/users";

    /// 批量获取用户信息
    pub const CONTACT_USER_BATCH_GET: &'static str = "/open-apis/contact/v3/users/batch";

    /// 搜索用户
    pub const CONTACT_USER_SEARCH: &'static str = "/open-apis/contact/v3/users/find_by_mobile";

    // 部门管理
    /// 获取部门信息
    pub const CONTACT_DEPARTMENT_GET: &'static str =
        "/open-apis/contact/v3/departments/{department_id}";

    /// 获取部门列表
    pub const CONTACT_DEPARTMENT_LIST: &'static str = "/open-apis/contact/v3/departments";

    /// 获取子部门列表
    pub const CONTACT_DEPARTMENT_CHILDREN: &'static str =
        "/open-apis/contact/v3/departments/{department_id}/children";

    // ==================== 应用管理服务端点 ====================

    /// 应用信息
    pub const APPLICATION_GET_APP_INFO: &'static str =
        "/open-apis/application/v6/applications/{app_id}";

    /// 获取应用管理员列表
    pub const APPLICATION_GET_ADMIN_LIST: &'static str =
        "/open-apis/application/v6/applications/{app_id}/app_admin_user_list";

    /// 应用商店信息
    pub const APPLICATION_GET_APP_STORE_INFO: &'static str =
        "/open-apis/application/v6/applications/{app_id}/app_version";

    // ==================== 日历服务端点 ====================

    // 日历管理
    /// 创建日历
    pub const CALENDAR_CREATE: &'static str = "/open-apis/calendar/v4/calendars";

    /// 获取日历
    pub const CALENDAR_GET: &'static str = "/open-apis/calendar/v4/calendars/{calendar_id}";

    /// 更新日历
    pub const CALENDAR_UPDATE: &'static str = "/open-apis/calendar/v4/calendars/{calendar_id}";

    /// 删除日历
    pub const CALENDAR_DELETE: &'static str = "/open-apis/calendar/v4/calendars/{calendar_id}";

    /// 获取日历列表
    pub const CALENDAR_LIST: &'static str = "/open-apis/calendar/v4/calendars";

    /// 设置主日历
    pub const CALENDAR_PRIMARY: &'static str =
        "/open-apis/calendar/v4/calendars/{calendar_id}/primary";

    /// 搜索日历
    pub const CALENDAR_SEARCH: &'static str =
        "/open-apis/calendar/v4/calendars/{calendar_id}/search";

    // 日程管理
    /// 创建日程
    pub const CALENDAR_EVENT_CREATE: &'static str =
        "/open-apis/calendar/v4/calendars/{calendar_id}/events";

    /// 获取日程
    pub const CALENDAR_EVENT_GET: &'static str =
        "/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}";

    /// 更新日程
    pub const CALENDAR_EVENT_UPDATE: &'static str =
        "/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}";

    /// 删除日程
    pub const CALENDAR_EVENT_DELETE: &'static str =
        "/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}";

    /// 获取日程列表
    pub const CALENDAR_EVENT_LIST: &'static str =
        "/open-apis/calendar/v4/calendars/{calendar_id}/events";

    /// 回复日程邀请
    pub const CALENDAR_EVENT_REPLY: &'static str =
        "/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}/reply";

    /// 搜索日程
    pub const CALENDAR_EVENT_SEARCH: &'static str =
        "/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}/search";

    // ==================== AI Document AI 相关端点 ====================
    /// 简历解析
    pub const DOCUMENT_AI_RESUME_PARSE: &'static str = "/open-apis/document_ai/v1/resume/parse";

    /// 身份证识别
    pub const DOCUMENT_AI_ID_CARD_RECOGNIZE: &'static str =
        "/open-apis/document_ai/v1/id_card/recognize";

    /// 驾驶证识别
    pub const DOCUMENT_AI_DRIVING_LICENSE_RECOGNIZE: &'static str =
        "/open-apis/document_ai/v1/driving_license/recognize";

    /// 银行卡识别
    pub const DOCUMENT_AI_BANK_CARD_RECOGNIZE: &'static str =
        "/open-apis/document_ai/v1/bank_card/recognize";

    /// 营业执照识别
    pub const DOCUMENT_AI_BUSINESS_LICENSE_RECOGNIZE: &'static str =
        "/open-apis/document_ai/v1/business_license/recognize";

    /// 增值税发票识别
    pub const DOCUMENT_AI_VAT_INVOICE_RECOGNIZE: &'static str =
        "/open-apis/document_ai/v1/vat_invoice/recognize";

    /// 合同字段提取
    pub const DOCUMENT_AI_CONTRACT_FIELD_EXTRACTION: &'static str =
        "/open-apis/document_ai/v1/contract/field_extraction";

    /// 名片识别
    pub const DOCUMENT_AI_BUSINESS_CARD_RECOGNIZE: &'static str =
        "/open-apis/document_ai/v1/business_card/recognize";

    /// 机动车发票识别
    pub const DOCUMENT_AI_VEHICLE_INVOICE_RECOGNIZE: &'static str =
        "/open-apis/document_ai/v1/vehicle_invoice/recognize";

    /// 健康证识别
    pub const DOCUMENT_AI_HEALTH_CERTIFICATE_RECOGNIZE: &'static str =
        "/open-apis/document_ai/v1/health_certificate/recognize";

    /// 港澳居民来往内地通行证识别
    pub const DOCUMENT_AI_HKM_MAINLAND_TRAVEL_PERMIT_RECOGNIZE: &'static str =
        "/open-apis/document_ai/v1/hkm_mainland_travel_permit/recognize";

    /// 台湾居民来往大陆通行证识别
    pub const DOCUMENT_AI_TW_MAINLAND_TRAVEL_PERMIT_RECOGNIZE: &'static str =
        "/open-apis/document_ai/v1/tw_mainland_travel_permit/recognize";

    /// 中国护照识别
    pub const DOCUMENT_AI_CHINESE_PASSPORT_RECOGNIZE: &'static str =
        "/open-apis/document_ai/v1/chinese_passport/recognize";

    /// 行驶证识别
    pub const DOCUMENT_AI_VEHICLE_LICENSE_RECOGNIZE: &'static str =
        "/open-apis/document_ai/v1/vehicle_license/recognize";

    /// 火车票识别
    pub const DOCUMENT_AI_TRAIN_INVOICE_RECOGNIZE: &'static str =
        "/open-apis/document_ai/v1/train_invoice/recognize";

    /// 出租车发票识别
    pub const DOCUMENT_AI_TAXI_INVOICE_RECOGNIZE: &'static str =
        "/open-apis/document_ai/v1/taxi_invoice/recognize";

    /// 食品生产许可证识别
    pub const DOCUMENT_AI_FOOD_PRODUCE_LICENSE_RECOGNIZE: &'static str =
        "/open-apis/document_ai/v1/food_produce_license/recognize";

    /// 食品经营许可证识别
    pub const DOCUMENT_AI_FOOD_MANAGE_LICENSE_RECOGNIZE: &'static str =
        "/open-apis/document_ai/v1/food_manage_license/recognize";

    // ==================== AI Services AI服务相关端点 ====================
    /// 语音文件识别
    pub const SPEECH_TO_TEXT_V1_FILE_RECOGNIZE: &'static str =
        "/open-apis/speech_to_text/v1/speech/file_recognize";

    /// 流式语音识别
    pub const SPEECH_TO_TEXT_V1_STREAM_RECOGNIZE: &'static str =
        "/open-apis/speech_to_text/v1/speech/stream_recognize";

    /// 光学字符识别
    pub const OPTICAL_CHAR_RECOGNITION_V1_BASIC_RECOGNIZE: &'static str =
        "/open-apis/optical_char_recognition/v1/image/basic_recognize";

    /// 语种检测
    pub const TRANSLATION_V1_TEXT_DETECT: &'static str = "/open-apis/translation/v1/text/detect";

    /// 文本翻译
    pub const TRANSLATION_V1_TEXT_TRANSLATE: &'static str =
        "/open-apis/translation/v1/text/translate";

    // ==================== E-Learning 在线学习相关端点 ====================
    /// 课程报名管理
    pub const ELEARNING_V2_COURSE_REGISTRATIONS: &'static str =
        "/open-apis/elearning/v2/course_registrations";

    /// 课程报名操作
    pub const ELEARNING_V2_COURSE_REGISTRATION_OPERATION: &'static str =
        "/open-apis/elearning/v2/course_registrations/{registration_id}";

    /// 课程报名统计
    pub const ELEARNING_V2_COURSE_REGISTRATIONS_STATISTICS: &'static str =
        "/open-apis/elearning/v2/course_registrations/statistics";

    // ==================== Tenant Tag 租户标签相关端点 ====================
    // 注意：TENANT_TAG相关端点已迁移至 core::endpoints::tenant_tag 模块

    // ==================== Task v2 相关端点 ====================
    /// 任务附件上传
    pub const TASK_V2_ATTACHMENTS_UPLOAD: &'static str = "/open-apis/task/v2/attachments/upload";

    /// 任务附件管理
    pub const TASK_V2_ATTACHMENTS: &'static str = "/open-apis/task/v2/attachments";

    /// 任务分组管理
    pub const TASK_V2_SECTIONS: &'static str = "/open-apis/task/v2/sections";

    /// 任务管理
    pub const TASK_V2_TASKS: &'static str = "/open-apis/task/v2/tasks";

    /// 任务自定义字段管理
    pub const TASK_V2_CUSTOM_FIELDS: &'static str = "/open-apis/task/v2/custom_fields";

    /// 任务清单管理
    pub const TASK_V2_TASKLISTS: &'static str = "/open-apis/task/v2/tasklists";

    // 参数化端点常量
    /// 获取/更新/删除特定附件
    pub const TASK_V2_ATTACHMENT_GET: &'static str =
        "/open-apis/task/v2/attachments/{attachment_guid}";

    /// 获取/更新/删除特定自定义字段
    pub const TASK_V2_CUSTOM_FIELD_GET: &'static str =
        "/open-apis/task/v2/custom_fields/{custom_field_guid}";

    /// 添加自定义字段选项
    pub const TASK_V2_CUSTOM_FIELD_ADD: &'static str =
        "/open-apis/task/v2/custom_fields/{custom_field_guid}/add";

    /// 移除自定义字段选项
    pub const TASK_V2_CUSTOM_FIELD_REMOVE: &'static str =
        "/open-apis/task/v2/custom_fields/{custom_field_guid}/remove";

    /// 自定义字段选项管理
    pub const TASK_V2_CUSTOM_FIELD_OPTIONS: &'static str =
        "/open-apis/task/v2/custom_fields/{custom_field_guid}/options";

    /// 获取/更新/删除特定自定义字段选项
    pub const TASK_V2_CUSTOM_FIELD_OPTION_GET: &'static str =
        "/open-apis/task/v2/custom_fields/{custom_field_guid}/options/{option_guid}";

    /// 获取/更新/删除特定分组
    pub const TASK_V2_SECTION_GET: &'static str = "/open-apis/task/v2/sections/{section_guid}";

    /// 分组任务管理
    pub const TASK_V2_SECTION_TASKS: &'static str =
        "/open-apis/task/v2/sections/{section_guid}/tasks";

    /// 获取/更新/删除特定任务清单
    pub const TASK_V2_TASKLIST_GET: &'static str = "/open-apis/task/v2/tasklists/{tasklist_guid}";

    /// 任务清单添加成员
    pub const TASK_V2_TASKLIST_ADD_MEMBERS: &'static str =
        "/open-apis/task/v2/tasklists/{tasklist_guid}/add_members";

    /// 任务清单移除成员
    pub const TASK_V2_TASKLIST_REMOVE_MEMBERS: &'static str =
        "/open-apis/task/v2/tasklists/{tasklist_guid}/remove_members";

    /// 任务清单任务管理
    pub const TASK_V2_TASKLIST_TASKS: &'static str =
        "/open-apis/task/v2/tasklists/{tasklist_guid}/tasks";

    /// 任务清单活动订阅
    pub const TASK_V2_TASKLIST_ACTIVITY_SUBSCRIPTIONS: &'static str =
        "/open-apis/task/v2/tasklists/{tasklist_guid}/activity_subscriptions";

    /// 获取/更新/删除特定活动订阅
    pub const TASK_V2_TASKLIST_ACTIVITY_SUBSCRIPTION_GET: &'static str = "/open-apis/task/v2/tasklists/{tasklist_guid}/activity_subscriptions/{activity_subscription_guid}";

    /// 获取/更新/删除特定任务
    pub const TASK_V2_TASK_GET: &'static str = "/open-apis/task/v2/tasks/{task_guid}";

    /// 任务添加成员
    pub const TASK_V2_TASK_ADD_MEMBERS: &'static str =
        "/open-apis/task/v2/tasks/{task_guid}/add_members";

    /// 任务移除成员
    pub const TASK_V2_TASK_REMOVE_MEMBERS: &'static str =
        "/open-apis/task/v2/tasks/{task_guid}/remove_members";

    /// 任务添加提醒
    pub const TASK_V2_TASK_ADD_REMINDERS: &'static str =
        "/open-apis/task/v2/tasks/{task_guid}/add_reminders";

    /// 任务移除提醒
    pub const TASK_V2_TASK_REMOVE_REMINDERS: &'static str =
        "/open-apis/task/v2/tasks/{task_guid}/remove_reminders";

    /// 任务加入清单
    pub const TASK_V2_TASK_ADD_TASKLIST: &'static str =
        "/open-apis/task/v2/tasks/{task_guid}/add_tasklist";

    /// 任务添加依赖
    pub const TASK_V2_TASK_ADD_DEPENDENCIES: &'static str =
        "/open-apis/task/v2/tasks/{task_guid}/add_dependencies";

    /// 任务移除依赖
    pub const TASK_V2_TASK_REMOVE_DEPENDENCIES: &'static str =
        "/open-apis/task/v2/tasks/{task_guid}/remove_dependencies";

    /// 任务评论管理
    pub const TASK_V2_TASK_COMMENTS: &'static str = "/open-apis/task/v2/tasks/{task_guid}/comments";

    /// 获取/更新/删除特定任务评论
    pub const TASK_V2_TASK_COMMENT_GET: &'static str =
        "/open-apis/task/v2/tasks/{task_guid}/comments/{comment_id}";

    /// 任务子任务管理
    pub const TASK_V2_TASK_SUBTASKS: &'static str = "/open-apis/task/v2/tasks/{task_guid}/subtasks";

    // ==================== Payroll 薪酬管理相关端点 ====================

    /// 成本分摊计划列表
    pub const PAYROLL_V1_COST_ALLOCATION_PLANS: &'static str =
        "/open-apis/payroll/v1/cost_allocation_plans";

    /// 账套项目列表  
    pub const PAYROLL_V1_ACCT_ITEMS: &'static str = "/open-apis/payroll/v1/acct_items";

    /// 数据源列表
    pub const PAYROLL_V1_DATASOURCES: &'static str = "/open-apis/payroll/v1/datasources";

    /// 数据源记录保存
    pub const PAYROLL_V1_DATASOURCE_RECORDS_SAVE: &'static str =
        "/open-apis/payroll/v1/datasources/{datasource_id}/records/save";

    /// 数据源记录查询
    pub const PAYROLL_V1_DATASOURCE_RECORDS_QUERY: &'static str =
        "/open-apis/payroll/v1/datasources/{datasource_id}/records/query";

    /// 薪酬活动详情查询
    pub const PAYROLL_V1_PAYMENT_DETAILS: &'static str =
        "/open-apis/payroll/v1/payment_activities/{payment_activity_id}/payment_details";

    /// 薪酬活动详情查询（通过查询接口）
    pub const PAYROLL_V1_PAYMENT_DETAILS_QUERY: &'static str =
        "/open-apis/payroll/v1/payment_activities/{payment_activity_id}/payment_details/query";

    /// 薪酬活动列表
    pub const PAYROLL_V1_PAYMENT_ACTIVITIES: &'static str =
        "/open-apis/payroll/v1/payment_activities";

    /// 薪酬活动归档
    pub const PAYROLL_V1_PAYMENT_ACTIVITY_ARCHIVE: &'static str =
        "/open-apis/payroll/v1/payment_activities/{payment_activity_id}/archive";

    /// 成本分摊报表列表
    pub const PAYROLL_V1_COST_ALLOCATION_REPORTS: &'static str =
        "/open-apis/payroll/v1/cost_allocation_reports";

    /// 薪酬组列表
    pub const PAYROLL_V1_PAYGROUPS: &'static str = "/open-apis/payroll/v1/paygroups";

    // ==================== APaaS 平台即服务相关端点 ====================

    // ===== 座位管理端点 =====
    /// 查询座位分配列表
    pub const APASS_V1_SEAT_ASSIGNMENT_LIST: &'static str =
        "/open-apis/apaas/v1/seat_assignment/list";
    /// 查询座位活动列表
    pub const APASS_V1_SEAT_ACTIVITY_LIST: &'static str = "/open-apis/apaas/v1/seat_activity/list";

    // ===== 流程管理端点 =====
    /// 执行流程
    pub const APASS_V1_FLOW_EXECUTE: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/flow/{flow_api_name}/execute";
    /// 查询用户任务
    pub const APASS_V1_FLOW_USER_TASK_QUERY: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/flow/user_task/query";
    /// 同意用户任务
    pub const APASS_V1_FLOW_USER_TASK_AGREE: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/flow/user_task/{task_id}/agree";
    /// 拒绝用户任务
    pub const APASS_V1_FLOW_USER_TASK_REJECT: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/flow/user_task/{task_id}/reject";
    /// 转发用户任务
    pub const APASS_V1_FLOW_USER_TASK_TRANSFER: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/flow/user_task/{task_id}/transfer";
    /// 添加用户任务处理人
    pub const APASS_V1_FLOW_USER_TASK_ADD_ASSIGNEE: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/flow/user_task/{task_id}/add_assignee";
    /// 抄送用户任务
    pub const APASS_V1_FLOW_USER_TASK_CC: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/flow/user_task/{task_id}/cc";
    /// 催办用户任务
    pub const APASS_V1_FLOW_USER_TASK_EXPEDITING: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/flow/user_task/{task_id}/expediting";
    /// 取消用户任务
    pub const APASS_V1_FLOW_USER_TASK_CANCEL: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/flow/user_task/{task_id}/cancel";
    /// 查询用户任务回退点
    pub const APASS_V1_FLOW_USER_TASK_ROLLBACK_POINTS: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/flow/user_task/{task_id}/rollback_points";
    /// 回退用户任务
    pub const APASS_V1_FLOW_USER_TASK_ROLLBACK: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/flow/user_task/{task_id}/rollback";
    /// 获取用户任务群聊
    pub const APASS_V1_FLOW_USER_TASK_CHAT_GROUP: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/flow/user_task/{task_id}/chat_group";

    // ===== 函数管理端点 =====
    /// 调用函数
    pub const APASS_V1_FUNCTION_INVOKE: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/function/{function_name}/invoke";

    // ===== 权限管理端点 =====
    /// 批量移除角色成员
    pub const APASS_V1_PERMISSION_ROLE_MEMBERS_BATCH_REMOVE: &'static str = "/open-apis/apaas/v1/application/{app_id}/permission/role/{role_api_name}/members/batch_remove";
    /// 批量添加角色成员
    pub const APASS_V1_PERMISSION_ROLE_MEMBERS_BATCH_CREATE: &'static str = "/open-apis/apaas/v1/application/{app_id}/permission/role/{role_api_name}/members/batch_create";
    /// 获取角色成员
    pub const APASS_V1_PERMISSION_ROLE_MEMBER_GET: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/permission/role/{role_api_name}/member/{user_id}";
    /// 批量移除记录权限成员
    pub const APASS_V1_PERMISSION_RECORD_MEMBERS_BATCH_REMOVE: &'static str = "/open-apis/apaas/v1/application/{app_id}/permission/record_permission/{record_permission_api_name}/members/batch_remove";
    /// 批量添加记录权限成员
    pub const APASS_V1_PERMISSION_RECORD_MEMBERS_BATCH_CREATE: &'static str = "/open-apis/apaas/v1/application/{app_id}/permission/record_permission/{record_permission_api_name}/members/batch_create";

    // ===== 对象管理端点 =====
    /// OQL 查询
    pub const APASS_V1_OBJECT_OQL: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/object/oql";
    /// 搜索记录
    pub const APASS_V1_OBJECT_RECORD_SEARCH: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/object/{object_api_name}/record/search";
    /// 获取记录
    pub const APASS_V1_OBJECT_RECORD_GET: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/object/{object_api_name}/record/{record_id}";
    /// 更新记录
    pub const APASS_V1_OBJECT_RECORD_UPDATE: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/object/{object_api_name}/record/{record_id}";
    /// 删除记录
    pub const APASS_V1_OBJECT_RECORD_DELETE: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/object/{object_api_name}/record/{record_id}";
    /// 创建记录
    pub const APASS_V1_OBJECT_RECORD_CREATE: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/object/{object_api_name}/record";
    /// 批量更新记录
    pub const APASS_V1_OBJECT_RECORD_BATCH_UPDATE: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/object/{object_api_name}/record/batch_update";
    /// 批量查询记录
    pub const APASS_V1_OBJECT_RECORD_BATCH_QUERY: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/object/{object_api_name}/record/batch_query";
    /// 批量删除记录
    pub const APASS_V1_OBJECT_RECORD_BATCH_DELETE: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/object/{object_api_name}/record/batch_delete";
    /// 批量创建记录
    pub const APASS_V1_OBJECT_RECORD_BATCH_CREATE: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/object/{object_api_name}/record/batch_create";

    // ===== 环境变量管理端点 =====
    /// 查询环境变量
    pub const APASS_V1_ENVIRONMENT_VARIABLE_QUERY: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/environment_variable/query";
    /// 获取环境变量
    pub const APASS_V1_ENVIRONMENT_VARIABLE_GET: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/environment_variable/{variable_name}";

    // ===== 审计日志管理端点 =====
    /// 审计日志列表
    pub const APASS_V1_AUDIT_LOG_LIST: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/audit_log/list";
    /// 获取审计日志
    pub const APASS_V1_AUDIT_LOG_GET: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/audit_log/{log_id}";
    /// 数据变更日志列表
    pub const APASS_V1_AUDIT_LOG_DATA_CHANGE_LOGS: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/audit_log/data_change_logs";
    /// 获取数据变更日志
    pub const APASS_V1_AUDIT_LOG_DATA_CHANGE_LOG_GET: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/audit_log/data_change_log/{log_id}";
    /// 审计事件列表
    pub const APASS_V1_AUDIT_LOG_AUDIT_EVENTS: &'static str =
        "/open-apis/apaas/v1/application/{app_id}/audit_log/audit_events";

    // ==================== Performance 绩效管理相关端点 ====================
    /// 评估数据查询
    pub const PERFORMANCE_V1_REVIEW_DATA_QUERY: &'static str =
        "/open-apis/performance/v1/review_data/query";

    /// 评估详情数据查询
    pub const PERFORMANCE_V1_REVIEW_DATA_DETAILS_QUERY: &'static str =
        "/open-apis/performance/v1/review_data/details/query";

    /// 阶段任务用户列表查询
    pub const PERFORMANCE_V1_STAGE_TASK_FIND_BY_USER_LIST: &'static str =
        "/open-apis/performance/v1/stage_task/find_by_user_list";

    /// 阶段任务分页查询
    pub const PERFORMANCE_V1_STAGE_TASK_FIND_BY_PAGE: &'static str =
        "/open-apis/performance/v1/stage_task/find_by_page";

    /// 指标详情查询
    pub const PERFORMANCE_V1_METRIC_DETAIL_QUERY: &'static str =
        "/open-apis/performance/v1/metric_detail/query";

    /// 指标详情导入
    pub const PERFORMANCE_V1_METRIC_DETAIL_IMPORT: &'static str =
        "/open-apis/performance/v1/metric_detail/import";

    /// 绩效周期列表查询
    pub const PERFORMANCE_SEMESTER_LIST: &'static str =
        "/open-apis/performance/v1/review_config/semester_activity/semesters";

    /// 绩效项目查询
    pub const PERFORMANCE_ACTIVITIES_QUERY: &'static str =
        "/open-apis/performance/v1/review_config/semester_activity/activities";

    /// 查询附加信息
    pub const PERFORMANCE_ADDITIONAL_INFO_QUERY: &'static str =
        "/open-apis/performance/v1/review_config/semester_activity/additional_information/query";

    /// 导入附加信息
    pub const PERFORMANCE_ADDITIONAL_INFO_IMPORT: &'static str =
        "/open-apis/performance/v1/review_config/semester_activity/additional_information/import";

    /// 删除附加信息
    pub const PERFORMANCE_ADDITIONAL_INFO_DELETE: &'static str =
        "/open-apis/performance/v1/review_config/semester_activity/additional_information/delete";

    /// 用户组关系写入
    pub const PERFORMANCE_USER_GROUP_WRITE: &'static str =
        "/open-apis/performance/v1/review_config/semester_activity/user_group_user_rel/write";

    /// 评估模板查询
    pub const PERFORMANCE_REVIEW_TEMPLATES_QUERY: &'static str =
        "/open-apis/performance/v1/review_config/review_templates/query";

    /// 评估项目查询
    pub const PERFORMANCE_REVIEW_ITEMS_QUERY: &'static str =
        "/open-apis/performance/v1/review_config/review_items/query";

    /// 标签问题查询
    pub const PERFORMANCE_TAG_QUESTIONS_QUERY: &'static str =
        "/open-apis/performance/v1/review_config/tag_questions/query";

    /// 指标查询
    pub const PERFORMANCE_METRICS_QUERY: &'static str =
        "/open-apis/performance/v1/review_config/metrics/query";

    /// 指标模板查询
    pub const PERFORMANCE_METRIC_TEMPLATES_QUERY: &'static str =
        "/open-apis/performance/v1/review_config/metric_templates/query";

    /// 指标字段查询
    pub const PERFORMANCE_METRIC_FIELDS_QUERY: &'static str =
        "/open-apis/performance/v1/review_config/metric_fields/query";

    /// 指标标签管理
    pub const PERFORMANCE_METRIC_TAGS: &'static str =
        "/open-apis/performance/v1/review_config/metric_tags";

    /// 被评估人查询
    pub const PERFORMANCE_REVIEWEES_QUERY: &'static str =
        "/open-apis/performance/v1/review_config/semester_activity/reviewees/query";

    // ==================== Personal Settings 个人设置相关端点 ====================

    /// 系统状态管理
    pub const PERSONAL_SETTINGS_V1_SYSTEM_STATUSES: &'static str =
        "/open-apis/personal_settings/v1/system_statuses";

    /// 系统状态操作
    pub const PERSONAL_SETTINGS_V1_SYSTEM_STATUS_OPERATION: &'static str =
        "/open-apis/personal_settings/v1/system_statuses/{system_status_id}";

    /// 批量开启系统状态
    pub const PERSONAL_SETTINGS_V1_SYSTEM_STATUS_BATCH_OPEN: &'static str =
        "/open-apis/personal_settings/v1/system_statuses/batch_open";

    /// 批量关闭系统状态
    pub const PERSONAL_SETTINGS_V1_SYSTEM_STATUS_BATCH_CLOSE: &'static str =
        "/open-apis/personal_settings/v1/system_statuses/batch_close";

    // ==================== CoreHR 人力资源管理相关端点 ====================
    /// 公司信息管理
    pub const COREHR_COMPANIES: &'static str = "/open-apis/corehr/v1/companies";

    /// 部门管理
    pub const COREHR_DEPARTMENTS: &'static str = "/open-apis/corehr/v1/departments";

    /// 部门批量获取
    pub const COREHR_DEPARTMENTS_BATCH_GET: &'static str =
        "/open-apis/corehr/v1/departments/batch_get";

    /// 部门树结构
    pub const COREHR_DEPARTMENTS_TREE: &'static str = "/open-apis/corehr/v1/departments/tree";

    /// 员工批量获取
    pub const COREHR_EMPLOYEES_BATCH_GET: &'static str = "/open-apis/corehr/v1/employees/batch_get";

    /// 员工搜索
    pub const COREHR_EMPLOYEES_SEARCH: &'static str = "/open-apis/corehr/v1/employees/search";

    /// 职位管理
    pub const COREHR_JOBS: &'static str = "/open-apis/corehr/v1/jobs";

    /// 职位族群管理
    pub const COREHR_JOB_FAMILIES: &'static str = "/open-apis/corehr/v1/job_families";

    /// 职级管理
    pub const COREHR_JOB_LEVELS: &'static str = "/open-apis/corehr/v1/job_levels";

    /// 职等管理
    pub const COREHR_JOB_GRADES: &'static str = "/open-apis/corehr/v1/job_grades";

    /// 职等查询
    pub const COREHR_JOB_GRADES_QUERY: &'static str = "/open-apis/corehr/v1/job_grades/query";

    /// 异动记录管理
    pub const COREHR_JOB_CHANGES: &'static str = "/open-apis/corehr/v1/job_changes";

    /// 异动记录搜索
    pub const COREHR_JOB_CHANGES_SEARCH: &'static str = "/open-apis/corehr/v1/job_changes/search";

    /// 待入职管理
    pub const COREHR_PRE_HIRES: &'static str = "/open-apis/corehr/v1/pre_hires";

    /// 待入职搜索
    pub const COREHR_PRE_HIRES_SEARCH: &'static str = "/open-apis/corehr/v1/pre_hires/search";

    /// 离职管理
    pub const COREHR_OFFBOARDINGS: &'static str = "/open-apis/corehr/v1/offboardings";

    /// 离职搜索
    pub const COREHR_OFFBOARDINGS_SEARCH: &'static str = "/open-apis/corehr/v1/offboardings/search";

    /// 基础信息枚举搜索
    pub const COREHR_BASIC_INFO_ENUM_SEARCH: &'static str =
        "/open-apis/corehr/v1/basic_info/enum/search";

    /// 地理位置数据搜索
    pub const COREHR_BASIC_INFO_LOCATION_SEARCH: &'static str =
        "/open-apis/corehr/v1/basic_info/location_data/search";

    /// 国籍搜索
    pub const COREHR_BASIC_INFO_NATIONALITY_SEARCH: &'static str =
        "/open-apis/corehr/v1/basic_info/nationality/search";

    /// 通用数据ID转换
    pub const COREHR_COMMON_DATA_ID_CONVERT: &'static str =
        "/open-apis/corehr/v1/common_data_id/convert";

    // ==================== Contact v3 通讯录相关端点 ====================
    /// 用户管理
    pub const CONTACT_V3_USERS: &'static str = "/open-apis/contact/v3/users";

    /// 用户批量获取
    pub const CONTACT_V3_USERS_BATCH: &'static str = "/open-apis/contact/v3/users/batch";

    /// 用户批量获取ID
    pub const CONTACT_V3_USERS_BATCH_GET_ID: &'static str =
        "/open-apis/contact/v3/users/batch_get_id";

    /// 根据部门查找用户
    pub const CONTACT_V3_USERS_FIND_BY_DEPARTMENT: &'static str =
        "/open-apis/contact/v3/users/find_by_department";

    /// 用户搜索
    pub const CONTACT_V3_USERS_SEARCH: &'static str = "/open-apis/contact/v3/users/search";

    /// 部门管理
    pub const CONTACT_V3_DEPARTMENTS: &'static str = "/open-apis/contact/v3/departments";

    /// 部门批量获取
    pub const CONTACT_V3_DEPARTMENTS_BATCH: &'static str =
        "/open-apis/contact/v3/departments/batch";

    /// 部门子部门查询
    pub const CONTACT_V3_DEPARTMENTS_CHILDREN: &'static str =
        "/open-apis/contact/v3/departments/children";

    /// 部门父部门查询
    pub const CONTACT_V3_DEPARTMENTS_PARENT: &'static str =
        "/open-apis/contact/v3/departments/parent";

    /// 部门搜索
    pub const CONTACT_V3_DEPARTMENTS_SEARCH: &'static str =
        "/open-apis/contact/v3/departments/search";

    /// 职务管理
    pub const CONTACT_V3_JOB_TITLES: &'static str = "/open-apis/contact/v3/job_titles";

    /// 职位族群管理
    pub const CONTACT_V3_JOB_FAMILIES: &'static str = "/open-apis/contact/v3/job_families";

    /// 职级管理
    pub const CONTACT_V3_JOB_LEVELS: &'static str = "/open-apis/contact/v3/job_levels";

    /// 功能角色管理
    pub const CONTACT_V3_FUNCTIONAL_ROLES: &'static str = "/open-apis/contact/v3/functional_roles";

    /// 用户管理 - 带参数的端点
    pub const CONTACT_V3_USER_GET: &'static str = "/open-apis/contact/v3/users/{user_id}";
    pub const CONTACT_V3_USER_UPDATE_ID: &'static str =
        "/open-apis/contact/v3/users/{user_id}/update_user_id";
    pub const CONTACT_V3_USER_RESURRECT: &'static str =
        "/open-apis/contact/v3/users/{user_id}/resurrect";

    /// 部门管理 - 带参数的端点
    pub const CONTACT_V3_DEPARTMENT_GET: &'static str =
        "/open-apis/contact/v3/departments/{department_id}";
    pub const CONTACT_V3_DEPARTMENT_UPDATE_ID: &'static str =
        "/open-apis/contact/v3/departments/{department_id}/update_department_id";

    /// 职务管理 - 带参数的端点
    pub const CONTACT_V3_JOB_TITLE_GET: &'static str =
        "/open-apis/contact/v3/job_titles/{job_title_id}";

    /// 职位族群管理 - 带参数的端点
    pub const CONTACT_V3_JOB_FAMILY_GET: &'static str =
        "/open-apis/contact/v3/job_families/{job_family_id}";

    /// 职级管理 - 带参数的端点
    pub const CONTACT_V3_JOB_LEVEL_GET: &'static str =
        "/open-apis/contact/v3/job_levels/{job_level_id}";

    /// 功能角色管理 - 带参数的端点
    pub const CONTACT_V3_FUNCTIONAL_ROLE_GET: &'static str =
        "/open-apis/contact/v3/functional_roles/{role_id}";
    pub const CONTACT_V3_FUNCTIONAL_ROLE_MEMBERS: &'static str =
        "/open-apis/contact/v3/functional_roles/{role_id}/members";
    pub const CONTACT_V3_FUNCTIONAL_ROLE_MEMBERS_BATCH_CREATE: &'static str =
        "/open-apis/contact/v3/functional_roles/{role_id}/members/batch_create";
    pub const CONTACT_V3_FUNCTIONAL_ROLE_MEMBERS_BATCH_DELETE: &'static str =
        "/open-apis/contact/v3/functional_roles/{role_id}/members/batch_delete";
    pub const CONTACT_V3_FUNCTIONAL_ROLE_MEMBERS_SCOPES: &'static str =
        "/open-apis/contact/v3/functional_roles/{role_id}/members/scopes";
    pub const CONTACT_V3_FUNCTIONAL_ROLE_MEMBER_GET: &'static str =
        "/open-apis/contact/v3/functional_roles/{role_id}/members/{member_id}";

    /// 群组管理
    pub const CONTACT_V3_GROUPS: &'static str = "/open-apis/contact/v3/groups";
    pub const CONTACT_V3_GROUPS_SIMPLELIST: &'static str =
        "/open-apis/contact/v3/groups/simplelist";
    pub const CONTACT_V3_GROUPS_MEMBER_BELONG: &'static str =
        "/open-apis/contact/v3/groups/member_belong";
    pub const CONTACT_V3_GROUP_GET: &'static str = "/open-apis/contact/v3/groups/{group_id}";
    pub const CONTACT_V3_GROUP_DETAIL: &'static str =
        "/open-apis/contact/v3/groups/{group_id}/detail";

    /// 群组成员管理
    pub const CONTACT_V3_GROUP_MEMBERS_ADD: &'static str =
        "/open-apis/contact/v3/groups/{group_id}/members/add";
    pub const CONTACT_V3_GROUP_MEMBERS_BATCH_ADD: &'static str =
        "/open-apis/contact/v3/groups/{group_id}/members/batch_add";
    pub const CONTACT_V3_GROUP_MEMBERS_REMOVE: &'static str =
        "/open-apis/contact/v3/groups/{group_id}/members/remove";
    pub const CONTACT_V3_GROUP_MEMBERS_BATCH_REMOVE: &'static str =
        "/open-apis/contact/v3/groups/{group_id}/members/batch_remove";
    pub const CONTACT_V3_GROUP_MEMBERS_SIMPLELIST: &'static str =
        "/open-apis/contact/v3/groups/{group_id}/members/simplelist";

    /// 单位管理
    pub const CONTACT_V3_UNITS: &'static str = "/open-apis/contact/v3/units";
    pub const CONTACT_V3_UNIT_GET: &'static str = "/open-apis/contact/v3/units/{unit_id}";
    pub const CONTACT_V3_UNIT_BIND_DEPARTMENT: &'static str =
        "/open-apis/contact/v3/units/{unit_id}/bind_department";
    pub const CONTACT_V3_UNIT_UNBIND_DEPARTMENT: &'static str =
        "/open-apis/contact/v3/units/{unit_id}/unbind_department";
    pub const CONTACT_V3_UNIT_LIST_DEPARTMENT: &'static str =
        "/open-apis/contact/v3/units/{unit_id}/list_department";

    /// 工作城市管理
    pub const CONTACT_V3_WORK_CITIES: &'static str = "/open-apis/contact/v3/work_cities";
    pub const CONTACT_V3_WORK_CITY_GET: &'static str =
        "/open-apis/contact/v3/work_cities/{work_city_id}";

    /// 员工类型枚举管理
    pub const CONTACT_V3_EMPLOYEE_TYPE_ENUMS: &'static str =
        "/open-apis/contact/v3/employee_type_enums";
    pub const CONTACT_V3_EMPLOYEE_TYPE_ENUM_GET: &'static str =
        "/open-apis/contact/v3/employee_type_enums/{enum_id}";

    /// 自定义属性管理
    pub const CONTACT_V3_CUSTOM_ATTRS: &'static str = "/open-apis/contact/v3/custom_attrs";

    /// 权限范围管理
    pub const CONTACT_V3_SCOPES: &'static str = "/open-apis/contact/v3/scopes";

    // ==================== 考勤服务端点 ====================

    // 考勤组管理
    /// 考勤组列表查询
    pub const ATTENDANCE_V1_GROUPS: &'static str = "/open-apis/attendance/v1/groups";

    /// 考勤组详情查询 (需要使用 EndpointBuilder::replace_param 替换 {group_id})
    pub const ATTENDANCE_V1_GROUP_GET: &'static str = "/open-apis/attendance/v1/groups/{group_id}";

    /// 考勤组删除 (需要使用 EndpointBuilder::replace_param 替换 {group_id})
    pub const ATTENDANCE_V1_GROUP_DELETE: &'static str =
        "/open-apis/attendance/v1/groups/{group_id}";

    /// 考勤组搜索
    pub const ATTENDANCE_V1_GROUPS_SEARCH: &'static str = "/open-apis/attendance/v1/groups/search";

    /// 考勤组用户列表 (需要使用 EndpointBuilder::replace_param 替换 {group_id})
    pub const ATTENDANCE_V1_GROUP_USERS: &'static str =
        "/open-apis/attendance/v1/groups/{group_id}/users";

    // 班次管理
    /// 班次列表查询
    pub const ATTENDANCE_V1_SHIFTS: &'static str = "/open-apis/attendance/v1/shifts";

    /// 班次详情查询 (需要使用 EndpointBuilder::replace_param 替换 {shift_id})
    pub const ATTENDANCE_V1_SHIFT_GET: &'static str = "/open-apis/attendance/v1/shifts/{shift_id}";

    /// 班次删除 (需要使用 EndpointBuilder::replace_param 替换 {shift_id})
    pub const ATTENDANCE_V1_SHIFT_DELETE: &'static str =
        "/open-apis/attendance/v1/shifts/{shift_id}";

    /// 班次查询
    pub const ATTENDANCE_V1_SHIFTS_QUERY: &'static str = "/open-apis/attendance/v1/shifts/query";

    // 用户任务管理
    /// 用户任务批量创建
    pub const ATTENDANCE_V1_USER_TASKS_BATCH_CREATE: &'static str =
        "/open-apis/attendance/v1/user_tasks/batch_create";

    /// 用户任务查询
    pub const ATTENDANCE_V1_USER_TASKS_QUERY: &'static str =
        "/open-apis/attendance/v1/user_tasks/query";

    /// 用户任务批量删除
    pub const ATTENDANCE_V1_USER_TASKS_BATCH_DELETE: &'static str =
        "/open-apis/attendance/v1/user_tasks/batch_del";

    /// 用户任务结果查询
    pub const ATTENDANCE_V1_USER_TASK_RESULTS_QUERY: &'static str =
        "/open-apis/attendance/v1/user_task_results/query";

    // 用户班表管理
    /// 用户班表批量创建
    pub const ATTENDANCE_V1_USER_DAILY_SHIFTS_BATCH_CREATE: &'static str =
        "/open-apis/attendance/v1/user_daily_shifts/batch_create";

    /// 用户班表查询
    pub const ATTENDANCE_V1_USER_DAILY_SHIFTS_QUERY: &'static str =
        "/open-apis/attendance/v1/user_daily_shifts/query";

    // 用户统计数据管理
    /// 用户统计数据查询
    pub const ATTENDANCE_V1_USER_STATS_DATAS_QUERY: &'static str =
        "/open-apis/attendance/v1/user_stats_datas/query";

    /// 用户统计数据字段查询
    pub const ATTENDANCE_V1_USER_STATS_DATAS_QUERY_FIELDS: &'static str =
        "/open-apis/attendance/v1/user_stats_datas/query_fields";

    /// 用户统计数据数据查询
    pub const ATTENDANCE_V1_USER_STATS_DATAS_QUERY_DATA: &'static str =
        "/open-apis/attendance/v1/user_stats_datas/query_data";

    /// 用户统计数据更新
    pub const ATTENDANCE_V1_USER_STATS_DATAS_UPDATE: &'static str =
        "/open-apis/attendance/v1/user_stats_datas/update";

    // 用户设置管理
    /// 用户设置查询
    pub const ATTENDANCE_V1_USER_SETTINGS_QUERY: &'static str =
        "/open-apis/attendance/v1/user_settings/query";

    // 用户审批管理
    /// 用户审批列表
    pub const ATTENDANCE_V1_USER_APPROVALS: &'static str =
        "/open-apis/attendance/v1/user_approvals";

    // 用户任务补救管理
    /// 用户任务补救列表
    pub const ATTENDANCE_V1_USER_TASK_REMEDYS: &'static str =
        "/open-apis/attendance/v1/user_task_remedys";

    // 假期管理
    /// 离职假期过期记录
    pub const ATTENDANCE_V1_LEAVE_EMPLOY_EXPIRE_RECORDS: &'static str =
        "/open-apis/attendance/v1/leave_employ_expire_records";

    // 归档规则管理
    /// 归档规则
    pub const ATTENDANCE_V1_ARCHIVE_RULES: &'static str = "/open-apis/attendance/v1/archive_rules";

    /// 归档规则用户统计字段 (需要使用 EndpointBuilder::replace_param 替换 {archive_rule_id})
    pub const ATTENDANCE_V1_ARCHIVE_RULE_USER_STATS_FIELDS: &'static str =
        "/open-apis/attendance/v1/archive_rules/{archive_rule_id}/user_stats_fields";

    /// 归档规则上传报表 (需要使用 EndpointBuilder::replace_param 替换 {archive_rule_id})
    pub const ATTENDANCE_V1_ARCHIVE_RULE_UPLOAD_REPORT: &'static str =
        "/open-apis/attendance/v1/archive_rules/{archive_rule_id}/upload_report";

    /// 归档规则删除报表 (需要使用 EndpointBuilder::replace_param 替换 {archive_rule_id})
    pub const ATTENDANCE_V1_ARCHIVE_RULE_DEL_REPORT: &'static str =
        "/open-apis/attendance/v1/archive_rules/{archive_rule_id}/del_report";

    // 用户设置管理
    /// 用户设置修改 (需要使用 EndpointBuilder::replace_param 替换 {user_id})
    pub const ATTENDANCE_V1_USER_SETTINGS_MODIFY: &'static str =
        "/open-apis/attendance/v1/user_settings/{user_id}/modify";

    /// 用户设置上传 (需要使用 EndpointBuilder::replace_param 替换 {user_id})
    pub const ATTENDANCE_V1_USER_SETTINGS_UPLOAD: &'static str =
        "/open-apis/attendance/v1/user_settings/{user_id}/upload";

    /// 用户设置下载 (需要使用 EndpointBuilder::replace_param 替换 {user_id})
    pub const ATTENDANCE_V1_USER_SETTINGS_DOWNLOAD: &'static str =
        "/open-apis/attendance/v1/user_settings/{user_id}/download";

    // 用户审批管理
    /// 用户审批处理 (需要使用 EndpointBuilder::replace_param 替换 {approval_id})
    pub const ATTENDANCE_V1_USER_APPROVAL_PROCESS: &'static str =
        "/open-apis/attendance/v1/user_approvals/{approval_id}/process";

    // 用户任务补救管理
    /// 查询用户允许的补救
    pub const ATTENDANCE_V1_USER_TASK_REMEDYS_QUERY_USER_ALLOWED_REMEDYS: &'static str =
        "/open-apis/attendance/v1/user_task_remedys/query_user_allowed_remedys";

    // 用户班表管理
    /// 用户班表批量创建临时
    pub const ATTENDANCE_V1_USER_DAILY_SHIFTS_BATCH_CREATE_TEMP: &'static str =
        "/open-apis/attendance/v1/user_daily_shifts/batch_create_temp";

    // 用户任务管理
    /// 用户任务获取 (需要使用 EndpointBuilder::replace_param 替换 {user_id})
    pub const ATTENDANCE_V1_USER_TASK_GET: &'static str =
        "/open-apis/attendance/v1/user_tasks/{user_id}/get";

    // 假期管理
    /// 假期计提记录获取 (需要使用 EndpointBuilder::replace_param 替换 {leave_accrual_record_id})
    pub const ATTENDANCE_V1_LEAVE_ACCRUAL_RECORD_GET: &'static str =
        "/open-apis/attendance/v1/leave_accrual_records/{leave_accrual_record_id}";

    // ==================== 客服工具服务端点 ====================

    // 工单管理
    /// 开始服务
    pub const HELPDESK_V1_START_SERVICE: &'static str = "/open-apis/helpdesk/v1/start_service";

    /// 工单列表
    pub const HELPDESK_V1_TICKETS: &'static str = "/open-apis/helpdesk/v1/tickets";

    /// 工单详情
    pub const HELPDESK_V1_TICKET_GET: &'static str = "/open-apis/helpdesk/v1/tickets/{ticket_id}";

    /// 工单更新
    pub const HELPDESK_V1_TICKET_UPDATE: &'static str =
        "/open-apis/helpdesk/v1/tickets/{ticket_id}";

    // 工单消息管理
    /// 工单消息列表
    pub const HELPDESK_V1_TICKET_MESSAGES: &'static str =
        "/open-apis/helpdesk/v1/tickets/{ticket_id}/messages";

    /// 工单消息创建
    pub const HELPDESK_V1_TICKET_MESSAGE_CREATE: &'static str =
        "/open-apis/helpdesk/v1/tickets/{ticket_id}/messages";

    /// 工单机器人消息
    pub const HELPDESK_V1_TICKET_BOT_MESSAGES: &'static str =
        "/open-apis/helpdesk/v1/tickets/{ticket_id}/bot_messages";

    // FAQ管理
    /// FAQ列表
    pub const HELPDESK_V1_FAQS: &'static str = "/open-apis/helpdesk/v1/faqs";

    /// FAQ详情
    pub const HELPDESK_V1_FAQ_GET: &'static str = "/open-apis/helpdesk/v1/faqs/{faq_id}";

    /// FAQ创建
    pub const HELPDESK_V1_FAQ_CREATE: &'static str = "/open-apis/helpdesk/v1/faqs";

    /// FAQ更新
    pub const HELPDESK_V1_FAQ_UPDATE: &'static str = "/open-apis/helpdesk/v1/faqs/{faq_id}";

    /// FAQ删除
    pub const HELPDESK_V1_FAQ_DELETE: &'static str = "/open-apis/helpdesk/v1/faqs/{faq_id}";

    /// FAQ图片
    pub const HELPDESK_V1_FAQ_IMAGE: &'static str =
        "/open-apis/helpdesk/v1/faqs/{faq_id}/image/{image_key}";

    /// FAQ搜索
    pub const HELPDESK_V1_FAQS_SEARCH: &'static str = "/open-apis/helpdesk/v1/faqs/search";

    // 事件管理
    /// 事件订阅
    pub const HELPDESK_V1_EVENTS_SUBSCRIBE: &'static str =
        "/open-apis/helpdesk/v1/events/subscribe";

    /// 事件取消订阅
    pub const HELPDESK_V1_EVENTS_UNSUBSCRIBE: &'static str =
        "/open-apis/helpdesk/v1/events/unsubscribe";

    // 自定义字段管理
    /// 工单自定义字段列表
    pub const HELPDESK_V1_TICKET_CUSTOMIZED_FIELDS: &'static str =
        "/open-apis/helpdesk/v1/ticket_customized_fields";

    /// 工单自定义字段详情
    pub const HELPDESK_V1_TICKET_CUSTOMIZED_FIELD_GET: &'static str =
        "/open-apis/helpdesk/v1/ticket_customized_fields/{field_id}";

    /// 工单自定义字段创建
    pub const HELPDESK_V1_TICKET_CUSTOMIZED_FIELD_CREATE: &'static str =
        "/open-apis/helpdesk/v1/ticket_customized_fields";

    /// 工单自定义字段更新
    pub const HELPDESK_V1_TICKET_CUSTOMIZED_FIELD_UPDATE: &'static str =
        "/open-apis/helpdesk/v1/ticket_customized_fields/{field_id}";

    /// 工单自定义字段删除
    pub const HELPDESK_V1_TICKET_CUSTOMIZED_FIELD_DELETE: &'static str =
        "/open-apis/helpdesk/v1/ticket_customized_fields/{field_id}";

    // 客服管理
    /// 客服详情
    pub const HELPDESK_V1_AGENT_GET: &'static str = "/open-apis/helpdesk/v1/agents/{agent_id}";

    /// 客服邮箱
    pub const HELPDESK_V1_AGENT_EMAIL: &'static str =
        "/open-apis/helpdesk/v1/agents/{agent_id}/agent_email";

    // 客服排班管理
    /// 客服排班列表
    pub const HELPDESK_V1_AGENT_SCHEDULES: &'static str =
        "/open-apis/helpdesk/v1/agents/{agent_id}/agent_schedules";

    /// 客服排班详情
    pub const HELPDESK_V1_AGENT_SCHEDULE_GET: &'static str =
        "/open-apis/helpdesk/v1/agents/{agent_id}/agent_schedules/{schedule_id}";

    /// 客服排班创建
    pub const HELPDESK_V1_AGENT_SCHEDULE_CREATE: &'static str =
        "/open-apis/helpdesk/v1/agents/{agent_id}/agent_schedules";

    /// 客服排班更新
    pub const HELPDESK_V1_AGENT_SCHEDULE_UPDATE: &'static str =
        "/open-apis/helpdesk/v1/agents/{agent_id}/agent_schedules/{schedule_id}";

    /// 客服排班删除
    pub const HELPDESK_V1_AGENT_SCHEDULE_DELETE: &'static str =
        "/open-apis/helpdesk/v1/agents/{agent_id}/agent_schedules/{schedule_id}";

    // 客服技能管理
    /// 客服技能列表
    pub const HELPDESK_V1_AGENT_SKILLS: &'static str = "/open-apis/helpdesk/v1/agent_skills";

    /// 客服技能详情
    pub const HELPDESK_V1_AGENT_SKILL_GET: &'static str =
        "/open-apis/helpdesk/v1/agent_skills/{skill_id}";

    /// 客服技能创建
    pub const HELPDESK_V1_AGENT_SKILL_CREATE: &'static str = "/open-apis/helpdesk/v1/agent_skills";

    /// 客服技能更新
    pub const HELPDESK_V1_AGENT_SKILL_UPDATE: &'static str =
        "/open-apis/helpdesk/v1/agent_skills/{skill_id}";

    /// 客服技能删除
    pub const HELPDESK_V1_AGENT_SKILL_DELETE: &'static str =
        "/open-apis/helpdesk/v1/agent_skills/{skill_id}";

    // 客服技能规则管理
    /// 客服技能规则列表
    pub const HELPDESK_V1_AGENT_SKILL_RULES: &'static str =
        "/open-apis/helpdesk/v1/agent_skill_rules";

    /// 客服技能规则操作选项
    pub const HELPDESK_V1_AGENT_SKILL_RULES_OPERATOR_OPTIONS: &'static str =
        "/open-apis/helpdesk/v1/agent_skill_rules/operator-options";

    // 分类管理
    /// 分类列表
    pub const HELPDESK_V1_CATEGORIES: &'static str = "/open-apis/helpdesk/v1/categories";

    /// 分类详情
    pub const HELPDESK_V1_CATEGORY_GET: &'static str =
        "/open-apis/helpdesk/v1/categories/{category_id}";

    /// 分类创建
    pub const HELPDESK_V1_CATEGORY_CREATE: &'static str = "/open-apis/helpdesk/v1/categories";

    /// 分类更新
    pub const HELPDESK_V1_CATEGORY_UPDATE: &'static str =
        "/open-apis/helpdesk/v1/categories/{category_id}";

    /// 分类删除
    pub const HELPDESK_V1_CATEGORY_DELETE: &'static str =
        "/open-apis/helpdesk/v1/categories/{category_id}";

    // 通知管理
    /// 通知列表
    pub const HELPDESK_V1_NOTIFICATIONS: &'static str = "/open-apis/helpdesk/v1/notifications";

    /// 通知详情
    pub const HELPDESK_V1_NOTIFICATION_GET: &'static str =
        "/open-apis/helpdesk/v1/notifications/{notification_id}";

    /// 通知预览
    pub const HELPDESK_V1_NOTIFICATION_PREVIEW: &'static str =
        "/open-apis/helpdesk/v1/notifications/{notification_id}/preview";

    /// 通知创建
    pub const HELPDESK_V1_NOTIFICATION_CREATE: &'static str =
        "/open-apis/helpdesk/v1/notifications";

    /// 通知更新
    pub const HELPDESK_V1_NOTIFICATION_UPDATE: &'static str =
        "/open-apis/helpdesk/v1/notifications/{notification_id}";

    /// 通知提交审核
    pub const HELPDESK_V1_NOTIFICATION_SUBMIT_APPROVE: &'static str =
        "/open-apis/helpdesk/v1/notifications/{notification_id}/submit_approve";

    /// 通知取消审核
    pub const HELPDESK_V1_NOTIFICATION_CANCEL_APPROVE: &'static str =
        "/open-apis/helpdesk/v1/notifications/{notification_id}/cancel_approve";

    /// 通知执行推送
    pub const HELPDESK_V1_NOTIFICATION_EXECUTE_SEND: &'static str =
        "/open-apis/helpdesk/v1/notifications/{notification_id}/execute_send";

    /// 通知取消发送
    pub const HELPDESK_V1_NOTIFICATION_CANCEL_SEND: &'static str =
        "/open-apis/helpdesk/v1/notifications/{notification_id}/cancel_send";

    // ==================== 应用管理服务端点 ====================

    // 应用管理
    /// 应用详情
    pub const APPLICATION_V6_APP_GET: &'static str =
        "/open-apis/application/v6/applications/{app_id}";

    /// 应用转移所有者
    pub const APPLICATION_V6_APP_TRANSFER_OWNER: &'static str =
        "/open-apis/application/v6/applications/{app_id}/transfer_owner";

    /// 应用协作者管理
    pub const APPLICATION_V6_APP_COLLABORATORS: &'static str =
        "/open-apis/application/v6/applications/{app_id}/collaborators";

    /// 应用协作者删除
    pub const APPLICATION_V6_APP_COLLABORATOR_DELETE: &'static str =
        "/open-apis/application/v6/applications/{app_id}/collaborators";

    /// 应用版本管理
    pub const APPLICATION_V6_APP_VERSIONS: &'static str =
        "/open-apis/application/v6/applications/{app_id}/versions";

    /// 应用版本详情
    pub const APPLICATION_V6_APP_VERSION_GET: &'static str =
        "/open-apis/application/v6/applications/{app_id}/versions/{version_id}";

    /// 待审核应用列表
    pub const APPLICATION_V6_APPS_UNDERAUDITLIST: &'static str =
        "/open-apis/application/v6/applications/underauditlist";

    /// 应用审核
    pub const APPLICATION_V6_APP_AUDIT: &'static str =
        "/open-apis/application/v6/applications/{app_id}/audit";

    /// 应用组管理
    pub const APPLICATION_V6_APP_GROUP: &'static str =
        "/open-apis/application/v6/applications/{app_id}/group";

    // 应用使用情况
    /// 应用使用概览
    pub const APPLICATION_V6_APP_USAGE_OVERVIEW: &'static str =
        "/open-apis/application/v6/app_usage/{app_id}/overview";

    /// 应用消息推送概览
    pub const APPLICATION_V6_APP_USAGE_MESSAGE_PUSH_OVERVIEW: &'static str =
        "/open-apis/application/v6/app_usage/{app_id}/message_push_overview";

    /// 应用部门使用概览
    pub const APPLICATION_V6_APP_USAGE_DEPARTMENT_OVERVIEW: &'static str =
        "/open-apis/application/v6/app_usage/{app_id}/department_overview";

    // 应用权限范围
    /// 应用权限范围申请
    pub const APPLICATION_V6_APP_SCOPE_APPLY: &'static str =
        "/open-apis/application/v6/applications/{app_id}/scope/apply";

    /// 应用权限范围查询
    pub const APPLICATION_V6_APP_SCOPE_GET: &'static str =
        "/open-apis/application/v6/applications/{app_id}/scope";

    // 应用徽章管理
    /// 应用徽章设置
    pub const APPLICATION_V6_APP_BADGE_SET: &'static str =
        "/open-apis/application/v6/app_badge/{app_id}/users/{user_id}/set";

    // 应用反馈管理
    /// 应用反馈列表
    pub const APPLICATION_V6_APPLICATION_FEEDBACK: &'static str =
        "/open-apis/application/v6/application_feedback";

    /// 应用反馈详情
    pub const APPLICATION_V6_APPLICATION_FEEDBACK_GET: &'static str =
        "/open-apis/application/v6/application_feedback/{feedback_id}";

    // 管理员应用管理
    /// 管理员应用列表
    pub const APPLICATION_V6_ADMIN_APPS: &'static str = "/open-apis/application/v6/admin/apps";

    /// 管理员应用启用
    pub const APPLICATION_V6_ADMIN_APP_ENABLE: &'static str =
        "/open-apis/application/v6/admin/apps/{app_id}/enable";

    /// 管理员应用管理员列表
    pub const APPLICATION_V6_ADMIN_APP_ADMINS: &'static str =
        "/open-apis/application/v6/admin/apps/{app_id}/admins";

    /// 应用可见性管理
    pub const APPLICATION_V6_ADMIN_APP_VISIBILITY: &'static str =
        "/open-apis/application/v6/admin/apps/{app_id}/visibility";

    /// 应用可见性创建
    pub const APPLICATION_V6_ADMIN_APP_VISIBILITY_CREATE: &'static str =
        "/open-apis/application/v6/admin/apps/{app_id}/visibility";

    /// 用户可用应用
    pub const APPLICATION_V6_ADMIN_USER_AVAILABLE_APPS: &'static str =
        "/open-apis/application/v6/admin/user_available_apps/{user_id}";

    /// 应用联系人范围配置查询
    pub const APPLICATION_V6_ADMIN_APP_CONTACTS_RANGE_CONFIGURATION_GET: &'static str =
        "/open-apis/application/v6/admin/apps/{app_id}/contacts_range_configuration";

    /// 应用联系人范围配置设置
    pub const APPLICATION_V6_ADMIN_APP_CONTACTS_RANGE_CONFIGURATION_SET: &'static str =
        "/open-apis/application/v6/admin/apps/{app_id}/contacts_range_configuration";

    /// 应用白黑名单检查
    pub const APPLICATION_V6_ADMIN_APP_CHECK_WHITE_BLACK_LIST: &'static str =
        "/open-apis/application/v6/admin/apps/{app_id}/check_white_black_list";

    /// 应用管理员管理权限
    pub const APPLICATION_V6_ADMIN_APP_ADMIN_MANAGEMENT_PERMISSIONS: &'static str =
        "/open-apis/application/v6/admin/apps/{app_id}/admins/{user_id}/management_permissions";

    /// 应用管理员验证
    pub const APPLICATION_V6_ADMIN_APP_ADMIN_VERIFY: &'static str =
        "/open-apis/application/v6/admin/apps/{app_id}/admins/{user_id}/verify";

    /// 应用版本联系人范围建议
    pub const APPLICATION_V6_APP_VERSION_CONTACTS_RANGE_SUGGEST: &'static str =
        "/open-apis/application/v6/applications/{app_id}/versions/{version_id}/contacts_range_suggest";

    // 应用商店付费信息
    /// 应用商店定价计划检查
    pub const APPLICATION_V6_APPSTORE_PAID_INFO_CHECK: &'static str =
        "/open-apis/application/v6/appstore_paid_info/{app_id}/users/{user_id}/pricing_plans/{pricing_plan_id}/check";

    /// 应用商店定价计划列表
    pub const APPLICATION_V6_APPSTORE_PAID_INFO_PRICING_PLANS: &'static str =
        "/open-apis/application/v6/appstore_paid_info/{app_id}/pricing_plans";

    /// 应用商店订单详情
    pub const APPLICATION_V6_APPSTORE_PAID_INFO_ORDER_GET: &'static str =
        "/open-apis/application/v6/appstore_paid_info/{app_id}/orders/{order_id}";

    // ==================== CloudDocs 云文档相关端点 ====================

    // Assistant 助手服务
    /// 文档订阅
    pub const ASSISTANT_V1_FILE_SUBSCRIPTION: &'static str =
        "/open-apis/assistant/v1/file/{}/{}/subscription";

    // Bitable 多维表格服务
    /// 多维表格应用管理
    pub const BITABLE_V1_APPS: &'static str = "/open-apis/bitable/v1/apps";
    pub const BITABLE_V1_APP_GET: &'static str = "/open-apis/bitable/v1/apps/{app_token}";
    pub const BITABLE_V1_APP_CREATE: &'static str = "/open-apis/bitable/v1/apps";
    pub const BITABLE_V1_APP_UPDATE: &'static str = "/open-apis/bitable/v1/apps/{app_token}";
    pub const BITABLE_V1_APP_COPY: &'static str = "/open-apis/bitable/v1/apps/{app_token}/copy";

    /// 多维表格数据表管理
    pub const BITABLE_V1_TABLES: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables";
    pub const BITABLE_V1_TABLE_GET: &'static str = "/open-apis/bitable/v1/apps/{}/tables/{}";
    pub const BITABLE_V1_TABLE_CREATE: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/tables";
    pub const BITABLE_V1_TABLE_PATCH: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}";
    pub const BITABLE_V1_TABLE_DELETE: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}";
    pub const BITABLE_V1_TABLES_BATCH_CREATE: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/tables/batch_create";
    pub const BITABLE_V1_TABLES_BATCH_DELETE: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/tables/batch_delete";

    /// 多维表格记录管理
    pub const BITABLE_V1_RECORDS: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records";
    pub const BITABLE_V1_RECORD_GET: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/{record_id}";
    pub const BITABLE_V1_RECORD_CREATE: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records";
    pub const BITABLE_V1_RECORD_UPDATE: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/{record_id}";
    pub const BITABLE_V1_RECORD_DELETE: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/{record_id}";
    pub const BITABLE_V1_RECORDS_BATCH_CREATE: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_create";
    pub const BITABLE_V1_RECORDS_BATCH_UPDATE: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_update";
    pub const BITABLE_V1_RECORDS_BATCH_GET: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_get";
    pub const BITABLE_V1_RECORDS_BATCH_DELETE: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_delete";
    pub const BITABLE_V1_RECORDS_SEARCH: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/search";

    /// 多维表格字段管理
    pub const BITABLE_V1_FIELDS: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields";
    pub const BITABLE_V1_FIELD_GET: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields/{field_id}";
    pub const BITABLE_V1_FIELD_CREATE: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields";
    pub const BITABLE_V1_FIELD_UPDATE: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields/{field_id}";
    pub const BITABLE_V1_FIELD_DELETE: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields/{field_id}";

    /// 多维表格视图管理
    pub const BITABLE_V1_VIEWS: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/views";
    pub const BITABLE_V1_VIEW_GET: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/views/{view_id}";
    pub const BITABLE_V1_VIEW_CREATE: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/views";
    pub const BITABLE_V1_VIEW_PATCH: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/views/{view_id}";
    pub const BITABLE_V1_VIEW_DELETE: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/views/{view_id}";

    /// 多维表格仪表板管理
    pub const BITABLE_V1_DASHBOARDS: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/dashboards";
    pub const BITABLE_V1_DASHBOARD_COPY: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/dashboards/{block_id}/copy";

    /// 多维表格角色管理
    pub const BITABLE_V1_ROLES: &'static str = "/open-apis/bitable/v1/apps/{app_token}/roles";
    pub const BITABLE_V1_ROLE_GET: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}";
    pub const BITABLE_V1_ROLE_CREATE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/roles";
    pub const BITABLE_V1_ROLE_UPDATE: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}";
    pub const BITABLE_V1_ROLE_DELETE: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}";

    /// 多维表格角色成员管理
    pub const BITABLE_V1_ROLE_MEMBERS: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members";
    pub const BITABLE_V1_ROLE_MEMBER_CREATE: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members";
    pub const BITABLE_V1_ROLE_MEMBER_DELETE: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members/{member_id}";
    pub const BITABLE_V1_ROLE_MEMBERS_BATCH_CREATE: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members/batch_create";
    pub const BITABLE_V1_ROLE_MEMBERS_BATCH_DELETE: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members/batch_delete";

    /// 多维表格表单管理
    pub const BITABLE_V1_FORMS: &'static str = "/open-apis/bitable/v1/apps/{app_token}/forms";
    pub const BITABLE_V1_FORM_GET: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/forms/{form_id}";
    pub const BITABLE_V1_FORM_PATCH: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/forms/{form_id}";
    pub const BITABLE_V1_FORM_PATCH_META: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/forms/{form_id}/questions";
    pub const BITABLE_V1_FORM_QUESTION: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/forms/{form_id}/questions/{question_id}";

    /// 多维表格工作流管理
    pub const BITABLE_V1_WORKFLOWS: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/workflows";
    pub const BITABLE_V1_WORKFLOW_UPDATE: &'static str =
        "/open-apis/bitable/v1/apps/{app_token}/workflows/{workflow_id}";

    // Board 白板服务
    /// 白板管理
    pub const BOARD_V1_WHITEBOARD_THUMBNAIL: &'static str =
        "/open-apis/whiteboard/v1/whiteboards/{}/thumbnail";
    pub const BOARD_V1_WHITEBOARD_NODES: &'static str = "/open-apis/board/v1/whiteboards/{}/nodes";

    // Comments 评论服务
    /// 评论管理
    pub const COMMENT_V1_COMMENTS: &'static str = "/open-apis/comment/v1/comments";
    pub const COMMENT_V1_COMMENT_GET: &'static str = "/open-apis/comment/v1/comments/{}";
    pub const COMMENT_V1_COMMENT_CREATE: &'static str = "/open-apis/comment/v1/comments";
    pub const COMMENT_V1_COMMENT_PATCH: &'static str = "/open-apis/comment/v1/comments/{}";
    pub const COMMENT_V1_COMMENTS_BATCH_QUERY: &'static str =
        "/open-apis/comment/v1/comments/batch_query";

    /// 评论回复管理
    pub const COMMENT_V1_COMMENT_REPLIES: &'static str =
        "/open-apis/comment/v1/comments/{}/replies";
    pub const COMMENT_V1_COMMENT_REPLY_UPDATE: &'static str =
        "/open-apis/comment/v1/comments/{comment_id}/replies/{reply_id}";
    pub const COMMENT_V1_COMMENT_REPLY_DELETE: &'static str =
        "/open-apis/comment/v1/comments/{comment_id}/replies/{reply_id}";

    // Docx 文档服务
    /// 文档管理
    pub const DOCX_V1_DOCUMENTS: &'static str = "/open-apis/docx/v1/documents";
    pub const DOCX_V1_DOCUMENT_GET: &'static str = "/open-apis/docx/v1/documents/{}";
    pub const DOCX_V1_DOCUMENT_RAW_CONTENT: &'static str =
        "/open-apis/docx/v1/documents/{}/raw_content";
    pub const DOCX_V1_DOCUMENT_CONVERT: &'static str = "/open-apis/docx/v1/documents/{}/convert";

    /// 文档块管理
    pub const DOCX_V1_DOCUMENT_BLOCKS: &'static str = "/open-apis/docx/v1/documents/{}/blocks";
    pub const DOCX_V1_DOCUMENT_BLOCK_GET: &'static str =
        "/open-apis/docx/v1/documents/{document_id}/blocks/{block_id}";
    pub const DOCX_V1_DOCUMENT_BLOCK_CHILDREN: &'static str =
        "/open-apis/docx/v1/documents/{}/blocks/{}/children";
    pub const DOCX_V1_DOCUMENT_BLOCKS_BATCH_UPDATE: &'static str =
        "/open-apis/docx/v1/documents/{document_id}/blocks/batch_update";
    pub const DOCX_V1_DOCUMENT_BLOCKS_BATCH_DELETE: &'static str =
        "/open-apis/docx/v1/documents/{}/blocks/batch_delete";

    // Drive 云盘服务
    /// 文件管理
    pub const DRIVE_V1_FILES: &'static str = "/open-apis/drive/v1/files";
    pub const DRIVE_V1_FILE_GET: &'static str = "/open-apis/drive/v1/files/{}";
    pub const DRIVE_V1_FILE_COPY: &'static str = "/open-apis/drive/v1/files/{}/copy";
    pub const DRIVE_V1_FILE_DOWNLOAD: &'static str = "/open-apis/drive/v1/files/{}/download";
    pub const DRIVE_V1_FILE_STATISTICS: &'static str = "/open-apis/drive/v1/files/{}/statistics";
    pub const DRIVE_V1_FILE_VIEW_RECORDS: &'static str =
        "/open-apis/drive/v1/files/{}/view_records";
    pub const DRIVE_V1_FILE_LIKE_RECORDS: &'static str =
        "/open-apis/drive/v1/files/{}/like_records";
    pub const DRIVE_V1_FILES_CREATE_FOLDER: &'static str =
        "/open-apis/drive/v1/files/create_folder";
    pub const DRIVE_V1_FILES_CREATE_SHORTCUT: &'static str =
        "/open-apis/drive/v1/files/create_shortcut";
    pub const DRIVE_V1_FILES_SEARCH: &'static str = "/open-apis/drive/v1/files/search";
    pub const DRIVE_V1_FILES_SUBSCRIBE: &'static str = "/open-apis/drive/v1/files/subscribe";

    /// 文件版本管理
    pub const DRIVE_V1_FILE_VERSIONS: &'static str = "/open-apis/drive/v1/files/{}/versions";
    pub const DRIVE_V1_FILE_VERSION_GET: &'static str = "/open-apis/drive/v1/files/{}/versions/{}";

    /// 文件订阅管理
    pub const DRIVE_V1_FILE_SUBSCRIPTIONS: &'static str =
        "/open-apis/drive/v1/files/{}/subscriptions/{}";

    /// 文件夹管理
    pub const DRIVE_V1_FOLDERS: &'static str = "/open-apis/drive/v1/folders";
    pub const DRIVE_V1_FOLDER_GET: &'static str = "/open-apis/drive/v1/folders/{}";
    pub const DRIVE_V1_FOLDER_CHILDREN: &'static str = "/open-apis/drive/v1/folders/{}/children";
    pub const DRIVE_V1_FOLDER_MOVE: &'static str = "/open-apis/drive/v1/folders/{}/move";
    pub const DRIVE_V1_FOLDERS_ROOT_FOLDER_META: &'static str =
        "/open-apis/drive/v1/folders/root_folder_meta";

    /// 文件上传管理
    pub const DRIVE_V1_FILES_UPLOAD_ALL: &'static str = "/open-apis/drive/v1/files/upload_all";
    pub const DRIVE_V1_FILES_UPLOAD_PREPARE: &'static str =
        "/open-apis/drive/v1/files/upload_prepare";
    pub const DRIVE_V1_FILES_UPLOAD_PART: &'static str = "/open-apis/drive/v1/files/upload_part";
    pub const DRIVE_V1_FILES_UPLOAD_FINISH: &'static str =
        "/open-apis/drive/v1/files/upload_finish";

    /// 媒体文件管理
    pub const DRIVE_V1_MEDIAS_UPLOAD_ALL: &'static str = "/open-apis/drive/v1/medias/upload_all";
    pub const DRIVE_V1_MEDIAS_UPLOAD_PREPARE: &'static str =
        "/open-apis/drive/v1/medias/upload_prepare";
    pub const DRIVE_V1_MEDIAS_UPLOAD_PART: &'static str = "/open-apis/drive/v1/medias/upload_part";
    pub const DRIVE_V1_MEDIAS_UPLOAD_FINISH: &'static str =
        "/open-apis/drive/v1/medias/upload_finish";
    pub const DRIVE_V1_MEDIAS_DOWNLOAD: &'static str = "/open-apis/drive/v1/medias/{}/download";
    pub const DRIVE_V1_MEDIAS_BATCH_GET_TMP_DOWNLOAD_URL: &'static str =
        "/open-apis/drive/v1/medias/batch_get_tmp_download_url";

    /// 导入任务管理
    pub const DRIVE_V1_IMPORT_TASKS: &'static str = "/open-apis/drive/v1/import_tasks";
    pub const DRIVE_V1_IMPORT_TASK_GET: &'static str = "/open-apis/drive/v1/import_tasks/{}";

    /// 元信息管理
    pub const DRIVE_V1_METAS_BATCH_QUERY: &'static str = "/open-apis/drive/v1/metas/batch_query";

    /// 任务管理
    pub const DRIVE_V1_TASK_GET: &'static str = "/open-apis/drive/v1/tasks/{}";

    /// 权限管理
    pub const DRIVE_V1_PERMISSIONS_MEMBERS: &'static str =
        "/open-apis/drive/v1/permissions/{}/members";
    pub const DRIVE_V1_PERMISSIONS_MEMBER_GET: &'static str =
        "/open-apis/drive/v1/permissions/{}/members/{}";
    pub const DRIVE_V1_PERMISSIONS_MEMBERS_BATCH_CREATE: &'static str =
        "/open-apis/drive/v1/permissions/{}/members/batch_create";
    pub const DRIVE_V1_PERMISSIONS_MEMBERS_AUTH: &'static str =
        "/open-apis/drive/v1/permissions/{}/members/auth";
    pub const DRIVE_V1_PERMISSIONS_MEMBERS_TRANSFER_OWNER: &'static str =
        "/open-apis/drive/v1/permissions/{}/members/transfer_owner";
    pub const DRIVE_V1_PERMISSIONS_PUBLIC: &'static str =
        "/open-apis/drive/v1/permissions/{}/public";
    pub const DRIVE_V1_PERMISSIONS_PUBLIC_PASSWORD: &'static str =
        "/open-apis/drive/v1/permissions/{}/public/password";

    /// Drive v2 权限管理
    pub const DRIVE_V2_PERMISSIONS_PUBLIC: &'static str =
        "/open-apis/drive/v2/permissions/{}/public";

    /// Drive Explorer
    pub const DRIVE_EXPLORER_V2_ROOT_FOLDER_META: &'static str =
        "/open-apis/drive/explorer/v2/root_folder/meta";
    pub const DRIVE_EXPLORER_V2_FOLDER_META: &'static str =
        "/open-apis/drive/explorer/v2/folder/{folder_token}/meta";

    // Sheets 电子表格服务
    /// 电子表格管理 - v3
    pub const SHEETS_V3_SPREADSHEETS: &'static str = "/open-apis/sheets/v3/spreadsheets";
    pub const SHEETS_V3_SPREADSHEET_GET: &'static str = "/open-apis/sheets/v3/spreadsheets/{}";
    pub const SHEETS_V3_SPREADSHEET_CREATE: &'static str = "/open-apis/sheets/v3/spreadsheets";
    pub const SHEETS_V3_SPREADSHEET_PATCH: &'static str = "/open-apis/sheets/v3/spreadsheets/{}";

    /// 电子表格工作表管理 - v3
    pub const SHEETS_V3_SPREADSHEET_SHEETS_QUERY: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/query";
    pub const SHEETS_V3_SPREADSHEET_SHEET_GET: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}";

    /// 电子表格数据操作 - v3
    pub const SHEETS_V3_SPREADSHEET_VALUES_GET: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{}/values/{}";
    pub const SHEETS_V3_SPREADSHEET_VALUES_BATCH_GET: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{}/values/batch_get";
    pub const SHEETS_V3_SPREADSHEET_VALUES_BATCH_UPDATE: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{}/values/batch_update";
    pub const SHEETS_V3_SPREADSHEET_VALUES_APPEND: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{}/values/{}/append";
    pub const SHEETS_V3_SPREADSHEET_VALUES_PREPEND: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{}/values/{}/prepend";
    pub const SHEETS_V3_SPREADSHEET_VALUES_IMAGE: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{}/values_image";

    /// 电子表格查找替换 - v3
    pub const SHEETS_V3_SPREADSHEET_SHEET_FIND: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/find";
    pub const SHEETS_V3_SPREADSHEET_SHEET_REPLACE: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/replace";

    /// 电子表格合并拆分单元格 - v3
    pub const SHEETS_V3_SPREADSHEET_MERGE_CELLS: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/merge_cells";
    pub const SHEETS_V3_SPREADSHEET_UNMERGE_CELLS: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/unmerge_cells";

    /// 电子表格样式管理 - v3
    pub const SHEETS_V3_SPREADSHEET_STYLE: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/style";
    pub const SHEETS_V3_SPREADSHEET_STYLES_BATCH_UPDATE: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/styles/batch_update";

    /// 电子表格行列管理 - v3
    pub const SHEETS_V3_SPREADSHEET_DIMENSION_RANGE: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/dimension_range";
    pub const SHEETS_V3_SPREADSHEET_DIMENSION_RANGE_INSERT: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/dimension_range:insert";
    pub const SHEETS_V3_SPREADSHEET_MOVE_DIMENSION: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/move_dimension";

    /// 电子表格条件格式 - v3
    pub const SHEETS_V3_SPREADSHEET_CONDITION_FORMAT: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/conditionFormat";

    /// 电子表格数据验证 - v3
    pub const SHEETS_V3_SPREADSHEET_DATA_VALIDATION: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/dataValidation";
    pub const SHEETS_V3_SPREADSHEET_DATA_VALIDATION_GET: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/dataValidation/{}";

    /// 电子表格保护范围 - v3
    pub const SHEETS_V3_SPREADSHEET_PROTECT_RANGE: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{}/protect_range";
    pub const SHEETS_V3_SPREADSHEET_PROTECT_RANGE_GET: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{}/protect_range/{}";

    /// 电子表格筛选器 - v3
    pub const SHEETS_V3_SPREADSHEET_FILTER: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter";

    /// 电子表格筛选视图 - v3
    pub const SHEETS_V3_SPREADSHEET_FILTER_VIEWS: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter_views";
    pub const SHEETS_V3_SPREADSHEET_FILTER_VIEW_GET: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter_views/{}";

    /// 电子表格筛选视图条件 - v3
    pub const SHEETS_V3_SPREADSHEET_FILTER_VIEW_CONDITIONS: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter_views/{}/conditions";
    pub const SHEETS_V3_SPREADSHEET_FILTER_VIEW_CONDITION_GET: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter_views/{}/conditions/{}";

    /// 电子表格浮动图片 - v3
    pub const SHEETS_V3_SPREADSHEET_FLOAT_IMAGES: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/float_images";
    pub const SHEETS_V3_SPREADSHEET_FLOAT_IMAGE_GET: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/float_images/{}";

    /// 电子表格管理 - v2
    pub const SHEETS_V2_SPREADSHEET_VALUES: &'static str =
        "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values";
    pub const SHEETS_V2_SPREADSHEET_VALUES_RANGE: &'static str =
        "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values/{range}";
    pub const SHEETS_V2_SPREADSHEET_VALUES_APPEND: &'static str =
        "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_append";
    pub const SHEETS_V2_SPREADSHEET_VALUES_PREPEND: &'static str =
        "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_prepend";
    pub const SHEETS_V2_SPREADSHEET_VALUES_BATCH_GET: &'static str =
        "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_batch_get";
    pub const SHEETS_V2_SPREADSHEET_VALUES_BATCH_UPDATE: &'static str =
        "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_batch_update";
    pub const SHEETS_V2_SPREADSHEET_VALUES_IMAGE: &'static str =
        "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_image";

    /// 电子表格样式管理 - v2
    pub const SHEETS_V2_SPREADSHEET_STYLE: &'static str =
        "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/style";
    pub const SHEETS_V2_SPREADSHEET_STYLES_BATCH_UPDATE: &'static str =
        "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/styles_batch_update";

    /// 电子表格合并拆分单元格 - v2
    pub const SHEETS_V2_SPREADSHEET_MERGE_CELLS: &'static str =
        "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/merge_cells";
    pub const SHEETS_V2_SPREADSHEET_UNMERGE_CELLS: &'static str =
        "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/unmerge_cells";

    /// 电子表格行列管理 - v2
    pub const SHEETS_V2_SPREADSHEET_DIMENSION_RANGE: &'static str =
        "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/dimension_range";
    pub const SHEETS_V2_SPREADSHEET_INSERT_DIMENSION_RANGE: &'static str =
        "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/insert_dimension_range";

    /// 电子表格工作表管理 - v2
    pub const SHEETS_V2_SPREADSHEET_SHEETS_BATCH_UPDATE: &'static str =
        "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/sheets_batch_update";

    // Wiki 知识库服务
    /// 知识库空间管理
    pub const WIKI_V2_SPACES: &'static str = "/open-apis/wiki/v2/spaces";
    pub const WIKI_V2_SPACE_GET: &'static str = "/open-apis/wiki/v2/spaces/{}";
    pub const WIKI_V2_SPACE_CREATE: &'static str = "/open-apis/wiki/v2/spaces";

    /// 知识库空间成员管理
    pub const WIKI_V2_SPACE_MEMBERS: &'static str = "/open-apis/wiki/v2/spaces/{}/members";
    pub const WIKI_V2_SPACE_MEMBER_CREATE: &'static str = "/open-apis/wiki/v2/spaces/{}/members";
    pub const WIKI_V2_SPACE_MEMBER_DELETE: &'static str = "/open-apis/wiki/v2/spaces/{}/members/{}";

    /// 知识库节点管理
    pub const WIKI_V2_SPACE_NODES: &'static str = "/open-apis/wiki/v2/spaces/{}/nodes";
    pub const WIKI_V2_SPACE_NODE_GET: &'static str = "/open-apis/wiki/v2/spaces/{}/nodes/{}";
    pub const WIKI_V2_SPACE_NODE_CREATE: &'static str = "/open-apis/wiki/v2/spaces/{}/nodes";
    pub const WIKI_V2_SPACE_NODE_COPY: &'static str = "/open-apis/wiki/v2/spaces/{}/nodes/{}/copy";
    pub const WIKI_V2_SPACE_NODE_MOVE: &'static str = "/open-apis/wiki/v2/spaces/{}/nodes/{}/move";
    pub const WIKI_V2_SPACE_NODE_UPDATE_TITLE: &'static str =
        "/open-apis/wiki/v2/spaces/{}/nodes/{}/update_title";

    /// 知识库设置管理
    pub const WIKI_V2_SPACE_SETTING_UPDATE: &'static str = "/open-apis/wiki/v2/spaces/{}/setting";

    /// 知识库搜索
    pub const WIKI_V2_SEARCH: &'static str = "/open-apis/wiki/v2/search";

    /// 知识库任务管理
    pub const WIKI_V2_TASKS_MOVE_DOCS_TO_WIKI: &'static str =
        "/open-apis/wiki/v2/tasks/move_docs_to_wiki";
    pub const WIKI_V2_TASK_GET: &'static str = "/open-apis/wiki/v2/tasks/{}";

    // ==================== 邮件服务端点 ====================

    // 邮件组管理
    /// 邮件组基础操作
    pub const MAIL_V1_MAILGROUPS: &'static str = "/open-apis/mail/v1/mailgroups";

    /// 邮件组详情操作 (需要使用 EndpointBuilder::replace_param 替换 {mailgroup_id})
    pub const MAIL_V1_MAILGROUP: &'static str = "/open-apis/mail/v1/mailgroups/{mailgroup_id}";

    // 邮件组管理员
    /// 批量创建邮件组管理员 (需要使用 EndpointBuilder::replace_param 替换 {mailgroup_id})
    pub const MAIL_V1_MAILGROUP_MANAGERS_BATCH_CREATE: &'static str =
        "/open-apis/mail/v1/mailgroups/{mailgroup_id}/managers/batch_create";

    /// 批量删除邮件组管理员 (需要使用 EndpointBuilder::replace_param 替换 {mailgroup_id})
    pub const MAIL_V1_MAILGROUP_MANAGERS_BATCH_DELETE: &'static str =
        "/open-apis/mail/v1/mailgroups/{mailgroup_id}/managers/batch_delete";

    /// 获取邮件组管理员列表 (需要使用 EndpointBuilder::replace_param 替换 {mailgroup_id})
    pub const MAIL_V1_MAILGROUP_MANAGERS: &'static str =
        "/open-apis/mail/v1/mailgroups/{mailgroup_id}/managers";

    // 用户邮箱事件
    /// 订阅用户邮箱事件 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id})
    pub const MAIL_V1_USER_MAILBOX_EVENTS_SUBSCRIBE: &'static str =
        "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/events/subscribe";

    /// 获取用户邮箱事件订阅状态 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id})
    pub const MAIL_V1_USER_MAILBOX_EVENTS_SUBSCRIPTION: &'static str =
        "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/events/subscription";

    /// 取消订阅用户邮箱事件 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id})
    pub const MAIL_V1_USER_MAILBOX_EVENTS_UNSUBSCRIBE: &'static str =
        "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/events/unsubscribe";

    // 用户邮箱文件夹
    /// 用户邮箱文件夹操作 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id})
    pub const MAIL_V1_USER_MAILBOX_FOLDERS: &'static str =
        "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/folders";

    /// 用户邮箱文件夹详情操作 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id} 和 {folder_id})
    pub const MAIL_V1_USER_MAILBOX_FOLDER: &'static str =
        "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/folders/{folder_id}";

    // 用户邮箱消息
    /// 用户邮箱消息操作 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id})
    pub const MAIL_V1_USER_MAILBOX_MESSAGES: &'static str =
        "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/messages";

    /// 用户邮箱消息详情 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id} 和 {message_id})
    pub const MAIL_V1_USER_MAILBOX_MESSAGE: &'static str =
        "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/messages/{message_id}";

    /// 通过卡片获取邮件消息 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id})
    pub const MAIL_V1_USER_MAILBOX_MESSAGES_GET_BY_CARD: &'static str =
        "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/messages/get_by_card";

    // 用户邮箱规则
    /// 用户邮箱规则操作 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id})
    pub const MAIL_V1_USER_MAILBOX_RULES: &'static str =
        "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/rules";

    /// 用户邮箱规则详情 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id} 和 {rule_id})
    pub const MAIL_V1_USER_MAILBOX_RULE: &'static str =
        "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/rules/{rule_id}";

    /// 重新排序用户邮箱规则 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id})
    pub const MAIL_V1_USER_MAILBOX_RULES_REORDER: &'static str =
        "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/rules/reorder";

    // 用户邮箱联系人
    /// 用户邮箱联系人操作 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id})
    pub const MAIL_V1_USER_MAILBOX_MAIL_CONTACTS: &'static str =
        "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/mail_contacts";

    /// 用户邮箱联系人详情 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id} 和 {contact_id})
    pub const MAIL_V1_USER_MAILBOX_MAIL_CONTACT: &'static str =
        "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/mail_contacts/{contact_id}";

    // 用户邮箱附件
    /// 获取邮件附件下载链接 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id}, {message_id} 和 {attachment_id})
    pub const MAIL_V1_USER_MAILBOX_MESSAGE_ATTACHMENT_DOWNLOAD_URL: &'static str =
        "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/messages/{message_id}/attachments/{attachment_id}/download_url";

    // ==================== 目录服务相关端点 ====================

    // ===== 部门管理端点 =====
    /// 创建部门
    pub const DIRECTORY_V1_DEPARTMENTS: &'static str = "/open-apis/directory/v1/departments";

    /// 删除/更新部门 (需要使用 EndpointBuilder::replace_param 替换 {department_id})
    pub const DIRECTORY_V1_DEPARTMENT_GET: &'static str =
        "/open-apis/directory/v1/departments/{department_id}";

    /// 搜索部门
    pub const DIRECTORY_V1_DEPARTMENTS_SEARCH: &'static str =
        "/open-apis/directory/v1/departments/search";

    /// 筛选部门
    pub const DIRECTORY_V1_DEPARTMENTS_FILTER: &'static str =
        "/open-apis/directory/v1/departments/filter";

    /// 批量获取部门
    pub const DIRECTORY_V1_DEPARTMENTS_MGET: &'static str =
        "/open-apis/directory/v1/departments/mget";

    // ===== 员工管理端点 =====
    /// 创建员工
    pub const DIRECTORY_V1_EMPLOYEES: &'static str = "/open-apis/directory/v1/employees";

    /// 删除/更新员工 (需要使用 EndpointBuilder::replace_param 替换 {employee_id})
    pub const DIRECTORY_V1_EMPLOYEE_GET: &'static str =
        "/open-apis/directory/v1/employees/{employee_id}";

    /// 员工待离职 (需要使用 EndpointBuilder::replace_param 替换 {employee_id})
    pub const DIRECTORY_V1_EMPLOYEE_TO_BE_RESIGNED: &'static str =
        "/open-apis/directory/v1/employees/{employee_id}/to_be_resigned";

    /// 员工转正 (需要使用 EndpointBuilder::replace_param 替换 {employee_id})
    pub const DIRECTORY_V1_EMPLOYEE_REGULAR: &'static str =
        "/open-apis/directory/v1/employees/{employee_id}/regular";

    /// 员工复活 (需要使用 EndpointBuilder::replace_param 替换 {employee_id})
    pub const DIRECTORY_V1_EMPLOYEE_RESURRECT: &'static str =
        "/open-apis/directory/v1/employees/{employee_id}/resurrect";

    /// 搜索员工
    pub const DIRECTORY_V1_EMPLOYEES_SEARCH: &'static str =
        "/open-apis/directory/v1/employees/search";

    /// 筛选员工
    pub const DIRECTORY_V1_EMPLOYEES_FILTER: &'static str =
        "/open-apis/directory/v1/employees/filter";

    /// 批量获取员工
    pub const DIRECTORY_V1_EMPLOYEES_MGET: &'static str = "/open-apis/directory/v1/employees/mget";

    // ==================== 搜索服务端点 ====================
    /// 搜索用户 V1
    pub const SEARCH_V1_USER: &'static str = "/open-apis/search/v1/user";

    /// 创建数据项
    pub const SEARCH_V2_DATA_ITEM_CREATE: &'static str =
        "/open-apis/search/v2/data_sources/{data_source_id}/items";

    /// 批量创建数据项
    pub const SEARCH_V2_DATA_ITEM_BATCH_CREATE: &'static str =
        "/open-apis/search/v2/data_sources/{data_source_id}/items/batch_create";

    /// 删除/获取数据项
    pub const SEARCH_V2_DATA_ITEM_OPERATION: &'static str =
        "/open-apis/search/v2/data_sources/{data_source_id}/items/{data_item_id}";

    /// 搜索消息
    pub const SEARCH_V2_MESSAGE: &'static str = "/open-apis/search/v2/message";

    /// 搜索应用
    pub const SEARCH_V2_APP: &'static str = "/open-apis/search/v2/app";

    /// 数据源操作（创建/列表）
    pub const SEARCH_V2_DATA_SOURCES: &'static str = "/open-apis/search/v2/data_sources";

    /// 数据源操作（删除/更新/获取）
    pub const SEARCH_V2_DATA_SOURCE_OPERATION: &'static str =
        "/open-apis/search/v2/data_sources/{data_source_id}";

    /// 创建Schema
    pub const SEARCH_V2_SCHEMA_CREATE: &'static str =
        "/open-apis/search/v2/data_sources/{data_source_id}/schemas";

    /// Schema操作（删除/更新/获取）
    pub const SEARCH_V2_SCHEMA_OPERATION: &'static str =
        "/open-apis/search/v2/data_sources/{data_source_id}/schemas/{schema_id}";

    // ==================== OKR服务端点 ====================
    /// 查询评分列表
    pub const OKR_V1_REVIEWS_QUERY: &'static str = "/open-apis/okr/v1/reviews/query";

    /// OKR列表
    pub const OKR_V1_OKRS: &'static str = "/open-apis/okr/v1/okrs";

    /// 批量获取OKR
    pub const OKR_V1_OKRS_BATCH_GET: &'static str = "/open-apis/okr/v1/okrs/batch_get";

    /// 周期规则列表
    pub const OKR_V1_PERIOD_RULES: &'static str = "/open-apis/okr/v1/period_rules";

    /// 周期列表
    pub const OKR_V1_PERIODS: &'static str = "/open-apis/okr/v1/periods";

    /// 周期详情
    pub const OKR_V1_PERIOD_GET: &'static str = "/open-apis/okr/v1/periods/{period_id}";

    /// 进展记录列表
    pub const OKR_V1_PROGRESS_RECORDS: &'static str = "/open-apis/okr/v1/progress_records";

    /// 进展记录操作（获取/更新/删除）
    pub const OKR_V1_PROGRESS_RECORD_OPERATION: &'static str =
        "/open-apis/okr/v1/progress_records/{progress_id}";

    /// 进展记录上传
    pub const OKR_V1_PROGRESS_RECORDS_UPLOAD: &'static str =
        "/open-apis/okr/v1/progress_records/upload";

    // ==================== Trust Party 可信第三方服务端点 ====================

    // === 关联组织管理端点 ===
    /// 获取可见关联组织列表
    pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATIONS: &'static str =
        "/open-apis/trust_party/v1/collaboration_organizations";

    /// 获取关联组织的部门和成员信息 (需要使用 EndpointBuilder::replace_param 替换 {org_id})
    pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_VISIBLE: &'static str =
        "/open-apis/trust_party/v1/collaboration_organizations/{org_id}/visible_organization";

    /// 获取关联组织详情 (需要使用 EndpointBuilder::replace_param 替换 {org_id})
    pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_GET: &'static str =
        "/open-apis/trust_party/v1/collaboration_organizations/{org_id}";

    /// 获取关联组织成员详情 (需要使用 EndpointBuilder::replace_params 替换 {org_id} 和 {user_id})
    pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_USER_GET: &'static str =
        "/open-apis/trust_party/v1/collaboration_organizations/{org_id}/users/{user_id}";

    /// 获取关联组织部门详情 (需要使用 EndpointBuilder::replace_params 替换 {org_id} 和 {department_id})
    pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_DEPARTMENT_GET: &'static str =
        "/open-apis/trust_party/v1/collaboration_organizations/{org_id}/departments/{department_id}";

    /// 获取关联组织双方共享成员范围 (需要使用 EndpointBuilder::replace_param 替换 {org_id})
    pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_SHARED_MEMBER_SCOPES: &'static str =
        "/open-apis/trust_party/v1/collaboration_organizations/{org_id}/shared_member_scopes";

    /// 管理员获取所有关联组织列表
    pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATIONS_ADMIN: &'static str =
        "/open-apis/trust_party/v1/collaboration_organizations/admin";

    // === 可搜可见规则管理端点 ===
    /// 可搜可见规则操作（创建/查询）
    pub const TRUST_PARTY_V1_SEARCHABLE_VISIBLE_RULES: &'static str =
        "/open-apis/trust_party/v1/searchable_and_visible_rules";

    /// 可搜可见规则操作（更新/删除） (需要使用 EndpointBuilder::replace_param 替换 {rule_id})
    pub const TRUST_PARTY_V1_SEARCHABLE_VISIBLE_RULE_OPERATION: &'static str =
        "/open-apis/trust_party/v1/searchable_and_visible_rules/{rule_id}";

    // ==================== Cardkit服务端点 ====================
    /// 创建卡片
    pub const CARDKIT_V1_CARDS: &'static str = "/open-apis/cardkit/v1/cards";

    /// 创建卡片元素
    pub const CARDKIT_V1_CARD_ELEMENTS: &'static str =
        "/open-apis/cardkit/v1/cards/{card_id}/elements";

    /// 卡片设置
    pub const CARDKIT_V1_CARD_SETTINGS: &'static str =
        "/open-apis/cardkit/v1/cards/{card_id}/settings";

    /// 更新卡片
    pub const CARDKIT_V1_CARD_UPDATE: &'static str = "/open-apis/cardkit/v1/cards/{card_id}";

    /// 批量更新卡片
    pub const CARDKIT_V1_CARD_BATCH_UPDATE: &'static str =
        "/open-apis/cardkit/v1/cards/{card_id}/batch_update";

    // ==================== Aily服务端点 ====================
    /// 会话列表/创建
    pub const AILY_V1_SESSIONS: &'static str = "/open-apis/aily/v1/sessions";

    /// 会话操作（获取/更新/删除）
    pub const AILY_V1_SESSION_OPERATION: &'static str = "/open-apis/aily/v1/sessions/{session_id}";

    /// 运行列表/创建
    pub const AILY_V1_RUNS: &'static str = "/open-apis/aily/v1/sessions/{session_id}/runs";

    /// 运行操作（获取）
    pub const AILY_V1_RUN_GET: &'static str =
        "/open-apis/aily/v1/sessions/{session_id}/runs/{run_id}";

    /// 取消运行
    pub const AILY_V1_RUN_CANCEL: &'static str =
        "/open-apis/aily/v1/sessions/{session_id}/runs/{run_id}/cancel";

    /// 知识库问答
    pub const AILY_V1_DATA_KNOWLEDGE_ASK: &'static str = "/open-apis/aily/v1/data_knowledge/ask";

    /// 知识库文件上传
    pub const AILY_V1_DATA_KNOWLEDGE_UPLOAD_FILE: &'static str =
        "/open-apis/aily/v1/data_knowledge/upload_file";

    /// 知识库操作（创建/列表）
    pub const AILY_V1_DATA_KNOWLEDGE: &'static str = "/open-apis/aily/v1/data_knowledge";

    /// 知识库操作（获取/删除）
    pub const AILY_V1_DATA_KNOWLEDGE_OPERATION: &'static str =
        "/open-apis/aily/v1/data_knowledge/{knowledge_id}";

    /// 知识库分类
    pub const AILY_V1_DATA_KNOWLEDGE_CATEGORIES: &'static str =
        "/open-apis/aily/v1/data_knowledge/categories";

    /// 消息操作（创建/列表）
    pub const AILY_V1_MESSAGES: &'static str = "/open-apis/aily/v1/sessions/{session_id}/messages";

    /// 消息获取
    pub const AILY_V1_MESSAGE_GET: &'static str =
        "/open-apis/aily/v1/sessions/{session_id}/messages/{message_id}";

    /// 技能启动
    pub const AILY_V1_SKILL_START: &'static str = "/open-apis/aily/v1/skills/{skill_id}/start";

    /// 技能获取
    pub const AILY_V1_SKILL_GET: &'static str = "/open-apis/aily/v1/skills/{skill_id}";

    /// 技能列表
    pub const AILY_V1_SKILLS: &'static str = "/open-apis/aily/v1/skills";

    // ==================== ACS服务端点 ====================
    /// 设备列表
    pub const ACS_V1_DEVICES: &'static str = "/open-apis/acs/v1/devices";

    /// 访客操作（创建/列表）
    pub const ACS_V1_VISITORS: &'static str = "/open-apis/acs/v1/visitors";

    /// 访客获取
    pub const ACS_V1_VISITOR_GET: &'static str = "/open-apis/acs/v1/visitors/{visitor_id}";

    /// 门禁记录列表
    pub const ACS_V1_ACCESS_RECORDS: &'static str = "/open-apis/acs/v1/access_records";

    /// 门禁记录人脸图像
    pub const ACS_V1_ACCESS_RECORD_FACE_IMAGE: &'static str =
        "/open-apis/acs/v1/access_records/{record_id}/face_image";

    /// 外部规则操作（创建/列表）
    pub const ACS_V1_RULE_EXTERNAL: &'static str = "/open-apis/acs/v1/rule_external";

    /// 外部规则操作（获取/删除）
    pub const ACS_V1_RULE_EXTERNAL_OPERATION: &'static str =
        "/open-apis/acs/v1/rule_external/{rule_id}";

    /// 外部规则设备绑定
    pub const ACS_V1_RULE_EXTERNAL_DEVICE_BIND: &'static str =
        "/open-apis/acs/v1/rule_external/device_bind";

    /// 用户操作（获取/删除）
    pub const ACS_V1_USER_OPERATION: &'static str = "/open-apis/acs/v1/users/{user_id}";

    /// 用户列表
    pub const ACS_V1_USERS: &'static str = "/open-apis/acs/v1/users";

    /// 用户人脸图像（获取/上传）
    pub const ACS_V1_USER_FACE_IMAGE: &'static str = "/open-apis/acs/v1/users/{user_id}/face_image";

    // ==================== 管理后台服务端点 ====================

    /// 勋章管理 - 创建勋章
    pub const ADMIN_V1_BADGES_CREATE: &'static str = "/open-apis/admin/v1/badges";

    /// 勋章管理 - 获取勋章列表
    pub const ADMIN_V1_BADGES_LIST: &'static str = "/open-apis/admin/v1/badges";

    /// 勋章管理 - 操作勋章（获取/更新）
    pub const ADMIN_V1_BADGES_OPERATION: &'static str = "/open-apis/admin/v1/badges/{badge_id}";

    /// 勋章管理 - 上传勋章图片
    pub const ADMIN_V1_BADGES_IMAGE_UPLOAD: &'static str = "/open-apis/admin/v1/badges/image";

    /// 勋章授予名单 - 创建授予名单
    pub const ADMIN_V1_BADGE_GRANTS_CREATE: &'static str = "/open-apis/admin/v1/badge_grants";

    /// 勋章授予名单 - 获取授予名单列表
    pub const ADMIN_V1_BADGE_GRANTS_LIST: &'static str = "/open-apis/admin/v1/badge_grants";

    /// 勋章授予名单 - 操作授予名单（获取/更新/删除）
    pub const ADMIN_V1_BADGE_GRANTS_OPERATION: &'static str =
        "/open-apis/admin/v1/badge_grants/{grant_id}";

    /// 密码管理 - 重置密码
    pub const ADMIN_V1_PASSWORD_RESET: &'static str = "/open-apis/admin/v1/password/reset";

    /// 数据报表 - 部门维度数据
    pub const ADMIN_V1_DATA_REPORT_DEPARTMENT: &'static str =
        "/open-apis/admin/v1/data_report/department";

    /// 数据报表 - 用户维度数据
    pub const ADMIN_V1_DATA_REPORT_USER: &'static str = "/open-apis/admin/v1/data_report/user";

    // ==================== 妙记服务端点 ====================

    /// 获取妙记信息 (需要使用 EndpointBuilder::replace_param 替换 {minute_token})
    pub const MINUTES_V1_MINUTE_GET: &'static str = "/open-apis/minutes/v1/{minute_token}";

    /// 获取妙记统计数据 (需要使用 EndpointBuilder::replace_param 替换 {minute_token})
    pub const MINUTES_V1_STATISTICS_GET: &'static str =
        "/open-apis/minutes/v1/{minute_token}/statistics";

    /// 导出妙记文字记录 (需要使用 EndpointBuilder::replace_param 替换 {minute_token})
    pub const MINUTES_V1_TRANSCRIPT_GET: &'static str =
        "/open-apis/minutes/v1/{minute_token}/transcript";

    /// 下载妙记音视频文件 (需要使用 EndpointBuilder::replace_param 替换 {minute_token})
    pub const MINUTES_V1_MEDIA_GET: &'static str = "/open-apis/minutes/v1/{minute_token}/media";

    // ==================== 实名认证服务端点 ====================
    /// 录入身份信息
    pub const HUMAN_AUTHENTICATION_V1_IDENTITIES: &'static str =
        "/open-apis/human_authentication/v1/identities";
    /// 上传人脸基准图片
    pub const HUMAN_AUTHENTICATION_V1_FACE_IMAGES: &'static str =
        "/open-apis/human_authentication/v1/face_images";
    /// 裁剪人脸图片
    pub const HUMAN_AUTHENTICATION_V1_FACE_IMAGES_CROP: &'static str =
        "/open-apis/human_authentication/v1/face_images/crop";
    /// 查询人脸认证结果 (需要使用 EndpointBuilder::replace_param 替换 {identity_id})
    pub const HUMAN_AUTHENTICATION_V1_IDENTITY_RESULT: &'static str =
        "/open-apis/human_authentication/v1/identities/{identity_id}/result";

    // ==================== MDM设备管理服务端点 ====================
    /// 批量查询国家/地区
    pub const MDM_V1_COUNTRY_REGIONS_BATCH_GET: &'static str =
        "/open-apis/mdm/v1/country_regions/batch_get";
    /// 分页查询国家/地区
    pub const MDM_V1_COUNTRY_REGIONS: &'static str = "/open-apis/mdm/v1/country_regions";
    /// 用户数据维度绑定
    pub const MDM_V1_USER_AUTH_DATA_RELATIONS_BIND: &'static str =
        "/open-apis/mdm/v1/user_auth_data_relations/bind";
    /// 用户数据维度解绑
    pub const MDM_V1_USER_AUTH_DATA_RELATIONS_UNBIND: &'static str =
        "/open-apis/mdm/v1/user_auth_data_relations/unbind";

    // ==================== Security and Compliance 安全与合规相关端点 ====================
    /// 行为审计日志数据获取
    pub const SECURITY_AND_COMPLIANCE_V1_AUDIT_DATAS: &'static str =
        "/open-apis/security_and_compliance/v1/audit_datas";

    /// OpenAPI 审计日志数据列表
    pub const SECURITY_AND_COMPLIANCE_V1_OPENAPI_LOGS_LIST_DATA: &'static str =
        "/open-apis/security_and_compliance/v1/openapi_logs/list_data";

    // ==================== Report 汇报相关端点 ====================
    /// 任务查询
    pub const REPORT_V1_TASKS_QUERY: &'static str = "/open-apis/report/v1/tasks/query";

    /// 规则查询
    pub const REPORT_V1_RULES_QUERY: &'static str = "/open-apis/report/v1/rules/query";

    /// 规则看板操作
    pub const REPORT_V1_RULE_VIEWS_OPERATION: &'static str =
        "/open-apis/report/v1/rule-views/{view_id}";

    // ==================== Authentication 用户认证相关端点 ====================
    /// 获取用户信息
    pub const AUTHEN_V1_USER_INFO: &'static str = "/open-apis/authen/v1/user_info";

    // ==================== Calendar 日历相关端点 (补充) ====================
    /// 日历管理
    pub const CALENDAR_V4_CALENDARS: &'static str = "/open-apis/calendar/v4/calendars";

    /// 日历详情操作
    pub const CALENDAR_V4_CALENDAR_OPERATION: &'static str =
        "/open-apis/calendar/v4/calendars/{calendar_id}";
}

/// API端点构建辅助函数
///
/// 提供用于动态构建包含路径参数的API端点的辅助函数。
/// 这些函数可以安全地替换路径模板中的占位符。
pub struct EndpointBuilder;

impl EndpointBuilder {
    /// 替换单个路径参数
    ///
    /// # 示例
    /// ```rust
    /// use open_lark::core::endpoints::EndpointBuilder;
    /// let path = EndpointBuilder::replace_param(
    ///     "/open-apis/vc/v1/rooms/{room_id}",
    ///     "room_id",
    ///     "room_123"
    /// );
    /// assert_eq!(path, "/open-apis/vc/v1/rooms/room_123");
    /// ```
    pub fn replace_param(template: &str, param_name: &str, value: &str) -> String {
        template.replace(&format!("{{{}}}", param_name), value)
    }

    /// 替换多个路径参数
    ///
    /// # 示例
    /// ```rust
    /// use std::collections::HashMap;
    /// use open_lark::core::endpoints::EndpointBuilder;
    /// let mut params = HashMap::new();
    /// params.insert("calendar_id".to_string(), "cal_123".to_string());
    /// params.insert("event_id".to_string(), "event_456".to_string());
    /// let path = EndpointBuilder::replace_params(
    ///     "/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}",
    ///     &params
    /// );
    /// assert_eq!(path, "/open-apis/calendar/v4/calendars/cal_123/events/event_456");
    /// ```
    pub fn replace_params(
        template: &str,
        params: &std::collections::HashMap<String, String>,
    ) -> String {
        let mut result = template.to_string();
        for (key, value) in params {
            result = result.replace(&format!("{{{}}}", key), value);
        }
        result
    }

    /// 从字符串数组替换多个路径参数
    ///
    /// # 示例
    /// ```rust
    /// use open_lark::core::endpoints::EndpointBuilder;
    /// let path = EndpointBuilder::replace_params_from_array(
    ///     "/open-apis/vc/v1/rooms/{room_id}/members/{user_id}",
    ///     &[("room_id", "room_123"), ("user_id", "user_456")]
    /// );
    /// assert_eq!(path, "/open-apis/vc/v1/rooms/room_123/members/user_456");
    /// ```
    pub fn replace_params_from_array(template: &str, params: &[(&str, &str)]) -> String {
        let mut result = template.to_string();
        for (key, value) in params {
            result = result.replace(&format!("{{{}}}", key), value);
        }
        result
    }

    /// 创建日历端点构建器
    pub fn calendar(calendar_id: &str) -> CalendarEndpointBuilder {
        CalendarEndpointBuilder {
            calendar_id: calendar_id.to_string(),
        }
    }

    /// 创建日历事件端点构建器
    pub fn calendar_event(calendar_id: &str, event_id: &str) -> CalendarEventEndpointBuilder {
        CalendarEventEndpointBuilder {
            calendar_id: calendar_id.to_string(),
            event_id: event_id.to_string(),
        }
    }

    /// 创建即时消息端点构建器
    pub fn im_message(message_id: &str) -> ImMessageEndpointBuilder {
        ImMessageEndpointBuilder {
            message_id: message_id.to_string(),
        }
    }

    // VC相关Builder方法已迁移至对应的vc服务模块

    /// 创建工作台端点构建器
    pub fn workplace() -> WorkplaceEndpointBuilder {
        WorkplaceEndpointBuilder {}
    }
}

/// 日历端点构建器
pub struct CalendarEndpointBuilder {
    calendar_id: String,
}

impl CalendarEndpointBuilder {
    /// 获取日历详情端点
    pub fn get(&self) -> String {
        EndpointBuilder::replace_param(Endpoints::CALENDAR_GET, "calendar_id", &self.calendar_id)
    }

    /// 更新日历端点
    pub fn patch(&self) -> String {
        EndpointBuilder::replace_param(Endpoints::CALENDAR_UPDATE, "calendar_id", &self.calendar_id)
    }

    /// 删除日历端点
    pub fn delete(&self) -> String {
        EndpointBuilder::replace_param(Endpoints::CALENDAR_DELETE, "calendar_id", &self.calendar_id)
    }

    /// 设置主日历端点
    pub fn set_primary(&self) -> String {
        EndpointBuilder::replace_param(
            Endpoints::CALENDAR_PRIMARY,
            "calendar_id",
            &self.calendar_id,
        )
    }

    /// 搜索日历端点
    pub fn search(&self) -> String {
        EndpointBuilder::replace_param(Endpoints::CALENDAR_SEARCH, "calendar_id", &self.calendar_id)
    }
}

/// 日历事件端点构建器
pub struct CalendarEventEndpointBuilder {
    calendar_id: String,
    event_id: String,
}

impl CalendarEventEndpointBuilder {
    /// 创建事件端点
    pub fn create(&self) -> String {
        EndpointBuilder::replace_param(
            Endpoints::CALENDAR_EVENT_CREATE,
            "calendar_id",
            &self.calendar_id,
        )
    }

    /// 获取事件详情端点
    pub fn get(&self) -> String {
        let temp = EndpointBuilder::replace_param(
            Endpoints::CALENDAR_EVENT_GET,
            "calendar_id",
            &self.calendar_id,
        );
        EndpointBuilder::replace_param(&temp, "event_id", &self.event_id)
    }

    /// 更新事件端点
    pub fn patch(&self) -> String {
        let temp = EndpointBuilder::replace_param(
            Endpoints::CALENDAR_EVENT_UPDATE,
            "calendar_id",
            &self.calendar_id,
        );
        EndpointBuilder::replace_param(&temp, "event_id", &self.event_id)
    }

    /// 删除事件端点
    pub fn delete(&self) -> String {
        let temp = EndpointBuilder::replace_param(
            Endpoints::CALENDAR_EVENT_DELETE,
            "calendar_id",
            &self.calendar_id,
        );
        EndpointBuilder::replace_param(&temp, "event_id", &self.event_id)
    }

    /// 回复事件邀请端点
    pub fn reply(&self) -> String {
        let temp = EndpointBuilder::replace_param(
            Endpoints::CALENDAR_EVENT_REPLY,
            "calendar_id",
            &self.calendar_id,
        );
        EndpointBuilder::replace_param(&temp, "event_id", &self.event_id)
    }

    /// 搜索事件端点
    pub fn search(&self) -> String {
        let temp = EndpointBuilder::replace_param(
            Endpoints::CALENDAR_EVENT_SEARCH,
            "calendar_id",
            &self.calendar_id,
        );
        EndpointBuilder::replace_param(&temp, "event_id", &self.event_id)
    }

    /// 获取事件列表端点
    pub fn list(&self) -> String {
        EndpointBuilder::replace_param(
            Endpoints::CALENDAR_EVENT_LIST,
            "calendar_id",
            &self.calendar_id,
        )
    }
}

/// 即时消息端点构建器
pub struct ImMessageEndpointBuilder {
    message_id: String,
}

impl ImMessageEndpointBuilder {
    /// 获取消息端点
    pub fn get(&self) -> String {
        EndpointBuilder::replace_param(Endpoints::IM_V1_MESSAGE_GET, "message_id", &self.message_id)
    }

    /// 更新消息端点
    pub fn patch(&self) -> String {
        EndpointBuilder::replace_param(
            Endpoints::IM_V1_MESSAGE_PATCH,
            "message_id",
            &self.message_id,
        )
    }

    /// 删除消息端点
    pub fn delete(&self) -> String {
        EndpointBuilder::replace_param(
            Endpoints::IM_V1_MESSAGE_DELETE,
            "message_id",
            &self.message_id,
        )
    }
}

// VC相关Builder已迁移至对应的vc服务模块

/// 工作台端点构建器
pub struct WorkplaceEndpointBuilder {}

impl WorkplaceEndpointBuilder {
    /// 获取工作台访问数据端点
    pub fn access_data(&self) -> String {
        Endpoints::WORKPLACE_ACCESS_DATA_SEARCH.to_string()
    }

    /// 获取应用推荐端点
    pub fn app_recommend(&self) -> String {
        Endpoints::WORKPLACE_APP_RECOMMEND_LIST.to_string()
    }
}

// ==================== Public Re-exports 公共常量导出 ====================
// Public re-exports for backward compatibility with direct imports

// LINGO constants
pub const LINGO_CLASSIFICATION_LIST: &str = Endpoints::LINGO_CLASSIFICATION_LIST;
pub const LINGO_DRAFT_CREATE: &str = Endpoints::LINGO_DRAFT_CREATE;
pub const LINGO_DRAFT_UPDATE: &str = Endpoints::LINGO_DRAFT_UPDATE;
pub const LINGO_ENTITY_CREATE: &str = Endpoints::LINGO_ENTITY_CREATE;
pub const LINGO_ENTITY_GET: &str = Endpoints::LINGO_ENTITY_GET;
pub const LINGO_ENTITY_UPDATE: &str = Endpoints::LINGO_ENTITY_UPDATE;
pub const LINGO_ENTITY_SEARCH: &str = Endpoints::LINGO_ENTITY_SEARCH;
pub const LINGO_ENTITY_MATCH: &str = Endpoints::LINGO_ENTITY_MATCH;
pub const LINGO_ENTITY_HIGHLIGHT: &str = Endpoints::LINGO_ENTITY_HIGHLIGHT;
pub const LINGO_FILE_UPLOAD: &str = Endpoints::LINGO_FILE_UPLOAD;
pub const LINGO_FILE_DOWNLOAD: &str = Endpoints::LINGO_FILE_DOWNLOAD;
pub const LINGO_REPO_LIST: &str = Endpoints::LINGO_REPO_LIST;

// VC constants
// VC constants - 已迁移至 core::endpoints::vc 模块

// TENANT constants - 已迁移至 core::endpoints::tenant 模块

// DIRECTORY constants
pub const DIRECTORY_V1_DEPARTMENTS: &str = Endpoints::DIRECTORY_V1_DEPARTMENTS;
pub const DIRECTORY_V1_DEPARTMENT_GET: &str = Endpoints::DIRECTORY_V1_DEPARTMENT_GET;
pub const DIRECTORY_V1_DEPARTMENTS_SEARCH: &str = Endpoints::DIRECTORY_V1_DEPARTMENTS_SEARCH;
pub const DIRECTORY_V1_DEPARTMENTS_FILTER: &str = Endpoints::DIRECTORY_V1_DEPARTMENTS_FILTER;
pub const DIRECTORY_V1_DEPARTMENTS_MGET: &str = Endpoints::DIRECTORY_V1_DEPARTMENTS_MGET;
pub const DIRECTORY_V1_EMPLOYEES: &str = Endpoints::DIRECTORY_V1_EMPLOYEES;
pub const DIRECTORY_V1_EMPLOYEE_GET: &str = Endpoints::DIRECTORY_V1_EMPLOYEE_GET;
pub const DIRECTORY_V1_EMPLOYEE_TO_BE_RESIGNED: &str =
    Endpoints::DIRECTORY_V1_EMPLOYEE_TO_BE_RESIGNED;
pub const DIRECTORY_V1_EMPLOYEE_REGULAR: &str = Endpoints::DIRECTORY_V1_EMPLOYEE_REGULAR;
pub const DIRECTORY_V1_EMPLOYEE_RESURRECT: &str = Endpoints::DIRECTORY_V1_EMPLOYEE_RESURRECT;
pub const DIRECTORY_V1_EMPLOYEES_SEARCH: &str = Endpoints::DIRECTORY_V1_EMPLOYEES_SEARCH;
pub const DIRECTORY_V1_EMPLOYEES_FILTER: &str = Endpoints::DIRECTORY_V1_EMPLOYEES_FILTER;
pub const DIRECTORY_V1_EMPLOYEES_MGET: &str = Endpoints::DIRECTORY_V1_EMPLOYEES_MGET;

// PAYROLL constants
pub const PAYROLL_V1_COST_ALLOCATION_PLANS: &str = Endpoints::PAYROLL_V1_COST_ALLOCATION_PLANS;
pub const PAYROLL_V1_ACCT_ITEMS: &str = Endpoints::PAYROLL_V1_ACCT_ITEMS;
pub const PAYROLL_V1_DATASOURCES: &str = Endpoints::PAYROLL_V1_DATASOURCES;
pub const PAYROLL_V1_DATASOURCE_RECORDS_SAVE: &str = Endpoints::PAYROLL_V1_DATASOURCE_RECORDS_SAVE;
pub const PAYROLL_V1_DATASOURCE_RECORDS_QUERY: &str =
    Endpoints::PAYROLL_V1_DATASOURCE_RECORDS_QUERY;
pub const PAYROLL_V1_PAYMENT_DETAILS: &str = Endpoints::PAYROLL_V1_PAYMENT_DETAILS;
pub const PAYROLL_V1_PAYMENT_DETAILS_QUERY: &str = Endpoints::PAYROLL_V1_PAYMENT_DETAILS_QUERY;
pub const PAYROLL_V1_PAYMENT_ACTIVITIES: &str = Endpoints::PAYROLL_V1_PAYMENT_ACTIVITIES;
pub const PAYROLL_V1_PAYMENT_ACTIVITY_ARCHIVE: &str =
    Endpoints::PAYROLL_V1_PAYMENT_ACTIVITY_ARCHIVE;
pub const PAYROLL_V1_COST_ALLOCATION_REPORTS: &str = Endpoints::PAYROLL_V1_COST_ALLOCATION_REPORTS;
pub const PAYROLL_V1_PAYGROUPS: &str = Endpoints::PAYROLL_V1_PAYGROUPS;

// SEARCH constants
pub const SEARCH_V1_USER: &str = Endpoints::SEARCH_V1_USER;
pub const SEARCH_V2_DATA_ITEM_CREATE: &str = Endpoints::SEARCH_V2_DATA_ITEM_CREATE;
pub const SEARCH_V2_DATA_ITEM_BATCH_CREATE: &str = Endpoints::SEARCH_V2_DATA_ITEM_BATCH_CREATE;
pub const SEARCH_V2_DATA_ITEM_OPERATION: &str = Endpoints::SEARCH_V2_DATA_ITEM_OPERATION;
pub const SEARCH_V2_MESSAGE: &str = Endpoints::SEARCH_V2_MESSAGE;
pub const SEARCH_V2_APP: &str = Endpoints::SEARCH_V2_APP;
pub const SEARCH_V2_DATA_SOURCES: &str = Endpoints::SEARCH_V2_DATA_SOURCES;
pub const SEARCH_V2_DATA_SOURCE_OPERATION: &str = Endpoints::SEARCH_V2_DATA_SOURCE_OPERATION;
pub const SEARCH_V2_SCHEMA_CREATE: &str = Endpoints::SEARCH_V2_SCHEMA_CREATE;
pub const SEARCH_V2_SCHEMA_OPERATION: &str = Endpoints::SEARCH_V2_SCHEMA_OPERATION;

// OKR constants
pub const OKR_V1_REVIEWS_QUERY: &str = Endpoints::OKR_V1_REVIEWS_QUERY;
pub const OKR_V1_OKRS: &str = Endpoints::OKR_V1_OKRS;
pub const OKR_V1_OKRS_BATCH_GET: &str = Endpoints::OKR_V1_OKRS_BATCH_GET;
pub const OKR_V1_PERIOD_RULES: &str = Endpoints::OKR_V1_PERIOD_RULES;
pub const OKR_V1_PERIODS: &str = Endpoints::OKR_V1_PERIODS;
pub const OKR_V1_PERIOD_GET: &str = Endpoints::OKR_V1_PERIOD_GET;
pub const OKR_V1_PROGRESS_RECORDS: &str = Endpoints::OKR_V1_PROGRESS_RECORDS;
pub const OKR_V1_PROGRESS_RECORD_OPERATION: &str = Endpoints::OKR_V1_PROGRESS_RECORD_OPERATION;
pub const OKR_V1_PROGRESS_RECORDS_UPLOAD: &str = Endpoints::OKR_V1_PROGRESS_RECORDS_UPLOAD;

// Trust Party constants
pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATIONS: &str =
    Endpoints::TRUST_PARTY_V1_COLLABORATION_ORGANIZATIONS;
pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_VISIBLE: &str =
    Endpoints::TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_VISIBLE;
pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_GET: &str =
    Endpoints::TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_GET;
pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_USER_GET: &str =
    Endpoints::TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_USER_GET;
pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_DEPARTMENT_GET: &str =
    Endpoints::TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_DEPARTMENT_GET;
pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_SHARED_MEMBER_SCOPES: &str =
    Endpoints::TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_SHARED_MEMBER_SCOPES;
pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATIONS_ADMIN: &str =
    Endpoints::TRUST_PARTY_V1_COLLABORATION_ORGANIZATIONS_ADMIN;
pub const TRUST_PARTY_V1_SEARCHABLE_VISIBLE_RULES: &str =
    Endpoints::TRUST_PARTY_V1_SEARCHABLE_VISIBLE_RULES;
pub const TRUST_PARTY_V1_SEARCHABLE_VISIBLE_RULE_OPERATION: &str =
    Endpoints::TRUST_PARTY_V1_SEARCHABLE_VISIBLE_RULE_OPERATION;

// CARDKIT constants
pub const CARDKIT_V1_CARDS: &str = Endpoints::CARDKIT_V1_CARDS;
pub const CARDKIT_V1_CARD_ELEMENTS: &str = Endpoints::CARDKIT_V1_CARD_ELEMENTS;
pub const CARDKIT_V1_CARD_SETTINGS: &str = Endpoints::CARDKIT_V1_CARD_SETTINGS;
pub const CARDKIT_V1_CARD_UPDATE: &str = Endpoints::CARDKIT_V1_CARD_UPDATE;
pub const CARDKIT_V1_CARD_BATCH_UPDATE: &str = Endpoints::CARDKIT_V1_CARD_BATCH_UPDATE;

// AILY constants
pub const AILY_V1_SESSIONS: &str = Endpoints::AILY_V1_SESSIONS;
pub const AILY_V1_SESSION_OPERATION: &str = Endpoints::AILY_V1_SESSION_OPERATION;
pub const AILY_V1_RUNS: &str = Endpoints::AILY_V1_RUNS;
pub const AILY_V1_RUN_GET: &str = Endpoints::AILY_V1_RUN_GET;
pub const AILY_V1_RUN_CANCEL: &str = Endpoints::AILY_V1_RUN_CANCEL;
pub const AILY_V1_DATA_KNOWLEDGE_ASK: &str = Endpoints::AILY_V1_DATA_KNOWLEDGE_ASK;
pub const AILY_V1_DATA_KNOWLEDGE_UPLOAD_FILE: &str = Endpoints::AILY_V1_DATA_KNOWLEDGE_UPLOAD_FILE;
pub const AILY_V1_DATA_KNOWLEDGE: &str = Endpoints::AILY_V1_DATA_KNOWLEDGE;
pub const AILY_V1_DATA_KNOWLEDGE_OPERATION: &str = Endpoints::AILY_V1_DATA_KNOWLEDGE_OPERATION;
pub const AILY_V1_DATA_KNOWLEDGE_CATEGORIES: &str = Endpoints::AILY_V1_DATA_KNOWLEDGE_CATEGORIES;
pub const AILY_V1_MESSAGES: &str = Endpoints::AILY_V1_MESSAGES;
pub const AILY_V1_MESSAGE_GET: &str = Endpoints::AILY_V1_MESSAGE_GET;
pub const AILY_V1_SKILL_START: &str = Endpoints::AILY_V1_SKILL_START;
pub const AILY_V1_SKILL_GET: &str = Endpoints::AILY_V1_SKILL_GET;
pub const AILY_V1_SKILLS: &str = Endpoints::AILY_V1_SKILLS;

// ACS constants
pub const ACS_V1_DEVICES: &str = Endpoints::ACS_V1_DEVICES;
pub const ACS_V1_VISITORS: &str = Endpoints::ACS_V1_VISITORS;
pub const ACS_V1_VISITOR_GET: &str = Endpoints::ACS_V1_VISITOR_GET;
pub const ACS_V1_ACCESS_RECORDS: &str = Endpoints::ACS_V1_ACCESS_RECORDS;
pub const ACS_V1_ACCESS_RECORD_FACE_IMAGE: &str = Endpoints::ACS_V1_ACCESS_RECORD_FACE_IMAGE;
pub const ACS_V1_RULE_EXTERNAL: &str = Endpoints::ACS_V1_RULE_EXTERNAL;
pub const ACS_V1_RULE_EXTERNAL_OPERATION: &str = Endpoints::ACS_V1_RULE_EXTERNAL_OPERATION;
pub const ACS_V1_RULE_EXTERNAL_DEVICE_BIND: &str = Endpoints::ACS_V1_RULE_EXTERNAL_DEVICE_BIND;
pub const ACS_V1_USER_OPERATION: &str = Endpoints::ACS_V1_USER_OPERATION;
pub const ACS_V1_USERS: &str = Endpoints::ACS_V1_USERS;
pub const ACS_V1_USER_FACE_IMAGE: &str = Endpoints::ACS_V1_USER_FACE_IMAGE;

// Personal Settings constants
pub const PERSONAL_SETTINGS_V1_SYSTEM_STATUSES: &str =
    Endpoints::PERSONAL_SETTINGS_V1_SYSTEM_STATUSES;
pub const PERSONAL_SETTINGS_V1_SYSTEM_STATUS_OPERATION: &str =
    Endpoints::PERSONAL_SETTINGS_V1_SYSTEM_STATUS_OPERATION;
pub const PERSONAL_SETTINGS_V1_SYSTEM_STATUS_BATCH_OPEN: &str =
    Endpoints::PERSONAL_SETTINGS_V1_SYSTEM_STATUS_BATCH_OPEN;
pub const PERSONAL_SETTINGS_V1_SYSTEM_STATUS_BATCH_CLOSE: &str =
    Endpoints::PERSONAL_SETTINGS_V1_SYSTEM_STATUS_BATCH_CLOSE;

// AI Services constants
pub const SPEECH_TO_TEXT_V1_FILE_RECOGNIZE: &str = Endpoints::SPEECH_TO_TEXT_V1_FILE_RECOGNIZE;
pub const SPEECH_TO_TEXT_V1_STREAM_RECOGNIZE: &str = Endpoints::SPEECH_TO_TEXT_V1_STREAM_RECOGNIZE;
pub const OPTICAL_CHAR_RECOGNITION_V1_BASIC_RECOGNIZE: &str =
    Endpoints::OPTICAL_CHAR_RECOGNITION_V1_BASIC_RECOGNIZE;
pub const TRANSLATION_V1_TEXT_DETECT: &str = Endpoints::TRANSLATION_V1_TEXT_DETECT;
pub const TRANSLATION_V1_TEXT_TRANSLATE: &str = Endpoints::TRANSLATION_V1_TEXT_TRANSLATE;

// E-Learning constants
pub const ELEARNING_V2_COURSE_REGISTRATIONS: &str = Endpoints::ELEARNING_V2_COURSE_REGISTRATIONS;
pub const ELEARNING_V2_COURSE_REGISTRATION_OPERATION: &str =
    Endpoints::ELEARNING_V2_COURSE_REGISTRATION_OPERATION;
pub const ELEARNING_V2_COURSE_REGISTRATIONS_STATISTICS: &str =
    Endpoints::ELEARNING_V2_COURSE_REGISTRATIONS_STATISTICS;

// Tenant Tag constants - 已迁移至 core::endpoints::tenant_tag 模块

// Performance constants
pub const PERFORMANCE_V1_REVIEW_DATA_QUERY: &str = Endpoints::PERFORMANCE_V1_REVIEW_DATA_QUERY;
pub const PERFORMANCE_V1_REVIEW_DATA_DETAILS_QUERY: &str =
    Endpoints::PERFORMANCE_V1_REVIEW_DATA_DETAILS_QUERY;
pub const PERFORMANCE_V1_STAGE_TASK_FIND_BY_USER_LIST: &str =
    Endpoints::PERFORMANCE_V1_STAGE_TASK_FIND_BY_USER_LIST;
pub const PERFORMANCE_V1_STAGE_TASK_FIND_BY_PAGE: &str =
    Endpoints::PERFORMANCE_V1_STAGE_TASK_FIND_BY_PAGE;
pub const PERFORMANCE_V1_METRIC_DETAIL_QUERY: &str = Endpoints::PERFORMANCE_V1_METRIC_DETAIL_QUERY;
pub const PERFORMANCE_V1_METRIC_DETAIL_IMPORT: &str =
    Endpoints::PERFORMANCE_V1_METRIC_DETAIL_IMPORT;

// Human Authentication constants
pub const HUMAN_AUTHENTICATION_V1_IDENTITIES: &str = Endpoints::HUMAN_AUTHENTICATION_V1_IDENTITIES;
pub const HUMAN_AUTHENTICATION_V1_FACE_IMAGES: &str =
    Endpoints::HUMAN_AUTHENTICATION_V1_FACE_IMAGES;
pub const HUMAN_AUTHENTICATION_V1_FACE_IMAGES_CROP: &str =
    Endpoints::HUMAN_AUTHENTICATION_V1_FACE_IMAGES_CROP;
pub const HUMAN_AUTHENTICATION_V1_IDENTITY_RESULT: &str =
    Endpoints::HUMAN_AUTHENTICATION_V1_IDENTITY_RESULT;

// MDM constants
pub const MDM_V1_COUNTRY_REGIONS_BATCH_GET: &str = Endpoints::MDM_V1_COUNTRY_REGIONS_BATCH_GET;
pub const MDM_V1_COUNTRY_REGIONS: &str = Endpoints::MDM_V1_COUNTRY_REGIONS;
pub const MDM_V1_USER_AUTH_DATA_RELATIONS_BIND: &str =
    Endpoints::MDM_V1_USER_AUTH_DATA_RELATIONS_BIND;
pub const MDM_V1_USER_AUTH_DATA_RELATIONS_UNBIND: &str =
    Endpoints::MDM_V1_USER_AUTH_DATA_RELATIONS_UNBIND;

// Security and Compliance constants
pub const SECURITY_AND_COMPLIANCE_V1_AUDIT_DATAS: &str =
    Endpoints::SECURITY_AND_COMPLIANCE_V1_AUDIT_DATAS;
pub const SECURITY_AND_COMPLIANCE_V1_OPENAPI_LOGS_LIST_DATA: &str =
    Endpoints::SECURITY_AND_COMPLIANCE_V1_OPENAPI_LOGS_LIST_DATA;

// Report constants
pub const REPORT_V1_TASKS_QUERY: &str = Endpoints::REPORT_V1_TASKS_QUERY;
pub const REPORT_V1_RULES_QUERY: &str = Endpoints::REPORT_V1_RULES_QUERY;
pub const REPORT_V1_RULE_VIEWS_OPERATION: &str = Endpoints::REPORT_V1_RULE_VIEWS_OPERATION;

// Authentication constants
pub const AUTHEN_V1_USER_INFO: &str = Endpoints::AUTHEN_V1_USER_INFO;

// Calendar constants (补充)
pub const CALENDAR_V4_CALENDARS: &str = Endpoints::CALENDAR_V4_CALENDARS;
pub const CALENDAR_V4_CALENDAR_OPERATION: &str = Endpoints::CALENDAR_V4_CALENDAR_OPERATION;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoint_builder() {
        let result = EndpointBuilder::replace_param("/api/users/{user_id}", "user_id", "123");
        assert_eq!(result, "/api/users/123");
    }

    #[test]
    fn test_multiple_params() {
        let mut params = std::collections::HashMap::new();
        params.insert("calendar_id".to_string(), "cal_123".to_string());
        params.insert("event_id".to_string(), "event_456".to_string());

        let result = EndpointBuilder::replace_params(
            "/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}",
            &params,
        );
        assert_eq!(
            result,
            "/open-apis/calendar/v4/calendars/cal_123/events/event_456"
        );
    }

    // VC room endpoints测试已迁移至对应的vc服务模块

    #[test]
    fn test_calendar_endpoints_builder() {
        let calendar_endpoints = EndpointBuilder::calendar("calendar_456");
        assert_eq!(
            calendar_endpoints.get(),
            "/open-apis/calendar/v4/calendars/calendar_456"
        );
        assert_eq!(
            calendar_endpoints.patch(),
            "/open-apis/calendar/v4/calendars/calendar_456"
        );
        assert_eq!(
            calendar_endpoints.delete(),
            "/open-apis/calendar/v4/calendars/calendar_456"
        );
        assert_eq!(
            calendar_endpoints.set_primary(),
            "/open-apis/calendar/v4/calendars/calendar_456/primary"
        );
        assert_eq!(
            calendar_endpoints.search(),
            "/open-apis/calendar/v4/calendars/calendar_456/search"
        );
    }

    #[test]
    fn test_calendar_event_endpoints_builder() {
        let event_endpoints = EndpointBuilder::calendar_event("calendar_456", "event_789");
        assert_eq!(
            event_endpoints.create(),
            "/open-apis/calendar/v4/calendars/calendar_456/events"
        );
        assert_eq!(
            event_endpoints.get(),
            "/open-apis/calendar/v4/calendars/calendar_456/events/event_789"
        );
        assert_eq!(
            event_endpoints.patch(),
            "/open-apis/calendar/v4/calendars/calendar_456/events/event_789"
        );
        assert_eq!(
            event_endpoints.delete(),
            "/open-apis/calendar/v4/calendars/calendar_456/events/event_789"
        );
        assert_eq!(
            event_endpoints.reply(),
            "/open-apis/calendar/v4/calendars/calendar_456/events/event_789/reply"
        );
        assert_eq!(
            event_endpoints.search(),
            "/open-apis/calendar/v4/calendars/calendar_456/events/event_789/search"
        );
        assert_eq!(
            event_endpoints.list(),
            "/open-apis/calendar/v4/calendars/calendar_456/events"
        );
    }

    #[test]
    fn test_im_message_endpoints_builder() {
        let message_endpoints = EndpointBuilder::im_message("message_123");
        assert_eq!(
            message_endpoints.get(),
            "/open-apis/im/v1/messages/message_123"
        );
        assert_eq!(
            message_endpoints.patch(),
            "/open-apis/im/v1/messages/message_123"
        );
        assert_eq!(
            message_endpoints.delete(),
            "/open-apis/im/v1/messages/message_123"
        );
    }

    #[test]
    fn test_workplace_endpoints() {
        let workplace_endpoints = EndpointBuilder::workplace();
        assert_eq!(
            workplace_endpoints.access_data(),
            "/open-apis/workplace/v1/workplace_access_data/search"
        );
        assert_eq!(
            workplace_endpoints.app_recommend(),
            "/open-apis/workplace/v1/app_recommend_rule/list"
        );
    }
}
