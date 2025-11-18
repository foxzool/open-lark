//! OpenLark People 服务端点定义
//!
//! 此模块包含人员管理相关的所有API端点，从 openlark-core 迁移而来。
//! 包含联系人管理、通讯录管理、个人设置等完整功能。
//!
//! # 服务模块包含
//!
//! - **contact**: 联系人管理（用户查询、联系人管理、联系人搜索）
//! - **directory**: 通讯录管理（部门架构、员工信息、组织关系）
//! - **personal_settings**: 个人设置（用户偏好、个人资料、隐私设置）
//!
//! # 使用示例
//!
//! ```rust
//! use openlark_people::endpoints::*;
//!
//! // 联系人管理
//! let users_endpoint = CONTACT_V3_USERS;
//! let contact_search_endpoint = CONTACT_V1_USERS_SEARCH;
//!
//! // 通讯录管理
//! let departments_endpoint = DIRECTORY_V1_DEPARTMENTS;
//! let employees_endpoint = DIRECTORY_V1_EMPLOYEES;
//!
//! // 个人设置
//! let settings_endpoint = PERSONAL_SETTINGS_V1_SYSTEM_STATUS;
//! ```

// 导入核心端点（auth, application等基础端点）
pub use openlark_core::endpoints::{apass, application, auth, platform_integration};

// ==================== Contact (联系人管理) v1/v3 ====================
// 联系人系统 - 用户查询、联系人管理、联系人搜索

/// Contact联系人管理 v3
/// 用户和联系人的现代化管理接口
pub const CONTACT_V3_USERS: &str = "/open-apis/contact/v3/users";
pub const CONTACT_V3_USER_BYPASS_GET: &str = "/open-apis/contact/v3/users/bypass/get";
pub const CONTACT_V3_USER_BYPASS_UPDATE: &str = "/open-apis/contact/v3/users/bypass/update";
pub const CONTACT_V3_USER_BATCH_GET: &str = "/open-apis/contact/v3/users/batch_get";
pub const CONTACT_V3_USER_DELETE: &str = "/open-apis/contact/v3/users/{user_id}";
pub const CONTACT_V3_USER_CREATE: &str = "/open-apis/contact/v3/users";
pub const CONTACT_V3_USER_UPDATE: &str = "/open-apis/contact/v3/users/{user_id}";
pub const CONTACT_V3_USER_GET: &str = "/open-apis/contact/v3/users/{user_id}";
pub const CONTACT_V3_USER_SEARCH: &str = "/open-apis/contact/v3/users/search";
pub const CONTACT_V3_USER_MOBILE_SEARCH: &str = "/open-apis/contact/v3/users/mobile_search";
pub const CONTACT_V3_USER_CREATED_BY_APP_SEARCH: &str =
    "/open-apis/contact/v3/users/created_by_app/search";
pub const CONTACT_V3_DEPARTMENTS: &str = "/open-apis/contact/v3/departments";
pub const CONTACT_V3_DEPARTMENT_DELETE: &str = "/open-apis/contact/v3/departments/{department_id}";
pub const CONTACT_V3_DEPARTMENT_CREATE: &str = "/open-apis/contact/v3/departments";
pub const CONTACT_V3_DEPARTMENT_UPDATE: &str = "/open-apis/contact/v3/departments/{department_id}";
pub const CONTACT_V3_DEPARTMENT_GET: &str = "/open-apis/contact/v3/departments/{department_id}";
pub const CONTACT_V3_DEPARTMENT_SEARCH: &str = "/open-apis/contact/v3/departments/search";
pub const CONTACT_V3_DEPARTMENT_CHILD: &str =
    "/open-apis/contact/v3/departments/{department_id}/child";
pub const CONTACT_V3_DEPARTMENT_USER: &str =
    "/open-apis/contact/v3/departments/{department_id}/user";
pub const CONTACT_V3_ROLE_MANAGEMENT: &str = "/open-apis/contact/v3/role_management/permissions";
pub const CONTACT_V3_USER_ROLE: &str = "/open-apis/contact/v3/user_role/list_user_roles";
pub const CONTACT_V3_USER_ROLE_DELETE: &str = "/open-apis/contact/v3/user_role/delete_user_role";

