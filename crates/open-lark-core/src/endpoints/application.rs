//! Application 服务端点常量定义（v6）

// ==================== 应用管理 ====================
pub const APPLICATION_V6_APP_GET: &str = "/open-apis/application/v6/applications/{app_id}";
pub const APPLICATION_V6_APP_TRANSFER_OWNER: &str =
    "/open-apis/application/v6/applications/{app_id}/transfer_owner";
pub const APPLICATION_V6_APP_COLLABORATORS: &str =
    "/open-apis/application/v6/applications/{app_id}/collaborators";
pub const APPLICATION_V6_APP_COLLABORATOR_DELETE: &str =
    "/open-apis/application/v6/applications/{app_id}/collaborators";
pub const APPLICATION_V6_APP_VERSIONS: &str =
    "/open-apis/application/v6/applications/{app_id}/versions";
pub const APPLICATION_V6_APP_VERSION_GET: &str =
    "/open-apis/application/v6/applications/{app_id}/versions/{version_id}";
pub const APPLICATION_V6_APPS_UNDERAUDITLIST: &str =
    "/open-apis/application/v6/applications/underauditlist";
pub const APPLICATION_V6_APP_AUDIT: &str = "/open-apis/application/v6/applications/{app_id}/audit";
pub const APPLICATION_V6_APP_GROUP: &str = "/open-apis/application/v6/applications/{app_id}/group";
pub const APPLICATION_V6_APP_VERSION_CONTACTS_RANGE_SUGGEST: &str =
    "/open-apis/application/v6/applications/{app_id}/versions/{version_id}/contacts_range_suggest";

// ==================== 应用使用情况 ====================
pub const APPLICATION_V6_APP_USAGE_OVERVIEW: &str =
    "/open-apis/application/v6/app_usage/{app_id}/overview";
pub const APPLICATION_V6_APP_USAGE_MESSAGE_PUSH_OVERVIEW: &str =
    "/open-apis/application/v6/app_usage/{app_id}/message_push_overview";
pub const APPLICATION_V6_APP_USAGE_DEPARTMENT_OVERVIEW: &str =
    "/open-apis/application/v6/app_usage/{app_id}/department_overview";

// ==================== 应用权限范围 ====================
pub const APPLICATION_V6_APP_SCOPE_APPLY: &str =
    "/open-apis/application/v6/applications/{app_id}/scope/apply";
pub const APPLICATION_V6_APP_SCOPE_GET: &str =
    "/open-apis/application/v6/applications/{app_id}/scope";

// ==================== 应用徽章 ====================
pub const APPLICATION_V6_APP_BADGE_SET: &str =
    "/open-apis/application/v6/app_badge/{app_id}/users/{user_id}/set";

// ==================== 应用反馈 ====================
pub const APPLICATION_V6_APPLICATION_FEEDBACK: &str =
    "/open-apis/application/v6/application_feedback";
pub const APPLICATION_V6_APPLICATION_FEEDBACK_GET: &str =
    "/open-apis/application/v6/application_feedback/{feedback_id}";

// ==================== 管理员应用管理 ====================
pub const APPLICATION_V6_ADMIN_APPS: &str = "/open-apis/application/v6/admin/apps";
pub const APPLICATION_V6_ADMIN_APP_ENABLE: &str =
    "/open-apis/application/v6/admin/apps/{app_id}/enable";
pub const APPLICATION_V6_ADMIN_APP_ADMINS: &str =
    "/open-apis/application/v6/admin/apps/{app_id}/admins";
pub const APPLICATION_V6_ADMIN_APP_VISIBILITY: &str =
    "/open-apis/application/v6/admin/apps/{app_id}/visibility";
pub const APPLICATION_V6_ADMIN_APP_VISIBILITY_CREATE: &str =
    "/open-apis/application/v6/admin/apps/{app_id}/visibility";
pub const APPLICATION_V6_ADMIN_USER_AVAILABLE_APPS: &str =
    "/open-apis/application/v6/admin/user_available_apps/{user_id}";
pub const APPLICATION_V6_ADMIN_APP_CONTACTS_RANGE_CONFIGURATION_GET: &str =
    "/open-apis/application/v6/admin/apps/{app_id}/contacts_range_configuration";
pub const APPLICATION_V6_ADMIN_APP_CONTACTS_RANGE_CONFIGURATION_SET: &str =
    "/open-apis/application/v6/admin/apps/{app_id}/contacts_range_configuration";
pub const APPLICATION_V6_ADMIN_APP_CHECK_WHITE_BLACK_LIST: &str =
    "/open-apis/application/v6/admin/apps/{app_id}/check_white_black_list";
pub const APPLICATION_V6_ADMIN_APP_ADMIN_MANAGEMENT_PERMISSIONS: &str =
    "/open-apis/application/v6/admin/apps/{app_id}/admins/{user_id}/management_permissions";
pub const APPLICATION_V6_ADMIN_APP_ADMIN_VERIFY: &str =
    "/open-apis/application/v6/admin/apps/{app_id}/admins/{user_id}/verify";

// ==================== 应用商店付费信息 ====================
pub const APPLICATION_V6_APPSTORE_PAID_INFO_CHECK: &str =
    "/open-apis/application/v6/appstore_paid_info/{app_id}/users/{user_id}/pricing_plans/{pricing_plan_id}/check";
pub const APPLICATION_V6_APPSTORE_PAID_INFO_PRICING_PLANS: &str =
    "/open-apis/application/v6/appstore_paid_info/{app_id}/pricing_plans";
pub const APPLICATION_V6_APPSTORE_PAID_INFO_ORDER_GET: &str =
    "/open-apis/application/v6/appstore_paid_info/{app_id}/orders/{order_id}";
