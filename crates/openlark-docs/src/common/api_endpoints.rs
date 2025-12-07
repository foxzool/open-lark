//! API端点定义
//!
//! 提供类型安全的API端点管理，替代字符串拼接方式。

/// Base API V2 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum BaseApiV2 {
    /// 新增自定义角色
    RoleCreate(String),
    /// 更新自定义角色
    RoleUpdate(String, String),
    /// 列出自定义角色
    RoleList(String),
}

impl BaseApiV2 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            BaseApiV2::RoleCreate(app_token) => {
                format!("/open-apis/base/v2/apps/{}/roles", app_token)
            },
            BaseApiV2::RoleUpdate(app_token, role_id) => {
                format!("/open-apis/base/v2/apps/{}/roles/{}", app_token, role_id)
            },
            BaseApiV2::RoleList(app_token) => {
                format!("/open-apis/base/v2/apps/{}/roles", app_token)
            },
        }
    }
}

/// Bitable API V1 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum BitableApiV1 {
    /// App管理相关
    AppCreate,
    AppCopy,
    AppGet(String),
    AppUpdate(String),
    AppList,
    DashboardList(String),

    /// 表格管理相关
    TableCreate(String),
    TableBatchCreate(String),
    TableUpdate(String, String),
    TableDelete(String, String),
    TableBatchDelete(String),
    TableGet(String, String),
    TableList(String),
    TablePatch(String, String),

    /// 字段管理相关
    FieldCreate(String, String),
    FieldUpdate(String, String, String),
    FieldDelete(String, String, String),
    FieldList(String, String),

    /// 视图管理相关
    ViewCreate(String, String),
    ViewUpdate(String, String, String),
    ViewDelete(String, String, String),
    ViewGet(String, String, String),
    ViewList(String, String),
    ViewPatch(String, String, String),

    /// 记录管理相关
    RecordCreate(String, String),
    RecordBatchCreate(String, String),
    RecordGet(String, String, String),
    RecordBatchGet(String, String),
    RecordUpdate(String, String, String),
    RecordBatchUpdate(String, String),
    RecordDelete(String, String, String),
    RecordBatchDelete(String, String),
    RecordList(String, String),
    RecordSearch(String, String),

    /// 权限管理相关
    RoleCreate(String),
    RoleUpdate(String, String),
    RoleDelete(String, String),
    RoleList(String),
    RoleMemberCreate(String, String),
    RoleMemberBatchCreate(String, String),
    RoleMemberDelete(String, String, String),
    RoleMemberBatchDelete(String, String),
    RoleMemberList(String, String),
}

