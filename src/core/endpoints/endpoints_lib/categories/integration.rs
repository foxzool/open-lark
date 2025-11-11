//! 集成和外部系统服务端点

/// 集成和外部系统服务端点
pub struct Integration;

impl Integration {
    /// / 搜索自定义工作台访问数据
    pub const WORKPLACE_ACCESS_DATA_SEARCH: &'static str = "/open-apis/workplace/v1/workplace_access_data/search";
    /// / 搜索工作台小部件访问数据
    pub const WORKPLACE_CUSTOM_ACCESS_DATA_SEARCH: &'static str = "/open-apis/workplace/v1/custom_workplace_access_data/search";
    /// ==================== 招聘相关端点 ====================
    pub const WORKPLACE_WIDGET_ACCESS_DATA_SEARCH: &'static str = "/open-apis/workplace/v1/custom_workplace_widget_access_data/search";
    /// / 获取推荐的推荐规则
    pub const WORKPLACE_APP_RECOMMEND_FAVOURITE: &'static str = "/open-apis/workplace/v1/app_recommend_rule/favourite";
    /// / 获取推荐规则列表
    pub const WORKPLACE_APP_RECOMMEND_RECOMMEND: &'static str = "/open-apis/workplace/v1/app_recommend_rule/recommend";
    /// ==================== 视频会议服务端点 ====================
    pub const WORKPLACE_APP_RECOMMEND_LIST: &'static str = "/open-apis/workplace/v1/app_recommend_rule/list";
    /// ==================== 联系人服务端点 ====================
    pub const APAAS_V1_APPS: &'static str = "/open-apis/apaas/v1/apps";
    /// / 获取用户列表
    pub const CONTACT_USER_GET: &'static str = "/open-apis/contact/v3/users/{user_id}";
    /// / 批量获取用户信息
    pub const CONTACT_USER_LIST: &'static str = "/open-apis/contact/v3/users";
    /// / 搜索用户
    pub const CONTACT_USER_BATCH_GET: &'static str = "/open-apis/contact/v3/users/batch";
    /// 部门管理
    pub const CONTACT_USER_SEARCH: &'static str = "/open-apis/contact/v3/users/find_by_mobile";
    /// / 获取部门列表
    pub const CONTACT_DEPARTMENT_GET: &'static str = "/open-apis/contact/v3/departments/{department_id}";
    /// / 获取子部门列表
    pub const CONTACT_DEPARTMENT_LIST: &'static str = "/open-apis/contact/v3/departments";
    /// ==================== 应用管理服务端点 ====================
    pub const CONTACT_DEPARTMENT_CHILDREN: &'static str = "/open-apis/contact/v3/departments/{department_id}/children";
    /// / 获取应用管理员列表
    pub const APPLICATION_GET_APP_INFO: &'static str = "/open-apis/application/v6/applications/{app_id}";
    /// ==================== 日历服务端点 ====================
    pub const APPLICATION_GET_APP_STORE_INFO: &'static str = "/open-apis/application/v6/applications/{app_id}/app_version";
    /// / 光学字符识别
    pub const SPEECH_TO_TEXT_V1_STREAM_RECOGNIZE: &'static str = "/open-apis/speech_to_text/v1/speech/stream_recognize";
    /// / 语种检测
    pub const OPTICAL_CHAR_RECOGNITION_V1_BASIC_RECOGNIZE: &'static str = "/open-apis/optical_char_recognition/v1/image/basic_recognize";
    /// / 文本翻译
    pub const TRANSLATION_V1_TEXT_DETECT: &'static str = "/open-apis/translation/v1/text/detect";
    /// ==================== E-Learning 在线学习相关端点 ====================
    pub const TRANSLATION_V1_TEXT_TRANSLATE: &'static str = "/open-apis/translation/v1/text/translate";
    /// / 课程报名操作
    pub const ELEARNING_V2_COURSE_REGISTRATIONS: &'static str = "/open-apis/elearning/v2/course_registrations";
    /// / 课程报名统计
    pub const ELEARNING_V2_COURSE_REGISTRATION_OPERATION: &'static str = "/open-apis/elearning/v2/course_registrations/{registration_id}";
    /// ==================== Tenant Tag 租户标签相关端点 ====================
    pub const ELEARNING_V2_COURSE_REGISTRATIONS_STATISTICS: &'static str = "/open-apis/elearning/v2/course_registrations/statistics";
    /// / 账套项目列表
    pub const PAYROLL_V1_COST_ALLOCATION_PLANS: &'static str = "/open-apis/payroll/v1/cost_allocation_plans";
    /// / 数据源列表
    pub const PAYROLL_V1_ACCT_ITEMS: &'static str = "/open-apis/payroll/v1/acct_items";
    /// / 数据源记录保存
    pub const PAYROLL_V1_DATASOURCES: &'static str = "/open-apis/payroll/v1/datasources";
    /// / 数据源记录查询
    pub const PAYROLL_V1_DATASOURCE_RECORDS_SAVE: &'static str = "/open-apis/payroll/v1/datasources/{datasource_id}/records/save";
    /// / 薪酬活动详情查询
    pub const PAYROLL_V1_DATASOURCE_RECORDS_QUERY: &'static str = "/open-apis/payroll/v1/datasources/{datasource_id}/records/query";
    /// / 薪酬活动详情查询（通过查询接口）
    pub const PAYROLL_V1_PAYMENT_DETAILS: &'static str = "/open-apis/payroll/v1/payment_activities/{payment_activity_id}/payment_details";
    /// / 薪酬活动列表
    pub const PAYROLL_V1_PAYMENT_DETAILS_QUERY: &'static str = "/open-apis/payroll/v1/payment_activities/{payment_activity_id}/payment_details/query";
    /// / 薪酬活动归档
    pub const PAYROLL_V1_PAYMENT_ACTIVITIES: &'static str = "/open-apis/payroll/v1/payment_activities";
    /// / 成本分摊报表列表
    pub const PAYROLL_V1_PAYMENT_ACTIVITY_ARCHIVE: &'static str = "/open-apis/payroll/v1/payment_activities/{payment_activity_id}/archive";
    /// / 薪酬组列表
    pub const PAYROLL_V1_COST_ALLOCATION_REPORTS: &'static str = "/open-apis/payroll/v1/cost_allocation_reports";
    /// ==================== APaaS 平台即服务相关端点 ====================
    pub const PAYROLL_V1_PAYGROUPS: &'static str = "/open-apis/payroll/v1/paygroups";
    /// / 查询座位活动列表
    pub const APASS_V1_SEAT_ASSIGNMENT_LIST: &'static str = "/open-apis/apaas/v1/seat_assignment/list";
    /// ===== 流程管理端点 =====
    pub const APASS_V1_SEAT_ACTIVITY_LIST: &'static str = "/open-apis/apaas/v1/seat_activity/list";
    /// / 查询用户任务
    pub const APASS_V1_FLOW_EXECUTE: &'static str = "/open-apis/apaas/v1/application/{app_id}/flow/{flow_api_name}/execute";
    /// ===== 权限管理端点 =====
    pub const APASS_V1_FUNCTION_INVOKE: &'static str = "/open-apis/apaas/v1/application/{app_id}/function/{function_name}/invoke";
    /// / 批量添加角色成员
    pub const APASS_V1_PERMISSION_ROLE_MEMBERS_BATCH_REMOVE: &'static str = "/open-apis/apaas/v1/application/{app_id}/permission/role/{role_api_name}/members/batch_remove";
    /// / 获取角色成员
    pub const APASS_V1_PERMISSION_ROLE_MEMBERS_BATCH_CREATE: &'static str = "/open-apis/apaas/v1/application/{app_id}/permission/role/{role_api_name}/members/batch_create";
    /// / 批量移除记录权限成员
    pub const APASS_V1_PERMISSION_ROLE_MEMBER_GET: &'static str = "/open-apis/apaas/v1/application/{app_id}/permission/role/{role_api_name}/member/{user_id}";
    /// / 批量添加记录权限成员
    pub const APASS_V1_PERMISSION_RECORD_MEMBERS_BATCH_REMOVE: &'static str = "/open-apis/apaas/v1/application/{app_id}/permission/record_permission/{record_permission_api_name}/members/batch_remove";
    /// ===== 对象管理端点 =====
    pub const APASS_V1_PERMISSION_RECORD_MEMBERS_BATCH_CREATE: &'static str = "/open-apis/apaas/v1/application/{app_id}/permission/record_permission/{record_permission_api_name}/members/batch_create";
    /// / 搜索记录
    pub const APASS_V1_OBJECT_OQL: &'static str = "/open-apis/apaas/v1/application/{app_id}/object/oql";
    /// / 获取记录
    pub const APASS_V1_OBJECT_RECORD_SEARCH: &'static str = "/open-apis/apaas/v1/application/{app_id}/object/{object_api_name}/record/search";
    /// / 更新记录
    pub const APASS_V1_OBJECT_RECORD_GET: &'static str = "/open-apis/apaas/v1/application/{app_id}/object/{object_api_name}/record/{record_id}";
    /// / 删除记录
    pub const APASS_V1_OBJECT_RECORD_UPDATE: &'static str = "/open-apis/apaas/v1/application/{app_id}/object/{object_api_name}/record/{record_id}";
    /// / 创建记录
    pub const APASS_V1_OBJECT_RECORD_DELETE: &'static str = "/open-apis/apaas/v1/application/{app_id}/object/{object_api_name}/record/{record_id}";
    /// / 批量更新记录
    pub const APASS_V1_OBJECT_RECORD_CREATE: &'static str = "/open-apis/apaas/v1/application/{app_id}/object/{object_api_name}/record";
    /// / 批量查询记录
    pub const APASS_V1_OBJECT_RECORD_BATCH_UPDATE: &'static str = "/open-apis/apaas/v1/application/{app_id}/object/{object_api_name}/record/batch_update";
    /// / 批量删除记录
    pub const APASS_V1_OBJECT_RECORD_BATCH_QUERY: &'static str = "/open-apis/apaas/v1/application/{app_id}/object/{object_api_name}/record/batch_query";
    /// / 批量创建记录
    pub const APASS_V1_OBJECT_RECORD_BATCH_DELETE: &'static str = "/open-apis/apaas/v1/application/{app_id}/object/{object_api_name}/record/batch_delete";
    /// ===== 环境变量管理端点 =====
    pub const APASS_V1_OBJECT_RECORD_BATCH_CREATE: &'static str = "/open-apis/apaas/v1/application/{app_id}/object/{object_api_name}/record/batch_create";
    /// / 获取环境变量
    pub const APASS_V1_ENVIRONMENT_VARIABLE_QUERY: &'static str = "/open-apis/apaas/v1/application/{app_id}/environment_variable/query";
    /// ===== 审计日志管理端点 =====
    pub const APASS_V1_ENVIRONMENT_VARIABLE_GET: &'static str = "/open-apis/apaas/v1/application/{app_id}/environment_variable/{variable_name}";
    /// / 获取审计日志
    pub const APASS_V1_AUDIT_LOG_LIST: &'static str = "/open-apis/apaas/v1/application/{app_id}/audit_log/list";
    /// / 数据变更日志列表
    pub const APASS_V1_AUDIT_LOG_GET: &'static str = "/open-apis/apaas/v1/application/{app_id}/audit_log/{log_id}";
    /// / 获取数据变更日志
    pub const APASS_V1_AUDIT_LOG_DATA_CHANGE_LOGS: &'static str = "/open-apis/apaas/v1/application/{app_id}/audit_log/data_change_logs";
    /// / 审计事件列表
    pub const APASS_V1_AUDIT_LOG_DATA_CHANGE_LOG_GET: &'static str = "/open-apis/apaas/v1/application/{app_id}/audit_log/data_change_log/{log_id}";
    /// ==================== Performance 绩效管理相关端点 ====================
    pub const APASS_V1_AUDIT_LOG_AUDIT_EVENTS: &'static str = "/open-apis/apaas/v1/application/{app_id}/audit_log/audit_events";
    /// / 评估详情数据查询
    pub const PERFORMANCE_V1_REVIEW_DATA_QUERY: &'static str = "/open-apis/performance/v1/review_data/query";
    /// / 阶段任务用户列表查询
    pub const PERFORMANCE_V1_REVIEW_DATA_DETAILS_QUERY: &'static str = "/open-apis/performance/v1/review_data/details/query";
    /// / 指标详情导入
    pub const PERFORMANCE_V1_METRIC_DETAIL_QUERY: &'static str = "/open-apis/performance/v1/metric_detail/query";
    /// / 绩效周期列表查询
    pub const PERFORMANCE_V1_METRIC_DETAIL_IMPORT: &'static str = "/open-apis/performance/v1/metric_detail/import";
    /// / 绩效项目查询
    pub const PERFORMANCE_SEMESTER_LIST: &'static str = "/open-apis/performance/v1/review_config/semester_activity/semesters";
    /// / 查询附加信息
    pub const PERFORMANCE_ACTIVITIES_QUERY: &'static str = "/open-apis/performance/v1/review_config/semester_activity/activities";
    /// / 导入附加信息
    pub const PERFORMANCE_ADDITIONAL_INFO_QUERY: &'static str = "/open-apis/performance/v1/review_config/semester_activity/additional_information/query";
    /// / 删除附加信息
    pub const PERFORMANCE_ADDITIONAL_INFO_IMPORT: &'static str = "/open-apis/performance/v1/review_config/semester_activity/additional_information/import";
    /// / 用户组关系写入
    pub const PERFORMANCE_ADDITIONAL_INFO_DELETE: &'static str = "/open-apis/performance/v1/review_config/semester_activity/additional_information/delete";
    /// / 评估模板查询
    pub const PERFORMANCE_USER_GROUP_WRITE: &'static str = "/open-apis/performance/v1/review_config/semester_activity/user_group_user_rel/write";
    /// / 评估项目查询
    pub const PERFORMANCE_REVIEW_TEMPLATES_QUERY: &'static str = "/open-apis/performance/v1/review_config/review_templates/query";
    /// / 标签问题查询
    pub const PERFORMANCE_REVIEW_ITEMS_QUERY: &'static str = "/open-apis/performance/v1/review_config/review_items/query";
    /// / 指标查询
    pub const PERFORMANCE_TAG_QUESTIONS_QUERY: &'static str = "/open-apis/performance/v1/review_config/tag_questions/query";
    /// / 指标模板查询
    pub const PERFORMANCE_METRICS_QUERY: &'static str = "/open-apis/performance/v1/review_config/metrics/query";
    /// / 指标字段查询
    pub const PERFORMANCE_METRIC_TEMPLATES_QUERY: &'static str = "/open-apis/performance/v1/review_config/metric_templates/query";
    /// / 指标标签管理
    pub const PERFORMANCE_METRIC_FIELDS_QUERY: &'static str = "/open-apis/performance/v1/review_config/metric_fields/query";
    /// / 被评估人查询
    pub const PERFORMANCE_METRIC_TAGS: &'static str = "/open-apis/performance/v1/review_config/metric_tags";
    /// ==================== Personal Settings 个人设置相关端点 ====================
    pub const PERFORMANCE_REVIEWEES_QUERY: &'static str = "/open-apis/performance/v1/review_config/semester_activity/reviewees/query";
    /// / 系统状态操作
    pub const PERSONAL_SETTINGS_V1_SYSTEM_STATUSES: &'static str = "/open-apis/personal_settings/v1/system_statuses";
    /// / 批量开启系统状态
    pub const PERSONAL_SETTINGS_V1_SYSTEM_STATUS_OPERATION: &'static str = "/open-apis/personal_settings/v1/system_statuses/{system_status_id}";
    /// / 批量关闭系统状态
    pub const PERSONAL_SETTINGS_V1_SYSTEM_STATUS_BATCH_OPEN: &'static str = "/open-apis/personal_settings/v1/system_statuses/batch_open";
    /// ==================== CoreHR 人力资源管理相关端点 ====================
    pub const PERSONAL_SETTINGS_V1_SYSTEM_STATUS_BATCH_CLOSE: &'static str = "/open-apis/personal_settings/v1/system_statuses/batch_close";
    /// / 部门管理
    pub const COREHR_COMPANIES: &'static str = "/open-apis/corehr/v1/companies";
    /// / 部门批量获取
    pub const COREHR_DEPARTMENTS: &'static str = "/open-apis/corehr/v1/departments";
    /// / 部门树结构
    pub const COREHR_DEPARTMENTS_BATCH_GET: &'static str = "/open-apis/corehr/v1/departments/batch_get";
    /// / 员工批量获取
    pub const COREHR_DEPARTMENTS_TREE: &'static str = "/open-apis/corehr/v1/departments/tree";
    /// / 员工搜索
    pub const COREHR_EMPLOYEES_BATCH_GET: &'static str = "/open-apis/corehr/v1/employees/batch_get";
    /// / 职位管理
    pub const COREHR_EMPLOYEES_SEARCH: &'static str = "/open-apis/corehr/v1/employees/search";
    /// / 职位族群管理
    pub const COREHR_JOBS: &'static str = "/open-apis/corehr/v1/jobs";
    /// / 职级管理
    pub const COREHR_JOB_FAMILIES: &'static str = "/open-apis/corehr/v1/job_families";
    /// / 职等管理
    pub const COREHR_JOB_LEVELS: &'static str = "/open-apis/corehr/v1/job_levels";
    /// / 职等查询
    pub const COREHR_JOB_GRADES: &'static str = "/open-apis/corehr/v1/job_grades";
    /// / 异动记录管理
    pub const COREHR_JOB_GRADES_QUERY: &'static str = "/open-apis/corehr/v1/job_grades/query";
    /// / 异动记录搜索
    pub const COREHR_JOB_CHANGES: &'static str = "/open-apis/corehr/v1/job_changes";
    /// / 待入职管理
    pub const COREHR_JOB_CHANGES_SEARCH: &'static str = "/open-apis/corehr/v1/job_changes/search";
    /// / 待入职搜索
    pub const COREHR_PRE_HIRES: &'static str = "/open-apis/corehr/v1/pre_hires";
    /// / 离职管理
    pub const COREHR_PRE_HIRES_SEARCH: &'static str = "/open-apis/corehr/v1/pre_hires/search";
    /// / 离职搜索
    pub const COREHR_OFFBOARDINGS: &'static str = "/open-apis/corehr/v1/offboardings";
    /// / 基础信息枚举搜索
    pub const COREHR_OFFBOARDINGS_SEARCH: &'static str = "/open-apis/corehr/v1/offboardings/search";
    /// / 地理位置数据搜索
    pub const COREHR_BASIC_INFO_ENUM_SEARCH: &'static str = "/open-apis/corehr/v1/basic_info/enum/search";
    /// / 国籍搜索
    pub const COREHR_BASIC_INFO_LOCATION_SEARCH: &'static str = "/open-apis/corehr/v1/basic_info/location_data/search";
    /// / 通用数据ID转换
    pub const COREHR_BASIC_INFO_NATIONALITY_SEARCH: &'static str = "/open-apis/corehr/v1/basic_info/nationality/search";
    /// ==================== Contact v3 通讯录相关端点 ====================
    pub const COREHR_COMMON_DATA_ID_CONVERT: &'static str = "/open-apis/corehr/v1/common_data_id/convert";
    /// / 用户批量获取
    pub const CONTACT_V3_USERS: &'static str = "/open-apis/contact/v3/users";
    /// / 用户批量获取ID
    pub const CONTACT_V3_USERS_BATCH: &'static str = "/open-apis/contact/v3/users/batch";
    /// / 根据部门查找用户
    pub const CONTACT_V3_USERS_BATCH_GET_ID: &'static str = "/open-apis/contact/v3/users/batch_get_id";
    /// / 用户搜索
    pub const CONTACT_V3_USERS_FIND_BY_DEPARTMENT: &'static str = "/open-apis/contact/v3/users/find_by_department";
    /// / 部门管理
    pub const CONTACT_V3_USERS_SEARCH: &'static str = "/open-apis/contact/v3/users/search";
    /// / 部门批量获取
    pub const CONTACT_V3_DEPARTMENTS: &'static str = "/open-apis/contact/v3/departments";
    /// / 部门子部门查询
    pub const CONTACT_V3_DEPARTMENTS_BATCH: &'static str = "/open-apis/contact/v3/departments/batch";
    /// / 部门父部门查询
    pub const CONTACT_V3_DEPARTMENTS_CHILDREN: &'static str = "/open-apis/contact/v3/departments/children";
    /// / 部门搜索
    pub const CONTACT_V3_DEPARTMENTS_PARENT: &'static str = "/open-apis/contact/v3/departments/parent";
    /// / 职务管理
    pub const CONTACT_V3_DEPARTMENTS_SEARCH: &'static str = "/open-apis/contact/v3/departments/search";
    /// / 职位族群管理
    pub const CONTACT_V3_JOB_TITLES: &'static str = "/open-apis/contact/v3/job_titles";
    /// / 职级管理
    pub const CONTACT_V3_JOB_FAMILIES: &'static str = "/open-apis/contact/v3/job_families";
    /// / 功能角色管理
    pub const CONTACT_V3_JOB_LEVELS: &'static str = "/open-apis/contact/v3/job_levels";
    /// / 用户管理 - 带参数的端点
    pub const CONTACT_V3_FUNCTIONAL_ROLES: &'static str = "/open-apis/contact/v3/functional_roles";
    /// / 部门管理 - 带参数的端点
    pub const CONTACT_V3_USER_RESURRECT: &'static str = "/open-apis/contact/v3/users/{user_id}/resurrect";
    /// / 职务管理 - 带参数的端点
    pub const CONTACT_V3_DEPARTMENT_UPDATE_ID: &'static str = "/open-apis/contact/v3/departments/{department_id}/update_department_id";
    /// / 职位族群管理 - 带参数的端点
    pub const CONTACT_V3_JOB_TITLE_GET: &'static str = "/open-apis/contact/v3/job_titles/{job_title_id}";
    /// / 职级管理 - 带参数的端点
    pub const CONTACT_V3_JOB_FAMILY_GET: &'static str = "/open-apis/contact/v3/job_families/{job_family_id}";
    /// / 功能角色管理 - 带参数的端点
    pub const CONTACT_V3_JOB_LEVEL_GET: &'static str = "/open-apis/contact/v3/job_levels/{job_level_id}";
    /// / 群组管理
    pub const CONTACT_V3_FUNCTIONAL_ROLE_MEMBER_GET: &'static str = "/open-apis/contact/v3/functional_roles/{role_id}/members/{member_id}";
    /// / 群组成员管理
    pub const CONTACT_V3_GROUP_DETAIL: &'static str = "/open-apis/contact/v3/groups/{group_id}/detail";
    /// / 单位管理
    pub const CONTACT_V3_GROUP_MEMBERS_SIMPLELIST: &'static str = "/open-apis/contact/v3/groups/{group_id}/members/simplelist";
    /// / 工作城市管理
    pub const CONTACT_V3_UNIT_LIST_DEPARTMENT: &'static str = "/open-apis/contact/v3/units/{unit_id}/list_department";
    /// / 员工类型枚举管理
    pub const CONTACT_V3_WORK_CITY_GET: &'static str = "/open-apis/contact/v3/work_cities/{work_city_id}";
    /// / 自定义属性管理
    pub const CONTACT_V3_EMPLOYEE_TYPE_ENUM_GET: &'static str = "/open-apis/contact/v3/employee_type_enums/{enum_id}";
    /// / 权限范围管理
    pub const CONTACT_V3_CUSTOM_ATTRS: &'static str = "/open-apis/contact/v3/custom_attrs";
    /// ==================== 考勤服务端点 ====================
    pub const CONTACT_V3_SCOPES: &'static str = "/open-apis/contact/v3/scopes";
    /// / 工单列表
    pub const HELPDESK_V1_START_SERVICE: &'static str = "/open-apis/helpdesk/v1/start_service";
    /// / 工单详情
    pub const HELPDESK_V1_TICKETS: &'static str = "/open-apis/helpdesk/v1/tickets";
    /// / 工单更新
    pub const HELPDESK_V1_TICKET_GET: &'static str = "/open-apis/helpdesk/v1/tickets/{ticket_id}";
    /// 工单消息管理
    pub const HELPDESK_V1_TICKET_UPDATE: &'static str = "/open-apis/helpdesk/v1/tickets/{ticket_id}";
    /// / FAQ详情
    pub const HELPDESK_V1_FAQS: &'static str = "/open-apis/helpdesk/v1/faqs";
    /// / FAQ创建
    pub const HELPDESK_V1_FAQ_GET: &'static str = "/open-apis/helpdesk/v1/faqs/{faq_id}";
    /// / FAQ更新
    pub const HELPDESK_V1_FAQ_CREATE: &'static str = "/open-apis/helpdesk/v1/faqs";
    /// / FAQ删除
    pub const HELPDESK_V1_FAQ_UPDATE: &'static str = "/open-apis/helpdesk/v1/faqs/{faq_id}";
    /// / FAQ图片
    pub const HELPDESK_V1_FAQ_DELETE: &'static str = "/open-apis/helpdesk/v1/faqs/{faq_id}";
    /// / FAQ搜索
    pub const HELPDESK_V1_FAQ_IMAGE: &'static str = "/open-apis/helpdesk/v1/faqs/{faq_id}/image/{image_key}";
    /// 事件管理
    pub const HELPDESK_V1_FAQS_SEARCH: &'static str = "/open-apis/helpdesk/v1/faqs/search";
    /// / 事件取消订阅
    pub const HELPDESK_V1_EVENTS_SUBSCRIBE: &'static str = "/open-apis/helpdesk/v1/events/subscribe";
    /// 自定义字段管理
    pub const HELPDESK_V1_EVENTS_UNSUBSCRIBE: &'static str = "/open-apis/helpdesk/v1/events/unsubscribe";
    /// / 工单自定义字段详情
    pub const HELPDESK_V1_TICKET_CUSTOMIZED_FIELDS: &'static str = "/open-apis/helpdesk/v1/ticket_customized_fields";
    /// / 工单自定义字段创建
    pub const HELPDESK_V1_TICKET_CUSTOMIZED_FIELD_GET: &'static str = "/open-apis/helpdesk/v1/ticket_customized_fields/{field_id}";
    /// / 工单自定义字段更新
    pub const HELPDESK_V1_TICKET_CUSTOMIZED_FIELD_CREATE: &'static str = "/open-apis/helpdesk/v1/ticket_customized_fields";
    /// / 工单自定义字段删除
    pub const HELPDESK_V1_TICKET_CUSTOMIZED_FIELD_UPDATE: &'static str = "/open-apis/helpdesk/v1/ticket_customized_fields/{field_id}";
    /// 客服管理
    pub const HELPDESK_V1_TICKET_CUSTOMIZED_FIELD_DELETE: &'static str = "/open-apis/helpdesk/v1/ticket_customized_fields/{field_id}";
    /// / 客服邮箱
    pub const HELPDESK_V1_AGENT_GET: &'static str = "/open-apis/helpdesk/v1/agents/{agent_id}";
    /// 客服排班管理
    pub const HELPDESK_V1_AGENT_EMAIL: &'static str = "/open-apis/helpdesk/v1/agents/{agent_id}/agent_email";
    /// / 客服排班详情
    pub const HELPDESK_V1_AGENT_SCHEDULES: &'static str = "/open-apis/helpdesk/v1/agents/{agent_id}/agent_schedules";
    /// / 客服排班创建
    pub const HELPDESK_V1_AGENT_SCHEDULE_GET: &'static str = "/open-apis/helpdesk/v1/agents/{agent_id}/agent_schedules/{schedule_id}";
    /// / 客服排班更新
    pub const HELPDESK_V1_AGENT_SCHEDULE_CREATE: &'static str = "/open-apis/helpdesk/v1/agents/{agent_id}/agent_schedules";
    /// / 客服排班删除
    pub const HELPDESK_V1_AGENT_SCHEDULE_UPDATE: &'static str = "/open-apis/helpdesk/v1/agents/{agent_id}/agent_schedules/{schedule_id}";
    /// 客服技能管理
    pub const HELPDESK_V1_AGENT_SCHEDULE_DELETE: &'static str = "/open-apis/helpdesk/v1/agents/{agent_id}/agent_schedules/{schedule_id}";
    /// / 客服技能详情
    pub const HELPDESK_V1_AGENT_SKILLS: &'static str = "/open-apis/helpdesk/v1/agent_skills";
    /// / 客服技能创建
    pub const HELPDESK_V1_AGENT_SKILL_GET: &'static str = "/open-apis/helpdesk/v1/agent_skills/{skill_id}";
    /// / 客服技能更新
    pub const HELPDESK_V1_AGENT_SKILL_CREATE: &'static str = "/open-apis/helpdesk/v1/agent_skills";
    /// / 客服技能删除
    pub const HELPDESK_V1_AGENT_SKILL_UPDATE: &'static str = "/open-apis/helpdesk/v1/agent_skills/{skill_id}";
    /// 客服技能规则管理
    pub const HELPDESK_V1_AGENT_SKILL_DELETE: &'static str = "/open-apis/helpdesk/v1/agent_skills/{skill_id}";
    /// / 客服技能规则操作选项
    pub const HELPDESK_V1_AGENT_SKILL_RULES: &'static str = "/open-apis/helpdesk/v1/agent_skill_rules";
    /// 分类管理
    pub const HELPDESK_V1_AGENT_SKILL_RULES_OPERATOR_OPTIONS: &'static str = "/open-apis/helpdesk/v1/agent_skill_rules/operator-options";
    /// / 分类详情
    pub const HELPDESK_V1_CATEGORIES: &'static str = "/open-apis/helpdesk/v1/categories";
    /// / 分类创建
    pub const HELPDESK_V1_CATEGORY_GET: &'static str = "/open-apis/helpdesk/v1/categories/{category_id}";
    /// / 分类更新
    pub const HELPDESK_V1_CATEGORY_CREATE: &'static str = "/open-apis/helpdesk/v1/categories";
    /// / 分类删除
    pub const HELPDESK_V1_CATEGORY_UPDATE: &'static str = "/open-apis/helpdesk/v1/categories/{category_id}";
    /// 通知管理
    pub const HELPDESK_V1_CATEGORY_DELETE: &'static str = "/open-apis/helpdesk/v1/categories/{category_id}";
    /// / 通知详情
    pub const HELPDESK_V1_NOTIFICATIONS: &'static str = "/open-apis/helpdesk/v1/notifications";
    /// / 通知预览
    pub const HELPDESK_V1_NOTIFICATION_GET: &'static str = "/open-apis/helpdesk/v1/notifications/{notification_id}";
    /// / 通知创建
    pub const HELPDESK_V1_NOTIFICATION_PREVIEW: &'static str = "/open-apis/helpdesk/v1/notifications/{notification_id}/preview";
    /// / 通知更新
    pub const HELPDESK_V1_NOTIFICATION_CREATE: &'static str = "/open-apis/helpdesk/v1/notifications";
    /// / 通知提交审核
    pub const HELPDESK_V1_NOTIFICATION_UPDATE: &'static str = "/open-apis/helpdesk/v1/notifications/{notification_id}";
    /// / 通知取消审核
    pub const HELPDESK_V1_NOTIFICATION_SUBMIT_APPROVE: &'static str = "/open-apis/helpdesk/v1/notifications/{notification_id}/submit_approve";
    /// / 通知执行推送
    pub const HELPDESK_V1_NOTIFICATION_CANCEL_APPROVE: &'static str = "/open-apis/helpdesk/v1/notifications/{notification_id}/cancel_approve";
    /// / 通知取消发送
    pub const HELPDESK_V1_NOTIFICATION_EXECUTE_SEND: &'static str = "/open-apis/helpdesk/v1/notifications/{notification_id}/execute_send";
    /// ==================== 应用管理服务端点 ====================
    pub const HELPDESK_V1_NOTIFICATION_CANCEL_SEND: &'static str = "/open-apis/helpdesk/v1/notifications/{notification_id}/cancel_send";
    /// / 应用转移所有者
    pub const APPLICATION_V6_APP_GET: &'static str = "/open-apis/application/v6/applications/{app_id}";
    /// / 应用协作者管理
    pub const APPLICATION_V6_APP_TRANSFER_OWNER: &'static str = "/open-apis/application/v6/applications/{app_id}/transfer_owner";
    /// / 应用协作者删除
    pub const APPLICATION_V6_APP_COLLABORATORS: &'static str = "/open-apis/application/v6/applications/{app_id}/collaborators";
    /// / 应用版本管理
    pub const APPLICATION_V6_APP_COLLABORATOR_DELETE: &'static str = "/open-apis/application/v6/applications/{app_id}/collaborators";
    /// / 应用版本详情
    pub const APPLICATION_V6_APP_VERSIONS: &'static str = "/open-apis/application/v6/applications/{app_id}/versions";
    /// / 待审核应用列表
    pub const APPLICATION_V6_APP_VERSION_GET: &'static str = "/open-apis/application/v6/applications/{app_id}/versions/{version_id}";
    /// / 应用审核
    pub const APPLICATION_V6_APPS_UNDERAUDITLIST: &'static str = "/open-apis/application/v6/applications/underauditlist";
    /// / 应用组管理
    pub const APPLICATION_V6_APP_AUDIT: &'static str = "/open-apis/application/v6/applications/{app_id}/audit";
    /// 应用使用情况
    pub const APPLICATION_V6_APP_GROUP: &'static str = "/open-apis/application/v6/applications/{app_id}/group";
    /// / 应用消息推送概览
    pub const APPLICATION_V6_APP_USAGE_OVERVIEW: &'static str = "/open-apis/application/v6/app_usage/{app_id}/overview";
    /// 应用权限范围
    pub const APPLICATION_V6_APP_USAGE_DEPARTMENT_OVERVIEW: &'static str = "/open-apis/application/v6/app_usage/{app_id}/department_overview";
    /// / 应用权限范围查询
    pub const APPLICATION_V6_APP_SCOPE_APPLY: &'static str = "/open-apis/application/v6/applications/{app_id}/scope/apply";
    /// 应用徽章管理
    pub const APPLICATION_V6_APP_SCOPE_GET: &'static str = "/open-apis/application/v6/applications/{app_id}/scope";
    /// 应用反馈管理
    pub const APPLICATION_V6_APP_BADGE_SET: &'static str = "/open-apis/application/v6/app_badge/{app_id}/users/{user_id}/set";
    /// / 应用反馈详情
    pub const APPLICATION_V6_APPLICATION_FEEDBACK: &'static str = "/open-apis/application/v6/application_feedback";
    /// 管理员应用管理
    pub const APPLICATION_V6_APPLICATION_FEEDBACK_GET: &'static str = "/open-apis/application/v6/application_feedback/{feedback_id}";
    /// 应用商店付费信息
    pub const APPLICATION_V6_APP_VERSION_CONTACTS_RANGE_SUGGEST: &'static str = "/open-apis/application/v6/applications/{app_id}/versions/{version_id}/contacts_range_suggest";
    /// / 应用商店定价计划列表
    pub const APPLICATION_V6_APPSTORE_PAID_INFO_CHECK: &'static str = "/open-apis/application/v6/appstore_paid_info/{app_id}/users/{user_id}/pricing_plans/{pricing_plan_id}/check";
    /// / 应用商店订单详情
    pub const APPLICATION_V6_APPSTORE_PAID_INFO_PRICING_PLANS: &'static str = "/open-apis/application/v6/appstore_paid_info/{app_id}/pricing_plans";
    /// ==================== CloudDocs 云文档相关端点 ====================
    pub const APPLICATION_V6_APPSTORE_PAID_INFO_ORDER_GET: &'static str = "/open-apis/application/v6/appstore_paid_info/{app_id}/orders/{order_id}";
    /// / 多维表格数据表管理
    pub const BITABLE_V1_APP_COPY: &'static str = "/open-apis/bitable/v1/apps/{app_token}/copy";
    /// / 多维表格记录管理
    pub const BITABLE_V1_TABLES_BATCH_DELETE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/batch_delete";
    /// / 多维表格字段管理
    pub const BITABLE_V1_RECORDS_SEARCH: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/search";
    /// / 多维表格视图管理
    pub const BITABLE_V1_FIELD_DELETE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields/{field_id}";
    /// / 多维表格仪表板管理
    pub const BITABLE_V1_VIEW_DELETE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/views/{view_id}";
    /// / 多维表格角色管理
    pub const BITABLE_V1_DASHBOARD_COPY: &'static str = "/open-apis/bitable/v1/apps/{app_token}/dashboards/{block_id}/copy";
    /// / 多维表格角色成员管理
    pub const BITABLE_V1_ROLE_DELETE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}";
    /// / 多维表格表单管理
    pub const BITABLE_V1_ROLE_MEMBERS_BATCH_DELETE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members/batch_delete";
    /// / 多维表格工作流管理
    pub const BITABLE_V1_FORM_QUESTION: &'static str = "/open-apis/bitable/v1/apps/{app_token}/forms/{form_id}/questions/{question_id}";
    /// Board 白板服务
    pub const BITABLE_V1_WORKFLOW_UPDATE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/workflows/{workflow_id}";
    /// Comments 评论服务
    pub const BOARD_V1_WHITEBOARD_NODES: &'static str = "/open-apis/board/v1/whiteboards/{}/nodes";
    /// / 评论回复管理
    pub const COMMENT_V1_COMMENTS_BATCH_QUERY: &'static str = "/open-apis/comment/v1/comments/batch_query";
    /// Docx 文档服务
    pub const COMMENT_V1_COMMENT_REPLY_DELETE: &'static str = "/open-apis/comment/v1/comments/{comment_id}/replies/{reply_id}";
    /// / 文档块管理
    pub const DOCX_V1_DOCUMENT_CONVERT: &'static str = "/open-apis/docx/v1/documents/{}/convert";
    /// / 群公告管理
    pub const DOCX_V1_DOCUMENT_BLOCKS_BATCH_DELETE: &'static str = "/open-apis/docx/v1/documents/{}/blocks/batch_delete";
    /// / 邮件组详情操作 (需要使用 EndpointBuilder::replace_param 替换 {mailgroup_id})
    pub const MAIL_V1_MAILGROUPS: &'static str = "/open-apis/mail/v1/mailgroups";
    /// 邮件组管理员
    pub const MAIL_V1_MAILGROUP: &'static str = "/open-apis/mail/v1/mailgroups/{mailgroup_id}";
    /// / 批量删除邮件组管理员 (需要使用 EndpointBuilder::replace_param 替换 {mailgroup_id})
    pub const MAIL_V1_MAILGROUP_MANAGERS_BATCH_CREATE: &'static str = "/open-apis/mail/v1/mailgroups/{mailgroup_id}/managers/batch_create";
    /// / 获取邮件组管理员列表 (需要使用 EndpointBuilder::replace_param 替换 {mailgroup_id})
    pub const MAIL_V1_MAILGROUP_MANAGERS_BATCH_DELETE: &'static str = "/open-apis/mail/v1/mailgroups/{mailgroup_id}/managers/batch_delete";
    /// 用户邮箱事件
    pub const MAIL_V1_MAILGROUP_MANAGERS: &'static str = "/open-apis/mail/v1/mailgroups/{mailgroup_id}/managers";
    /// / 获取用户邮箱事件订阅状态 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id})
    pub const MAIL_V1_USER_MAILBOX_EVENTS_SUBSCRIBE: &'static str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/events/subscribe";
    /// / 取消订阅用户邮箱事件 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id})
    pub const MAIL_V1_USER_MAILBOX_EVENTS_SUBSCRIPTION: &'static str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/events/subscription";
    /// 用户邮箱文件夹
    pub const MAIL_V1_USER_MAILBOX_EVENTS_UNSUBSCRIBE: &'static str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/events/unsubscribe";
    /// / 用户邮箱文件夹详情操作 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id} 和 {folder_id})
    pub const MAIL_V1_USER_MAILBOX_FOLDERS: &'static str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/folders";
    /// 用户邮箱消息
    pub const MAIL_V1_USER_MAILBOX_FOLDER: &'static str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/folders/{folder_id}";
    /// / 用户邮箱规则详情 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id} 和 {rule_id})
    pub const MAIL_V1_USER_MAILBOX_RULES: &'static str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/rules";
    /// / 重新排序用户邮箱规则 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id})
    pub const MAIL_V1_USER_MAILBOX_RULE: &'static str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/rules/{rule_id}";
    /// 用户邮箱联系人
    pub const MAIL_V1_USER_MAILBOX_RULES_REORDER: &'static str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/rules/reorder";
    /// / 用户邮箱联系人详情 (需要使用 EndpointBuilder::replace_param 替换 {user_mailbox_id} 和 {contact_id})
    pub const MAIL_V1_USER_MAILBOX_MAIL_CONTACTS: &'static str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/mail_contacts";
    /// 用户邮箱附件
    pub const MAIL_V1_USER_MAILBOX_MAIL_CONTACT: &'static str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/mail_contacts/{contact_id}";
    /// / 删除/更新部门 (需要使用 EndpointBuilder::replace_param 替换 {department_id})
    pub const DIRECTORY_V1_DEPARTMENTS: &'static str = "/open-apis/directory/v1/departments";
    /// / 搜索部门
    pub const DIRECTORY_V1_DEPARTMENT_GET: &'static str = "/open-apis/directory/v1/departments/{department_id}";
    /// / 筛选部门
    pub const DIRECTORY_V1_DEPARTMENTS_SEARCH: &'static str = "/open-apis/directory/v1/departments/search";
    /// / 批量获取部门
    pub const DIRECTORY_V1_DEPARTMENTS_FILTER: &'static str = "/open-apis/directory/v1/departments/filter";
    /// ===== 员工管理端点 =====
    pub const DIRECTORY_V1_DEPARTMENTS_MGET: &'static str = "/open-apis/directory/v1/departments/mget";
    /// / 删除/更新员工 (需要使用 EndpointBuilder::replace_param 替换 {employee_id})
    pub const DIRECTORY_V1_EMPLOYEES: &'static str = "/open-apis/directory/v1/employees";
    /// / 员工待离职 (需要使用 EndpointBuilder::replace_param 替换 {employee_id})
    pub const DIRECTORY_V1_EMPLOYEE_GET: &'static str = "/open-apis/directory/v1/employees/{employee_id}";
    /// / 员工转正 (需要使用 EndpointBuilder::replace_param 替换 {employee_id})
    pub const DIRECTORY_V1_EMPLOYEE_TO_BE_RESIGNED: &'static str = "/open-apis/directory/v1/employees/{employee_id}/to_be_resigned";
    /// / 员工复活 (需要使用 EndpointBuilder::replace_param 替换 {employee_id})
    pub const DIRECTORY_V1_EMPLOYEE_REGULAR: &'static str = "/open-apis/directory/v1/employees/{employee_id}/regular";
    /// / 搜索员工
    pub const DIRECTORY_V1_EMPLOYEE_RESURRECT: &'static str = "/open-apis/directory/v1/employees/{employee_id}/resurrect";
    /// / 筛选员工
    pub const DIRECTORY_V1_EMPLOYEES_SEARCH: &'static str = "/open-apis/directory/v1/employees/search";
    /// / 批量获取员工
    pub const DIRECTORY_V1_EMPLOYEES_FILTER: &'static str = "/open-apis/directory/v1/employees/filter";
    /// ==================== 搜索服务端点 ====================
    pub const DIRECTORY_V1_EMPLOYEES_MGET: &'static str = "/open-apis/directory/v1/employees/mget";
    /// / 创建数据项
    pub const SEARCH_V1_USER: &'static str = "/open-apis/search/v1/user";
    /// / 批量创建数据项
    pub const SEARCH_V2_DATA_ITEM_CREATE: &'static str = "/open-apis/search/v2/data_sources/{data_source_id}/items";
    /// / 删除/获取数据项
    pub const SEARCH_V2_DATA_ITEM_BATCH_CREATE: &'static str = "/open-apis/search/v2/data_sources/{data_source_id}/items/batch_create";
    /// / 搜索消息
    pub const SEARCH_V2_DATA_ITEM_OPERATION: &'static str = "/open-apis/search/v2/data_sources/{data_source_id}/items/{data_item_id}";
    /// / 搜索云文档对象
    pub const SEARCH_V2_APP: &'static str = "/open-apis/search/v2/app";
    /// / 数据源操作（创建/列表）
    pub const SUITE_DOCS_SEARCH_OBJECT: &'static str = "/open-apis/suite/docs-api/search/object";
    /// / 数据源操作（删除/更新/获取）
    pub const SEARCH_V2_DATA_SOURCES: &'static str = "/open-apis/search/v2/data_sources";
    /// / 创建Schema
    pub const SEARCH_V2_DATA_SOURCE_OPERATION: &'static str = "/open-apis/search/v2/data_sources/{data_source_id}";
    /// / Schema操作（删除/更新/获取）
    pub const SEARCH_V2_SCHEMA_CREATE: &'static str = "/open-apis/search/v2/data_sources/{data_source_id}/schemas";
    /// ==================== OKR服务端点 ====================
    pub const SEARCH_V2_SCHEMA_OPERATION: &'static str = "/open-apis/search/v2/data_sources/{data_source_id}/schemas/{schema_id}";
    /// / OKR列表
    pub const OKR_V1_REVIEWS_QUERY: &'static str = "/open-apis/okr/v1/reviews/query";
    /// / 批量获取OKR
    pub const OKR_V1_OKRS: &'static str = "/open-apis/okr/v1/okrs";
    /// / 周期规则列表
    pub const OKR_V1_OKRS_BATCH_GET: &'static str = "/open-apis/okr/v1/okrs/batch_get";
    /// / 周期列表
    pub const OKR_V1_PERIOD_RULES: &'static str = "/open-apis/okr/v1/period_rules";
    /// / 周期详情
    pub const OKR_V1_PERIODS: &'static str = "/open-apis/okr/v1/periods";
    /// / 进展记录列表
    pub const OKR_V1_PERIOD_GET: &'static str = "/open-apis/okr/v1/periods/{period_id}";
    /// / 进展记录操作（获取/更新/删除）
    pub const OKR_V1_PROGRESS_RECORDS: &'static str = "/open-apis/okr/v1/progress_records";
    /// / 进展记录上传
    pub const OKR_V1_PROGRESS_RECORD_OPERATION: &'static str = "/open-apis/okr/v1/progress_records/{progress_id}";
    /// ==================== Trust Party 可信第三方服务端点 ====================
    pub const OKR_V1_PROGRESS_RECORDS_UPLOAD: &'static str = "/open-apis/okr/v1/progress_records/upload";
    /// / 获取关联组织的部门和成员信息 (需要使用 EndpointBuilder::replace_param 替换 {org_id})
    pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATIONS: &'static str = "/open-apis/trust_party/v1/collaboration_organizations";
    /// / 获取关联组织详情 (需要使用 EndpointBuilder::replace_param 替换 {org_id})
    pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_VISIBLE: &'static str = "/open-apis/trust_party/v1/collaboration_organizations/{org_id}/visible_organization";
    /// / 获取关联组织成员详情 (需要使用 EndpointBuilder::replace_params 替换 {org_id} 和 {user_id})
    pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_GET: &'static str = "/open-apis/trust_party/v1/collaboration_organizations/{org_id}";
    /// / 获取关联组织部门详情 (需要使用 EndpointBuilder::replace_params 替换 {org_id} 和 {department_id})
    pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_USER_GET: &'static str = "/open-apis/trust_party/v1/collaboration_organizations/{org_id}/users/{user_id}";
    /// / 获取关联组织双方共享成员范围 (需要使用 EndpointBuilder::replace_param 替换 {org_id})
    pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_DEPARTMENT_GET: &'static str = "/open-apis/trust_party/v1/collaboration_organizations/{org_id}/departments/{department_id}";
    /// / 管理员获取所有关联组织列表
    pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_SHARED_MEMBER_SCOPES: &'static str = "/open-apis/trust_party/v1/collaboration_organizations/{org_id}/shared_member_scopes";
    /// === 可搜可见规则管理端点 ===
    pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATIONS_ADMIN: &'static str = "/open-apis/trust_party/v1/collaboration_organizations/admin";
    /// / 可搜可见规则操作（更新/删除） (需要使用 EndpointBuilder::replace_param 替换 {rule_id})
    pub const TRUST_PARTY_V1_SEARCHABLE_VISIBLE_RULES: &'static str = "/open-apis/trust_party/v1/searchable_and_visible_rules";
    /// ==================== Cardkit服务端点 ====================
    pub const TRUST_PARTY_V1_SEARCHABLE_VISIBLE_RULE_OPERATION: &'static str = "/open-apis/trust_party/v1/searchable_and_visible_rules/{rule_id}";
    /// / 创建卡片元素
    pub const CARDKIT_V1_CARDS: &'static str = "/open-apis/cardkit/v1/cards";
    /// / 更新卡片元素
    pub const CARDKIT_V1_CARD_ELEMENTS: &'static str = "/open-apis/cardkit/v1/cards/{card_id}/elements";
    /// / 更新组件属性
    pub const CARDKIT_V1_CARD_ELEMENTS_UPDATE: &'static str = "/open-apis/cardkit/v1/cards/{card_id}/elements/{element_id}";
    /// / 删除卡片元素
    pub const CARDKIT_V1_CARD_ELEMENTS_PATCH: &'static str = "/open-apis/cardkit/v1/cards/{card_id}/elements/{element_id}";
    /// / 卡片设置
    pub const CARDKIT_V1_CARD_ELEMENTS_DELETE: &'static str = "/open-apis/cardkit/v1/cards/{card_id}/elements/{element_id}";
    /// / 更新卡片
    pub const CARDKIT_V1_CARD_SETTINGS: &'static str = "/open-apis/cardkit/v1/cards/{card_id}/settings";
    /// / 批量更新卡片
    pub const CARDKIT_V1_CARD_UPDATE: &'static str = "/open-apis/cardkit/v1/cards/{card_id}";
    /// ==================== Aily服务端点 ====================
    pub const CARDKIT_V1_CARD_BATCH_UPDATE: &'static str = "/open-apis/cardkit/v1/cards/{card_id}/batch_update";
    /// / 会话操作（获取/更新/删除）
    pub const AILY_V1_SESSIONS: &'static str = "/open-apis/aily/v1/sessions";
    /// / 运行列表/创建
    pub const AILY_V1_SESSION_OPERATION: &'static str = "/open-apis/aily/v1/sessions/{session_id}";
    /// / 运行操作（获取）
    pub const AILY_V1_RUNS: &'static str = "/open-apis/aily/v1/sessions/{session_id}/runs";
    /// / 取消运行
    pub const AILY_V1_RUN_GET: &'static str = "/open-apis/aily/v1/sessions/{session_id}/runs/{run_id}";
    /// / 知识库问答
    pub const AILY_V1_RUN_CANCEL: &'static str = "/open-apis/aily/v1/sessions/{session_id}/runs/{run_id}/cancel";
    /// / 知识库文件上传
    pub const AILY_V1_DATA_KNOWLEDGE_ASK: &'static str = "/open-apis/aily/v1/data_knowledge/ask";
    /// / 知识库操作（创建/列表）
    pub const AILY_V1_DATA_KNOWLEDGE_UPLOAD_FILE: &'static str = "/open-apis/aily/v1/data_knowledge/upload_file";
    /// / 知识库操作（获取/删除）
    pub const AILY_V1_DATA_KNOWLEDGE: &'static str = "/open-apis/aily/v1/data_knowledge";
    /// / 知识库分类
    pub const AILY_V1_DATA_KNOWLEDGE_OPERATION: &'static str = "/open-apis/aily/v1/data_knowledge/{knowledge_id}";
    /// / 消息操作（创建/列表）
    pub const AILY_V1_DATA_KNOWLEDGE_CATEGORIES: &'static str = "/open-apis/aily/v1/data_knowledge/categories";
    /// / 技能获取
    pub const AILY_V1_SKILL_START: &'static str = "/open-apis/aily/v1/skills/{skill_id}/start";
    /// / 技能列表
    pub const AILY_V1_SKILL_GET: &'static str = "/open-apis/aily/v1/skills/{skill_id}";
    /// ==================== ACS服务端点 ====================
    pub const AILY_V1_SKILLS: &'static str = "/open-apis/aily/v1/skills";
    /// / 访客操作（创建/列表）
    pub const ACS_V1_DEVICES: &'static str = "/open-apis/acs/v1/devices";
    /// / 访客获取
    pub const ACS_V1_VISITORS: &'static str = "/open-apis/acs/v1/visitors";
    /// / 门禁记录列表
    pub const ACS_V1_VISITOR_GET: &'static str = "/open-apis/acs/v1/visitors/{visitor_id}";
    /// / 门禁记录人脸图像
    pub const ACS_V1_ACCESS_RECORDS: &'static str = "/open-apis/acs/v1/access_records";
    /// / 外部规则操作（创建/列表）
    pub const ACS_V1_ACCESS_RECORD_FACE_IMAGE: &'static str = "/open-apis/acs/v1/access_records/{record_id}/face_image";
    /// / 外部规则操作（获取/删除）
    pub const ACS_V1_RULE_EXTERNAL: &'static str = "/open-apis/acs/v1/rule_external";
    /// / 外部规则设备绑定
    pub const ACS_V1_RULE_EXTERNAL_OPERATION: &'static str = "/open-apis/acs/v1/rule_external/{rule_id}";
    /// / 用户操作（获取/删除）
    pub const ACS_V1_RULE_EXTERNAL_DEVICE_BIND: &'static str = "/open-apis/acs/v1/rule_external/device_bind";
    /// / 用户列表
    pub const ACS_V1_USER_OPERATION: &'static str = "/open-apis/acs/v1/users/{user_id}";
    /// / 用户人脸图像（获取/上传）
    pub const ACS_V1_USERS: &'static str = "/open-apis/acs/v1/users";
    /// ==================== 管理后台服务端点 ====================
    pub const ACS_V1_USER_FACE_IMAGE: &'static str = "/open-apis/acs/v1/users/{user_id}/face_image";
    /// / 获取妙记统计数据 (需要使用 EndpointBuilder::replace_param 替换 {minute_token})
    pub const MINUTES_V1_MINUTE_GET: &'static str = "/open-apis/minutes/v1/{minute_token}";
    /// / 导出妙记文字记录 (需要使用 EndpointBuilder::replace_param 替换 {minute_token})
    pub const MINUTES_V1_STATISTICS_GET: &'static str = "/open-apis/minutes/v1/{minute_token}/statistics";
    /// / 下载妙记音视频文件 (需要使用 EndpointBuilder::replace_param 替换 {minute_token})
    pub const MINUTES_V1_TRANSCRIPT_GET: &'static str = "/open-apis/minutes/v1/{minute_token}/transcript";
    /// ==================== 实名认证服务端点 ====================
    pub const MINUTES_V1_MEDIA_GET: &'static str = "/open-apis/minutes/v1/{minute_token}/media";
    /// / 上传人脸基准图片
    pub const HUMAN_AUTHENTICATION_V1_IDENTITIES: &'static str = "/open-apis/human_authentication/v1/identities";
    /// / 裁剪人脸图片
    pub const HUMAN_AUTHENTICATION_V1_FACE_IMAGES: &'static str = "/open-apis/human_authentication/v1/face_images";
    /// / 查询人脸认证结果 (需要使用 EndpointBuilder::replace_param 替换 {identity_id})
    pub const HUMAN_AUTHENTICATION_V1_FACE_IMAGES_CROP: &'static str = "/open-apis/human_authentication/v1/face_images/crop";
    /// ==================== MDM设备管理服务端点 ====================
    pub const HUMAN_AUTHENTICATION_V1_IDENTITY_RESULT: &'static str = "/open-apis/human_authentication/v1/identities/{identity_id}/result";
    /// / 分页查询国家/地区
    pub const MDM_V1_COUNTRY_REGIONS_BATCH_GET: &'static str = "/open-apis/mdm/v1/country_regions/batch_get";
    /// / 用户数据维度绑定
    pub const MDM_V1_COUNTRY_REGIONS: &'static str = "/open-apis/mdm/v1/country_regions";
    /// / OpenAPI 审计日志数据列表
    pub const SECURITY_AND_COMPLIANCE_V1_AUDIT_DATAS: &'static str = "/open-apis/security_and_compliance/v1/audit_datas";
    /// ==================== Report 汇报相关端点 ====================
    pub const SECURITY_AND_COMPLIANCE_V1_OPENAPI_LOGS_LIST_DATA: &'static str = "/open-apis/security_and_compliance/v1/openapi_logs/list_data";
    /// / 规则查询
    pub const REPORT_V1_TASKS_QUERY: &'static str = "/open-apis/report/v1/tasks/query";
    /// / 规则看板操作
    pub const REPORT_V1_RULES_QUERY: &'static str = "/open-apis/report/v1/rules/query";
    /// ==================== Authentication 用户认证相关端点 ====================
    pub const REPORT_V1_RULE_VIEWS_OPERATION: &'static str = "/open-apis/report/v1/rule-views/{view_id}";
}
