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
//! // 优化前: 每次调用都创建新字符串 (~60字节堆分配)
//! api_path: "/open-apis/workplace/v1/workplace_access_data/search".to_string(),
//!
//! // 优化后: 使用静态常量
//! api_path: Endpoints::WORKPLACE_ACCESS_DATA_SEARCH.to_string(), // 仅在需要时分配
//! // 或者进一步优化为
//! api_path: Endpoints::WORKPLACE_ACCESS_DATA_SEARCH, // 零分配（如果API支持&str）
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
    /// 内推账户相关端点
    pub const HIRE_REFERRAL_ACCOUNTS: &'static str = "/open-apis/hire/v1/referral_accounts";
    pub const HIRE_REFERRAL_INCOME_RECORDS: &'static str =
        "/open-apis/hire/v1/referral_income_records";
    pub const HIRE_REFERRAL_WITHDRAWALS: &'static str = "/open-apis/hire/v1/referral_withdrawals";
    pub const HIRE_REFERRAL_STATISTICS: &'static str = "/open-apis/hire/v1/referral_statistics";

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

    // 会议室管理
    /// 获取会议室列表
    pub const VC_ROOM_LIST: &'static str = "/open-apis/vc/v1/rooms";

    /// 获取会议室详情
    pub const VC_ROOM_GET: &'static str = "/open-apis/vc/v1/rooms/{room_id}";

    /// 创建会议室
    pub const VC_ROOM_CREATE: &'static str = "/open-apis/vc/v1/rooms";

    /// 更新会议室
    pub const VC_ROOM_UPDATE: &'static str = "/open-apis/vc/v1/rooms/{room_id}";

    /// 删除会议室
    pub const VC_ROOM_DELETE: &'static str = "/open-apis/vc/v1/rooms/{room_id}";

    // 会议管理
    /// 获取会议列表
    pub const VC_MEETING_LIST: &'static str = "/open-apis/vc/v1/meetings";

    /// 获取会议详情
    pub const VC_MEETING_GET: &'static str = "/open-apis/vc/v1/meetings/{meeting_id}";

    /// 创建会议
    pub const VC_MEETING_CREATE: &'static str = "/open-apis/vc/v1/meetings";

    /// 更新会议
    pub const VC_MEETING_UPDATE: &'static str = "/open-apis/vc/v1/meetings/{meeting_id}";

    /// 结束会议
    pub const VC_MEETING_END: &'static str = "/open-apis/vc/v1/meetings/{meeting_id}/end";

    /// 邀请参会者
    pub const VC_MEETING_INVITE: &'static str = "/open-apis/vc/v1/meetings/{meeting_id}/invite";

    /// 移除参会者
    pub const VC_MEETING_KICKOUT: &'static str = "/open-apis/vc/v1/meetings/{meeting_id}/kickout";

    // 录制管理
    /// 获取录制列表
    pub const VC_RECORDING_LIST: &'static str = "/open-apis/vc/v1/meetings/{meeting_id}/recordings";

    /// 获取录制详情
    pub const VC_RECORDING_GET: &'static str = "/open-apis/vc/v1/recordings/{recording_id}";

    /// 开始录制
    pub const VC_RECORDING_START: &'static str =
        "/open-apis/vc/v1/meetings/{meeting_id}/recordings/start";

    /// 停止录制
    pub const VC_RECORDING_STOP: &'static str =
        "/open-apis/vc/v1/meetings/{meeting_id}/recordings/stop";

    // 预约管理
    /// 获取预约列表
    pub const VC_RESERVE_LIST: &'static str = "/open-apis/vc/v1/reserves";

    /// 获取预约详情
    pub const VC_RESERVE_GET: &'static str = "/open-apis/vc/v1/reserves/{reserve_id}";

    /// 创建预约
    pub const VC_RESERVE_CREATE: &'static str = "/open-apis/vc/v1/reserves";

    /// 更新预约
    pub const VC_RESERVE_UPDATE: &'static str = "/open-apis/vc/v1/reserves/{reserve_id}";

    /// 删除预约
    pub const VC_RESERVE_DELETE: &'static str = "/open-apis/vc/v1/reserves/{reserve_id}";

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

    // ==================== Performance 绩效管理相关端点 ====================
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

    // ==================== CloudDocs 云文档相关端点 ====================
    
    // Assistant 助手服务
    /// 文档订阅
    pub const ASSISTANT_V1_FILE_SUBSCRIPTION: &'static str = "/open-apis/assistant/v1/file/{}/{}/subscription";

    // Bitable 多维表格服务
    /// 多维表格应用管理
    pub const BITABLE_V1_APPS: &'static str = "/open-apis/bitable/v1/apps";
    pub const BITABLE_V1_APP_GET: &'static str = "/open-apis/bitable/v1/apps/{}";
    pub const BITABLE_V1_APP_CREATE: &'static str = "/open-apis/bitable/v1/apps";
    pub const BITABLE_V1_APP_UPDATE: &'static str = "/open-apis/bitable/v1/apps/{}";
    pub const BITABLE_V1_APP_COPY: &'static str = "/open-apis/bitable/v1/apps/{}/copy";

    /// 多维表格数据表管理
    pub const BITABLE_V1_TABLES: &'static str = "/open-apis/bitable/v1/apps/{}/tables";
    pub const BITABLE_V1_TABLE_GET: &'static str = "/open-apis/bitable/v1/apps/{}/tables/{}";
    pub const BITABLE_V1_TABLE_CREATE: &'static str = "/open-apis/bitable/v1/apps/{}/tables";
    pub const BITABLE_V1_TABLE_PATCH: &'static str = "/open-apis/bitable/v1/apps/{}/tables/{}";
    pub const BITABLE_V1_TABLE_DELETE: &'static str = "/open-apis/bitable/v1/apps/{}/tables/{}";
    pub const BITABLE_V1_TABLES_BATCH_CREATE: &'static str = "/open-apis/bitable/v1/apps/{}/tables/batch_create";
    pub const BITABLE_V1_TABLES_BATCH_DELETE: &'static str = "/open-apis/bitable/v1/apps/{}/tables/batch_delete";

    /// 多维表格记录管理
    pub const BITABLE_V1_RECORDS: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records";
    pub const BITABLE_V1_RECORD_GET: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/{record_id}";
    pub const BITABLE_V1_RECORD_CREATE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records";
    pub const BITABLE_V1_RECORD_UPDATE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/{record_id}";
    pub const BITABLE_V1_RECORD_DELETE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/{record_id}";
    pub const BITABLE_V1_RECORDS_BATCH_CREATE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_create";
    pub const BITABLE_V1_RECORDS_BATCH_UPDATE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_update";
    pub const BITABLE_V1_RECORDS_BATCH_GET: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_get";
    pub const BITABLE_V1_RECORDS_BATCH_DELETE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_delete";
    pub const BITABLE_V1_RECORDS_SEARCH: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/search";

    /// 多维表格字段管理
    pub const BITABLE_V1_FIELDS: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields";
    pub const BITABLE_V1_FIELD_GET: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields/{field_id}";
    pub const BITABLE_V1_FIELD_CREATE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields";
    pub const BITABLE_V1_FIELD_UPDATE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields/{field_id}";
    pub const BITABLE_V1_FIELD_DELETE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields/{field_id}";

    /// 多维表格视图管理
    pub const BITABLE_V1_VIEWS: &'static str = "/open-apis/bitable/v1/apps/{}/tables/{}/views";
    pub const BITABLE_V1_VIEW_GET: &'static str = "/open-apis/bitable/v1/apps/{}/tables/{}/views/{}";
    pub const BITABLE_V1_VIEW_CREATE: &'static str = "/open-apis/bitable/v1/apps/{}/tables/{}/views";
    pub const BITABLE_V1_VIEW_PATCH: &'static str = "/open-apis/bitable/v1/apps/{}/tables/{}/views/{}";
    pub const BITABLE_V1_VIEW_DELETE: &'static str = "/open-apis/bitable/v1/apps/{}/tables/{}/views/{}";

    /// 多维表格仪表板管理
    pub const BITABLE_V1_DASHBOARDS: &'static str = "/open-apis/bitable/v1/apps/{app_token}/dashboards";
    pub const BITABLE_V1_DASHBOARD_COPY: &'static str = "/open-apis/bitable/v1/apps/{app_token}/dashboards/{block_id}/copy";

    /// 多维表格角色管理
    pub const BITABLE_V1_ROLES: &'static str = "/open-apis/bitable/v1/apps/{app_token}/roles";
    pub const BITABLE_V1_ROLE_GET: &'static str = "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}";
    pub const BITABLE_V1_ROLE_CREATE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/roles";
    pub const BITABLE_V1_ROLE_UPDATE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}";
    pub const BITABLE_V1_ROLE_DELETE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}";

    /// 多维表格角色成员管理
    pub const BITABLE_V1_ROLE_MEMBERS: &'static str = "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members";
    pub const BITABLE_V1_ROLE_MEMBER_CREATE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members";
    pub const BITABLE_V1_ROLE_MEMBER_DELETE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members/{member_id}";
    pub const BITABLE_V1_ROLE_MEMBERS_BATCH_CREATE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members/batch_create";
    pub const BITABLE_V1_ROLE_MEMBERS_BATCH_DELETE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members/batch_delete";

    /// 多维表格表单管理
    pub const BITABLE_V1_FORMS: &'static str = "/open-apis/bitable/v1/apps/{app_token}/forms";
    pub const BITABLE_V1_FORM_GET: &'static str = "/open-apis/bitable/v1/apps/{app_token}/forms/{form_id}";
    pub const BITABLE_V1_FORM_PATCH: &'static str = "/open-apis/bitable/v1/apps/{app_token}/forms/{form_id}";
    pub const BITABLE_V1_FORM_PATCH_META: &'static str = "/open-apis/bitable/v1/apps/{app_token}/forms/{form_id}/questions";

    /// 多维表格工作流管理
    pub const BITABLE_V1_WORKFLOWS: &'static str = "/open-apis/bitable/v1/apps/{app_token}/workflows";
    pub const BITABLE_V1_WORKFLOW_UPDATE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/workflows/{workflow_id}";

    // Board 白板服务
    /// 白板管理
    pub const BOARD_V1_WHITEBOARD_THUMBNAIL: &'static str = "/open-apis/whiteboard/v1/whiteboards/{}/thumbnail";
    pub const BOARD_V1_WHITEBOARD_NODES: &'static str = "/open-apis/board/v1/whiteboards/{}/nodes";

    // Comments 评论服务
    /// 评论管理
    pub const COMMENT_V1_COMMENTS: &'static str = "/open-apis/comment/v1/comments";
    pub const COMMENT_V1_COMMENT_GET: &'static str = "/open-apis/comment/v1/comments/{}";
    pub const COMMENT_V1_COMMENT_CREATE: &'static str = "/open-apis/comment/v1/comments";
    pub const COMMENT_V1_COMMENT_PATCH: &'static str = "/open-apis/comment/v1/comments/{}";
    pub const COMMENT_V1_COMMENTS_BATCH_QUERY: &'static str = "/open-apis/comment/v1/comments/batch_query";

    /// 评论回复管理
    pub const COMMENT_V1_COMMENT_REPLIES: &'static str = "/open-apis/comment/v1/comments/{}/replies";
    pub const COMMENT_V1_COMMENT_REPLY_UPDATE: &'static str = "/open-apis/comment/v1/comments/{}/replies/{}";
    pub const COMMENT_V1_COMMENT_REPLY_DELETE: &'static str = "/open-apis/comment/v1/comments/{}/replies/{}";

    // Docx 文档服务
    /// 文档管理
    pub const DOCX_V1_DOCUMENTS: &'static str = "/open-apis/docx/v1/documents";
    pub const DOCX_V1_DOCUMENT_GET: &'static str = "/open-apis/docx/v1/documents/{}";
    pub const DOCX_V1_DOCUMENT_RAW_CONTENT: &'static str = "/open-apis/docx/v1/documents/{}/raw_content";
    pub const DOCX_V1_DOCUMENT_CONVERT: &'static str = "/open-apis/docx/v1/documents/{}/convert";

    /// 文档块管理
    pub const DOCX_V1_DOCUMENT_BLOCKS: &'static str = "/open-apis/docx/v1/documents/{}/blocks";
    pub const DOCX_V1_DOCUMENT_BLOCK_GET: &'static str = "/open-apis/docx/v1/documents/{}/blocks/{}";
    pub const DOCX_V1_DOCUMENT_BLOCK_CHILDREN: &'static str = "/open-apis/docx/v1/documents/{}/blocks/{}/children";
    pub const DOCX_V1_DOCUMENT_BLOCKS_BATCH_UPDATE: &'static str = "/open-apis/docx/v1/documents/{}/blocks/batch_update";
    pub const DOCX_V1_DOCUMENT_BLOCKS_BATCH_DELETE: &'static str = "/open-apis/docx/v1/documents/{}/blocks/batch_delete";

    // Drive 云盘服务
    /// 文件管理
    pub const DRIVE_V1_FILES: &'static str = "/open-apis/drive/v1/files";
    pub const DRIVE_V1_FILE_GET: &'static str = "/open-apis/drive/v1/files/{}";
    pub const DRIVE_V1_FILE_COPY: &'static str = "/open-apis/drive/v1/files/{}/copy";
    pub const DRIVE_V1_FILE_DOWNLOAD: &'static str = "/open-apis/drive/v1/files/{}/download";
    pub const DRIVE_V1_FILE_STATISTICS: &'static str = "/open-apis/drive/v1/files/{}/statistics";
    pub const DRIVE_V1_FILE_VIEW_RECORDS: &'static str = "/open-apis/drive/v1/files/{}/view_records";
    pub const DRIVE_V1_FILE_LIKE_RECORDS: &'static str = "/open-apis/drive/v1/files/{}/like_records";
    pub const DRIVE_V1_FILES_CREATE_FOLDER: &'static str = "/open-apis/drive/v1/files/create_folder";
    pub const DRIVE_V1_FILES_CREATE_SHORTCUT: &'static str = "/open-apis/drive/v1/files/create_shortcut";
    pub const DRIVE_V1_FILES_SEARCH: &'static str = "/open-apis/drive/v1/files/search";
    pub const DRIVE_V1_FILES_SUBSCRIBE: &'static str = "/open-apis/drive/v1/files/subscribe";

    /// 文件版本管理
    pub const DRIVE_V1_FILE_VERSIONS: &'static str = "/open-apis/drive/v1/files/{}/versions";
    pub const DRIVE_V1_FILE_VERSION_GET: &'static str = "/open-apis/drive/v1/files/{}/versions/{}";

    /// 文件订阅管理
    pub const DRIVE_V1_FILE_SUBSCRIPTIONS: &'static str = "/open-apis/drive/v1/files/{}/subscriptions/{}";

    /// 文件夹管理
    pub const DRIVE_V1_FOLDERS: &'static str = "/open-apis/drive/v1/folders";
    pub const DRIVE_V1_FOLDER_GET: &'static str = "/open-apis/drive/v1/folders/{}";
    pub const DRIVE_V1_FOLDER_CHILDREN: &'static str = "/open-apis/drive/v1/folders/{}/children";
    pub const DRIVE_V1_FOLDER_MOVE: &'static str = "/open-apis/drive/v1/folders/{}/move";
    pub const DRIVE_V1_FOLDERS_ROOT_FOLDER_META: &'static str = "/open-apis/drive/v1/folders/root_folder_meta";

    /// 文件上传管理
    pub const DRIVE_V1_FILES_UPLOAD_ALL: &'static str = "/open-apis/drive/v1/files/upload_all";
    pub const DRIVE_V1_FILES_UPLOAD_PREPARE: &'static str = "/open-apis/drive/v1/files/upload_prepare";
    pub const DRIVE_V1_FILES_UPLOAD_PART: &'static str = "/open-apis/drive/v1/files/upload_part";
    pub const DRIVE_V1_FILES_UPLOAD_FINISH: &'static str = "/open-apis/drive/v1/files/upload_finish";

    /// 媒体文件管理
    pub const DRIVE_V1_MEDIAS_UPLOAD_ALL: &'static str = "/open-apis/drive/v1/medias/upload_all";
    pub const DRIVE_V1_MEDIAS_UPLOAD_PREPARE: &'static str = "/open-apis/drive/v1/medias/upload_prepare";
    pub const DRIVE_V1_MEDIAS_UPLOAD_PART: &'static str = "/open-apis/drive/v1/medias/upload_part";
    pub const DRIVE_V1_MEDIAS_UPLOAD_FINISH: &'static str = "/open-apis/drive/v1/medias/upload_finish";
    pub const DRIVE_V1_MEDIAS_DOWNLOAD: &'static str = "/open-apis/drive/v1/medias/{}/download";
    pub const DRIVE_V1_MEDIAS_BATCH_GET_TMP_DOWNLOAD_URL: &'static str = "/open-apis/drive/v1/medias/batch_get_tmp_download_url";

    /// 导入任务管理
    pub const DRIVE_V1_IMPORT_TASKS: &'static str = "/open-apis/drive/v1/import_tasks";
    pub const DRIVE_V1_IMPORT_TASK_GET: &'static str = "/open-apis/drive/v1/import_tasks/{}";

    /// 元信息管理
    pub const DRIVE_V1_METAS_BATCH_QUERY: &'static str = "/open-apis/drive/v1/metas/batch_query";

    /// 任务管理
    pub const DRIVE_V1_TASK_GET: &'static str = "/open-apis/drive/v1/tasks/{}";

    /// 权限管理
    pub const DRIVE_V1_PERMISSIONS_MEMBERS: &'static str = "/open-apis/drive/v1/permissions/{}/members";
    pub const DRIVE_V1_PERMISSIONS_MEMBER_GET: &'static str = "/open-apis/drive/v1/permissions/{}/members/{}";
    pub const DRIVE_V1_PERMISSIONS_MEMBERS_BATCH_CREATE: &'static str = "/open-apis/drive/v1/permissions/{}/members/batch_create";
    pub const DRIVE_V1_PERMISSIONS_MEMBERS_AUTH: &'static str = "/open-apis/drive/v1/permissions/{}/members/auth";
    pub const DRIVE_V1_PERMISSIONS_MEMBERS_TRANSFER_OWNER: &'static str = "/open-apis/drive/v1/permissions/{}/members/transfer_owner";
    pub const DRIVE_V1_PERMISSIONS_PUBLIC: &'static str = "/open-apis/drive/v1/permissions/{}/public";
    pub const DRIVE_V1_PERMISSIONS_PUBLIC_PASSWORD: &'static str = "/open-apis/drive/v1/permissions/{}/public/password";

    /// Drive v2 权限管理
    pub const DRIVE_V2_PERMISSIONS_PUBLIC: &'static str = "/open-apis/drive/v2/permissions/{}/public";

    /// Drive Explorer
    pub const DRIVE_EXPLORER_V2_ROOT_FOLDER_META: &'static str = "/open-apis/drive/explorer/v2/root_folder/meta";
    pub const DRIVE_EXPLORER_V2_FOLDER_META: &'static str = "/open-apis/drive/explorer/v2/folder/{folder_token}/meta";

    // Sheets 电子表格服务
    /// 电子表格管理 - v3
    pub const SHEETS_V3_SPREADSHEETS: &'static str = "/open-apis/sheets/v3/spreadsheets";
    pub const SHEETS_V3_SPREADSHEET_GET: &'static str = "/open-apis/sheets/v3/spreadsheets/{}";
    pub const SHEETS_V3_SPREADSHEET_CREATE: &'static str = "/open-apis/sheets/v3/spreadsheets";
    pub const SHEETS_V3_SPREADSHEET_PATCH: &'static str = "/open-apis/sheets/v3/spreadsheets/{}";

    /// 电子表格工作表管理 - v3
    pub const SHEETS_V3_SPREADSHEET_SHEETS_QUERY: &'static str = "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/query";
    pub const SHEETS_V3_SPREADSHEET_SHEET_GET: &'static str = "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}";

    /// 电子表格数据操作 - v3
    pub const SHEETS_V3_SPREADSHEET_VALUES_GET: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/values/{}";
    pub const SHEETS_V3_SPREADSHEET_VALUES_BATCH_GET: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/values/batch_get";
    pub const SHEETS_V3_SPREADSHEET_VALUES_BATCH_UPDATE: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/values/batch_update";
    pub const SHEETS_V3_SPREADSHEET_VALUES_APPEND: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/values/{}/append";
    pub const SHEETS_V3_SPREADSHEET_VALUES_PREPEND: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/values/{}/prepend";
    pub const SHEETS_V3_SPREADSHEET_VALUES_IMAGE: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/values_image";

    /// 电子表格查找替换 - v3
    pub const SHEETS_V3_SPREADSHEET_SHEET_FIND: &'static str = "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/find";
    pub const SHEETS_V3_SPREADSHEET_SHEET_REPLACE: &'static str = "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/replace";

    /// 电子表格合并拆分单元格 - v3
    pub const SHEETS_V3_SPREADSHEET_MERGE_CELLS: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/merge_cells";
    pub const SHEETS_V3_SPREADSHEET_UNMERGE_CELLS: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/unmerge_cells";

    /// 电子表格样式管理 - v3
    pub const SHEETS_V3_SPREADSHEET_STYLE: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/style";
    pub const SHEETS_V3_SPREADSHEET_STYLES_BATCH_UPDATE: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/styles/batch_update";

    /// 电子表格行列管理 - v3
    pub const SHEETS_V3_SPREADSHEET_DIMENSION_RANGE: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/dimension_range";
    pub const SHEETS_V3_SPREADSHEET_DIMENSION_RANGE_INSERT: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/dimension_range:insert";
    pub const SHEETS_V3_SPREADSHEET_MOVE_DIMENSION: &'static str = "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/move_dimension";

    /// 电子表格条件格式 - v3
    pub const SHEETS_V3_SPREADSHEET_CONDITION_FORMAT: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/conditionFormat";

    /// 电子表格数据验证 - v3
    pub const SHEETS_V3_SPREADSHEET_DATA_VALIDATION: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/dataValidation";
    pub const SHEETS_V3_SPREADSHEET_DATA_VALIDATION_GET: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/dataValidation/{}";

    /// 电子表格保护范围 - v3
    pub const SHEETS_V3_SPREADSHEET_PROTECT_RANGE: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/protect_range";
    pub const SHEETS_V3_SPREADSHEET_PROTECT_RANGE_GET: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/protect_range/{}";

    /// 电子表格筛选器 - v3
    pub const SHEETS_V3_SPREADSHEET_FILTER: &'static str = "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter";

    /// 电子表格筛选视图 - v3
    pub const SHEETS_V3_SPREADSHEET_FILTER_VIEWS: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter_views";
    pub const SHEETS_V3_SPREADSHEET_FILTER_VIEW_GET: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter_views/{}";

    /// 电子表格筛选视图条件 - v3
    pub const SHEETS_V3_SPREADSHEET_FILTER_VIEW_CONDITIONS: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter_views/{}/conditions";
    pub const SHEETS_V3_SPREADSHEET_FILTER_VIEW_CONDITION_GET: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter_views/{}/conditions/{}";

    /// 电子表格浮动图片 - v3
    pub const SHEETS_V3_SPREADSHEET_FLOAT_IMAGES: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/float_images";
    pub const SHEETS_V3_SPREADSHEET_FLOAT_IMAGE_GET: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/float_images/{}";

    /// 电子表格管理 - v2
    pub const SHEETS_V2_SPREADSHEET_VALUES: &'static str = "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values";
    pub const SHEETS_V2_SPREADSHEET_VALUES_RANGE: &'static str = "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values/{range}";
    pub const SHEETS_V2_SPREADSHEET_VALUES_APPEND: &'static str = "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_append";
    pub const SHEETS_V2_SPREADSHEET_VALUES_PREPEND: &'static str = "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_prepend";
    pub const SHEETS_V2_SPREADSHEET_VALUES_BATCH_GET: &'static str = "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_batch_get";
    pub const SHEETS_V2_SPREADSHEET_VALUES_BATCH_UPDATE: &'static str = "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_batch_update";
    pub const SHEETS_V2_SPREADSHEET_VALUES_IMAGE: &'static str = "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_image";

    /// 电子表格样式管理 - v2
    pub const SHEETS_V2_SPREADSHEET_STYLE: &'static str = "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/style";
    pub const SHEETS_V2_SPREADSHEET_STYLES_BATCH_UPDATE: &'static str = "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/styles_batch_update";

    /// 电子表格合并拆分单元格 - v2
    pub const SHEETS_V2_SPREADSHEET_MERGE_CELLS: &'static str = "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/merge_cells";
    pub const SHEETS_V2_SPREADSHEET_UNMERGE_CELLS: &'static str = "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/unmerge_cells";

    /// 电子表格行列管理 - v2
    pub const SHEETS_V2_SPREADSHEET_DIMENSION_RANGE: &'static str = "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/dimension_range";
    pub const SHEETS_V2_SPREADSHEET_INSERT_DIMENSION_RANGE: &'static str = "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/insert_dimension_range";

    /// 电子表格工作表管理 - v2
    pub const SHEETS_V2_SPREADSHEET_SHEETS_BATCH_UPDATE: &'static str = "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/sheets_batch_update";

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
    pub const WIKI_V2_SPACE_NODE_UPDATE_TITLE: &'static str = "/open-apis/wiki/v2/spaces/{}/nodes/{}/update_title";

    /// 知识库设置管理
    pub const WIKI_V2_SPACE_SETTING_UPDATE: &'static str = "/open-apis/wiki/v2/spaces/{}/setting";

    /// 知识库搜索
    pub const WIKI_V2_SEARCH: &'static str = "/open-apis/wiki/v2/search";

    /// 知识库任务管理
    pub const WIKI_V2_TASKS_MOVE_DOCS_TO_WIKI: &'static str = "/open-apis/wiki/v2/tasks/move_docs_to_wiki";
    pub const WIKI_V2_TASK_GET: &'static str = "/open-apis/wiki/v2/tasks/{}";
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
    /// params.insert("calendar_id", "cal_123".to_string());
    /// params.insert("event_id", "event_456".to_string());
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

    /// 构建VC会议室相关端点
    pub fn vc_room(room_id: &str) -> VCRoomEndpoints {
        VCRoomEndpoints {
            room_id: room_id.to_string(),
        }
    }

    /// 构建VC会议相关端点
    pub fn vc_meeting(meeting_id: &str) -> VCMeetingEndpoints {
        VCMeetingEndpoints {
            meeting_id: meeting_id.to_string(),
        }
    }

    /// 构建IM相关端点
    pub fn im_message(message_id: &str) -> IMMessageEndpoints {
        IMMessageEndpoints {
            message_id: message_id.to_string(),
        }
    }

    /// 构建Drive文件相关端点
    pub fn drive_file(file_token: &str) -> DriveFileEndpoints {
        DriveFileEndpoints {
            file_token: file_token.to_string(),
        }
    }

    /// 构建日历相关端点
    pub fn calendar(calendar_id: &str) -> CalendarEndpoints {
        CalendarEndpoints {
            calendar_id: calendar_id.to_string(),
        }
    }

    /// 构建日程相关端点
    pub fn calendar_event(calendar_id: &str, event_id: &str) -> CalendarEventEndpoints {
        CalendarEventEndpoints {
            calendar_id: calendar_id.to_string(),
            event_id: event_id.to_string(),
        }
    }
}