impl BitableApiV1 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            // App管理
            BitableApiV1::AppCreate => "/open-apis/bitable/v1/apps".to_string(),
            BitableApiV1::AppCopy => "/open-apis/bitable/v1/apps/copy".to_string(),
            BitableApiV1::AppGet(app_token) => {
                format!("/open-apis/bitable/v1/apps/{}", app_token)
            },
            BitableApiV1::AppUpdate(app_token) => {
                format!("/open-apis/bitable/v1/apps/{}", app_token)
            },
            BitableApiV1::AppList => "/open-apis/bitable/v1/apps".to_string(),
            BitableApiV1::DashboardList(app_token) => {
                format!("/open-apis/bitable/v1/apps/{}/dashboard/list", app_token)
            },

            // 表格管理
            BitableApiV1::TableCreate(app_token) => {
                format!("/open-apis/bitable/v1/apps/{}/tables", app_token)
            },
            BitableApiV1::TableBatchCreate(app_token) => {
                format!("/open-apis/bitable/v1/apps/{}/tables/batch_create", app_token)
            },
            BitableApiV1::TableUpdate(app_token, table_id) => {
                format!("/open-apis/bitable/v1/apps/{}/tables/{}", app_token, table_id)
            },
            BitableApiV1::TableDelete(app_token, table_id) => {
                format!("/open-apis/bitable/v1/apps/{}/tables/{}", app_token, table_id)
            },
            BitableApiV1::TableBatchDelete(app_token) => {
                format!("/open-apis/bitable/v1/apps/{}/tables/batch_delete", app_token)
            },
            BitableApiV1::TableGet(app_token, table_id) => {
                format!("/open-apis/bitable/v1/apps/{}/tables/{}", app_token, table_id)
            },
            BitableApiV1::TableList(app_token) => {
                format!("/open-apis/bitable/v1/apps/{}/tables", app_token)
            },
            BitableApiV1::TablePatch(app_token, table_id) => {
                format!("/open-apis/bitable/v1/apps/{}/tables/{}", app_token, table_id)
            },

            // 字段管理
            BitableApiV1::FieldCreate(app_token, table_id) => {
                format!("/open-apis/bitable/v1/apps/{}/tables/{}/fields", app_token, table_id)
            },
            BitableApiV1::FieldUpdate(app_token, table_id, field_id) => {
                format!("/open-apis/bitable/v1/apps/{}/tables/{}/fields/{}", app_token, table_id, field_id)
            },
            BitableApiV1::FieldDelete(app_token, table_id, field_id) => {
                format!("/open-apis/bitable/v1/apps/{}/tables/{}/fields/{}", app_token, table_id, field_id)
            },
            BitableApiV1::FieldList(app_token, table_id) => {
                format!("/open-apis/bitable/v1/apps/{}/tables/{}/fields", app_token, table_id)
            },

            // 视图管理
            BitableApiV1::ViewCreate(app_token, table_id) => {
                format!("/open-apis/bitable/v1/apps/{}/tables/{}/views", app_token, table_id)
            },
            BitableApiV1::ViewUpdate(app_token, table_id, view_id) => {
                format!("/open-apis/bitable/v1/apps/{}/tables/{}/views/{}", app_token, table_id, view_id)
            },
            BitableApiV1::ViewDelete(app_token, table_id, view_id) => {
                format!("/open-apis/bitable/v1/apps/{}/tables/{}/views/{}", app_token, table_id, view_id)
            },
            BitableApiV1::ViewGet(app_token, table_id, view_id) => {
                format!("/open-apis/bitable/v1/apps/{}/tables/{}/views/{}", app_token, table_id, view_id)
            },
            BitableApiV1::ViewList(app_token, table_id) => {
                format!("/open-apis/bitable/v1/apps/{}/tables/{}/views", app_token, table_id)
            },
            BitableApiV1::ViewPatch(app_token, table_id, view_id) => {
                format!("/open-apis/bitable/v1/apps/{}/tables/{}/views/{}", app_token, table_id, view_id)
            },

            // 记录管理
            BitableApiV1::RecordCreate(app_token, table_id) => {
                format!("/open-apis/bitable/v1/apps/{}/tables/{}/records", app_token, table_id)
            },
            BitableApiV1::RecordBatchCreate(app_token, table_id) => {
                format!("/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_create", app_token, table_id)
            },
            BitableApiV1::RecordGet(app_token, table_id, record_id) => {
                format!("/open-apis/bitable/v1/apps/{}/tables/{}/records/{}", app_token, table_id, record_id)
            },
            BitableApiV1::RecordBatchGet(app_token, table_id) => {
                format!("/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_get", app_token, table_id)
            },
            BitableApiV1::RecordUpdate(app_token, table_id, record_id) => {
                format!("/open-apis/bitable/v1/apps/{}/tables/{}/records/{}", app_token, table_id, record_id)
            },
            BitableApiV1::RecordBatchUpdate(app_token, table_id) => {
                format!("/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_update", app_token, table_id)
            },
            BitableApiV1::RecordDelete(app_token, table_id, record_id) => {
                format!("/open-apis/bitable/v1/apps/{}/tables/{}/records/{}", app_token, table_id, record_id)
            },
            BitableApiV1::RecordBatchDelete(app_token, table_id) => {
                format!("/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_delete", app_token, table_id)
            },
            BitableApiV1::RecordList(app_token, table_id) => {
                format!("/open-apis/bitable/v1/apps/{}/tables/{}/records", app_token, table_id)
            },
            BitableApiV1::RecordSearch(app_token, table_id) => {
                format!("/open-apis/bitable/v1/apps/{}/tables/{}/records/search", app_token, table_id)
            },

            // 权限管理
            BitableApiV1::RoleCreate(app_token) => {
                format!("/open-apis/bitable/v1/apps/{}/roles", app_token)
            },
            BitableApiV1::RoleUpdate(app_token, role_id) => {
                format!("/open-apis/bitable/v1/apps/{}/roles/{}", app_token, role_id)
            },
            BitableApiV1::RoleDelete(app_token, role_id) => {
                format!("/open-apis/bitable/v1/apps/{}/roles/{}", app_token, role_id)
            },
            BitableApiV1::RoleList(app_token) => {
                format!("/open-apis/bitable/v1/apps/{}/roles", app_token)
            },
            BitableApiV1::RoleMemberCreate(app_token, role_id) => {
                format!("/open-apis/bitable/v1/apps/{}/roles/{}/members", app_token, role_id)
            },
            BitableApiV1::RoleMemberBatchCreate(app_token, role_id) => {
                format!("/open-apis/bitable/v1/apps/{}/roles/{}/members/batch_create", app_token, role_id)
            },
            BitableApiV1::RoleMemberDelete(app_token, role_id, member_id) => {
                format!("/open-apis/bitable/v1/apps/{}/roles/{}/members/{}", app_token, role_id, member_id)
            },
            BitableApiV1::RoleMemberBatchDelete(app_token, role_id) => {
                format!("/open-apis/bitable/v1/apps/{}/roles/{}/members/batch_delete", app_token, role_id)
            },
            BitableApiV1::RoleMemberList(app_token, role_id) => {
                format!("/open-apis/bitable/v1/apps/{}/roles/{}/members", app_token, role_id)
            },
        }
    }
}

/// Minutes API V1 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum MinutesApiV1 {
    /// 获取妙记信息
    Get(String),
    /// 下载妙记音视频文件
    MediaGet(String),
    /// 导出妙记文字记录
    TranscriptGet(String),
    /// 获取妙记统计数据
    StatisticsGet(String),
}

impl MinutesApiV1 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            MinutesApiV1::Get(minute_token) => {
                format!("/open-apis/minutes/v1/minutes/{}", minute_token)
            },
            MinutesApiV1::MediaGet(minute_token) => {
                format!("/open-apis/minutes/v1/minutes/{}/media", minute_token)
            },
            MinutesApiV1::TranscriptGet(minute_token) => {
                format!("/open-apis/minutes/v1/minutes/{}/transcript", minute_token)
            },
            MinutesApiV1::StatisticsGet(minute_token) => {
                format!("/open-apis/minutes/v1/minutes/{}/statistics", minute_token)
            },
        }
    }
}