/// Contact联系人管理 v1
/// 传统联系人管理接口
pub const CONTACT_V1_USERS_SEARCH: &str = "/open-apis/contact/v1/users/search";
pub const CONTACT_V1_USER_GET: &str = "/open-apis/contact/v1/users/{user_id}";
pub const CONTACT_V1_BATCH_GET: &str = "/open-apis/contact/v1/users/batch/get";
pub const CONTACT_V1_USER_CREATED_BY_APP_SEARCH: &str =
    "/open-apis/contact/v1/users/created_by_app/search";
pub const CONTACT_V1_DEPARTMENTS: &str = "/open-apis/contact/v1/departments";
pub const CONTACT_V1_DEPARTMENT_GET: &str = "/open-apis/contact/v1/departments/{department_id}";
pub const CONTACT_V1_DEPARTMENT_USER: &str =
    "/open-apis/contact/v1/departments/{department_id}/user";
pub const CONTACT_V1_USER_TENANT: &str = "/open-apis/contact/v1/users/{user_id}/tenant";
pub const CONTACT_V1_USER_INFO: &str = "/open-apis/contact/v1/users/{user_id}/info";
pub const CONTACT_V1_CONTACT_USER: &str = "/open-apis/contact/v1/contact_user";
pub const CONTACT_V1_CONTACT_USER_CREATE: &str = "/open-apis/contact/v1/contact_user/create";
pub const CONTACT_V1_CONTACT_USER_DELETE: &str = "/open-apis/contact/v1/contact_user/delete";
pub const CONTACT_V1_CONTACT_USER_UPDATE: &str = "/open-apis/contact/v1/contact_user/update";

// ==================== Directory (通讯录管理) v1 ====================
// 通讯录系统 - 部门架构、员工信息、组织关系

/// Directory通讯录管理 v1
/// 企业级通讯录和组织架构管理
pub const DIRECTORY_V1_EMPLOYEES: &str = "/open-apis/directory/v1/employees";
pub const DIRECTORY_V1_EMPLOYEE_GET: &str = "/open-apis/directory/v1/employees/{employee_id}";
pub const DIRECTORY_V1_EMPLOYEE_QUERY: &str = "/open-apis/directory/v1/employees/query";
pub const DIRECTORY_V1_EMPLOYEE_CREATE: &str = "/open-apis/directory/v1/employees";
pub const DIRECTORY_V1_EMPLOYEE_UPDATE: &str = "/open-apis/directory/v1/employees/{employee_id}";
pub const DIRECTORY_V1_EMPLOYEE_DELETE: &str = "/open-apis/directory/v1/employees/{employee_id}";
pub const DIRECTORY_V1_EMPLOYEE_BATCH_DELETE: &str =
    "/open-apis/directory/v1/employees/batch_delete";
pub const DIRECTORY_V1_EMPLOYEE_BATCH_CREATE: &str =
    "/open-apis/directory/v1/employees/batch_create";
pub const DIRECTORY_V1_EMPLOYEE_BATCH_UPDATE: &str =
    "/open-apis/directory/v1/employees/batch_update";
pub const DIRECTORY_V1_EMPLOYEE_STATUS_UPDATE: &str =
    "/open-apis/directory/v1/employee/status/update";
pub const DIRECTORY_V1_EMPLOYEE_DEPARTMENTS_GET: &str =
    "/open-apis/directory/v1/employees/{employee_id}/departments";
pub const DIRECTORY_V1_EMPLOYEE_DEPARTMENTS_UPDATE: &str =
    "/open-apis/directory/v1/employees/{employee_id}/departments";

/// Directory部门管理 v1
/// 部门和组织架构管理
pub const DIRECTORY_V1_DEPARTMENTS: &str = "/open-apis/directory/v1/departments";
pub const DIRECTORY_V1_DEPARTMENT_GET: &str = "/open-apis/directory/v1/departments/{department_id}";
pub const DIRECTORY_V1_DEPARTMENT_CREATE: &str = "/open-apis/directory/v1/departments";
pub const DIRECTORY_V1_DEPARTMENT_UPDATE: &str =
    "/open-apis/directory/v1/departments/{department_id}";
