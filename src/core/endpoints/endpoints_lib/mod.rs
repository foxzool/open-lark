//! API端点常量定义模块 - 统一架构
//!
//! 本模块提供飞书开放平台API端点的统一、分层架构，旨在：
//! 1. 消除代码重复 - 统一管理所有API端点常量
//! 2. 提高可维护性 - 按业务功能分层组织
//! 3. 保持向后兼容 - 渐进式迁移策略
//! 4. 改善开发体验 - 清晰的命名和分类
//!
//! # 架构设计
//!
//! - **核心层**: `Endpoints` 结构体提供主要常量访问
//! - **分类层**: 按业务功能分组的端点集合
//! - **兼容层**: 保留原有接口以确保平滑迁移
//!
//! # 使用示例
//!
//! ```rust
//! use open_lark::core::endpoints::Endpoints;
//!
//! // 直接访问
//! let endpoint = Endpoints::IM_MESSAGE_SEND;
//!
//! // 分类访问
//! let messaging = Endpoints::messaging();
//! let endpoint = messaging.send_message;
//! ```

/// 飞书API端点统一入口
///
/// 提供对所有飞书开放平台API端点的类型安全访问。
/// 所有端点按业务功能分组，便于查找和维护。
pub struct Endpoints;

/// 即时消息相关端点
pub struct Messaging;

/// 内容管理相关端点（云盘、文件、文档）
pub struct Content;

/// 人力资源管理相关端点
pub struct HrManagement;

/// 协作工具相关端点（日历、审批、任务）
pub struct Collaboration;

/// AI服务相关端点
pub struct AiServices;

/// 管理和认证相关端点
pub struct Admin;

/// 集成和外部系统相关端点
pub struct Integration;

impl Endpoints {
    // ==================== 即时消息相关端点 ====================

    /// 发送消息
    pub const IM_V1_SEND_MESSAGE: &'static str = "/open-apis/im/v1/messages";

    /// 获取消息详情
    pub const IM_V1_GET_MESSAGE: &'static str = "/open-apis/im/v1/messages/{message_id}";

    /// 更新消息内容
    pub const IM_V1_UPDATE_MESSAGE: &'static str = "/open-apis/im/v1/messages/{message_id}";

    /// 删除消息
    pub const IM_V1_DELETE_MESSAGE: &'static str = "/open-apis/im/v1/messages/{message_id}";

    /// 获取消息已读用户
    pub const IM_V1_READ_MESSAGE: &'static str = "/open-apis/im/v1/messages/{message_id}/read_users";

    /// 获取消息列表
    pub const IM_V1_LIST_MESSAGE: &'static str = "/open-apis/im/v1/messages";

    /// 回复消息
    pub const IM_V1_REPLY_MESSAGE: &'static str = "/open-apis/im/v1/messages/{message_id}/reply";

    /// 消息表情回复
    pub const IM_V1_MESSAGE_REACTIONS: &'static str = "/open-apis/im/v1/messages/{message_id}/reactions";

    /// 删除消息表情回复
    pub const IM_V1_DELETE_MESSAGE_REACTION: &'static str =
        "/open-apis/im/v1/messages/{message_id}/reactions/{reaction_id}";

    /// 批量发送消息
    pub const IM_V1_BATCH_MESSAGES: &'static str = "/open-apis/im/v1/batch_messages";

    /// 创建群聊
    pub const IM_CHAT_CREATE: &'static str = "/open-apis/im/v1/chats";

    /// 获取会话信息
    pub const IM_CHAT_GET: &'static str = "/open-apis/im/v1/chats/{chat_id}";

    /// 更新会话信息
    pub const IM_CHAT_UPDATE: &'static str = "/open-apis/im/v1/chats/{chat_id}";

    /// 解散群聊
    pub const IM_CHAT_DELETE: &'static str = "/open-apis/im/v1/chats/{chat_id}";

    /// 获取群成员列表
    pub const IM_CHAT_MEMBERS: &'static str = "/open-apis/im/v1/chats/{chat_id}/members";

    /// Pin消息
    pub const IM_V1_PINS: &'static str = "/open-apis/im/v1/pins";

    /// 上传文件
    pub const IM_V1_FILES: &'static str = "/open-apis/im/v1/files";

    /// 下载文件
    pub const IM_V1_DOWNLOAD_FILE: &'static str = "/open-apis/im/v1/files/{file_key}";

    /// 上传图片
    pub const IM_V1_IMAGES: &'static str = "/open-apis/im/v1/images";

    /// 下载图片
    pub const IM_V1_DOWNLOAD_IMAGE: &'static str = "/open-apis/im/v1/images/{image_key}";

    // ==================== 内容管理相关端点 ====================
    // 注意：内容管理相关常量已移至 Content 结构体中

    /// 即时消息服务端点
    pub const fn messaging() -> Messaging {
        Messaging
    }

    /// 内容管理服务端点
    pub const fn content() -> Content {
        Content
    }

    /// 人力资源管理端点
    pub const fn hr_management() -> HrManagement {
        HrManagement
    }

    /// 协作工具端点
    pub const fn collaboration() -> Collaboration {
        Collaboration
    }

    /// AI服务端点
    pub const fn ai_services() -> AiServices {
        AiServices
    }

    /// 管理和认证端点
    pub const fn admin() -> Admin {
        Admin
    }

