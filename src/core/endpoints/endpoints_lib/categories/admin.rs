//! 管理和认证服务端点

/// 管理和认证服务端点
pub struct Admin;

impl Admin {
    /// / 获取用户访问令牌
    pub const AUTH_ACCESS_TOKEN: &'static str = "/open-apis/auth/v3/app_access_token";
    /// / 刷新用户访问令牌
    pub const AUTH_USER_ACCESS_TOKEN: &'static str = "/open-apis/auth/v3/user_access_token";
    /// / 获取登录预授权码
    pub const AUTH_REFRESH_USER_TOKEN: &'static str =
        "/open-apis/auth/v3/refresh_user_access_token";
    /// / 获取用户信息
    pub const AUTH_LOGIN_PRE_AUTH: &'static str = "/open-apis/authen/v1/oidc/access_token";
    /// ==================== APAAS应用开放平台服务端点 ====================
    pub const AUTH_USER_INFO: &'static str = "/open-apis/authen/v1/user_info";
    /// / 应用商店信息
    pub const APPLICATION_GET_ADMIN_LIST: &'static str =
        "/open-apis/application/v6/applications/{app_id}/app_admin_user_list";
    /// / 管理员应用启用
    pub const APPLICATION_V6_ADMIN_APPS: &'static str = "/open-apis/application/v6/admin/apps";
    /// / 管理员应用管理员列表
    pub const APPLICATION_V6_ADMIN_APP_ENABLE: &'static str =
        "/open-apis/application/v6/admin/apps/{app_id}/enable";
    /// / 应用可见性管理
    pub const APPLICATION_V6_ADMIN_APP_ADMINS: &'static str =
        "/open-apis/application/v6/admin/apps/{app_id}/admins";
    /// / 应用可见性创建
    pub const APPLICATION_V6_ADMIN_APP_VISIBILITY: &'static str =
        "/open-apis/application/v6/admin/apps/{app_id}/visibility";
    /// / 用户可用应用
    pub const APPLICATION_V6_ADMIN_APP_VISIBILITY_CREATE: &'static str =
        "/open-apis/application/v6/admin/apps/{app_id}/visibility";
    /// / 应用联系人范围配置查询
    pub const APPLICATION_V6_ADMIN_USER_AVAILABLE_APPS: &'static str =
        "/open-apis/application/v6/admin/user_available_apps/{user_id}";
    /// / 应用联系人范围配置设置
    pub const APPLICATION_V6_ADMIN_APP_CONTACTS_RANGE_CONFIGURATION_GET: &'static str =
        "/open-apis/application/v6/admin/apps/{app_id}/contacts_range_configuration";
    /// / 应用白黑名单检查
    pub const APPLICATION_V6_ADMIN_APP_CONTACTS_RANGE_CONFIGURATION_SET: &'static str =
        "/open-apis/application/v6/admin/apps/{app_id}/contacts_range_configuration";
    /// / 应用管理员管理权限
    pub const APPLICATION_V6_ADMIN_APP_CHECK_WHITE_BLACK_LIST: &'static str =
        "/open-apis/application/v6/admin/apps/{app_id}/check_white_black_list";
    /// / 应用管理员验证
    pub const APPLICATION_V6_ADMIN_APP_ADMIN_MANAGEMENT_PERMISSIONS: &'static str =
        "/open-apis/application/v6/admin/apps/{app_id}/admins/{user_id}/management_permissions";
    /// / 应用版本联系人范围建议
    pub const APPLICATION_V6_ADMIN_APP_ADMIN_VERIFY: &'static str =
        "/open-apis/application/v6/admin/apps/{app_id}/admins/{user_id}/verify";
    /// / 勋章管理 - 获取勋章列表
    pub const ADMIN_V1_BADGES_CREATE: &'static str = "/open-apis/admin/v1/badges";
    /// / 勋章管理 - 操作勋章（获取/更新）
    pub const ADMIN_V1_BADGES_LIST: &'static str = "/open-apis/admin/v1/badges";
    /// / 勋章管理 - 上传勋章图片
    pub const ADMIN_V1_BADGES_OPERATION: &'static str = "/open-apis/admin/v1/badges/{badge_id}";
    /// / 勋章授予名单 - 创建授予名单
    pub const ADMIN_V1_BADGES_IMAGE_UPLOAD: &'static str = "/open-apis/admin/v1/badges/image";
    /// / 勋章授予名单 - 获取授予名单列表
    pub const ADMIN_V1_BADGE_GRANTS_CREATE: &'static str = "/open-apis/admin/v1/badge_grants";
    /// / 勋章授予名单 - 操作授予名单（获取/更新/删除）
    pub const ADMIN_V1_BADGE_GRANTS_LIST: &'static str = "/open-apis/admin/v1/badge_grants";
    /// / 密码管理 - 重置密码
    pub const ADMIN_V1_BADGE_GRANTS_OPERATION: &'static str =
        "/open-apis/admin/v1/badge_grants/{grant_id}";
    /// / 数据报表 - 部门维度数据
    pub const ADMIN_V1_PASSWORD_RESET: &'static str = "/open-apis/admin/v1/password/reset";
    /// / 数据报表 - 用户维度数据
    pub const ADMIN_V1_DATA_REPORT_DEPARTMENT: &'static str =
        "/open-apis/admin/v1/data_report/department";
    /// ==================== 妙记服务端点 ====================
    pub const ADMIN_V1_DATA_REPORT_USER: &'static str = "/open-apis/admin/v1/data_report/user";
    /// / 用户数据维度解绑
    pub const MDM_V1_USER_AUTH_DATA_RELATIONS_BIND: &'static str =
        "/open-apis/mdm/v1/user_auth_data_relations/bind";
    /// ==================== Security and Compliance 安全与合规相关端点 ====================
    pub const MDM_V1_USER_AUTH_DATA_RELATIONS_UNBIND: &'static str =
        "/open-apis/mdm/v1/user_auth_data_relations/unbind";
    /// ==================== Passport 护照服务相关端点 ====================
    pub const AUTHEN_V1_USER_INFO: &'static str = "/open-apis/authen/v1/user_info";
    /// ==================== Calendar 日历相关端点 (补充) ====================
    pub const PASSPORT_V1_SESSIONS_LOGOUT: &'static str = "/open-apis/passport/v1/sessions/logout";
}