pub const DIRECTORY_V1_DEPARTMENT_DELETE: &str =
    "/open-apis/directory/v1/departments/{department_id}";
pub const DIRECTORY_V1_DEPARTMENT_PARENT_UPDATE: &str =
    "/open-apis/directory/v1/departments/{department_id}/parent";
pub const DIRECTORY_V1_DEPARTMENT_MEMBER_ADD: &str =
    "/open-apis/directory/v1/departments/{department_id}/members/add";
pub const DIRECTORY_V1_DEPARTMENT_MEMBER_REMOVE: &str =
    "/open-apis/directory/v1/departments/{department_id}/members/remove";
pub const DIRECTORY_V1_DEPARTMENT_MEMBER_UPDATE: &str =
    "/open-apis/directory/v1/departments/{department_id}/members/{member_id}";
pub const DIRECTORY_V1_DEPARTMENT_MEMBERS_GET: &str =
    "/open-apis/directory/v1/departments/{department_id}/members";

/// Directory员工扩展信息 v1
/// 员工详细信息和扩展字段管理
pub const DIRECTORY_V1_EMPLOYEE_EXTENDED: &str = "/open-apis/directory/v1/employee_extended";
pub const DIRECTORY_V1_EMPLOYEE_FIELD_GET: &str =
    "/open-apis/directory/v1/employee_extended/{employee_id}";
pub const DIRECTORY_V1_EMPLOYEE_FIELD_CREATE: &str = "/open-apis/directory/v1/employee_extended";
pub const DIRECTORY_V1_EMPLOYEE_FIELD_UPDATE: &str =
    "/open-apis/directory/v1/employee_extended/{employee_id}";
pub const DIRECTORY_V1_EMPLOYEE_FIELD_DELETE: &str =
    "/open-apis/directory/v1/employee_extended/{employee_id}";
pub const DIRECTORY_V1_EMPLOYEE_FIELD_BATCH_CREATE: &str =
    "/open-apis/directory/v1/employee_extended/batch_create";
pub const DIRECTORY_V1_EMPLOYEE_FIELD_BATCH_UPDATE: &str =
    "/open-apis/directory/v1/employee_extended/batch_update";
pub const DIRECTORY_V1_EMPLOYEE_FIELD_BATCH_DELETE: &str =
    "/open-apis/directory/v1/employee_extended/batch_delete";

/// Directory部门扩展信息 v1
/// 部门详细信息和扩展字段管理
pub const DIRECTORY_V1_DEPARTMENT_EXTENDED: &str = "/open-apis/directory/v1/department_extended";
pub const DIRECTORY_V1_DEPARTMENT_FIELD_GET: &str =
    "/open-apis/directory/v1/department_extended/{department_id}";
pub const DIRECTORY_V1_DEPARTMENT_FIELD_CREATE: &str =
    "/open-apis/directory/v1/department_extended";
pub const DIRECTORY_V1_DEPARTMENT_FIELD_UPDATE: &str =
    "/open-apis/directory/v1/department_extended/{department_id}";
pub const DIRECTORY_V1_DEPARTMENT_FIELD_DELETE: &str =
    "/open-apis/directory/v1/department_extended/{department_id}";

// ==================== Personal Settings (个人设置) v1 ====================
// 个人设置系统 - 用户偏好、个人资料、隐私设置

/// Personal Settings个人设置 v1
/// 用户个人偏好和设置管理
pub const PERSONAL_SETTINGS_V1_SYSTEM_STATUS: &str =
    "/open-apis/personal_settings/v1/system_status";
pub const PERSONAL_SETTINGS_V1_USER_STATUS: &str = "/open-apis/personal_settings/v1/user_status";

// ==================== 兼容性别名 ====================