/// VC会议室端点构建器
pub struct VCRoomEndpoints {
    room_id: String,
}

impl VCRoomEndpoints {
    /// 获取会议室详情端点
    pub fn get(&self) -> String {
        EndpointBuilder::replace_param(Endpoints::VC_ROOM_GET, "room_id", &self.room_id)
    }

    /// 更新会议室端点
    pub fn update(&self) -> String {
        EndpointBuilder::replace_param(Endpoints::VC_ROOM_UPDATE, "room_id", &self.room_id)
    }

    /// 删除会议室端点
    pub fn delete(&self) -> String {
        EndpointBuilder::replace_param(Endpoints::VC_ROOM_DELETE, "room_id", &self.room_id)
    }
}

/// VC会议端点构建器
pub struct VCMeetingEndpoints {
    meeting_id: String,
}

impl VCMeetingEndpoints {
    /// 获取会议详情端点
    pub fn get(&self) -> String {
        EndpointBuilder::replace_param(Endpoints::VC_MEETING_GET, "meeting_id", &self.meeting_id)
    }

    /// 更新会议端点
    pub fn update(&self) -> String {
        EndpointBuilder::replace_param(Endpoints::VC_MEETING_UPDATE, "meeting_id", &self.meeting_id)
    }

    /// 结束会议端点
    pub fn end(&self) -> String {
        EndpointBuilder::replace_param(Endpoints::VC_MEETING_END, "meeting_id", &self.meeting_id)
    }

