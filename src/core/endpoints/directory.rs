//! directory 服务端点常量定义
//!
//! 目录服务相关 API 端点常量，包括：
//! - 部门管理
//! - 员工管理
//! - 搜索和筛选

/// 部门管理
pub const DIRECTORY_V1_DEPARTMENTS: &str = "/open-apis/directory/v1/departments";
pub const DIRECTORY_V1_DEPARTMENT_GET: &str = "/open-apis/directory/v1/departments/{department_id}";
pub const DIRECTORY_V1_DEPARTMENTS_SEARCH: &str = "/open-apis/directory/v1/departments/search";
pub const DIRECTORY_V1_DEPARTMENTS_FILTER: &str = "/open-apis/directory/v1/departments/filter";
pub const DIRECTORY_V1_DEPARTMENTS_MGET: &str = "/open-apis/directory/v1/departments/mget";

/// 员工管理
pub const DIRECTORY_V1_EMPLOYEES: &str = "/open-apis/directory/v1/employees";
pub const DIRECTORY_V1_EMPLOYEE_GET: &str = "/open-apis/directory/v1/employees/{employee_id}";
pub const DIRECTORY_V1_EMPLOYEES_SEARCH: &str = "/open-apis/directory/v1/employees/search";
pub const DIRECTORY_V1_EMPLOYEES_FILTER: &str = "/open-apis/directory/v1/employees/filter";
pub const DIRECTORY_V1_EMPLOYEES_MGET: &str = "/open-apis/directory/v1/employees/mget";

/// 员工状态管理
pub const DIRECTORY_V1_EMPLOYEE_REGULAR: &str =
    "/open-apis/directory/v1/employees/{employee_id}/regular";
pub const DIRECTORY_V1_EMPLOYEE_RESURRECT: &str =
    "/open-apis/directory/v1/employees/{employee_id}/resurrect";
pub const DIRECTORY_V1_EMPLOYEE_TO_BE_RESIGNED: &str =
    "/open-apis/directory/v1/employees/{employee_id}/to_be_resigned";
