//! Contact (通讯录) 端点定义
//!
//! 企业通讯录管理，包括用户、部门、用户组、角色等信息。
//!
//! # 使用示例
//!
//! ```rust
//! use openlark_communication::endpoints::contact::*;
//!
//! let users_endpoint = CONTACT_V3_USERS;
//! let departments_endpoint = CONTACT_V3_DEPARTMENTS;
//! ```

// ==================== Contact (通讯录) v1/v2（历史版本）====================

/// Contact 应用管理员管理范围 v1（历史版本）
pub const CONTACT_V1_USER_ADMIN_SCOPE_GET: &str = "/open-apis/contact/v1/user/admin_scope/get";

/// Contact 批量新增用户 v2（历史版本）
pub const CONTACT_V2_USER_BATCH_ADD: &str = "/open-apis/contact/v2/user/batch_add";

/// Contact 批量新增部门 v2（历史版本）
pub const CONTACT_V2_DEPARTMENT_BATCH_ADD: &str = "/open-apis/contact/v2/department/batch_add";

/// Contact 查询批量任务执行状态 v2（历史版本）
pub const CONTACT_V2_TASK_GET: &str = "/open-apis/contact/v2/task/get";

/// Contact 获取角色列表 v2（历史版本）
pub const CONTACT_V2_ROLE_LIST: &str = "/open-apis/contact/v2/role/list";

// ==================== Contact (通讯录) v3 ====================

/// Contact 权限范围 v3
pub const CONTACT_V3_SCOPES: &str = "/open-apis/contact/v3/scopes";

/// Contact 部门 v3
pub const CONTACT_V3_DEPARTMENTS: &str = "/open-apis/contact/v3/departments";
pub const CONTACT_V3_DEPARTMENTS_BATCH: &str = "/open-apis/contact/v3/departments/batch";
pub const CONTACT_V3_DEPARTMENTS_PARENT: &str = "/open-apis/contact/v3/departments/parent";
pub const CONTACT_V3_DEPARTMENTS_SEARCH: &str = "/open-apis/contact/v3/departments/search";
pub const CONTACT_V3_DEPARTMENTS_UNBIND_DEPARTMENT_CHAT: &str =
    "/open-apis/contact/v3/departments/unbind_department_chat";

/// Contact 角色（functional role）v3
pub const CONTACT_V3_FUNCTIONAL_ROLES: &str = "/open-apis/contact/v3/functional_roles";

/// Contact 用户组 v3
pub const CONTACT_V3_GROUP: &str = "/open-apis/contact/v3/group";
pub const CONTACT_V3_GROUP_SIMPLELIST: &str = "/open-apis/contact/v3/group/simplelist";
pub const CONTACT_V3_GROUP_MEMBER_BELONG: &str = "/open-apis/contact/v3/group/member_belong";

/// Contact 企业自定义用户字段 v3
pub const CONTACT_V3_CUSTOM_ATTRS: &str = "/open-apis/contact/v3/custom_attrs";

/// Contact 人员类型 v3
pub const CONTACT_V3_EMPLOYEE_TYPE_ENUMS: &str = "/open-apis/contact/v3/employee_type_enums";

/// Contact 单位 v3
pub const CONTACT_V3_UNIT: &str = "/open-apis/contact/v3/unit";
pub const CONTACT_V3_UNIT_BIND_DEPARTMENT: &str = "/open-apis/contact/v3/unit/bind_department";
pub const CONTACT_V3_UNIT_UNBIND_DEPARTMENT: &str = "/open-apis/contact/v3/unit/unbind_department";
pub const CONTACT_V3_UNIT_LIST_DEPARTMENT: &str = "/open-apis/contact/v3/unit/list_department";

/// Contact 职级 v3
pub const CONTACT_V3_JOB_LEVELS: &str = "/open-apis/contact/v3/job_levels";

/// Contact 序列 v3
pub const CONTACT_V3_JOB_FAMILIES: &str = "/open-apis/contact/v3/job_families";