    /// 邀请参会者端点
    pub fn invite(&self) -> String {
        EndpointBuilder::replace_param(Endpoints::VC_MEETING_INVITE, "meeting_id", &self.meeting_id)
    }

    /// 移除参会者端点
    pub fn kickout(&self) -> String {
        EndpointBuilder::replace_param(
            Endpoints::VC_MEETING_KICKOUT,
            "meeting_id",
            &self.meeting_id,
        )
    }

    /// 获取录制列表端点
    pub fn recording_list(&self) -> String {
        EndpointBuilder::replace_param(Endpoints::VC_RECORDING_LIST, "meeting_id", &self.meeting_id)
    }

    /// 开始录制端点
    pub fn recording_start(&self) -> String {
        EndpointBuilder::replace_param(
            Endpoints::VC_RECORDING_START,
            "meeting_id",
            &self.meeting_id,
        )
    }

    /// 停止录制端点
    pub fn recording_stop(&self) -> String {
        EndpointBuilder::replace_param(Endpoints::VC_RECORDING_STOP, "meeting_id", &self.meeting_id)
    }
}

/// IM消息端点构建器
pub struct IMMessageEndpoints {
    message_id: String,
}

impl IMMessageEndpoints {
    /// 获取消息详情端点
    pub fn get(&self) -> String {
        EndpointBuilder::replace_param(Endpoints::IM_V1_GET_MESSAGE, "message_id", &self.message_id)
    }