    /// 集成和外部系统端点
    pub const fn integration() -> Integration {
        Integration
    }
}

    // ==================== 兼容性别名 ====================

    pub const ACS_V1_ACCESS_RECORDS: &'static str = "/open-apis/acs/v1/access_records";

    pub const ACS_V1_ACCESS_RECORD_FACE_IMAGE: &'static str = "/open-apis/acs/v1/access_records/{record_id}/face_image";

    pub const ACS_V1_DEVICES: &'static str = "/open-apis/acs/v1/devices";

    pub const ACS_V1_RULE_EXTERNAL: &'static str = "/open-apis/acs/v1/rule_external";

    pub const ACS_V1_RULE_EXTERNAL_DEVICE_BIND: &'static str = "/open-apis/acs/v1/rule_external/device_bind";

    pub const ACS_V1_RULE_EXTERNAL_OPERATION: &'static str = "/open-apis/acs/v1/rule_external/{rule_id}";

    pub const ACS_V1_USERS: &'static str = "/open-apis/acs/v1/users";

    pub const ACS_V1_USER_FACE_IMAGE: &'static str = "/open-apis/acs/v1/users/{user_id}/face_image";

    pub const ACS_V1_USER_OPERATION: &'static str = "/open-apis/acs/v1/users/{user_id}";

    pub const ACS_V1_VISITORS: &'static str = "/open-apis/acs/v1/visitors";

    pub const ACS_V1_VISITOR_GET: &'static str = "/open-apis/acs/v1/visitors/{visitor_id}";

    pub const ADMIN_V1_BADGES_CREATE: &'static str = "/open-apis/admin/v1/badges";

    pub const ADMIN_V1_BADGES_IMAGE_UPLOAD: &'static str = "/open-apis/admin/v1/badges/image";

    pub const ADMIN_V1_BADGES_LIST: &'static str = "/open-apis/admin/v1/badges";

    pub const ADMIN_V1_BADGES_OPERATION: &'static str = "/open-apis/admin/v1/badges/{badge_id}";

    pub const ADMIN_V1_BADGE_GRANTS_CREATE: &'static str = "/open-apis/admin/v1/badge_grants";

    pub const ADMIN_V1_BADGE_GRANTS_LIST: &'static str = "/open-apis/admin/v1/badge_grants";

    pub const ADMIN_V1_BADGE_GRANTS_OPERATION: &'static str = "/open-apis/admin/v1/badge_grants/{grant_id}";

    pub const ADMIN_V1_DATA_REPORT_DEPARTMENT: &'static str = "/open-apis/admin/v1/data_report/department";

    pub const ADMIN_V1_DATA_REPORT_USER: &'static str = "/open-apis/admin/v1/data_report/user";

    pub const ADMIN_V1_PASSWORD_RESET: &'static str = "/open-apis/admin/v1/password/reset";

    pub const AILY_V1_DATA_KNOWLEDGE: &'static str = "/open-apis/aily/v1/data_knowledge";

    pub const AILY_V1_DATA_KNOWLEDGE_ASK: &'static str = "/open-apis/aily/v1/data_knowledge/ask";

    pub const AILY_V1_DATA_KNOWLEDGE_CATEGORIES: &'static str = "/open-apis/aily/v1/data_knowledge/categories";

    pub const AILY_V1_DATA_KNOWLEDGE_OPERATION: &'static str = "/open-apis/aily/v1/data_knowledge/{knowledge_id}";

    pub const AILY_V1_DATA_KNOWLEDGE_UPLOAD_FILE: &'static str = "/open-apis/aily/v1/data_knowledge/upload_file";

    pub const AILY_V1_MESSAGES: &'static str = "/open-apis/aily/v1/sessions/{session_id}/messages";

    pub const AILY_V1_MESSAGE_GET: &'static str = "/open-apis/aily/v1/sessions/{session_id}/messages/{message_id}";

    pub const AILY_V1_RUNS: &'static str = "/open-apis/aily/v1/sessions/{session_id}/runs";

    pub const AILY_V1_RUN_CANCEL: &'static str = "/open-apis/aily/v1/sessions/{session_id}/runs/{run_id}/cancel";

    pub const AILY_V1_RUN_GET: &'static str = "/open-apis/aily/v1/sessions/{session_id}/runs/{run_id}";

    pub const AILY_V1_SESSIONS: &'static str = "/open-apis/aily/v1/sessions";

    pub const AILY_V1_SESSION_OPERATION: &'static str = "/open-apis/aily/v1/sessions/{session_id}";

    pub const AILY_V1_SKILLS: &'static str = "/open-apis/aily/v1/skills";

    pub const AILY_V1_SKILL_GET: &'static str = "/open-apis/aily/v1/skills/{skill_id}";

    pub const AILY_V1_SKILL_START: &'static str = "/open-apis/aily/v1/skills/{skill_id}/start";

    pub const APAAS_V1_APPS: &'static str = "/open-apis/apaas/v1/apps";

    pub const APASS_V1_AUDIT_LOG_AUDIT_EVENTS: &'static str = "/open-apis/apaas/v1/application/{app_id}/audit_log/audit_events";

    pub const APASS_V1_AUDIT_LOG_DATA_CHANGE_LOGS: &'static str = "/open-apis/apaas/v1/application/{app_id}/audit_log/data_change_logs";

    pub const APASS_V1_AUDIT_LOG_DATA_CHANGE_LOG_GET: &'static str = "/open-apis/apaas/v1/application/{app_id}/audit_log/data_change_log/{log_id}";

    pub const APASS_V1_AUDIT_LOG_GET: &'static str = "/open-apis/apaas/v1/application/{app_id}/audit_log/{log_id}";

    pub const APASS_V1_AUDIT_LOG_LIST: &'static str = "/open-apis/apaas/v1/application/{app_id}/audit_log/list";

    pub const APASS_V1_ENVIRONMENT_VARIABLE_GET: &'static str = "/open-apis/apaas/v1/application/{app_id}/environment_variable/{variable_name}";

    pub const APASS_V1_ENVIRONMENT_VARIABLE_QUERY: &'static str = "/open-apis/apaas/v1/application/{app_id}/environment_variable/query";

    pub const APASS_V1_FLOW_EXECUTE: &'static str = "/open-apis/apaas/v1/application/{app_id}/flow/{flow_api_name}/execute";

    pub const APASS_V1_FLOW_USER_TASK_ADD_ASSIGNEE: &'static str = "/open-apis/apaas/v1/application/{app_id}/flow/user_task/{task_id}/add_assignee";

    pub const APASS_V1_FLOW_USER_TASK_AGREE: &'static str = "/open-apis/apaas/v1/application/{app_id}/flow/user_task/{task_id}/agree";

    pub const APASS_V1_FLOW_USER_TASK_CANCEL: &'static str = "/open-apis/apaas/v1/application/{app_id}/flow/user_task/{task_id}/cancel";

    pub const APASS_V1_FLOW_USER_TASK_CC: &'static str = "/open-apis/apaas/v1/application/{app_id}/flow/user_task/{task_id}/cc";

    pub const APASS_V1_FLOW_USER_TASK_CHAT_GROUP: &'static str = "/open-apis/apaas/v1/application/{app_id}/flow/user_task/{task_id}/chat_group";

    pub const APASS_V1_FLOW_USER_TASK_EXPEDITING: &'static str = "/open-apis/apaas/v1/application/{app_id}/flow/user_task/{task_id}/expediting";

    pub const APASS_V1_FLOW_USER_TASK_QUERY: &'static str = "/open-apis/apaas/v1/application/{app_id}/flow/user_task/query";

    pub const APASS_V1_FLOW_USER_TASK_REJECT: &'static str = "/open-apis/apaas/v1/application/{app_id}/flow/user_task/{task_id}/reject";

    pub const APASS_V1_FLOW_USER_TASK_ROLLBACK: &'static str = "/open-apis/apaas/v1/application/{app_id}/flow/user_task/{task_id}/rollback";

    pub const APASS_V1_FLOW_USER_TASK_ROLLBACK_POINTS: &'static str = "/open-apis/apaas/v1/application/{app_id}/flow/user_task/{task_id}/rollback_points";

    pub const APASS_V1_FLOW_USER_TASK_TRANSFER: &'static str = "/open-apis/apaas/v1/application/{app_id}/flow/user_task/{task_id}/transfer";

    pub const APASS_V1_FUNCTION_INVOKE: &'static str = "/open-apis/apaas/v1/application/{app_id}/function/{function_name}/invoke";

    pub const APASS_V1_OBJECT_OQL: &'static str = "/open-apis/apaas/v1/application/{app_id}/object/oql";

    pub const APASS_V1_OBJECT_RECORD_BATCH_CREATE: &'static str = "/open-apis/apaas/v1/application/{app_id}/object/{object_api_name}/record/batch_create";

    pub const APASS_V1_OBJECT_RECORD_BATCH_DELETE: &'static str = "/open-apis/apaas/v1/application/{app_id}/object/{object_api_name}/record/batch_delete";

    pub const APASS_V1_OBJECT_RECORD_BATCH_QUERY: &'static str = "/open-apis/apaas/v1/application/{app_id}/object/{object_api_name}/record/batch_query";

    pub const APASS_V1_OBJECT_RECORD_BATCH_UPDATE: &'static str = "/open-apis/apaas/v1/application/{app_id}/object/{object_api_name}/record/batch_update";

    pub const APASS_V1_OBJECT_RECORD_CREATE: &'static str = "/open-apis/apaas/v1/application/{app_id}/object/{object_api_name}/record";

    pub const APASS_V1_OBJECT_RECORD_DELETE: &'static str = "/open-apis/apaas/v1/application/{app_id}/object/{object_api_name}/record/{record_id}";

    pub const APASS_V1_OBJECT_RECORD_GET: &'static str = "/open-apis/apaas/v1/application/{app_id}/object/{object_api_name}/record/{record_id}";

    pub const APASS_V1_OBJECT_RECORD_SEARCH: &'static str = "/open-apis/apaas/v1/application/{app_id}/object/{object_api_name}/record/search";

    pub const APASS_V1_OBJECT_RECORD_UPDATE: &'static str = "/open-apis/apaas/v1/application/{app_id}/object/{object_api_name}/record/{record_id}";

    pub const APASS_V1_PERMISSION_RECORD_MEMBERS_BATCH_CREATE: &'static str = "/open-apis/apaas/v1/application/{app_id}/permission/record_permission/{record_permission_api_name}/members/batch_create";

    pub const APASS_V1_PERMISSION_RECORD_MEMBERS_BATCH_REMOVE: &'static str = "/open-apis/apaas/v1/application/{app_id}/permission/record_permission/{record_permission_api_name}/members/batch_remove";

    pub const APASS_V1_PERMISSION_ROLE_MEMBERS_BATCH_CREATE: &'static str = "/open-apis/apaas/v1/application/{app_id}/permission/role/{role_api_name}/members/batch_create";

    pub const APASS_V1_PERMISSION_ROLE_MEMBERS_BATCH_REMOVE: &'static str = "/open-apis/apaas/v1/application/{app_id}/permission/role/{role_api_name}/members/batch_remove";

    pub const APASS_V1_PERMISSION_ROLE_MEMBER_GET: &'static str = "/open-apis/apaas/v1/application/{app_id}/permission/role/{role_api_name}/member/{user_id}";

    pub const APASS_V1_SEAT_ACTIVITY_LIST: &'static str = "/open-apis/apaas/v1/seat_activity/list";

    pub const APASS_V1_SEAT_ASSIGNMENT_LIST: &'static str = "/open-apis/apaas/v1/seat_assignment/list";

    pub const APPLICATION_GET_ADMIN_LIST: &'static str = "/open-apis/application/v6/applications/{app_id}/app_admin_user_list";

    pub const APPLICATION_GET_APP_INFO: &'static str = "/open-apis/application/v6/applications/{app_id}";

    pub const APPLICATION_GET_APP_STORE_INFO: &'static str = "/open-apis/application/v6/applications/{app_id}/app_version";

    pub const APPLICATION_V6_ADMIN_APPS: &'static str = "/open-apis/application/v6/admin/apps";

    pub const APPLICATION_V6_ADMIN_APP_ADMINS: &'static str = "/open-apis/application/v6/admin/apps/{app_id}/admins";

    pub const APPLICATION_V6_ADMIN_APP_ADMIN_MANAGEMENT_PERMISSIONS: &'static str = "/open-apis/application/v6/admin/apps/{app_id}/admins/{user_id}/management_permissions";

    pub const APPLICATION_V6_ADMIN_APP_ADMIN_VERIFY: &'static str = "/open-apis/application/v6/admin/apps/{app_id}/admins/{user_id}/verify";

    pub const APPLICATION_V6_ADMIN_APP_CHECK_WHITE_BLACK_LIST: &'static str = "/open-apis/application/v6/admin/apps/{app_id}/check_white_black_list";

    pub const APPLICATION_V6_ADMIN_APP_CONTACTS_RANGE_CONFIGURATION_GET: &'static str = "/open-apis/application/v6/admin/apps/{app_id}/contacts_range_configuration";

    pub const APPLICATION_V6_ADMIN_APP_CONTACTS_RANGE_CONFIGURATION_SET: &'static str = "/open-apis/application/v6/admin/apps/{app_id}/contacts_range_configuration";

    pub const APPLICATION_V6_ADMIN_APP_ENABLE: &'static str = "/open-apis/application/v6/admin/apps/{app_id}/enable";

    pub const APPLICATION_V6_ADMIN_APP_VISIBILITY: &'static str = "/open-apis/application/v6/admin/apps/{app_id}/visibility";

    pub const APPLICATION_V6_ADMIN_APP_VISIBILITY_CREATE: &'static str = "/open-apis/application/v6/admin/apps/{app_id}/visibility";

    pub const APPLICATION_V6_ADMIN_USER_AVAILABLE_APPS: &'static str = "/open-apis/application/v6/admin/user_available_apps/{user_id}";

    pub const APPLICATION_V6_APPLICATION_FEEDBACK: &'static str = "/open-apis/application/v6/application_feedback";

    pub const APPLICATION_V6_APPLICATION_FEEDBACK_GET: &'static str = "/open-apis/application/v6/application_feedback/{feedback_id}";

    pub const APPLICATION_V6_APPSTORE_PAID_INFO_CHECK: &'static str = "/open-apis/application/v6/appstore_paid_info/{app_id}/users/{user_id}/pricing_plans/{pricing_plan_id}/check";

    pub const APPLICATION_V6_APPSTORE_PAID_INFO_ORDER_GET: &'static str = "/open-apis/application/v6/appstore_paid_info/{app_id}/orders/{order_id}";

    pub const APPLICATION_V6_APPSTORE_PAID_INFO_PRICING_PLANS: &'static str = "/open-apis/application/v6/appstore_paid_info/{app_id}/pricing_plans";

    pub const APPLICATION_V6_APPS_UNDERAUDITLIST: &'static str = "/open-apis/application/v6/applications/underauditlist";

    pub const APPLICATION_V6_APP_AUDIT: &'static str = "/open-apis/application/v6/applications/{app_id}/audit";

    pub const APPLICATION_V6_APP_BADGE_SET: &'static str = "/open-apis/application/v6/app_badge/{app_id}/users/{user_id}/set";

    pub const APPLICATION_V6_APP_COLLABORATORS: &'static str = "/open-apis/application/v6/applications/{app_id}/collaborators";

    pub const APPLICATION_V6_APP_COLLABORATOR_DELETE: &'static str = "/open-apis/application/v6/applications/{app_id}/collaborators";

    pub const APPLICATION_V6_APP_GET: &'static str = "/open-apis/application/v6/applications/{app_id}";

    pub const APPLICATION_V6_APP_GROUP: &'static str = "/open-apis/application/v6/applications/{app_id}/group";

    pub const APPLICATION_V6_APP_SCOPE_APPLY: &'static str = "/open-apis/application/v6/applications/{app_id}/scope/apply";

    pub const APPLICATION_V6_APP_SCOPE_GET: &'static str = "/open-apis/application/v6/applications/{app_id}/scope";

    pub const APPLICATION_V6_APP_TRANSFER_OWNER: &'static str = "/open-apis/application/v6/applications/{app_id}/transfer_owner";

    pub const APPLICATION_V6_APP_USAGE_DEPARTMENT_OVERVIEW: &'static str = "/open-apis/application/v6/app_usage/{app_id}/department_overview";

    pub const APPLICATION_V6_APP_USAGE_MESSAGE_PUSH_OVERVIEW: &'static str = "/open-apis/application/v6/app_usage/{app_id}/message_push_overview";

    pub const APPLICATION_V6_APP_USAGE_OVERVIEW: &'static str = "/open-apis/application/v6/app_usage/{app_id}/overview";

    pub const APPLICATION_V6_APP_VERSIONS: &'static str = "/open-apis/application/v6/applications/{app_id}/versions";

    pub const APPLICATION_V6_APP_VERSION_CONTACTS_RANGE_SUGGEST: &'static str = "/open-apis/application/v6/applications/{app_id}/versions/{version_id}/contacts_range_suggest";

    pub const APPLICATION_V6_APP_VERSION_GET: &'static str = "/open-apis/application/v6/applications/{app_id}/versions/{version_id}";

    pub const APPROVAL_V4_APPROVALS: &'static str = "/open-apis/approval/v4/approvals";

    pub const APPROVAL_V4_APPROVALS_SEARCH: &'static str = "/open-apis/approval/v4/approvals/search";

    pub const APPROVAL_V4_APPROVAL_GET: &'static str = "/open-apis/approval/v4/approvals/{approval_code}";

    pub const APPROVAL_V4_EXTERNAL_APPROVALS: &'static str = "/open-apis/approval/v4/external_approvals";

    pub const APPROVAL_V4_EXTERNAL_APPROVAL_GET: &'static str = "/open-apis/approval/v4/external_approvals/{approval_code}";

    pub const APPROVAL_V4_EXTERNAL_INSTANCES: &'static str = "/open-apis/approval/v4/external_instances";

    pub const APPROVAL_V4_EXTERNAL_INSTANCE_CHECK: &'static str = "/open-apis/approval/v4/external_instances/{instance_code}/check";

    pub const APPROVAL_V4_EXTERNAL_TASKS: &'static str = "/open-apis/approval/v4/external_tasks";

    pub const APPROVAL_V4_FILE_UPLOAD: &'static str = "/open-apis/approval/v4/files/upload";

    pub const APPROVAL_V4_INSTANCES: &'static str = "/open-apis/approval/v4/instances";

    pub const APPROVAL_V4_INSTANCES_LIST: &'static str = "/open-apis/approval/v4/instances";

    pub const APPROVAL_V4_INSTANCES_SEARCH: &'static str = "/open-apis/approval/v4/instances/search";

    pub const APPROVAL_V4_INSTANCES_SEARCH_CC: &'static str = "/open-apis/approval/v4/instances/search_cc";

    pub const APPROVAL_V4_INSTANCE_CANCEL: &'static str = "/open-apis/approval/v4/instances/{instance_code}/cancel";

    pub const APPROVAL_V4_INSTANCE_CC: &'static str = "/open-apis/approval/v4/instances/{instance_code}/cc";

    pub const APPROVAL_V4_INSTANCE_COMMENTS_CREATE: &'static str = "/open-apis/approval/v4/instances/{instance_code}/comments";

    pub const APPROVAL_V4_INSTANCE_COMMENTS_LIST: &'static str = "/open-apis/approval/v4/instances/{instance_code}/comments";

    pub const APPROVAL_V4_INSTANCE_COMMENTS_REPLY: &'static str = "/open-apis/approval/v4/instances/{instance_code}/comments";

    pub const APPROVAL_V4_INSTANCE_COMMENT_DELETE: &'static str = "/open-apis/approval/v4/instances/{instance_code}/comments/{comment_id}";

    pub const APPROVAL_V4_INSTANCE_GET: &'static str = "/open-apis/approval/v4/instances/{instance_code}";

    pub const APPROVAL_V4_INSTANCE_PREVIEW: &'static str = "/open-apis/approval/v4/instances/preview";

    pub const APPROVAL_V4_MESSAGES: &'static str = "/open-apis/approval/v4/messages";

    pub const APPROVAL_V4_MESSAGE_PATCH: &'static str = "/open-apis/approval/v4/messages/{message_id}";

    pub const APPROVAL_V4_TASKS_QUERY: &'static str = "/open-apis/approval/v4/tasks/query";

    pub const APPROVAL_V4_TASKS_SEARCH: &'static str = "/open-apis/approval/v4/tasks/search";

    pub const APPROVAL_V4_TASK_ADD_SIGN: &'static str = "/open-apis/approval/v4/tasks/{task_id}/add_sign";

    pub const APPROVAL_V4_TASK_APPROVE: &'static str = "/open-apis/approval/v4/tasks/{task_id}/approve";

    pub const APPROVAL_V4_TASK_REJECT: &'static str = "/open-apis/approval/v4/tasks/{task_id}/reject";

    pub const APPROVAL_V4_TASK_RESUBMIT: &'static str = "/open-apis/approval/v4/tasks/{task_id}/resubmit";

    pub const APPROVAL_V4_TASK_SPECIFIED_ROLLBACK: &'static str = "/open-apis/approval/v4/tasks/{task_id}/specified_rollback";

    pub const APPROVAL_V4_TASK_TRANSFER: &'static str = "/open-apis/approval/v4/tasks/{task_id}/transfer";

    pub const ASSISTANT_V1_FILE_SUBSCRIPTION: &'static str = "/open-apis/assistant/v1/file/{}/{}/subscription";

    pub const ATTENDANCE_V1_ARCHIVE_RULES: &'static str = "/open-apis/attendance/v1/archive_rules";

    pub const ATTENDANCE_V1_ARCHIVE_RULE_DEL_REPORT: &'static str = "/open-apis/attendance/v1/archive_rules/{archive_rule_id}/del_report";

    pub const ATTENDANCE_V1_ARCHIVE_RULE_UPLOAD_REPORT: &'static str = "/open-apis/attendance/v1/archive_rules/{archive_rule_id}/upload_report";

    pub const ATTENDANCE_V1_ARCHIVE_RULE_USER_STATS_FIELDS: &'static str = "/open-apis/attendance/v1/archive_rules/{archive_rule_id}/user_stats_fields";

    pub const ATTENDANCE_V1_GROUPS: &'static str = "/open-apis/attendance/v1/groups";

    pub const ATTENDANCE_V1_GROUPS_SEARCH: &'static str = "/open-apis/attendance/v1/groups/search";

    pub const ATTENDANCE_V1_GROUP_DELETE: &'static str = "/open-apis/attendance/v1/groups/{group_id}";

    pub const ATTENDANCE_V1_GROUP_GET: &'static str = "/open-apis/attendance/v1/groups/{group_id}";

    pub const ATTENDANCE_V1_GROUP_USERS: &'static str = "/open-apis/attendance/v1/groups/{group_id}/users";

    pub const ATTENDANCE_V1_LEAVE_ACCRUAL_RECORD_GET: &'static str = "/open-apis/attendance/v1/leave_accrual_records/{leave_accrual_record_id}";

    pub const ATTENDANCE_V1_LEAVE_EMPLOY_EXPIRE_RECORDS: &'static str = "/open-apis/attendance/v1/leave_employ_expire_records";

    pub const ATTENDANCE_V1_SHIFTS: &'static str = "/open-apis/attendance/v1/shifts";

    pub const ATTENDANCE_V1_SHIFTS_QUERY: &'static str = "/open-apis/attendance/v1/shifts/query";

    pub const ATTENDANCE_V1_SHIFT_DELETE: &'static str = "/open-apis/attendance/v1/shifts/{shift_id}";

    pub const ATTENDANCE_V1_SHIFT_GET: &'static str = "/open-apis/attendance/v1/shifts/{shift_id}";

    pub const ATTENDANCE_V1_USER_APPROVALS: &'static str = "/open-apis/attendance/v1/user_approvals";

    pub const ATTENDANCE_V1_USER_APPROVAL_PROCESS: &'static str = "/open-apis/attendance/v1/user_approvals/{approval_id}/process";

    pub const ATTENDANCE_V1_USER_DAILY_SHIFTS_BATCH_CREATE: &'static str = "/open-apis/attendance/v1/user_daily_shifts/batch_create";

    pub const ATTENDANCE_V1_USER_DAILY_SHIFTS_BATCH_CREATE_TEMP: &'static str = "/open-apis/attendance/v1/user_daily_shifts/batch_create_temp";

    pub const ATTENDANCE_V1_USER_DAILY_SHIFTS_QUERY: &'static str = "/open-apis/attendance/v1/user_daily_shifts/query";

    pub const ATTENDANCE_V1_USER_SETTINGS_DOWNLOAD: &'static str = "/open-apis/attendance/v1/user_settings/{user_id}/download";

    pub const ATTENDANCE_V1_USER_SETTINGS_MODIFY: &'static str = "/open-apis/attendance/v1/user_settings/{user_id}/modify";

    pub const ATTENDANCE_V1_USER_SETTINGS_QUERY: &'static str = "/open-apis/attendance/v1/user_settings/query";

    pub const ATTENDANCE_V1_USER_SETTINGS_UPLOAD: &'static str = "/open-apis/attendance/v1/user_settings/{user_id}/upload";

    pub const ATTENDANCE_V1_USER_STATS_DATAS_QUERY: &'static str = "/open-apis/attendance/v1/user_stats_datas/query";

    pub const ATTENDANCE_V1_USER_STATS_DATAS_QUERY_DATA: &'static str = "/open-apis/attendance/v1/user_stats_datas/query_data";

    pub const ATTENDANCE_V1_USER_STATS_DATAS_QUERY_FIELDS: &'static str = "/open-apis/attendance/v1/user_stats_datas/query_fields";

    pub const ATTENDANCE_V1_USER_STATS_DATAS_UPDATE: &'static str = "/open-apis/attendance/v1/user_stats_datas/update";

    pub const ATTENDANCE_V1_USER_TASKS_BATCH_CREATE: &'static str = "/open-apis/attendance/v1/user_tasks/batch_create";

    pub const ATTENDANCE_V1_USER_TASKS_BATCH_DELETE: &'static str = "/open-apis/attendance/v1/user_tasks/batch_del";

    pub const ATTENDANCE_V1_USER_TASKS_QUERY: &'static str = "/open-apis/attendance/v1/user_tasks/query";

    pub const ATTENDANCE_V1_USER_TASK_GET: &'static str = "/open-apis/attendance/v1/user_tasks/{user_id}/get";

    pub const ATTENDANCE_V1_USER_TASK_REMEDYS: &'static str = "/open-apis/attendance/v1/user_task_remedys";

    pub const ATTENDANCE_V1_USER_TASK_REMEDYS_QUERY_USER_ALLOWED_REMEDYS: &'static str = "/open-apis/attendance/v1/user_task_remedys/query_user_allowed_remedys";

    pub const ATTENDANCE_V1_USER_TASK_RESULTS_QUERY: &'static str = "/open-apis/attendance/v1/user_task_results/query";

    pub const AUTHEN_V1_USER_INFO: &'static str = "/open-apis/authen/v1/user_info";

    pub const AUTH_ACCESS_TOKEN: &'static str = "/open-apis/auth/v3/app_access_token";

    pub const AUTH_LOGIN_PRE_AUTH: &'static str = "/open-apis/authen/v1/oidc/access_token";

    pub const AUTH_REFRESH_USER_TOKEN: &'static str = "/open-apis/auth/v3/refresh_user_access_token";

    pub const AUTH_USER_ACCESS_TOKEN: &'static str = "/open-apis/auth/v3/user_access_token";

    pub const AUTH_USER_INFO: &'static str = "/open-apis/authen/v1/user_info";

    pub const BITABLE_V1_APPS: &'static str = "/open-apis/bitable/v1/apps";

    pub const BITABLE_V1_APP_COPY: &'static str = "/open-apis/bitable/v1/apps/{app_token}/copy";

    pub const BITABLE_V1_APP_CREATE: &'static str = "/open-apis/bitable/v1/apps";

    pub const BITABLE_V1_APP_GET: &'static str = "/open-apis/bitable/v1/apps/{app_token}";

    pub const BITABLE_V1_APP_UPDATE: &'static str = "/open-apis/bitable/v1/apps/{app_token}";

    pub const BITABLE_V1_DASHBOARDS: &'static str = "/open-apis/bitable/v1/apps/{app_token}/dashboards";

    pub const BITABLE_V1_DASHBOARD_COPY: &'static str = "/open-apis/bitable/v1/apps/{app_token}/dashboards/{block_id}/copy";

    pub const BITABLE_V1_FIELDS: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields";

    pub const BITABLE_V1_FIELD_CREATE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields";

    pub const BITABLE_V1_FIELD_DELETE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields/{field_id}";

    pub const BITABLE_V1_FIELD_GET: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields/{field_id}";

    pub const BITABLE_V1_FIELD_UPDATE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields/{field_id}";

    pub const BITABLE_V1_FORMS: &'static str = "/open-apis/bitable/v1/apps/{app_token}/forms";

    pub const BITABLE_V1_FORM_GET: &'static str = "/open-apis/bitable/v1/apps/{app_token}/forms/{form_id}";

    pub const BITABLE_V1_FORM_PATCH: &'static str = "/open-apis/bitable/v1/apps/{app_token}/forms/{form_id}";

    pub const BITABLE_V1_FORM_PATCH_META: &'static str = "/open-apis/bitable/v1/apps/{app_token}/forms/{form_id}/questions";

    pub const BITABLE_V1_FORM_QUESTION: &'static str = "/open-apis/bitable/v1/apps/{app_token}/forms/{form_id}/questions/{question_id}";

    pub const BITABLE_V1_RECORDS: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records";

    pub const BITABLE_V1_RECORDS_BATCH_CREATE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_create";

    pub const BITABLE_V1_RECORDS_BATCH_DELETE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_delete";

    pub const BITABLE_V1_RECORDS_BATCH_GET: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_get";

    pub const BITABLE_V1_RECORDS_BATCH_UPDATE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_update";

    pub const BITABLE_V1_RECORDS_SEARCH: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/search";

    pub const BITABLE_V1_RECORD_CREATE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records";

    pub const BITABLE_V1_RECORD_DELETE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/{record_id}";

    pub const BITABLE_V1_RECORD_GET: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/{record_id}";

    pub const BITABLE_V1_RECORD_UPDATE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/{record_id}";

    pub const BITABLE_V1_ROLES: &'static str = "/open-apis/bitable/v1/apps/{app_token}/roles";

    pub const BITABLE_V1_ROLE_CREATE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/roles";

    pub const BITABLE_V1_ROLE_DELETE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}";

    pub const BITABLE_V1_ROLE_GET: &'static str = "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}";

    pub const BITABLE_V1_ROLE_MEMBERS: &'static str = "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members";

    pub const BITABLE_V1_ROLE_MEMBERS_BATCH_CREATE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members/batch_create";

    pub const BITABLE_V1_ROLE_MEMBERS_BATCH_DELETE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members/batch_delete";

    pub const BITABLE_V1_ROLE_MEMBER_CREATE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members";

    pub const BITABLE_V1_ROLE_MEMBER_DELETE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members/{member_id}";

    pub const BITABLE_V1_ROLE_UPDATE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}";

    pub const BITABLE_V1_TABLES: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables";

    pub const BITABLE_V1_TABLES_BATCH_CREATE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/batch_create";

    pub const BITABLE_V1_TABLES_BATCH_DELETE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/batch_delete";

    pub const BITABLE_V1_TABLE_CREATE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables";

    pub const BITABLE_V1_TABLE_DELETE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}";

    pub const BITABLE_V1_TABLE_GET: &'static str = "/open-apis/bitable/v1/apps/{}/tables/{}";

    pub const BITABLE_V1_TABLE_PATCH: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}";

    pub const BITABLE_V1_VIEWS: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/views";

    pub const BITABLE_V1_VIEW_CREATE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/views";

    pub const BITABLE_V1_VIEW_DELETE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/views/{view_id}";

    pub const BITABLE_V1_VIEW_GET: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/views/{view_id}";

    pub const BITABLE_V1_VIEW_PATCH: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/views/{view_id}";

    pub const BITABLE_V1_WORKFLOWS: &'static str = "/open-apis/bitable/v1/apps/{app_token}/workflows";

    pub const BITABLE_V1_WORKFLOW_UPDATE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/workflows/{workflow_id}";

    pub const BOARD_V1_WHITEBOARD_NODES: &'static str = "/open-apis/board/v1/whiteboards/{}/nodes";

    pub const BOARD_V1_WHITEBOARD_THUMBNAIL: &'static str = "/open-apis/whiteboard/v1/whiteboards/{}/thumbnail";

    pub const CALENDAR_CREATE: &'static str = "/open-apis/calendar/v4/calendars";

    pub const CALENDAR_DELETE: &'static str = "/open-apis/calendar/v4/calendars/{calendar_id}";

    pub const CALENDAR_EVENT_CREATE: &'static str = "/open-apis/calendar/v4/calendars/{calendar_id}/events";

    pub const CALENDAR_EVENT_DELETE: &'static str = "/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}";

    pub const CALENDAR_EVENT_GET: &'static str = "/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}";

    pub const CALENDAR_EVENT_LIST: &'static str = "/open-apis/calendar/v4/calendars/{calendar_id}/events";

    pub const CALENDAR_EVENT_REPLY: &'static str = "/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}/reply";

    pub const CALENDAR_EVENT_SEARCH: &'static str = "/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}/search";

    pub const CALENDAR_EVENT_UPDATE: &'static str = "/open-apis/calendar/v4/calendars/{calendar_id}/events/{event_id}";

    pub const CALENDAR_GET: &'static str = "/open-apis/calendar/v4/calendars/{calendar_id}";

    pub const CALENDAR_LIST: &'static str = "/open-apis/calendar/v4/calendars";

    pub const CALENDAR_PRIMARY: &'static str = "/open-apis/calendar/v4/calendars/{calendar_id}/primary";

    pub const CALENDAR_SEARCH: &'static str = "/open-apis/calendar/v4/calendars/{calendar_id}/search";

    pub const CALENDAR_UPDATE: &'static str = "/open-apis/calendar/v4/calendars/{calendar_id}";

    pub const CALENDAR_V4_CALENDARS: &'static str = "/open-apis/calendar/v4/calendars";

    pub const CALENDAR_V4_CALENDAR_OPERATION: &'static str = "/open-apis/calendar/v4/calendars/{calendar_id}";

    pub const CARDKIT_V1_CARDS: &'static str = "/open-apis/cardkit/v1/cards";

    pub const CARDKIT_V1_CARD_BATCH_UPDATE: &'static str = "/open-apis/cardkit/v1/cards/{card_id}/batch_update";

    pub const CARDKIT_V1_CARD_ELEMENTS: &'static str = "/open-apis/cardkit/v1/cards/{card_id}/elements";

    pub const CARDKIT_V1_CARD_ELEMENTS_DELETE: &'static str = "/open-apis/cardkit/v1/cards/{card_id}/elements/{element_id}";

    pub const CARDKIT_V1_CARD_ELEMENTS_PATCH: &'static str = "/open-apis/cardkit/v1/cards/{card_id}/elements/{element_id}";

    pub const CARDKIT_V1_CARD_ELEMENTS_UPDATE: &'static str = "/open-apis/cardkit/v1/cards/{card_id}/elements/{element_id}";

    pub const CARDKIT_V1_CARD_SETTINGS: &'static str = "/open-apis/cardkit/v1/cards/{card_id}/settings";

    pub const CARDKIT_V1_CARD_UPDATE: &'static str = "/open-apis/cardkit/v1/cards/{card_id}";

    pub const COMMENT_V1_COMMENTS: &'static str = "/open-apis/comment/v1/comments";

    pub const COMMENT_V1_COMMENTS_BATCH_QUERY: &'static str = "/open-apis/comment/v1/comments/batch_query";

    pub const COMMENT_V1_COMMENT_CREATE: &'static str = "/open-apis/comment/v1/comments";

    pub const COMMENT_V1_COMMENT_GET: &'static str = "/open-apis/comment/v1/comments/{}";

    pub const COMMENT_V1_COMMENT_PATCH: &'static str = "/open-apis/comment/v1/comments/{}";

    pub const COMMENT_V1_COMMENT_REPLIES: &'static str = "/open-apis/comment/v1/comments/{}/replies";

    pub const COMMENT_V1_COMMENT_REPLY_DELETE: &'static str = "/open-apis/comment/v1/comments/{comment_id}/replies/{reply_id}";

    pub const COMMENT_V1_COMMENT_REPLY_UPDATE: &'static str = "/open-apis/comment/v1/comments/{comment_id}/replies/{reply_id}";

    pub const CONTACT_DEPARTMENT_CHILDREN: &'static str = "/open-apis/contact/v3/departments/{department_id}/children";

    pub const CONTACT_DEPARTMENT_GET: &'static str = "/open-apis/contact/v3/departments/{department_id}";

    pub const CONTACT_DEPARTMENT_LIST: &'static str = "/open-apis/contact/v3/departments";

    pub const CONTACT_USER_BATCH_GET: &'static str = "/open-apis/contact/v3/users/batch";

    pub const CONTACT_USER_GET: &'static str = "/open-apis/contact/v3/users/{user_id}";

    pub const CONTACT_USER_LIST: &'static str = "/open-apis/contact/v3/users";

    pub const CONTACT_USER_SEARCH: &'static str = "/open-apis/contact/v3/users/find_by_mobile";

    pub const CONTACT_V3_CUSTOM_ATTRS: &'static str = "/open-apis/contact/v3/custom_attrs";

    pub const CONTACT_V3_DEPARTMENTS: &'static str = "/open-apis/contact/v3/departments";

    pub const CONTACT_V3_DEPARTMENTS_BATCH: &'static str = "/open-apis/contact/v3/departments/batch";

    pub const CONTACT_V3_DEPARTMENTS_CHILDREN: &'static str = "/open-apis/contact/v3/departments/children";

    pub const CONTACT_V3_DEPARTMENTS_PARENT: &'static str = "/open-apis/contact/v3/departments/parent";

    pub const CONTACT_V3_DEPARTMENTS_SEARCH: &'static str = "/open-apis/contact/v3/departments/search";

    pub const CONTACT_V3_DEPARTMENT_GET: &'static str = "/open-apis/contact/v3/departments/{department_id}";

    pub const CONTACT_V3_DEPARTMENT_UPDATE_ID: &'static str = "/open-apis/contact/v3/departments/{department_id}/update_department_id";

    pub const CONTACT_V3_EMPLOYEE_TYPE_ENUMS: &'static str = "/open-apis/contact/v3/employee_type_enums";

    pub const CONTACT_V3_EMPLOYEE_TYPE_ENUM_GET: &'static str = "/open-apis/contact/v3/employee_type_enums/{enum_id}";

    pub const CONTACT_V3_FUNCTIONAL_ROLES: &'static str = "/open-apis/contact/v3/functional_roles";

    pub const CONTACT_V3_FUNCTIONAL_ROLE_GET: &'static str = "/open-apis/contact/v3/functional_roles/{role_id}";

    pub const CONTACT_V3_FUNCTIONAL_ROLE_MEMBERS: &'static str = "/open-apis/contact/v3/functional_roles/{role_id}/members";

    pub const CONTACT_V3_FUNCTIONAL_ROLE_MEMBERS_BATCH_CREATE: &'static str = "/open-apis/contact/v3/functional_roles/{role_id}/members/batch_create";

    pub const CONTACT_V3_FUNCTIONAL_ROLE_MEMBERS_BATCH_DELETE: &'static str = "/open-apis/contact/v3/functional_roles/{role_id}/members/batch_delete";

    pub const CONTACT_V3_FUNCTIONAL_ROLE_MEMBERS_SCOPES: &'static str = "/open-apis/contact/v3/functional_roles/{role_id}/members/scopes";

    pub const CONTACT_V3_FUNCTIONAL_ROLE_MEMBER_GET: &'static str = "/open-apis/contact/v3/functional_roles/{role_id}/members/{member_id}";

    pub const CONTACT_V3_GROUPS: &'static str = "/open-apis/contact/v3/groups";

    pub const CONTACT_V3_GROUPS_MEMBER_BELONG: &'static str = "/open-apis/contact/v3/groups/member_belong";

    pub const CONTACT_V3_GROUPS_SIMPLELIST: &'static str = "/open-apis/contact/v3/groups/simplelist";

    pub const CONTACT_V3_GROUP_DETAIL: &'static str = "/open-apis/contact/v3/groups/{group_id}/detail";

    pub const CONTACT_V3_GROUP_GET: &'static str = "/open-apis/contact/v3/groups/{group_id}";

    pub const CONTACT_V3_GROUP_MEMBERS_ADD: &'static str = "/open-apis/contact/v3/groups/{group_id}/members/add";

    pub const CONTACT_V3_GROUP_MEMBERS_BATCH_ADD: &'static str = "/open-apis/contact/v3/groups/{group_id}/members/batch_add";

    pub const CONTACT_V3_GROUP_MEMBERS_BATCH_REMOVE: &'static str = "/open-apis/contact/v3/groups/{group_id}/members/batch_remove";

    pub const CONTACT_V3_GROUP_MEMBERS_REMOVE: &'static str = "/open-apis/contact/v3/groups/{group_id}/members/remove";

    pub const CONTACT_V3_GROUP_MEMBERS_SIMPLELIST: &'static str = "/open-apis/contact/v3/groups/{group_id}/members/simplelist";

    pub const CONTACT_V3_JOB_FAMILIES: &'static str = "/open-apis/contact/v3/job_families";

    pub const CONTACT_V3_JOB_FAMILY_GET: &'static str = "/open-apis/contact/v3/job_families/{job_family_id}";

    pub const CONTACT_V3_JOB_LEVELS: &'static str = "/open-apis/contact/v3/job_levels";

    pub const CONTACT_V3_JOB_LEVEL_GET: &'static str = "/open-apis/contact/v3/job_levels/{job_level_id}";

    pub const CONTACT_V3_JOB_TITLES: &'static str = "/open-apis/contact/v3/job_titles";

    pub const CONTACT_V3_JOB_TITLE_GET: &'static str = "/open-apis/contact/v3/job_titles/{job_title_id}";

    pub const CONTACT_V3_SCOPES: &'static str = "/open-apis/contact/v3/scopes";

    pub const CONTACT_V3_UNITS: &'static str = "/open-apis/contact/v3/units";

    pub const CONTACT_V3_UNIT_BIND_DEPARTMENT: &'static str = "/open-apis/contact/v3/units/{unit_id}/bind_department";

    pub const CONTACT_V3_UNIT_GET: &'static str = "/open-apis/contact/v3/units/{unit_id}";

    pub const CONTACT_V3_UNIT_LIST_DEPARTMENT: &'static str = "/open-apis/contact/v3/units/{unit_id}/list_department";

    pub const CONTACT_V3_UNIT_UNBIND_DEPARTMENT: &'static str = "/open-apis/contact/v3/units/{unit_id}/unbind_department";

    pub const CONTACT_V3_USERS: &'static str = "/open-apis/contact/v3/users";

    pub const CONTACT_V3_USERS_BATCH: &'static str = "/open-apis/contact/v3/users/batch";

    pub const CONTACT_V3_USERS_BATCH_GET_ID: &'static str = "/open-apis/contact/v3/users/batch_get_id";

    pub const CONTACT_V3_USERS_FIND_BY_DEPARTMENT: &'static str = "/open-apis/contact/v3/users/find_by_department";

    pub const CONTACT_V3_USERS_SEARCH: &'static str = "/open-apis/contact/v3/users/search";

    pub const CONTACT_V3_USER_GET: &'static str = "/open-apis/contact/v3/users/{user_id}";

    pub const CONTACT_V3_USER_RESURRECT: &'static str = "/open-apis/contact/v3/users/{user_id}/resurrect";

    pub const CONTACT_V3_USER_UPDATE_ID: &'static str = "/open-apis/contact/v3/users/{user_id}/update_user_id";

    pub const CONTACT_V3_WORK_CITIES: &'static str = "/open-apis/contact/v3/work_cities";

    pub const CONTACT_V3_WORK_CITY_GET: &'static str = "/open-apis/contact/v3/work_cities/{work_city_id}";

    pub const COREHR_BASIC_INFO_ENUM_SEARCH: &'static str = "/open-apis/corehr/v1/basic_info/enum/search";

    pub const COREHR_BASIC_INFO_LOCATION_SEARCH: &'static str = "/open-apis/corehr/v1/basic_info/location_data/search";

    pub const COREHR_BASIC_INFO_NATIONALITY_SEARCH: &'static str = "/open-apis/corehr/v1/basic_info/nationality/search";

    pub const COREHR_COMMON_DATA_ID_CONVERT: &'static str = "/open-apis/corehr/v1/common_data_id/convert";

    pub const COREHR_COMPANIES: &'static str = "/open-apis/corehr/v1/companies";

    pub const COREHR_DEPARTMENTS: &'static str = "/open-apis/corehr/v1/departments";

    pub const COREHR_DEPARTMENTS_BATCH_GET: &'static str = "/open-apis/corehr/v1/departments/batch_get";

    pub const COREHR_DEPARTMENTS_TREE: &'static str = "/open-apis/corehr/v1/departments/tree";

    pub const COREHR_EMPLOYEES_BATCH_GET: &'static str = "/open-apis/corehr/v1/employees/batch_get";

    pub const COREHR_EMPLOYEES_SEARCH: &'static str = "/open-apis/corehr/v1/employees/search";

    pub const COREHR_JOBS: &'static str = "/open-apis/corehr/v1/jobs";

    pub const COREHR_JOB_CHANGES: &'static str = "/open-apis/corehr/v1/job_changes";

    pub const COREHR_JOB_CHANGES_SEARCH: &'static str = "/open-apis/corehr/v1/job_changes/search";

    pub const COREHR_JOB_FAMILIES: &'static str = "/open-apis/corehr/v1/job_families";

    pub const COREHR_JOB_GRADES: &'static str = "/open-apis/corehr/v1/job_grades";

    pub const COREHR_JOB_GRADES_QUERY: &'static str = "/open-apis/corehr/v1/job_grades/query";

    pub const COREHR_JOB_LEVELS: &'static str = "/open-apis/corehr/v1/job_levels";

    pub const COREHR_OFFBOARDINGS: &'static str = "/open-apis/corehr/v1/offboardings";

    pub const COREHR_OFFBOARDINGS_SEARCH: &'static str = "/open-apis/corehr/v1/offboardings/search";

    pub const COREHR_PRE_HIRES: &'static str = "/open-apis/corehr/v1/pre_hires";

    pub const COREHR_PRE_HIRES_SEARCH: &'static str = "/open-apis/corehr/v1/pre_hires/search";

    pub const DIRECTORY_V1_DEPARTMENTS: &'static str = "/open-apis/directory/v1/departments";

    pub const DIRECTORY_V1_DEPARTMENTS_FILTER: &'static str = "/open-apis/directory/v1/departments/filter";

    pub const DIRECTORY_V1_DEPARTMENTS_MGET: &'static str = "/open-apis/directory/v1/departments/mget";

    pub const DIRECTORY_V1_DEPARTMENTS_SEARCH: &'static str = "/open-apis/directory/v1/departments/search";

    pub const DIRECTORY_V1_DEPARTMENT_GET: &'static str = "/open-apis/directory/v1/departments/{department_id}";

    pub const DIRECTORY_V1_EMPLOYEES: &'static str = "/open-apis/directory/v1/employees";

    pub const DIRECTORY_V1_EMPLOYEES_FILTER: &'static str = "/open-apis/directory/v1/employees/filter";

    pub const DIRECTORY_V1_EMPLOYEES_MGET: &'static str = "/open-apis/directory/v1/employees/mget";

    pub const DIRECTORY_V1_EMPLOYEES_SEARCH: &'static str = "/open-apis/directory/v1/employees/search";

    pub const DIRECTORY_V1_EMPLOYEE_GET: &'static str = "/open-apis/directory/v1/employees/{employee_id}";

    pub const DIRECTORY_V1_EMPLOYEE_REGULAR: &'static str = "/open-apis/directory/v1/employees/{employee_id}/regular";

    pub const DIRECTORY_V1_EMPLOYEE_RESURRECT: &'static str = "/open-apis/directory/v1/employees/{employee_id}/resurrect";

    pub const DIRECTORY_V1_EMPLOYEE_TO_BE_RESIGNED: &'static str = "/open-apis/directory/v1/employees/{employee_id}/to_be_resigned";

    pub const DOCUMENT_AI_BANK_CARD_RECOGNIZE: &'static str = "/open-apis/document_ai/v1/bank_card/recognize";

    pub const DOCUMENT_AI_BUSINESS_CARD_RECOGNIZE: &'static str = "/open-apis/document_ai/v1/business_card/recognize";

    pub const DOCUMENT_AI_BUSINESS_LICENSE_RECOGNIZE: &'static str = "/open-apis/document_ai/v1/business_license/recognize";

    pub const DOCUMENT_AI_CHINESE_PASSPORT_RECOGNIZE: &'static str = "/open-apis/document_ai/v1/chinese_passport/recognize";

    pub const DOCUMENT_AI_CONTRACT_FIELD_EXTRACTION: &'static str = "/open-apis/document_ai/v1/contract/field_extraction";

    pub const DOCUMENT_AI_DRIVING_LICENSE_RECOGNIZE: &'static str = "/open-apis/document_ai/v1/driving_license/recognize";

    pub const DOCUMENT_AI_FOOD_MANAGE_LICENSE_RECOGNIZE: &'static str = "/open-apis/document_ai/v1/food_manage_license/recognize";

    pub const DOCUMENT_AI_FOOD_PRODUCE_LICENSE_RECOGNIZE: &'static str = "/open-apis/document_ai/v1/food_produce_license/recognize";

    pub const DOCUMENT_AI_HEALTH_CERTIFICATE_RECOGNIZE: &'static str = "/open-apis/document_ai/v1/health_certificate/recognize";

    pub const DOCUMENT_AI_HKM_MAINLAND_TRAVEL_PERMIT_RECOGNIZE: &'static str = "/open-apis/document_ai/v1/hkm_mainland_travel_permit/recognize";

    pub const DOCUMENT_AI_ID_CARD_RECOGNIZE: &'static str = "/open-apis/document_ai/v1/id_card/recognize";

    pub const DOCUMENT_AI_RESUME_PARSE: &'static str = "/open-apis/document_ai/v1/resume/parse";

    pub const DOCUMENT_AI_TAXI_INVOICE_RECOGNIZE: &'static str = "/open-apis/document_ai/v1/taxi_invoice/recognize";

    pub const DOCUMENT_AI_TRAIN_INVOICE_RECOGNIZE: &'static str = "/open-apis/document_ai/v1/train_invoice/recognize";

    pub const DOCUMENT_AI_TW_MAINLAND_TRAVEL_PERMIT_RECOGNIZE: &'static str = "/open-apis/document_ai/v1/tw_mainland_travel_permit/recognize";

    pub const DOCUMENT_AI_VAT_INVOICE_RECOGNIZE: &'static str = "/open-apis/document_ai/v1/vat_invoice/recognize";

    pub const DOCUMENT_AI_VEHICLE_INVOICE_RECOGNIZE: &'static str = "/open-apis/document_ai/v1/vehicle_invoice/recognize";

    pub const DOCUMENT_AI_VEHICLE_LICENSE_RECOGNIZE: &'static str = "/open-apis/document_ai/v1/vehicle_license/recognize";

    pub const DOCX_V1_CHAT_ANNOUNCEMENT: &'static str = "/open-apis/docx/v1/chats/{}/announcement";

    pub const DOCX_V1_CHAT_ANNOUNCEMENT_BLOCK: &'static str = "/open-apis/docx/v1/chats/{}/announcement/blocks/{}";

    pub const DOCX_V1_DOCUMENTS: &'static str = "/open-apis/docx/v1/documents";

    pub const DOCX_V1_DOCUMENT_BLOCKS: &'static str = "/open-apis/docx/v1/documents/{}/blocks";

    pub const DOCX_V1_DOCUMENT_BLOCKS_BATCH_DELETE: &'static str = "/open-apis/docx/v1/documents/{}/blocks/batch_delete";

    pub const DOCX_V1_DOCUMENT_BLOCKS_BATCH_UPDATE: &'static str = "/open-apis/docx/v1/documents/{document_id}/blocks/batch_update";

    pub const DOCX_V1_DOCUMENT_BLOCK_CHILDREN: &'static str = "/open-apis/docx/v1/documents/{}/blocks/{}/children";

    pub const DOCX_V1_DOCUMENT_BLOCK_DESCENDANT_CREATE: &'static str = "/open-apis/docx/v1/documents/{document_id}/blocks/{block_id}/descendant";

    pub const DOCX_V1_DOCUMENT_BLOCK_GET: &'static str = "/open-apis/docx/v1/documents/{document_id}/blocks/{block_id}";

    pub const DOCX_V1_DOCUMENT_CONVERT: &'static str = "/open-apis/docx/v1/documents/{}/convert";

    pub const DOCX_V1_DOCUMENT_GET: &'static str = "/open-apis/docx/v1/documents/{}";

    pub const DOCX_V1_DOCUMENT_RAW_CONTENT: &'static str = "/open-apis/docx/v1/documents/{}/raw_content";

    pub const DRIVE_EXPLORER_V2_FOLDER_META: &'static str = "/open-apis/drive/explorer/v2/folder/{folder_token}/meta";

    pub const DRIVE_EXPLORER_V2_ROOT_FOLDER_META: &'static str = "/open-apis/drive/explorer/v2/root_folder/meta";

    pub const DRIVE_V1_COPY: &'static str = "/open-apis/drive/v1/files/{file_token}/copy";

    pub const DRIVE_V1_CREATE_FOLDER: &'static str = "/open-apis/drive/v1/files/create_folder";

    pub const DRIVE_V1_DELETE: &'static str = "/open-apis/drive/v1/files/{file_token}";

    pub const DRIVE_V1_DOWNLOAD: &'static str = "/open-apis/drive/v1/files/{file_token}/download";

    pub const DRIVE_V1_EXPORT_TASKS: &'static str = "/open-apis/drive/v1/export_tasks";

    pub const DRIVE_V1_EXPORT_TASK_GET: &'static str = "/open-apis/drive/v1/export_tasks/{}";

    pub const DRIVE_V1_FILES: &'static str = "/open-apis/drive/v1/files";

    pub const DRIVE_V1_FILES_CREATE_FOLDER: &'static str = "/open-apis/drive/v1/files/create_folder";

    pub const DRIVE_V1_FILES_CREATE_SHORTCUT: &'static str = "/open-apis/drive/v1/files/create_shortcut";

    pub const DRIVE_V1_FILES_DELETE: &'static str = "/open-apis/drive/v1/files/{}";

    pub const DRIVE_V1_FILES_SEARCH: &'static str = "/open-apis/drive/v1/files/search";

    pub const DRIVE_V1_FILES_SUBSCRIBE: &'static str = "/open-apis/drive/v1/files/subscribe";

    pub const DRIVE_V1_FILES_UPLOAD_ALL: &'static str = "/open-apis/drive/v1/files/upload_all";

    pub const DRIVE_V1_FILES_UPLOAD_FINISH: &'static str = "/open-apis/drive/v1/files/upload_finish";

    pub const DRIVE_V1_FILES_UPLOAD_PART: &'static str = "/open-apis/drive/v1/files/upload_part";

    pub const DRIVE_V1_FILES_UPLOAD_PREPARE: &'static str = "/open-apis/drive/v1/files/upload_prepare";

    pub const DRIVE_V1_FILE_COPY: &'static str = "/open-apis/drive/v1/files/{}/copy";

    pub const DRIVE_V1_FILE_DELETE_SUBSCRIBE: &'static str = "/open-apis/drive/v1/files/{}/delete_subscribe";

    pub const DRIVE_V1_FILE_DOWNLOAD: &'static str = "/open-apis/drive/v1/files/{}/download";

    pub const DRIVE_V1_FILE_GET: &'static str = "/open-apis/drive/v1/files/{}";

    pub const DRIVE_V1_FILE_GET_SUBSCRIBE: &'static str = "/open-apis/drive/v1/files/{}/get_subscribe";

    pub const DRIVE_V1_FILE_LIKE_RECORDS: &'static str = "/open-apis/drive/v1/files/{}/like_records";

    pub const DRIVE_V1_FILE_STATISTICS: &'static str = "/open-apis/drive/v1/files/{}/statistics";

    pub const DRIVE_V1_FILE_SUBSCRIPTIONS: &'static str = "/open-apis/drive/v1/files/{}/subscriptions/{}";

    pub const DRIVE_V1_FILE_VERSIONS: &'static str = "/open-apis/drive/v1/files/{}/versions";

    pub const DRIVE_V1_FILE_VERSION_GET: &'static str = "/open-apis/drive/v1/files/{}/versions/{}";

    pub const DRIVE_V1_FILE_VIEW_RECORDS: &'static str = "/open-apis/drive/v1/files/{}/view_records";

    pub const DRIVE_V1_FOLDERS: &'static str = "/open-apis/drive/v1/folders";

    pub const DRIVE_V1_FOLDERS_ROOT_FOLDER_META: &'static str = "/open-apis/drive/v1/folders/root_folder_meta";

    pub const DRIVE_V1_FOLDER_CHILDREN: &'static str = "/open-apis/drive/v1/folders/{}/children";

    pub const DRIVE_V1_FOLDER_GET: &'static str = "/open-apis/drive/v1/folders/{}";

    pub const DRIVE_V1_FOLDER_MOVE: &'static str = "/open-apis/drive/v1/folders/{}/move";

    pub const DRIVE_V1_GET_META: &'static str = "/open-apis/drive/v1/metas/{token}";

    pub const DRIVE_V1_IMPORT_TASKS: &'static str = "/open-apis/drive/v1/import_tasks";

    pub const DRIVE_V1_IMPORT_TASK_GET: &'static str = "/open-apis/drive/v1/import_tasks/{}";

    pub const DRIVE_V1_LIST_FILES: &'static str = "/open-apis/drive/v1/files";

    pub const DRIVE_V1_MEDIAS_BATCH_GET_TMP_DOWNLOAD_URL: &'static str = "/open-apis/drive/v1/medias/batch_get_tmp_download_url";

    pub const DRIVE_V1_MEDIAS_DOWNLOAD: &'static str = "/open-apis/drive/v1/medias/{}/download";

    pub const DRIVE_V1_MEDIAS_UPLOAD_ALL: &'static str = "/open-apis/drive/v1/medias/upload_all";

    pub const DRIVE_V1_MEDIAS_UPLOAD_FINISH: &'static str = "/open-apis/drive/v1/medias/upload_finish";

    pub const DRIVE_V1_MEDIAS_UPLOAD_PART: &'static str = "/open-apis/drive/v1/medias/upload_part";

    pub const DRIVE_V1_MEDIAS_UPLOAD_PREPARE: &'static str = "/open-apis/drive/v1/medias/upload_prepare";

    pub const DRIVE_V1_METAS_BATCH_QUERY: &'static str = "/open-apis/drive/v1/metas/batch_query";

    pub const DRIVE_V1_MOVE: &'static str = "/open-apis/drive/v1/files/{file_token}/move";

    pub const DRIVE_V1_PERMISSIONS_MEMBERS: &'static str = "/open-apis/drive/v1/permissions/{}/members";

    pub const DRIVE_V1_PERMISSIONS_MEMBERS_AUTH: &'static str = "/open-apis/drive/v1/permissions/{}/members/auth";

    pub const DRIVE_V1_PERMISSIONS_MEMBERS_BATCH_CREATE: &'static str = "/open-apis/drive/v1/permissions/{}/members/batch_create";

    pub const DRIVE_V1_PERMISSIONS_MEMBERS_TRANSFER_OWNER: &'static str = "/open-apis/drive/v1/permissions/{}/members/transfer_owner";

    pub const DRIVE_V1_PERMISSIONS_MEMBER_GET: &'static str = "/open-apis/drive/v1/permissions/{}/members/{}";

    pub const DRIVE_V1_PERMISSIONS_PUBLIC: &'static str = "/open-apis/drive/v1/permissions/{}/public";

    pub const DRIVE_V1_PERMISSIONS_PUBLIC_PASSWORD: &'static str = "/open-apis/drive/v1/permissions/{}/public/password";

    pub const DRIVE_V1_TASK_GET: &'static str = "/open-apis/drive/v1/tasks/{}";

    pub const DRIVE_V1_UPLOAD_FINISH: &'static str = "/open-apis/drive/v1/files/upload_finish";

    pub const DRIVE_V1_UPLOAD_PART: &'static str = "/open-apis/drive/v1/files/upload_part";

    pub const DRIVE_V1_UPLOAD_PREPARE: &'static str = "/open-apis/drive/v1/files/upload_prepare";

    pub const DRIVE_V2_PERMISSIONS_PUBLIC: &'static str = "/open-apis/drive/v2/permissions/{}/public";

    pub const ELEARNING_V2_COURSE_REGISTRATIONS: &'static str = "/open-apis/elearning/v2/course_registrations";

    pub const ELEARNING_V2_COURSE_REGISTRATIONS_STATISTICS: &'static str = "/open-apis/elearning/v2/course_registrations/statistics";

    pub const ELEARNING_V2_COURSE_REGISTRATION_OPERATION: &'static str = "/open-apis/elearning/v2/course_registrations/{registration_id}";

    pub const HELPDESK_V1_AGENT_EMAIL: &'static str = "/open-apis/helpdesk/v1/agents/{agent_id}/agent_email";

    pub const HELPDESK_V1_AGENT_GET: &'static str = "/open-apis/helpdesk/v1/agents/{agent_id}";

    pub const HELPDESK_V1_AGENT_SCHEDULES: &'static str = "/open-apis/helpdesk/v1/agents/{agent_id}/agent_schedules";

    pub const HELPDESK_V1_AGENT_SCHEDULE_CREATE: &'static str = "/open-apis/helpdesk/v1/agents/{agent_id}/agent_schedules";

    pub const HELPDESK_V1_AGENT_SCHEDULE_DELETE: &'static str = "/open-apis/helpdesk/v1/agents/{agent_id}/agent_schedules/{schedule_id}";

    pub const HELPDESK_V1_AGENT_SCHEDULE_GET: &'static str = "/open-apis/helpdesk/v1/agents/{agent_id}/agent_schedules/{schedule_id}";

    pub const HELPDESK_V1_AGENT_SCHEDULE_UPDATE: &'static str = "/open-apis/helpdesk/v1/agents/{agent_id}/agent_schedules/{schedule_id}";

    pub const HELPDESK_V1_AGENT_SKILLS: &'static str = "/open-apis/helpdesk/v1/agent_skills";

    pub const HELPDESK_V1_AGENT_SKILL_CREATE: &'static str = "/open-apis/helpdesk/v1/agent_skills";

    pub const HELPDESK_V1_AGENT_SKILL_DELETE: &'static str = "/open-apis/helpdesk/v1/agent_skills/{skill_id}";

    pub const HELPDESK_V1_AGENT_SKILL_GET: &'static str = "/open-apis/helpdesk/v1/agent_skills/{skill_id}";

    pub const HELPDESK_V1_AGENT_SKILL_RULES: &'static str = "/open-apis/helpdesk/v1/agent_skill_rules";

    pub const HELPDESK_V1_AGENT_SKILL_RULES_OPERATOR_OPTIONS: &'static str = "/open-apis/helpdesk/v1/agent_skill_rules/operator-options";

    pub const HELPDESK_V1_AGENT_SKILL_UPDATE: &'static str = "/open-apis/helpdesk/v1/agent_skills/{skill_id}";

    pub const HELPDESK_V1_CATEGORIES: &'static str = "/open-apis/helpdesk/v1/categories";

    pub const HELPDESK_V1_CATEGORY_CREATE: &'static str = "/open-apis/helpdesk/v1/categories";

    pub const HELPDESK_V1_CATEGORY_DELETE: &'static str = "/open-apis/helpdesk/v1/categories/{category_id}";

    pub const HELPDESK_V1_CATEGORY_GET: &'static str = "/open-apis/helpdesk/v1/categories/{category_id}";

    pub const HELPDESK_V1_CATEGORY_UPDATE: &'static str = "/open-apis/helpdesk/v1/categories/{category_id}";

    pub const HELPDESK_V1_EVENTS_SUBSCRIBE: &'static str = "/open-apis/helpdesk/v1/events/subscribe";

    pub const HELPDESK_V1_EVENTS_UNSUBSCRIBE: &'static str = "/open-apis/helpdesk/v1/events/unsubscribe";

    pub const HELPDESK_V1_FAQS: &'static str = "/open-apis/helpdesk/v1/faqs";

    pub const HELPDESK_V1_FAQS_SEARCH: &'static str = "/open-apis/helpdesk/v1/faqs/search";

    pub const HELPDESK_V1_FAQ_CREATE: &'static str = "/open-apis/helpdesk/v1/faqs";

    pub const HELPDESK_V1_FAQ_DELETE: &'static str = "/open-apis/helpdesk/v1/faqs/{faq_id}";

    pub const HELPDESK_V1_FAQ_GET: &'static str = "/open-apis/helpdesk/v1/faqs/{faq_id}";

    pub const HELPDESK_V1_FAQ_IMAGE: &'static str = "/open-apis/helpdesk/v1/faqs/{faq_id}/image/{image_key}";

    pub const HELPDESK_V1_FAQ_UPDATE: &'static str = "/open-apis/helpdesk/v1/faqs/{faq_id}";

    pub const HELPDESK_V1_NOTIFICATIONS: &'static str = "/open-apis/helpdesk/v1/notifications";

    pub const HELPDESK_V1_NOTIFICATION_CANCEL_APPROVE: &'static str = "/open-apis/helpdesk/v1/notifications/{notification_id}/cancel_approve";

    pub const HELPDESK_V1_NOTIFICATION_CANCEL_SEND: &'static str = "/open-apis/helpdesk/v1/notifications/{notification_id}/cancel_send";

    pub const HELPDESK_V1_NOTIFICATION_CREATE: &'static str = "/open-apis/helpdesk/v1/notifications";

    pub const HELPDESK_V1_NOTIFICATION_EXECUTE_SEND: &'static str = "/open-apis/helpdesk/v1/notifications/{notification_id}/execute_send";

    pub const HELPDESK_V1_NOTIFICATION_GET: &'static str = "/open-apis/helpdesk/v1/notifications/{notification_id}";

    pub const HELPDESK_V1_NOTIFICATION_PREVIEW: &'static str = "/open-apis/helpdesk/v1/notifications/{notification_id}/preview";

    pub const HELPDESK_V1_NOTIFICATION_SUBMIT_APPROVE: &'static str = "/open-apis/helpdesk/v1/notifications/{notification_id}/submit_approve";

    pub const HELPDESK_V1_NOTIFICATION_UPDATE: &'static str = "/open-apis/helpdesk/v1/notifications/{notification_id}";

    pub const HELPDESK_V1_START_SERVICE: &'static str = "/open-apis/helpdesk/v1/start_service";

    pub const HELPDESK_V1_TICKETS: &'static str = "/open-apis/helpdesk/v1/tickets";

    pub const HELPDESK_V1_TICKET_BOT_MESSAGES: &'static str = "/open-apis/helpdesk/v1/tickets/{ticket_id}/bot_messages";

    pub const HELPDESK_V1_TICKET_CUSTOMIZED_FIELDS: &'static str = "/open-apis/helpdesk/v1/ticket_customized_fields";

    pub const HELPDESK_V1_TICKET_CUSTOMIZED_FIELD_CREATE: &'static str = "/open-apis/helpdesk/v1/ticket_customized_fields";

    pub const HELPDESK_V1_TICKET_CUSTOMIZED_FIELD_DELETE: &'static str = "/open-apis/helpdesk/v1/ticket_customized_fields/{field_id}";

    pub const HELPDESK_V1_TICKET_CUSTOMIZED_FIELD_GET: &'static str = "/open-apis/helpdesk/v1/ticket_customized_fields/{field_id}";

    pub const HELPDESK_V1_TICKET_CUSTOMIZED_FIELD_UPDATE: &'static str = "/open-apis/helpdesk/v1/ticket_customized_fields/{field_id}";

    pub const HELPDESK_V1_TICKET_GET: &'static str = "/open-apis/helpdesk/v1/tickets/{ticket_id}";

    pub const HELPDESK_V1_TICKET_MESSAGES: &'static str = "/open-apis/helpdesk/v1/tickets/{ticket_id}/messages";

    pub const HELPDESK_V1_TICKET_MESSAGE_CREATE: &'static str = "/open-apis/helpdesk/v1/tickets/{ticket_id}/messages";

    pub const HELPDESK_V1_TICKET_UPDATE: &'static str = "/open-apis/helpdesk/v1/tickets/{ticket_id}";

    pub const HIRE_REFERRAL_ACCOUNT_BALANCE: &'static str = "/open-apis/hire/v1/referral_accounts/{user_id}/balance";

    pub const HIRE_REFERRAL_ACCOUNT_DISABLE: &'static str = "/open-apis/hire/v1/referral_accounts/{user_id}/disable";

    pub const HIRE_REFERRAL_ACCOUNT_ENABLE: &'static str = "/open-apis/hire/v1/referral_accounts/{user_id}/enable";

    pub const HIRE_REFERRAL_INCOME_RECORDS: &'static str = "/open-apis/hire/v1/referral_income_records";

    pub const HIRE_REFERRAL_STATISTICS: &'static str = "/open-apis/hire/v1/referral_statistics";

    pub const HIRE_REFERRAL_WITHDRAWALS: &'static str = "/open-apis/hire/v1/referral_withdrawals";

    pub const HIRE_REFERRAL_WITHDRAWAL_APPROVE: &'static str = "/open-apis/hire/v1/referral_withdrawals/{withdrawal_id}/approve";

    pub const HIRE_V1_AGENCIES: &'static str = "/open-apis/hire/v1/agencies";

    pub const HIRE_V1_AGENCIES_CONSULTANTS: &'static str = "/open-apis/hire/v1/agencies/{agency_id}/consultants";

    pub const HIRE_V1_AGENCY_CONSULTANTS: &'static str = "/open-apis/hire/v1/agency_consultants";

    pub const HIRE_V1_AGENCY_CONSULTANTS_BY_ID: &'static str = "/open-apis/hire/v1/agencies/{agency_id}/consultants";

    pub const HIRE_V1_AGENCY_RECOMMENDATIONS: &'static str = "/open-apis/hire/v1/agency_recommendations";

    pub const HIRE_V1_AGENCY_RECOMMENDATION_CONFIRM: &'static str = "/open-apis/hire/v1/agency_recommendations/{recommendation_id}/confirm";

    pub const HIRE_V1_AGENCY_RECOMMENDATION_REJECT: &'static str = "/open-apis/hire/v1/agency_recommendations/{recommendation_id}/reject";

    pub const HIRE_V1_APPLICATIONS: &'static str = "/open-apis/hire/v1/applications";

    pub const HIRE_V1_APPLICATION_ADVANCE: &'static str = "/open-apis/hire/v1/applications/{application_id}/advance";

    pub const HIRE_V1_APPLICATION_EVALUATIONS: &'static str = "/open-apis/hire/v1/applications/{application_id}/evaluations";

    pub const HIRE_V1_APPLICATION_GET: &'static str = "/open-apis/hire/v1/applications/{application_id}";

    pub const HIRE_V1_APPLICATION_INTERVIEWS: &'static str = "/open-apis/hire/v1/applications/{application_id}/interviews";

    pub const HIRE_V1_APPLICATION_OFFER: &'static str = "/open-apis/hire/v1/applications/{application_id}/offer";

    pub const HIRE_V1_APPLICATION_REJECT: &'static str = "/open-apis/hire/v1/applications/{application_id}/reject";

    pub const HIRE_V1_ATTACHMENTS: &'static str = "/open-apis/hire/v1/attachments";

    pub const HIRE_V1_ATTACHMENTS_BATCH_DELETE: &'static str = "/open-apis/hire/v1/attachments/batch_delete";

    pub const HIRE_V1_ATTACHMENTS_BATCH_DOWNLOAD: &'static str = "/open-apis/hire/v1/attachments/batch_download";

    pub const HIRE_V1_ATTACHMENT_DOWNLOAD: &'static str = "/open-apis/hire/v1/attachments/{attachment_id}/download";

    pub const HIRE_V1_ATTACHMENT_GET: &'static str = "/open-apis/hire/v1/attachments/{attachment_id}";

    pub const HIRE_V1_ATTACHMENT_PREVIEW: &'static str = "/open-apis/hire/v1/attachments/{attachment_id}/preview";

    pub const HIRE_V1_ATTACHMENT_STATISTICS: &'static str = "/open-apis/hire/v1/attachment_statistics";

    pub const HIRE_V1_ATTACHMENT_UPLOAD: &'static str = "/open-apis/hire/v1/attachments/upload";

    pub const HIRE_V1_BACKGROUND_CHECK_ORDERS: &'static str = "/open-apis/hire/v1/background_check_orders";

    pub const HIRE_V1_BACKGROUND_CHECK_ORDERS_BATCH: &'static str = "/open-apis/hire/v1/background_check_orders/batch";

    pub const HIRE_V1_BACKGROUND_CHECK_ORDER_CANCEL: &'static str = "/open-apis/hire/v1/background_check_orders/{order_id}/cancel";

    pub const HIRE_V1_BACKGROUND_CHECK_ORDER_GET: &'static str = "/open-apis/hire/v1/background_check_orders/{order_id}";

    pub const HIRE_V1_BACKGROUND_CHECK_ORDER_REPORT: &'static str = "/open-apis/hire/v1/background_check_orders/{order_id}/report";

    pub const HIRE_V1_BACKGROUND_CHECK_PACKAGES: &'static str = "/open-apis/hire/v1/background_check_packages";

    pub const HIRE_V1_EXAM_ARRANGEMENTS: &'static str = "/open-apis/hire/v1/exam_arrangements";

    pub const HIRE_V1_EXAM_PAPERS: &'static str = "/open-apis/hire/v1/exam_papers";

    pub const HIRE_V1_EXAM_RECORDS: &'static str = "/open-apis/hire/v1/exam_records";

    pub const HIRE_V1_EXAM_RECORD_CANCEL: &'static str = "/open-apis/hire/v1/exam_records/{record_id}/cancel";

    pub const HIRE_V1_EXAM_RECORD_GET: &'static str = "/open-apis/hire/v1/exam_records/{record_id}";

    pub const HIRE_V1_EXAM_RECORD_RESCHEDULE: &'static str = "/open-apis/hire/v1/exam_records/{record_id}/reschedule";

    pub const HIRE_V1_EXAM_STATISTICS: &'static str = "/open-apis/hire/v1/exam_statistics";

    pub const HIRE_V1_EXAM_SUBMISSIONS: &'static str = "/open-apis/hire/v1/exam_submissions";

    pub const HIRE_V1_EXTERNAL_SYSTEMS: &'static str = "/open-apis/hire/v1/external_systems";

    pub const HIRE_V1_EXTERNAL_SYSTEMS_CANDIDATES: &'static str = "/open-apis/hire/v1/external_systems/candidates";

    pub const HIRE_V1_EXTERNAL_SYSTEMS_CANDIDATES_CONVERT: &'static str = "/open-apis/hire/v1/external_systems/candidates/{external_candidate_id}/convert";

    pub const HIRE_V1_EXTERNAL_SYSTEMS_CANDIDATES_IMPORT: &'static str = "/open-apis/hire/v1/external_systems/candidates/import";

    pub const HIRE_V1_EXTERNAL_SYSTEMS_SYNC_RECORDS: &'static str = "/open-apis/hire/v1/external_systems/sync_records";

    pub const HIRE_V1_EXTERNAL_SYSTEMS_SYNC_TASKS: &'static str = "/open-apis/hire/v1/external_systems/sync_tasks";

    pub const HIRE_V1_EXTERNAL_SYSTEMS_TEST_CONNECTION: &'static str = "/open-apis/hire/v1/external_systems/{system_config_id}/test_connection";

    pub const HIRE_V1_INTERVIEWS: &'static str = "/open-apis/hire/v1/interviews";

    pub const HIRE_V1_INTERVIEW_ARRANGEMENTS: &'static str = "/open-apis/hire/v1/interview_arrangements";

    pub const HIRE_V1_INTERVIEW_CANCEL: &'static str = "/open-apis/hire/v1/interviews/{interview_id}/cancel";

    pub const HIRE_V1_INTERVIEW_EVALUATIONS: &'static str = "/open-apis/hire/v1/interview_evaluations";

    pub const HIRE_V1_INTERVIEW_EVALUATIONS_BY_ID: &'static str = "/open-apis/hire/v1/interviews/{interview_id}/evaluations";

    pub const HIRE_V1_INTERVIEW_GET: &'static str = "/open-apis/hire/v1/interviews/{interview_id}";

    pub const HIRE_V1_INTERVIEW_RESCHEDULE: &'static str = "/open-apis/hire/v1/interviews/{interview_id}/reschedule";

    pub const HIRE_V1_INTERVIEW_SETTINGS: &'static str = "/open-apis/hire/v1/interview_settings";

    pub const HIRE_V1_INTERVIEW_SETTING_GET: &'static str = "/open-apis/hire/v1/interview_settings/{settings_id}";

    pub const HIRE_V1_JOBS: &'static str = "/open-apis/hire/v1/jobs";

    pub const HIRE_V1_JOB_CLOSE: &'static str = "/open-apis/hire/v1/jobs/{job_id}/close";

    pub const HIRE_V1_JOB_COMBINED_CREATE: &'static str = "/open-apis/hire/v1/jobs/combined_create";

    pub const HIRE_V1_JOB_COMBINED_UPDATE: &'static str = "/open-apis/hire/v1/jobs/{job_id}/combined_update";

    pub const HIRE_V1_JOB_GET_DETAIL: &'static str = "/open-apis/hire/v1/jobs/{job_id}/get_detail";

    pub const HIRE_V1_JOB_OPEN: &'static str = "/open-apis/hire/v1/jobs/{job_id}/open";

    pub const HIRE_V1_JOB_PROCESSES: &'static str = "/open-apis/hire/v1/job_processes";

    pub const HIRE_V1_JOB_PROCESS_GET: &'static str = "/open-apis/hire/v1/job_processes/{process_id}";

    pub const HIRE_V1_JOB_REQUIREMENTS: &'static str = "/open-apis/hire/v1/job_requirements";

    pub const HIRE_V1_JOB_REQUIREMENT_GET: &'static str = "/open-apis/hire/v1/job_requirements/{requirement_id}";

    pub const HIRE_V1_LOCATIONS: &'static str = "/open-apis/hire/v1/locations";

    pub const HIRE_V1_LOCATIONS_QUERY: &'static str = "/open-apis/hire/v1/locations/query";

    pub const HIRE_V1_OFFERS: &'static str = "/open-apis/hire/v1/offers";

    pub const HIRE_V1_OFFER_GET: &'static str = "/open-apis/hire/v1/offers/{offer_id}";

    pub const HIRE_V1_OFFER_SEND: &'static str = "/open-apis/hire/v1/offers/{offer_id}/send";

    pub const HIRE_V1_OFFER_SETTINGS: &'static str = "/open-apis/hire/v1/offer_settings";

    pub const HIRE_V1_OFFER_SETTING_GET: &'static str = "/open-apis/hire/v1/offer_settings/{settings_id}";

    pub const HIRE_V1_OFFER_WITHDRAW: &'static str = "/open-apis/hire/v1/offers/{offer_id}/withdraw";

    pub const HIRE_V1_ONBOARDINGS: &'static str = "/open-apis/hire/v1/onboardings";

    pub const HIRE_V1_ONBOARDING_PROGRESS: &'static str = "/open-apis/hire/v1/onboardings/{onboarding_id}/progress/{progress_id}";

    pub const HIRE_V1_REFERRALS: &'static str = "/open-apis/hire/v1/referrals";

    pub const HIRE_V1_REFERRAL_ACCOUNTS: &'static str = "/open-apis/hire/v1/referral_accounts";

    pub const HIRE_V1_REFERRAL_ACCOUNT_GET: &'static str = "/open-apis/hire/v1/referral_accounts/{user_id}";

    pub const HIRE_V1_REFERRAL_GET: &'static str = "/open-apis/hire/v1/referrals/{referral_id}";

    pub const HIRE_V1_REFERRAL_GRANT_REWARD: &'static str = "/open-apis/hire/v1/referrals/{referral_id}/grant_reward";

    pub const HIRE_V1_REFERRAL_REWARD_SETTINGS: &'static str = "/open-apis/hire/v1/referral_reward_settings";

    pub const HIRE_V1_REGISTRATION_FORMS: &'static str = "/open-apis/hire/v1/registration_forms";

    pub const HIRE_V1_ROLES: &'static str = "/open-apis/hire/v1/roles";

    pub const HIRE_V1_ROLE_GET: &'static str = "/open-apis/hire/v1/roles/{role_id}";

    pub const HIRE_V1_SUBJECTS: &'static str = "/open-apis/hire/v1/subjects";

    pub const HIRE_V1_SUBJECT_DISABLE: &'static str = "/open-apis/hire/v1/subjects/{subject_id}/disable";

    pub const HIRE_V1_SUBJECT_ENABLE: &'static str = "/open-apis/hire/v1/subjects/{subject_id}/enable";

    pub const HIRE_V1_SUBJECT_GET: &'static str = "/open-apis/hire/v1/subjects/{subject_id}";

    pub const HIRE_V1_TALENTS: &'static str = "/open-apis/hire/v1/talents";

    pub const HIRE_V1_TALENTS_BATCH_IMPORT: &'static str = "/open-apis/hire/v1/talents/batch_import";

    pub const HIRE_V1_TALENT_APPLICATIONS: &'static str = "/open-apis/hire/v1/talents/{talent_id}/applications";

    pub const HIRE_V1_TALENT_GET: &'static str = "/open-apis/hire/v1/talents/{talent_id}";

    pub const HIRE_V1_TALENT_POOLS: &'static str = "/open-apis/hire/v1/talent_pools";

    pub const HIRE_V1_TALENT_POOL_GET: &'static str = "/open-apis/hire/v1/talent_pools/{pool_id}";

    pub const HIRE_V1_TALENT_POOL_TALENTS: &'static str = "/open-apis/hire/v1/talent_pools/{pool_id}/talents";

    pub const HIRE_V1_TALENT_POOL_TALENT_GET: &'static str = "/open-apis/hire/v1/talent_pools/{pool_id}/talents/{talent_id}";

    pub const HIRE_V1_TALENT_TAGS: &'static str = "/open-apis/hire/v1/talent_tags";

    pub const HIRE_V1_USER_ROLES: &'static str = "/open-apis/hire/v1/users/{user_id}/roles";

    pub const HIRE_V1_WEBSITE_APPLICATIONS: &'static str = "/open-apis/hire/v1/website/applications";

    pub const HIRE_V1_WEBSITE_APPLICATION_CONVERT: &'static str = "/open-apis/hire/v1/website/applications/{website_application_id}/convert";

    pub const HIRE_V1_WEBSITE_CONFIGURATION: &'static str = "/open-apis/hire/v1/website/configuration";

    pub const HIRE_V1_WEBSITE_JOBS: &'static str = "/open-apis/hire/v1/website/jobs";

    pub const HIRE_V1_WEBSITE_JOBS_PUBLISH: &'static str = "/open-apis/hire/v1/website/jobs/publish";

    pub const HIRE_V1_WEBSITE_JOB_UNPUBLISH: &'static str = "/open-apis/hire/v1/website/jobs/{job_id}/unpublish";

    pub const HIRE_V1_WEBSITE_STATISTICS: &'static str = "/open-apis/hire/v1/website/statistics";

    pub const HUMAN_AUTHENTICATION_V1_FACE_IMAGES: &'static str = "/open-apis/human_authentication/v1/face_images";

    pub const HUMAN_AUTHENTICATION_V1_FACE_IMAGES_CROP: &'static str = "/open-apis/human_authentication/v1/face_images/crop";

    pub const HUMAN_AUTHENTICATION_V1_IDENTITIES: &'static str = "/open-apis/human_authentication/v1/identities";

    pub const HUMAN_AUTHENTICATION_V1_IDENTITY_RESULT: &'static str = "/open-apis/human_authentication/v1/identities/{identity_id}/result";

    pub const IM_CHAT_ADD_MEMBERS: &'static str = "/open-apis/im/v1/chats/{chat_id}/members";

    pub const IM_CHAT_CREATE: &'static str = "/open-apis/im/v1/chats";

    pub const IM_CHAT_DELETE: &'static str = "/open-apis/im/v1/chats/{chat_id}";

    pub const IM_CHAT_GET: &'static str = "/open-apis/im/v1/chats/{chat_id}";

    pub const IM_CHAT_MEMBERS: &'static str = "/open-apis/im/v1/chats/{chat_id}/members";

    pub const IM_CHAT_REMOVE_MEMBERS: &'static str = "/open-apis/im/v1/chats/{chat_id}/members/batch_delete";

    pub const IM_CHAT_UPDATE: &'static str = "/open-apis/im/v1/chats/{chat_id}";

    pub const IM_V1_BATCH_MESSAGES: &'static str = "/open-apis/im/v1/batch_messages";

    pub const IM_V1_BATCH_MESSAGE_PROGRESS: &'static str = "/open-apis/im/v1/batch_messages/{batch_message_id}/get_progress";

    pub const IM_V1_BATCH_MESSAGE_READ_USER: &'static str = "/open-apis/im/v1/batch_messages/{batch_message_id}/read_user";

    pub const IM_V1_DELETE_BATCH_MESSAGE: &'static str = "/open-apis/im/v1/batch_messages/{batch_message_id}";

    pub const IM_V1_DELETE_MESSAGE: &'static str = "/open-apis/im/v1/messages/{message_id}";

    pub const IM_V1_DELETE_MESSAGE_REACTION: &'static str = "/open-apis/im/v1/messages/{message_id}/reactions/{reaction_id}";

    pub const IM_V1_DELETE_PIN: &'static str = "/open-apis/im/v1/pins/{pin_id}";

    pub const IM_V1_DOWNLOAD_FILE: &'static str = "/open-apis/im/v1/files/{file_key}";

    pub const IM_V1_DOWNLOAD_IMAGE: &'static str = "/open-apis/im/v1/images/{image_key}";

    pub const IM_V1_FILES: &'static str = "/open-apis/im/v1/files";

    pub const IM_V1_FORWARD_MESSAGE: &'static str = "/open-apis/im/v1/messages/{message_id}/forward";

    pub const IM_V1_GET_MESSAGE: &'static str = "/open-apis/im/v1/messages/{message_id}";

    pub const IM_V1_IMAGES: &'static str = "/open-apis/im/v1/images";

    pub const IM_V1_IMAGES_UPLOAD: &'static str = "/open-apis/im/v1/images";

    pub const IM_V1_LIST_MESSAGE: &'static str = "/open-apis/im/v1/messages";

    pub const IM_V1_MESSAGE_DELAY_UPDATE: &'static str = "/open-apis/im/v1/messages/{message_id}/delay_update";

    pub const IM_V1_MESSAGE_DELETE: &'static str = "/open-apis/im/v1/messages/{message_id}";

    pub const IM_V1_MESSAGE_GET: &'static str = "/open-apis/im/v1/messages/{message_id}";

    pub const IM_V1_MESSAGE_PATCH: &'static str = "/open-apis/im/v1/messages/{message_id}";

    pub const IM_V1_MESSAGE_REACTIONS: &'static str = "/open-apis/im/v1/messages/{message_id}/reactions";

    pub const IM_V1_MESSAGE_URGENT_APP: &'static str = "/open-apis/im/v1/messages/{message_id}/urgent_app";

    pub const IM_V1_MESSAGE_URGENT_PHONE: &'static str = "/open-apis/im/v1/messages/{message_id}/urgent_phone";

    pub const IM_V1_MESSAGE_URGENT_SMS: &'static str = "/open-apis/im/v1/messages/{message_id}/urgent_sms";

    pub const IM_V1_MESSAGE_URL_PREVIEW_BATCH_UPDATE: &'static str = "/open-apis/im/v1/messages/{message_id}/url_preview/batch_update";

    pub const IM_V1_PINS: &'static str = "/open-apis/im/v1/pins";

    pub const IM_V1_READ_MESSAGE: &'static str = "/open-apis/im/v1/messages/{message_id}/read_users";

    pub const IM_V1_REPLY_MESSAGE: &'static str = "/open-apis/im/v1/messages/{message_id}/reply";

    pub const IM_V1_SEND_MESSAGE: &'static str = "/open-apis/im/v1/messages";

    pub const IM_V1_UPDATE_MESSAGE: &'static str = "/open-apis/im/v1/messages/{message_id}";

    pub const IM_V2_APP_FEED_CARD: &'static str = "/open-apis/im/v2/app_feed_card";

    pub const IM_V2_DELETE_APP_FEED_CARD: &'static str = "/open-apis/im/v2/app_feed_card/{card_id}";

    pub const IM_V2_GET_APP_FEED_CARD: &'static str = "/open-apis/im/v2/app_feed_card/{card_id}";

    pub const IM_V2_GROUPS_BOTS_PATCH: &'static str = "/open-apis/im/v2/groups-bots/patch";

    pub const IM_V2_GROUPS_BOTS_TIME_SENSITIVE: &'static str = "/open-apis/im/v2/groups-bots/bot_time_sentive";

    pub const IM_V2_GROUPS_BOTS_UPDATE: &'static str = "/open-apis/im/v2/groups-bots/{message_id}/update";

    pub const LINGO_CLASSIFICATION_LIST: &'static str = "/open-apis/lingo/v1/classifications";

    pub const LINGO_DRAFT_CREATE: &'static str = "/open-apis/lingo/v1/drafts";

    pub const LINGO_DRAFT_UPDATE: &'static str = "/open-apis/lingo/v1/drafts/{draft_id}";

    pub const LINGO_ENTITY_CREATE: &'static str = "/open-apis/lingo/v1/entities";

    pub const LINGO_ENTITY_GET: &'static str = "/open-apis/lingo/v1/entities/{entity_id}";

    pub const LINGO_ENTITY_HIGHLIGHT: &'static str = "/open-apis/lingo/v1/entities/highlight";

    pub const LINGO_ENTITY_MATCH: &'static str = "/open-apis/lingo/v1/entities/match";

    pub const LINGO_ENTITY_SEARCH: &'static str = "/open-apis/lingo/v1/entities/search";

    pub const LINGO_ENTITY_UPDATE: &'static str = "/open-apis/lingo/v1/entities/{entity_id}";

    pub const LINGO_FILE_DOWNLOAD: &'static str = "/open-apis/lingo/v1/file/download/{file_token}";

    pub const LINGO_FILE_UPLOAD: &'static str = "/open-apis/lingo/v1/file/upload";

    pub const LINGO_REPO_LIST: &'static str = "/open-apis/lingo/v1/repos";

    pub const MAIL_V1_MAILGROUP: &'static str = "/open-apis/mail/v1/mailgroups/{mailgroup_id}";

    pub const MAIL_V1_MAILGROUPS: &'static str = "/open-apis/mail/v1/mailgroups";

    pub const MAIL_V1_MAILGROUP_MANAGERS: &'static str = "/open-apis/mail/v1/mailgroups/{mailgroup_id}/managers";

    pub const MAIL_V1_MAILGROUP_MANAGERS_BATCH_CREATE: &'static str = "/open-apis/mail/v1/mailgroups/{mailgroup_id}/managers/batch_create";

    pub const MAIL_V1_MAILGROUP_MANAGERS_BATCH_DELETE: &'static str = "/open-apis/mail/v1/mailgroups/{mailgroup_id}/managers/batch_delete";

    pub const MAIL_V1_USER_MAILBOX_EVENTS_SUBSCRIBE: &'static str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/events/subscribe";

    pub const MAIL_V1_USER_MAILBOX_EVENTS_SUBSCRIPTION: &'static str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/events/subscription";

    pub const MAIL_V1_USER_MAILBOX_EVENTS_UNSUBSCRIBE: &'static str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/events/unsubscribe";

    pub const MAIL_V1_USER_MAILBOX_FOLDER: &'static str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/folders/{folder_id}";

    pub const MAIL_V1_USER_MAILBOX_FOLDERS: &'static str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/folders";

    pub const MAIL_V1_USER_MAILBOX_MAIL_CONTACT: &'static str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/mail_contacts/{contact_id}";

    pub const MAIL_V1_USER_MAILBOX_MAIL_CONTACTS: &'static str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/mail_contacts";

    pub const MAIL_V1_USER_MAILBOX_MESSAGE: &'static str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/messages/{message_id}";

    pub const MAIL_V1_USER_MAILBOX_MESSAGES: &'static str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/messages";

    pub const MAIL_V1_USER_MAILBOX_MESSAGES_GET_BY_CARD: &'static str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/messages/get_by_card";

    pub const MAIL_V1_USER_MAILBOX_MESSAGE_ATTACHMENT_DOWNLOAD_URL: &'static str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/messages/{message_id}/attachments/{attachment_id}/download_url";

    pub const MAIL_V1_USER_MAILBOX_RULE: &'static str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/rules/{rule_id}";

    pub const MAIL_V1_USER_MAILBOX_RULES: &'static str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/rules";

    pub const MAIL_V1_USER_MAILBOX_RULES_REORDER: &'static str = "/open-apis/mail/v1/user_mailboxes/{user_mailbox_id}/rules/reorder";

    pub const MDM_V1_COUNTRY_REGIONS: &'static str = "/open-apis/mdm/v1/country_regions";

    pub const MDM_V1_COUNTRY_REGIONS_BATCH_GET: &'static str = "/open-apis/mdm/v1/country_regions/batch_get";

    pub const MDM_V1_USER_AUTH_DATA_RELATIONS_BIND: &'static str = "/open-apis/mdm/v1/user_auth_data_relations/bind";

    pub const MDM_V1_USER_AUTH_DATA_RELATIONS_UNBIND: &'static str = "/open-apis/mdm/v1/user_auth_data_relations/unbind";

    pub const MESSAGE_V4_BATCH_SEND: &'static str = "/open-apis/message/v4/batch_send/";

    pub const MINUTES_V1_MEDIA_GET: &'static str = "/open-apis/minutes/v1/{minute_token}/media";

    pub const MINUTES_V1_MINUTE_GET: &'static str = "/open-apis/minutes/v1/{minute_token}";

    pub const MINUTES_V1_STATISTICS_GET: &'static str = "/open-apis/minutes/v1/{minute_token}/statistics";

    pub const MINUTES_V1_TRANSCRIPT_GET: &'static str = "/open-apis/minutes/v1/{minute_token}/transcript";

    pub const OKR_V1_OKRS: &'static str = "/open-apis/okr/v1/okrs";

    pub const OKR_V1_OKRS_BATCH_GET: &'static str = "/open-apis/okr/v1/okrs/batch_get";

    pub const OKR_V1_PERIODS: &'static str = "/open-apis/okr/v1/periods";

    pub const OKR_V1_PERIOD_GET: &'static str = "/open-apis/okr/v1/periods/{period_id}";

    pub const OKR_V1_PERIOD_RULES: &'static str = "/open-apis/okr/v1/period_rules";

    pub const OKR_V1_PROGRESS_RECORDS: &'static str = "/open-apis/okr/v1/progress_records";

    pub const OKR_V1_PROGRESS_RECORDS_UPLOAD: &'static str = "/open-apis/okr/v1/progress_records/upload";

    pub const OKR_V1_PROGRESS_RECORD_OPERATION: &'static str = "/open-apis/okr/v1/progress_records/{progress_id}";

    pub const OKR_V1_REVIEWS_QUERY: &'static str = "/open-apis/okr/v1/reviews/query";

    pub const OPTICAL_CHAR_RECOGNITION_V1_BASIC_RECOGNIZE: &'static str = "/open-apis/optical_char_recognition/v1/image/basic_recognize";

    pub const PASSPORT_V1_SESSIONS_LOGOUT: &'static str = "/open-apis/passport/v1/sessions/logout";

    pub const PAYROLL_V1_ACCT_ITEMS: &'static str = "/open-apis/payroll/v1/acct_items";

    pub const PAYROLL_V1_COST_ALLOCATION_PLANS: &'static str = "/open-apis/payroll/v1/cost_allocation_plans";

    pub const PAYROLL_V1_COST_ALLOCATION_REPORTS: &'static str = "/open-apis/payroll/v1/cost_allocation_reports";

    pub const PAYROLL_V1_DATASOURCES: &'static str = "/open-apis/payroll/v1/datasources";

    pub const PAYROLL_V1_DATASOURCE_RECORDS_QUERY: &'static str = "/open-apis/payroll/v1/datasources/{datasource_id}/records/query";

    pub const PAYROLL_V1_DATASOURCE_RECORDS_SAVE: &'static str = "/open-apis/payroll/v1/datasources/{datasource_id}/records/save";

    pub const PAYROLL_V1_PAYGROUPS: &'static str = "/open-apis/payroll/v1/paygroups";

    pub const PAYROLL_V1_PAYMENT_ACTIVITIES: &'static str = "/open-apis/payroll/v1/payment_activities";

    pub const PAYROLL_V1_PAYMENT_ACTIVITY_ARCHIVE: &'static str = "/open-apis/payroll/v1/payment_activities/{payment_activity_id}/archive";

    pub const PAYROLL_V1_PAYMENT_DETAILS: &'static str = "/open-apis/payroll/v1/payment_activities/{payment_activity_id}/payment_details";

    pub const PAYROLL_V1_PAYMENT_DETAILS_QUERY: &'static str = "/open-apis/payroll/v1/payment_activities/{payment_activity_id}/payment_details/query";

    pub const PERFORMANCE_ACTIVITIES_QUERY: &'static str = "/open-apis/performance/v1/review_config/semester_activity/activities";

    pub const PERFORMANCE_ADDITIONAL_INFO_DELETE: &'static str = "/open-apis/performance/v1/review_config/semester_activity/additional_information/delete";

    pub const PERFORMANCE_ADDITIONAL_INFO_IMPORT: &'static str = "/open-apis/performance/v1/review_config/semester_activity/additional_information/import";

    pub const PERFORMANCE_ADDITIONAL_INFO_QUERY: &'static str = "/open-apis/performance/v1/review_config/semester_activity/additional_information/query";

    pub const PERFORMANCE_METRICS_QUERY: &'static str = "/open-apis/performance/v1/review_config/metrics/query";

    pub const PERFORMANCE_METRIC_FIELDS_QUERY: &'static str = "/open-apis/performance/v1/review_config/metric_fields/query";

    pub const PERFORMANCE_METRIC_TAGS: &'static str = "/open-apis/performance/v1/review_config/metric_tags";

    pub const PERFORMANCE_METRIC_TEMPLATES_QUERY: &'static str = "/open-apis/performance/v1/review_config/metric_templates/query";

    pub const PERFORMANCE_REVIEWEES_QUERY: &'static str = "/open-apis/performance/v1/review_config/semester_activity/reviewees/query";

    pub const PERFORMANCE_REVIEW_ITEMS_QUERY: &'static str = "/open-apis/performance/v1/review_config/review_items/query";

    pub const PERFORMANCE_REVIEW_TEMPLATES_QUERY: &'static str = "/open-apis/performance/v1/review_config/review_templates/query";

    pub const PERFORMANCE_SEMESTER_LIST: &'static str = "/open-apis/performance/v1/review_config/semester_activity/semesters";

    pub const PERFORMANCE_TAG_QUESTIONS_QUERY: &'static str = "/open-apis/performance/v1/review_config/tag_questions/query";

    pub const PERFORMANCE_USER_GROUP_WRITE: &'static str = "/open-apis/performance/v1/review_config/semester_activity/user_group_user_rel/write";

    pub const PERFORMANCE_V1_METRIC_DETAIL_IMPORT: &'static str = "/open-apis/performance/v1/metric_detail/import";

    pub const PERFORMANCE_V1_METRIC_DETAIL_QUERY: &'static str = "/open-apis/performance/v1/metric_detail/query";

    pub const PERFORMANCE_V1_REVIEW_DATA_DETAILS_QUERY: &'static str = "/open-apis/performance/v1/review_data/details/query";

    pub const PERFORMANCE_V1_REVIEW_DATA_QUERY: &'static str = "/open-apis/performance/v1/review_data/query";

    pub const PERFORMANCE_V1_STAGE_TASK_FIND_BY_PAGE: &'static str = "/open-apis/performance/v1/stage_task/find_by_page";

    pub const PERFORMANCE_V1_STAGE_TASK_FIND_BY_USER_LIST: &'static str = "/open-apis/performance/v1/stage_task/find_by_user_list";

    pub const PERSONAL_SETTINGS_V1_SYSTEM_STATUSES: &'static str = "/open-apis/personal_settings/v1/system_statuses";

    pub const PERSONAL_SETTINGS_V1_SYSTEM_STATUS_BATCH_CLOSE: &'static str = "/open-apis/personal_settings/v1/system_statuses/batch_close";

    pub const PERSONAL_SETTINGS_V1_SYSTEM_STATUS_BATCH_OPEN: &'static str = "/open-apis/personal_settings/v1/system_statuses/batch_open";

    pub const PERSONAL_SETTINGS_V1_SYSTEM_STATUS_OPERATION: &'static str = "/open-apis/personal_settings/v1/system_statuses/{system_status_id}";

    pub const REPORT_V1_RULES_QUERY: &'static str = "/open-apis/report/v1/rules/query";

    pub const REPORT_V1_RULE_VIEWS_OPERATION: &'static str = "/open-apis/report/v1/rule-views/{view_id}";

    pub const REPORT_V1_TASKS_QUERY: &'static str = "/open-apis/report/v1/tasks/query";

    pub const SEARCH_V1_USER: &'static str = "/open-apis/search/v1/user";

    pub const SEARCH_V2_APP: &'static str = "/open-apis/search/v2/app";

    pub const SEARCH_V2_DATA_ITEM_BATCH_CREATE: &'static str = "/open-apis/search/v2/data_sources/{data_source_id}/items/batch_create";

    pub const SEARCH_V2_DATA_ITEM_CREATE: &'static str = "/open-apis/search/v2/data_sources/{data_source_id}/items";

    pub const SEARCH_V2_DATA_ITEM_OPERATION: &'static str = "/open-apis/search/v2/data_sources/{data_source_id}/items/{data_item_id}";

    pub const SEARCH_V2_DATA_SOURCES: &'static str = "/open-apis/search/v2/data_sources";

    pub const SEARCH_V2_DATA_SOURCE_OPERATION: &'static str = "/open-apis/search/v2/data_sources/{data_source_id}";

    pub const SEARCH_V2_MESSAGE: &'static str = "/open-apis/search/v2/message";

    pub const SEARCH_V2_SCHEMA_CREATE: &'static str = "/open-apis/search/v2/data_sources/{data_source_id}/schemas";

    pub const SEARCH_V2_SCHEMA_OPERATION: &'static str = "/open-apis/search/v2/data_sources/{data_source_id}/schemas/{schema_id}";

    pub const SECURITY_AND_COMPLIANCE_V1_AUDIT_DATAS: &'static str = "/open-apis/security_and_compliance/v1/audit_datas";

    pub const SECURITY_AND_COMPLIANCE_V1_OPENAPI_LOGS_LIST_DATA: &'static str = "/open-apis/security_and_compliance/v1/openapi_logs/list_data";

    pub const SHEETS_V2_SPREADSHEET_DIMENSION_RANGE: &'static str = "/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/dimension_range";

    pub const SHEETS_V2_SPREADSHEET_INSERT_DIMENSION_RANGE: &'static str = "/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/insert_dimension_range";

    pub const SHEETS_V2_SPREADSHEET_MERGE_CELLS: &'static str = "/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/merge_cells";

    pub const SHEETS_V2_SPREADSHEET_SHEETS_BATCH_UPDATE: &'static str = "/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/sheets_batch_update";

    pub const SHEETS_V2_SPREADSHEET_STYLE: &'static str = "/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/style";

    pub const SHEETS_V2_SPREADSHEET_STYLES_BATCH_UPDATE: &'static str = "/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/styles_batch_update";

    pub const SHEETS_V2_SPREADSHEET_UNMERGE_CELLS: &'static str = "/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/unmerge_cells";

    pub const SHEETS_V2_SPREADSHEET_VALUES: &'static str = "/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values";

    pub const SHEETS_V2_SPREADSHEET_VALUES_APPEND: &'static str = "/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values_append";

    pub const SHEETS_V2_SPREADSHEET_VALUES_BATCH_GET: &'static str = "/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values_batch_get";

    pub const SHEETS_V2_SPREADSHEET_VALUES_BATCH_UPDATE: &'static str = "/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values_batch_update";

    pub const SHEETS_V2_SPREADSHEET_VALUES_GET: &'static str = "/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values/:range";

    pub const SHEETS_V2_SPREADSHEET_VALUES_IMAGE: &'static str = "/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values_image";

    pub const SHEETS_V2_SPREADSHEET_VALUES_PREPEND: &'static str = "/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values_prepend";

    pub const SHEETS_V2_SPREADSHEET_VALUES_RANGE: &'static str = "/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values/{range}";

    pub const SHEETS_V3_SPREADSHEETS: &'static str = "/open-apis/sheets/v3/spreadsheets";

    pub const SHEETS_V3_SPREADSHEET_CONDITION_FORMAT: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/conditionFormat";

    pub const SHEETS_V3_SPREADSHEET_CREATE: &'static str = "/open-apis/sheets/v3/spreadsheets";

    pub const SHEETS_V3_SPREADSHEET_DATA_VALIDATION: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/dataValidation";

    pub const SHEETS_V3_SPREADSHEET_DATA_VALIDATION_GET: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/dataValidation/{}";

    pub const SHEETS_V3_SPREADSHEET_DIMENSION_RANGE: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/dimension_range";

    pub const SHEETS_V3_SPREADSHEET_DIMENSION_RANGE_INSERT: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/dimension_range:insert";

    pub const SHEETS_V3_SPREADSHEET_FILTER: &'static str = "/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/sheets/{sheet_id}/filter";

    pub const SHEETS_V3_SPREADSHEET_FILTER_VIEWS: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter_views";

    pub const SHEETS_V3_SPREADSHEET_FILTER_VIEW_CONDITIONS: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter_views/{}/conditions";

    pub const SHEETS_V3_SPREADSHEET_FILTER_VIEW_CONDITION_GET: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter_views/{}/conditions/{}";

    pub const SHEETS_V3_SPREADSHEET_FILTER_VIEW_GET: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter_views/{}";

    pub const SHEETS_V3_SPREADSHEET_FLOAT_IMAGES: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/float_images";

    pub const SHEETS_V3_SPREADSHEET_FLOAT_IMAGE_GET: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/float_images/{}";

    pub const SHEETS_V3_SPREADSHEET_GET: &'static str = "/open-apis/sheets/v3/spreadsheets/{}";

    pub const SHEETS_V3_SPREADSHEET_MERGE_CELLS: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/merge_cells";

    pub const SHEETS_V3_SPREADSHEET_MOVE_DIMENSION: &'static str = "/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/sheets/{sheet_id}/move_dimension";

    pub const SHEETS_V3_SPREADSHEET_PATCH: &'static str = "/open-apis/sheets/v3/spreadsheets/{}";

    pub const SHEETS_V3_SPREADSHEET_PROTECT_RANGE: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/protect_range";

    pub const SHEETS_V3_SPREADSHEET_PROTECT_RANGE_GET: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/protect_range/{}";

    pub const SHEETS_V3_SPREADSHEET_SHEETS_QUERY: &'static str = "/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/sheets/query";

    pub const SHEETS_V3_SPREADSHEET_SHEET_FIND: &'static str = "/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/sheets/{sheet_id}/find";

    pub const SHEETS_V3_SPREADSHEET_SHEET_GET: &'static str = "/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/sheets/{sheet_id}";

    pub const SHEETS_V3_SPREADSHEET_SHEET_REPLACE: &'static str = "/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/sheets/{sheet_id}/replace";

    pub const SHEETS_V3_SPREADSHEET_STYLE: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/style";

    pub const SHEETS_V3_SPREADSHEET_STYLES_BATCH_UPDATE: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/styles/batch_update";

    pub const SHEETS_V3_SPREADSHEET_UNMERGE_CELLS: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/unmerge_cells";

    pub const SHEETS_V3_SPREADSHEET_VALUES_APPEND: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/values/{}/append";

    pub const SHEETS_V3_SPREADSHEET_VALUES_BATCH_GET: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/values/batch_get";

    pub const SHEETS_V3_SPREADSHEET_VALUES_BATCH_UPDATE: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/values/batch_update";

    pub const SHEETS_V3_SPREADSHEET_VALUES_GET: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/values/{}";

    pub const SHEETS_V3_SPREADSHEET_VALUES_IMAGE: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/values_image";

    pub const SHEETS_V3_SPREADSHEET_VALUES_PREPEND: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/values/{}/prepend";

    pub const SPEECH_TO_TEXT_V1_FILE_RECOGNIZE: &'static str = "/open-apis/speech_to_text/v1/speech/file_recognize";

    pub const SPEECH_TO_TEXT_V1_STREAM_RECOGNIZE: &'static str = "/open-apis/speech_to_text/v1/speech/stream_recognize";

    pub const SUITE_DOCS_SEARCH_OBJECT: &'static str = "/open-apis/suite/docs-api/search/object";

    pub const TASK_V2_ATTACHMENTS: &'static str = "/open-apis/task/v2/attachments";

    pub const TASK_V2_ATTACHMENTS_UPLOAD: &'static str = "/open-apis/task/v2/attachments/upload";

    pub const TASK_V2_ATTACHMENT_GET: &'static str = "/open-apis/task/v2/attachments/{attachment_guid}";

    pub const TASK_V2_CUSTOM_FIELDS: &'static str = "/open-apis/task/v2/custom_fields";

    pub const TASK_V2_CUSTOM_FIELD_ADD: &'static str = "/open-apis/task/v2/custom_fields/{custom_field_guid}/add";

    pub const TASK_V2_CUSTOM_FIELD_GET: &'static str = "/open-apis/task/v2/custom_fields/{custom_field_guid}";

    pub const TASK_V2_CUSTOM_FIELD_OPTIONS: &'static str = "/open-apis/task/v2/custom_fields/{custom_field_guid}/options";

    pub const TASK_V2_CUSTOM_FIELD_OPTION_GET: &'static str = "/open-apis/task/v2/custom_fields/{custom_field_guid}/options/{option_guid}";

    pub const TASK_V2_CUSTOM_FIELD_REMOVE: &'static str = "/open-apis/task/v2/custom_fields/{custom_field_guid}/remove";

    pub const TASK_V2_SECTIONS: &'static str = "/open-apis/task/v2/sections";

    pub const TASK_V2_SECTION_GET: &'static str = "/open-apis/task/v2/sections/{section_guid}";

    pub const TASK_V2_SECTION_TASKS: &'static str = "/open-apis/task/v2/sections/{section_guid}/tasks";

    pub const TASK_V2_TASKLISTS: &'static str = "/open-apis/task/v2/tasklists";

    pub const TASK_V2_TASKLIST_ACTIVITY_SUBSCRIPTIONS: &'static str = "/open-apis/task/v2/tasklists/{tasklist_guid}/activity_subscriptions";

    pub const TASK_V2_TASKLIST_ACTIVITY_SUBSCRIPTION_GET: &'static str = "/open-apis/task/v2/tasklists/{tasklist_guid}/activity_subscriptions/{activity_subscription_guid}";

    pub const TASK_V2_TASKLIST_ADD_MEMBERS: &'static str = "/open-apis/task/v2/tasklists/{tasklist_guid}/add_members";

    pub const TASK_V2_TASKLIST_GET: &'static str = "/open-apis/task/v2/tasklists/{tasklist_guid}";

    pub const TASK_V2_TASKLIST_REMOVE_MEMBERS: &'static str = "/open-apis/task/v2/tasklists/{tasklist_guid}/remove_members";

    pub const TASK_V2_TASKLIST_TASKS: &'static str = "/open-apis/task/v2/tasklists/{tasklist_guid}/tasks";

    pub const TASK_V2_TASKS: &'static str = "/open-apis/task/v2/tasks";

    pub const TASK_V2_TASK_ADD_DEPENDENCIES: &'static str = "/open-apis/task/v2/tasks/{task_guid}/add_dependencies";

    pub const TASK_V2_TASK_ADD_MEMBERS: &'static str = "/open-apis/task/v2/tasks/{task_guid}/add_members";

    pub const TASK_V2_TASK_ADD_REMINDERS: &'static str = "/open-apis/task/v2/tasks/{task_guid}/add_reminders";

    pub const TASK_V2_TASK_ADD_TASKLIST: &'static str = "/open-apis/task/v2/tasks/{task_guid}/add_tasklist";

    pub const TASK_V2_TASK_COMMENTS: &'static str = "/open-apis/task/v2/tasks/{task_guid}/comments";

    pub const TASK_V2_TASK_COMMENT_GET: &'static str = "/open-apis/task/v2/tasks/{task_guid}/comments/{comment_id}";

    pub const TASK_V2_TASK_GET: &'static str = "/open-apis/task/v2/tasks/{task_guid}";

    pub const TASK_V2_TASK_REMOVE_DEPENDENCIES: &'static str = "/open-apis/task/v2/tasks/{task_guid}/remove_dependencies";

    pub const TASK_V2_TASK_REMOVE_MEMBERS: &'static str = "/open-apis/task/v2/tasks/{task_guid}/remove_members";

    pub const TASK_V2_TASK_REMOVE_REMINDERS: &'static str = "/open-apis/task/v2/tasks/{task_guid}/remove_reminders";

    pub const TASK_V2_TASK_SUBTASKS: &'static str = "/open-apis/task/v2/tasks/{task_guid}/subtasks";

    pub const TRANSLATION_V1_TEXT_DETECT: &'static str = "/open-apis/translation/v1/text/detect";

    pub const TRANSLATION_V1_TEXT_TRANSLATE: &'static str = "/open-apis/translation/v1/text/translate";

    pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATIONS: &'static str = "/open-apis/trust_party/v1/collaboration_organizations";

    pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATIONS_ADMIN: &'static str = "/open-apis/trust_party/v1/collaboration_organizations/admin";

    pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_DEPARTMENT_GET: &'static str = "/open-apis/trust_party/v1/collaboration_organizations/{org_id}/departments/{department_id}";

    pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_GET: &'static str = "/open-apis/trust_party/v1/collaboration_organizations/{org_id}";

    pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_SHARED_MEMBER_SCOPES: &'static str = "/open-apis/trust_party/v1/collaboration_organizations/{org_id}/shared_member_scopes";

    pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_USER_GET: &'static str = "/open-apis/trust_party/v1/collaboration_organizations/{org_id}/users/{user_id}";

    pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_VISIBLE: &'static str = "/open-apis/trust_party/v1/collaboration_organizations/{org_id}/visible_organization";

    pub const TRUST_PARTY_V1_SEARCHABLE_VISIBLE_RULES: &'static str = "/open-apis/trust_party/v1/searchable_and_visible_rules";

    pub const TRUST_PARTY_V1_SEARCHABLE_VISIBLE_RULE_OPERATION: &'static str = "/open-apis/trust_party/v1/searchable_and_visible_rules/{rule_id}";

    pub const WIKI_V2_SEARCH: &'static str = "/open-apis/wiki/v2/search";

    pub const WIKI_V2_SPACES: &'static str = "/open-apis/wiki/v2/spaces";

    pub const WIKI_V2_SPACE_CREATE: &'static str = "/open-apis/wiki/v2/spaces";

    pub const WIKI_V2_SPACE_GET: &'static str = "/open-apis/wiki/v2/spaces/{}";

    pub const WIKI_V2_SPACE_MEMBERS: &'static str = "/open-apis/wiki/v2/spaces/{}/members";

    pub const WIKI_V2_SPACE_MEMBER_CREATE: &'static str = "/open-apis/wiki/v2/spaces/{}/members";

    pub const WIKI_V2_SPACE_MEMBER_DELETE: &'static str = "/open-apis/wiki/v2/spaces/{}/members/{}";

    pub const WIKI_V2_SPACE_NODES: &'static str = "/open-apis/wiki/v2/spaces/{}/nodes";

    pub const WIKI_V2_SPACE_NODE_COPY: &'static str = "/open-apis/wiki/v2/spaces/{}/nodes/{}/copy";

    pub const WIKI_V2_SPACE_NODE_CREATE: &'static str = "/open-apis/wiki/v2/spaces/{}/nodes";

    pub const WIKI_V2_SPACE_NODE_GET: &'static str = "/open-apis/wiki/v2/spaces/{}/nodes/{}";

    pub const WIKI_V2_SPACE_NODE_MOVE: &'static str = "/open-apis/wiki/v2/spaces/{}/nodes/{}/move";

    pub const WIKI_V2_SPACE_NODE_UPDATE_TITLE: &'static str = "/open-apis/wiki/v2/spaces/{}/nodes/{}/update_title";

    pub const WIKI_V2_SPACE_SETTING_UPDATE: &'static str = "/open-apis/wiki/v2/spaces/{}/setting";

    pub const WIKI_V2_TASKS_MOVE_DOCS_TO_WIKI: &'static str = "/open-apis/wiki/v2/tasks/move_docs_to_wiki";

    pub const WIKI_V2_TASK_GET: &'static str = "/open-apis/wiki/v2/tasks/{}";

    pub const WORKPLACE_ACCESS_DATA_SEARCH: &'static str = "/open-apis/workplace/v1/workplace_access_data/search";

    pub const WORKPLACE_APP_RECOMMEND_FAVOURITE: &'static str = "/open-apis/workplace/v1/app_recommend_rule/favourite";

    pub const WORKPLACE_APP_RECOMMEND_LIST: &'static str = "/open-apis/workplace/v1/app_recommend_rule/list";

    pub const WORKPLACE_APP_RECOMMEND_RECOMMEND: &'static str = "/open-apis/workplace/v1/app_recommend_rule/recommend";

    pub const WORKPLACE_CUSTOM_ACCESS_DATA_SEARCH: &'static str = "/open-apis/workplace/v1/custom_workplace_access_data/search";

    pub const WORKPLACE_WIDGET_ACCESS_DATA_SEARCH: &'static str = "/open-apis/workplace/v1/custom_workplace_widget_access_data/search";

    // ==================== 分类访问方法 ====================

    /// 获取即时消息相关端点
    pub fn messaging() -> Messaging {
        Messaging
    }

    /// 获取内容管理相关端点
    pub fn content() -> Content {
        Content
    }

    /// 获取人力资源管理相关端点
    pub fn hr_management() -> HrManagement {
        HrManagement
    }

    /// 获取协作工具相关端点
    pub fn collaboration() -> Collaboration {
        Collaboration
    }

    /// 获取AI服务相关端点
    pub fn ai_services() -> AiServices {
        AiServices
    }

    /// 获取管理和认证相关端点
    pub fn admin() -> Admin {
        Admin
    }

    /// 获取集成和外部系统相关端点
    pub fn integration() -> Integration {
        Integration
    }

// 重新导出分类模块，提供更细粒度的访问方式
pub mod categories;