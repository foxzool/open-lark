//! API路径常量定义
//!
//! 提供各种API路径的常量定义，避免硬编码字符串。

/// Base API V2 路径常量
pub mod base_v2 {
    /// 角色管理路径
    pub const ROLES: &str = "/open-apis/base/v2/apps/:app_token/roles";
}

/// Bitable API V1 路径常量
pub mod bitable_v1 {
    /// 应用管理路径
    pub const APPS: &str = "/open-apis/bitable/v1/apps";
    pub const APPS_COPY: &str = "/open-apis/bitable/v1/apps/copy";

    /// 表格管理路径模板
    pub const TABLES: &str = "/open-apis/bitable/v1/apps/:app_token/tables";
    pub const TABLES_BATCH_CREATE: &str = "/open-apis/bitable/v1/apps/:app_token/tables/batch_create";
    pub const TABLES_BATCH_DELETE: &str = "/open-apis/bitable/v1/apps/:app_token/tables/batch_delete";

    /// 字段管理路径模板
    pub const FIELDS: &str = "/open-apis/bitable/v1/apps/:app_token/tables/:table_id/fields";

    /// 视图管理路径模板
    pub const VIEWS: &str = "/open-apis/bitable/v1/apps/:app_token/tables/:table_id/views";

    /// 记录管理路径模板
    pub const RECORDS: &str = "/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records";
    pub const RECORDS_BATCH_CREATE: &str = "/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/batch_create";
    pub const RECORDS_BATCH_GET: &str = "/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/batch_get";
    pub const RECORDS_BATCH_UPDATE: &str = "/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/batch_update";
    pub const RECORDS_BATCH_DELETE: &str = "/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/batch_delete";
    pub const RECORDS_SEARCH: &str = "/open-apis/bitable/v1/apps/:app_token/tables/:table_id/records/search";

    /// 权限管理路径模板
    pub const ROLES: &str = "/open-apis/bitable/v1/apps/:app_token/roles";
    pub const ROLE_MEMBERS: &str = "/open-apis/bitable/v1/apps/:app_token/roles/:role_id/members";
    pub const ROLE_MEMBERS_BATCH_CREATE: &str = "/open-apis/bitable/v1/apps/:app_token/roles/:role_id/members/batch_create";
    pub const ROLE_MEMBERS_BATCH_DELETE: &str = "/open-apis/bitable/v1/apps/:app_token/roles/:role_id/members/batch_delete";
}

/// URL 路径格式化工具
pub struct PathFormatter;

impl PathFormatter {
    /// 格式化包含app_token的路径
    pub fn with_app_token(template: &str, app_token: &str) -> String {
        template.replace(":app_token", app_token)
    }

    /// 格式化包含table_id的路径
    pub fn with_table_id(template: &str, app_token: &str, table_id: &str) -> String {
        template
            .replace(":app_token", app_token)
            .replace(":table_id", table_id)
    }

    /// 格式化包含field_id的路径
    pub fn with_field_id(template: &str, app_token: &str, table_id: &str, field_id: &str) -> String {
        template
            .replace(":app_token", app_token)
            .replace(":table_id", table_id)
            .replace(":field_id", field_id)
    }

    /// 格式化包含view_id的路径
    pub fn with_view_id(template: &str, app_token: &str, table_id: &str, view_id: &str) -> String {
        template
            .replace(":app_token", app_token)
            .replace(":table_id", table_id)
            .replace(":view_id", view_id)
    }

    /// 格式化包含record_id的路径
    pub fn with_record_id(template: &str, app_token: &str, table_id: &str, record_id: &str) -> String {
        template
            .replace(":app_token", app_token)
            .replace(":table_id", table_id)
            .replace(":record_id", record_id)
    }

    /// 格式化包含role_id的路径
    pub fn with_role_id(template: &str, app_token: &str, role_id: &str) -> String {
        template
            .replace(":app_token", app_token)
            .replace(":role_id", role_id)
    }

    /// 格式化包含member_id的路径
    pub fn with_member_id(template: &str, app_token: &str, role_id: &str, member_id: &str) -> String {
        template
            .replace(":app_token", app_token)
            .replace(":role_id", role_id)
            .replace(":member_id", member_id)
    }
}