    /// 更新消息端点
    pub fn update(&self) -> String {
        EndpointBuilder::replace_param(
            Endpoints::IM_V1_UPDATE_MESSAGE,
            "message_id",
            &self.message_id,
        )
    }

    /// 删除消息端点
    pub fn delete(&self) -> String {
        EndpointBuilder::replace_param(
            Endpoints::IM_V1_DELETE_MESSAGE,
            "message_id",
            &self.message_id,
        )
    }

    /// 消息已读回执端点
    pub fn read_users(&self) -> String {
        EndpointBuilder::replace_param(
            Endpoints::IM_V1_READ_MESSAGE,
            "message_id",
            &self.message_id,
        )
    }
}

/// Drive文件端点构建器
pub struct DriveFileEndpoints {
    file_token: String,
}

impl DriveFileEndpoints {
    /// 复制文件端点
    pub fn copy(&self) -> String {
        EndpointBuilder::replace_param(Endpoints::DRIVE_V1_COPY, "file_token", &self.file_token)
    }

    /// 移动文件端点
    pub fn r#move(&self) -> String {
        EndpointBuilder::replace_param(Endpoints::DRIVE_V1_MOVE, "file_token", &self.file_token)
    }

    /// 删除文件端点
    pub fn delete(&self) -> String {
        EndpointBuilder::replace_param(Endpoints::DRIVE_V1_DELETE, "file_token", &self.file_token)
    }

    /// 下载文件端点
    pub fn download(&self) -> String {
        EndpointBuilder::replace_param(Endpoints::DRIVE_V1_DOWNLOAD, "file_token", &self.file_token)
    }
}