/// Contact 职务 v3
pub const CONTACT_V3_JOB_TITLES: &str = "/open-apis/contact/v3/job_titles";

/// Contact 工作城市 v3
pub const CONTACT_V3_WORK_CITIES: &str = "/open-apis/contact/v3/work_cities";

/// Contact 用户 v3
pub const CONTACT_V3_USERS: &str = "/open-apis/contact/v3/users";
pub const CONTACT_V3_USER: &str = "/open-apis/contact/v3/users/{user_id}";
pub const CONTACT_V3_USERS_BATCH: &str = "/open-apis/contact/v3/users/batch";
pub const CONTACT_V3_USERS_FIND_BY_DEPARTMENT: &str =
    "/open-apis/contact/v3/users/find_by_department";
pub const CONTACT_V3_USERS_BATCH_GET_ID: &str = "/open-apis/contact/v3/users/batch_get_id";
pub const CONTACT_V3_USER_UPDATE_USER_ID: &str =
    "/open-apis/contact/v3/users/{user_id}/update_user_id";
pub const CONTACT_V3_USER_RESURRECT: &str = "/open-apis/contact/v3/users/{user_id}/resurrect";

// ==================== User/Search ====================

/// 搜索用户
pub const SEARCH_V1_USER: &str = "/open-apis/search/v1/user";

/// 查询应用管理员列表
pub const USER_V4_APP_ADMIN_USER_LIST: &str = "/open-apis/user/v4/app_admin_user/list";

// ==================== 兼容性别名 ====================

/// 为保持向后兼容性，提供一些简短的别名
/// IM别名
pub const SEND_MESSAGE: &str = IM_V1_MESSAGES;
pub const CHAT_CREATE: &str = IM_V1_CHATS;

/// VC别名
pub const ROOM_LIST: &str = VC_V1_ROOMS;
pub const MEETING_LIST: &str = VC_V1_MEETINGS;

/// Mail别名
pub const MAILGROUP_LIST: &str = MAIL_V1_MAILGROUPS;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contact_v3_endpoints() {
        assert!(CONTACT_V3_SCOPES.starts_with("/open-apis/contact/v3/"));
        assert!(CONTACT_V3_DEPARTMENTS.contains("departments"));
        assert!(CONTACT_V3_FUNCTIONAL_ROLES.contains("functional_roles"));
        assert!(CONTACT_V3_GROUP.contains("group"));
        assert!(CONTACT_V3_USERS.contains("users"));
        assert!(CONTACT_V3_JOB_LEVELS.contains("job_levels"));
        assert!(CONTACT_V3_JOB_FAMILIES.contains("job_families"));
        assert!(CONTACT_V3_WORK_CITIES.contains("work_cities"));
    }

    #[test]
    fn test_contact_v2_endpoints() {
        assert!(CONTACT_V2_USER_BATCH_ADD.starts_with("/open-apis/contact/v2/"));
        assert!(CONTACT_V2_DEPARTMENT_BATCH_ADD.contains("department"));
        assert!(CONTACT_V2_TASK_GET.contains("task"));
        assert!(CONTACT_V2_ROLE_LIST.contains("role"));
    }

    #[test]
    fn test_search_endpoints() {
        assert!(SEARCH_V1_USER.starts_with("/open-apis/search/v1/"));
        assert!(USER_V4_APP_ADMIN_USER_LIST.starts_with("/open-apis/user/v4/"));
    }

    #[test]
    fn test_backward_compatibility() {
        assert_eq!(SEND_MESSAGE, IM_V1_MESSAGES);
        assert_eq!(CHAT_CREATE, IM_V1_CHATS);
        assert_eq!(ROOM_LIST, VC_V1_ROOMS);
        assert_eq!(MEETING_LIST, VC_V1_MEETINGS);
        assert_eq!(MAILGROUP_LIST, MAIL_V1_MAILGROUPS);
    }
}