/// 为保持向后兼容性，提供一些简短的别名
/// Contact别名
pub const USER_SEARCH: &str = CONTACT_V1_USERS_SEARCH;
pub const USER_GET: &str = CONTACT_V1_USER_GET;
pub const DEPARTMENT_LIST: &str = CONTACT_V1_DEPARTMENTS;
pub const DEPARTMENT_GET: &str = CONTACT_V1_DEPARTMENT_GET;

/// Directory别名
pub const EMPLOYEE_LIST: &str = DIRECTORY_V1_EMPLOYEES;
pub const EMPLOYEE_GET: &str = DIRECTORY_V1_EMPLOYEE_GET;
pub const EMPLOYEE_CREATE: &str = DIRECTORY_V1_EMPLOYEE_CREATE;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contact_endpoints() {
        // 验证Contact端点
        assert!(CONTACT_V3_USERS.starts_with("/open-apis/contact/v3/"));
        assert!(CONTACT_V3_USERS.contains("users"));
        assert!(CONTACT_V1_USERS_SEARCH.starts_with("/open-apis/contact/v1/"));
        assert!(CONTACT_V1_USERS_SEARCH.contains("search"));
    }

    #[test]
    fn test_directory_endpoints() {
        // 验证Directory端点
        assert!(DIRECTORY_V1_EMPLOYEES.starts_with("/open-apis/directory/v1/"));
        assert!(DIRECTORY_V1_EMPLOYEES.contains("employees"));
        assert!(DIRECTORY_V1_DEPARTMENTS.contains("departments"));
        assert!(DIRECTORY_V1_EMPLOYEE_EXTENDED.contains("employee_extended"));
    }

    #[test]
    fn test_personal_settings_endpoints() {
        // 验证Personal Settings端点
        assert!(PERSONAL_SETTINGS_V1_SYSTEM_STATUS.starts_with("/open-apis/personal_settings/v1/"));
        assert!(PERSONAL_SETTINGS_V1_SYSTEM_STATUS.contains("system_status"));
        assert!(PERSONAL_SETTINGS_V1_USER_STATUS.contains("user_status"));
    }

    #[test]
    fn test_backward_compatibility() {
        // 验证兼容性别名
        assert_eq!(USER_SEARCH, CONTACT_V1_USERS_SEARCH);
        assert_eq!(USER_GET, CONTACT_V1_USER_GET);
        assert_eq!(DEPARTMENT_LIST, CONTACT_V1_DEPARTMENTS);
        assert_eq!(EMPLOYEE_LIST, DIRECTORY_V1_EMPLOYEES);
        assert_eq!(EMPLOYEE_CREATE, DIRECTORY_V1_EMPLOYEE_CREATE);
    }

    #[test]
    fn test_endpoint_parameter_placeholders() {
        // 测试关键端点是否包含正确的参数占位符
        assert!(CONTACT_V3_USER_DELETE.contains("{user_id}"));
        assert!(CONTACT_V3_USER_UPDATE.contains("{user_id}"));
        assert!(CONTACT_V3_USER_GET.contains("{user_id}"));
        assert!(CONTACT_V3_DEPARTMENT_DELETE.contains("{department_id}"));
        assert!(CONTACT_V3_DEPARTMENT_UPDATE.contains("{department_id}"));
        assert!(CONTACT_V3_DEPARTMENT_GET.contains("{department_id}"));
        assert!(CONTACT_V3_DEPARTMENT_CHILD.contains("{department_id}"));
        assert!(CONTACT_V3_DEPARTMENT_USER.contains("{department_id}"));
        assert!(CONTACT_V1_USER_GET.contains("{user_id}"));
        assert!(CONTACT_V1_DEPARTMENT_GET.contains("{department_id}"));
        assert!(CONTACT_V1_DEPARTMENT_USER.contains("{department_id}"));
        assert!(CONTACT_V1_USER_TENANT.contains("{user_id}"));
        assert!(CONTACT_V1_USER_INFO.contains("{user_id}"));
        assert!(DIRECTORY_V1_EMPLOYEE_GET.contains("{employee_id}"));
        assert!(DIRECTORY_V1_EMPLOYEE_UPDATE.contains("{employee_id}"));
        assert!(DIRECTORY_V1_EMPLOYEE_DELETE.contains("{employee_id}"));
        assert!(DIRECTORY_V1_EMPLOYEE_DEPARTMENTS_GET.contains("{employee_id}"));
        assert!(DIRECTORY_V1_EMPLOYEE_DEPARTMENTS_UPDATE.contains("{employee_id}"));
        assert!(DIRECTORY_V1_DEPARTMENT_GET.contains("{department_id}"));
        assert!(DIRECTORY_V1_DEPARTMENT_UPDATE.contains("{department_id}"));
        assert!(DIRECTORY_V1_DEPARTMENT_DELETE.contains("{department_id}"));
        assert!(DIRECTORY_V1_DEPARTMENT_PARENT_UPDATE.contains("{department_id}"));
        assert!(DIRECTORY_V1_DEPARTMENT_MEMBER_ADD.contains("{department_id}"));
        assert!(DIRECTORY_V1_DEPARTMENT_MEMBER_REMOVE.contains("{department_id}"));
        assert!(DIRECTORY_V1_DEPARTMENT_MEMBER_UPDATE.contains("{department_id}"));
        assert!(DIRECTORY_V1_DEPARTMENT_MEMBER_UPDATE.contains("{member_id}"));
        assert!(DIRECTORY_V1_DEPARTMENT_MEMBERS_GET.contains("{department_id}"));
        assert!(DIRECTORY_V1_EMPLOYEE_FIELD_GET.contains("{employee_id}"));
        assert!(DIRECTORY_V1_EMPLOYEE_FIELD_UPDATE.contains("{employee_id}"));
        assert!(DIRECTORY_V1_EMPLOYEE_FIELD_DELETE.contains("{employee_id}"));
        assert!(DIRECTORY_V1_DEPARTMENT_FIELD_GET.contains("{department_id}"));
        assert!(DIRECTORY_V1_DEPARTMENT_FIELD_UPDATE.contains("{department_id}"));
        assert!(DIRECTORY_V1_DEPARTMENT_FIELD_DELETE.contains("{department_id}"));
    }

    #[test]
    fn test_service_grouping() {
        // 测试服务分组的正确性
        let contact_endpoints = [
            CONTACT_V3_USERS,
            CONTACT_V1_USERS_SEARCH,
            CONTACT_V3_DEPARTMENTS,
        ];
        for endpoint in contact_endpoints {
            assert!(
                endpoint.contains("/contact/"),
                "{} 应该包含 /contact/",
                endpoint
            );
        }

        let directory_endpoints = [
            DIRECTORY_V1_EMPLOYEES,
            DIRECTORY_V1_DEPARTMENTS,
            DIRECTORY_V1_EMPLOYEE_EXTENDED,
        ];
        for endpoint in directory_endpoints {
            assert!(
                endpoint.contains("/directory/"),
                "{} 应该包含 /directory/",
                endpoint
            );
        }

        let personal_settings_endpoints = [
            PERSONAL_SETTINGS_V1_SYSTEM_STATUS,
            PERSONAL_SETTINGS_V1_USER_STATUS,
        ];
        for endpoint in personal_settings_endpoints {
            assert!(
                endpoint.contains("/personal_settings/"),
                "{} 应该包含 /personal_settings/",
                endpoint
            );
        }
    }

    #[test]
    fn test_version_consistency() {
        // 测试版本一致性
        let v3_endpoints = [
            CONTACT_V3_USERS,
            CONTACT_V3_DEPARTMENTS,
            CONTACT_V3_USER_DELETE,
        ];
        for endpoint in v3_endpoints {
            assert!(endpoint.contains("/v3/"), "{} 应该包含 /v3/", endpoint);
        }

        let v1_endpoints = [
            CONTACT_V1_USERS_SEARCH,
            DIRECTORY_V1_EMPLOYEES,
            PERSONAL_SETTINGS_V1_SYSTEM_STATUS,
        ];
        for endpoint in v1_endpoints {
            assert!(endpoint.contains("/v1/"), "{} 应该包含 /v1/", endpoint);
        }
    }
} // Endpoints and EndpointBuilder are now available directly from openlark_core::endpoints