/// 日历端点构建器
pub struct CalendarEndpoints {
    calendar_id: String,
}

impl CalendarEndpoints {
    /// 获取日历详情端点
    pub fn get(&self) -> String {
        EndpointBuilder::replace_param(Endpoints::CALENDAR_GET, "calendar_id", &self.calendar_id)
    }

    /// 更新日历端点
    pub fn update(&self) -> String {
        EndpointBuilder::replace_param(Endpoints::CALENDAR_UPDATE, "calendar_id", &self.calendar_id)
    }

    /// 删除日历端点
    pub fn delete(&self) -> String {
        EndpointBuilder::replace_param(Endpoints::CALENDAR_DELETE, "calendar_id", &self.calendar_id)
    }

    /// 创建日程端点
    pub fn create_event(&self) -> String {
        EndpointBuilder::replace_param(
            Endpoints::CALENDAR_EVENT_CREATE,
            "calendar_id",
            &self.calendar_id,
        )
    }

    /// 获取日程列表端点
    pub fn event_list(&self) -> String {
        EndpointBuilder::replace_param(
            Endpoints::CALENDAR_EVENT_LIST,
            "calendar_id",
            &self.calendar_id,
        )
    }
}

/// 日程端点构建器
pub struct CalendarEventEndpoints {
    calendar_id: String,
    event_id: String,
}

impl CalendarEventEndpoints {
    /// 获取日程详情端点
    pub fn get(&self) -> String {
        let template = Endpoints::CALENDAR_EVENT_GET;
        let mut result = EndpointBuilder::replace_param(template, "calendar_id", &self.calendar_id);
        result = EndpointBuilder::replace_param(&result, "event_id", &self.event_id);
        result
    }

    /// 更新日程端点
    pub fn update(&self) -> String {
        let template = Endpoints::CALENDAR_EVENT_UPDATE;
        let mut result = EndpointBuilder::replace_param(template, "calendar_id", &self.calendar_id);
        result = EndpointBuilder::replace_param(&result, "event_id", &self.event_id);
        result
    }

    /// 删除日程端点
    pub fn delete(&self) -> String {
        let template = Endpoints::CALENDAR_EVENT_DELETE;
        let mut result = EndpointBuilder::replace_param(template, "calendar_id", &self.calendar_id);
        result = EndpointBuilder::replace_param(&result, "event_id", &self.event_id);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_workplace_endpoints() {
        assert_eq!(
            Endpoints::WORKPLACE_ACCESS_DATA_SEARCH,
            "/open-apis/workplace/v1/workplace_access_data/search"
        );
        assert_eq!(
            Endpoints::WORKPLACE_APP_RECOMMEND_LIST,
            "/open-apis/workplace/v1/app_recommend_rule/list"
        );
    }

    #[test]
    fn test_vc_endpoints() {
        assert_eq!(Endpoints::VC_ROOM_LIST, "/open-apis/vc/v1/rooms");
        assert_eq!(Endpoints::VC_MEETING_CREATE, "/open-apis/vc/v1/meetings");
    }

    #[test]
    fn test_endpoint_builder() {
        let result = EndpointBuilder::replace_param(
            "/open-apis/vc/v1/rooms/{room_id}",
            "room_id",
            "room_123",
        );
        assert_eq!(result, "/open-apis/vc/v1/rooms/room_123");
    }

    #[test]
    fn test_multiple_params() {
        let mut params = std::collections::HashMap::new();
        params.insert("calendar_id", "cal_123".to_string());
        params.insert("event_id", "event_456".to_string());

        let result = EndpointBuilder::replace_params(
            "/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}",
            &params,
        );
        assert_eq!(
            result,
            "/open-apis/calendar/v4/calendars/cal_123/events/event_456"
        );
    }

    #[test]
    fn test_vc_room_endpoints_builder() {
        let room_endpoints = EndpointBuilder::vc_room("room_123");
        assert_eq!(room_endpoints.get(), "/open-apis/vc/v1/rooms/room_123");
        assert_eq!(room_endpoints.update(), "/open-apis/vc/v1/rooms/room_123");
        assert_eq!(room_endpoints.delete(), "/open-apis/vc/v1/rooms/room_123");
    }

    #[test]
    fn test_im_message_endpoints_builder() {
        let message_endpoints = EndpointBuilder::im_message("msg_123");
        assert_eq!(message_endpoints.get(), "/open-apis/im/v1/messages/msg_123");
        assert_eq!(
            message_endpoints.update(),
            "/open-apis/im/v1/messages/msg_123"
        );
        assert_eq!(
            message_endpoints.delete(),
            "/open-apis/im/v1/messages/msg_123"
        );
    }

    #[test]
    fn test_calendar_endpoints_builder() {
        let calendar_endpoints = EndpointBuilder::calendar("cal_123");
        assert_eq!(
            calendar_endpoints.get(),
            "/open-apis/calendar/v4/calendars/cal_123"
        );
        assert_eq!(
            calendar_endpoints.update(),
            "/open-apis/calendar/v4/calendars/cal_123"
        );
        assert_eq!(
            calendar_endpoints.delete(),
            "/open-apis/calendar/v4/calendars/cal_123"
        );
    }

    #[test]
    fn test_calendar_event_endpoints_builder() {
        let event_endpoints = EndpointBuilder::calendar_event("cal_123", "event_456");
        assert_eq!(
            event_endpoints.get(),
            "/open-apis/calendar/v4/calendars/cal_123/events/event_456"
        );
        assert_eq!(
            event_endpoints.update(),
            "/open-apis/calendar/v4/calendars/cal_123/events/event_456"
        );
        assert_eq!(
            event_endpoints.delete(),
            "/open-apis/calendar/v4/calendars/cal_123/events/event_456"
        );
    }
}
