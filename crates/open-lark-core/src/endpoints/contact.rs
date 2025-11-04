//! Contact 服务端点常量定义（v3）

// ==================== 用户管理 ====================
pub const CONTACT_V3_USERS: &str = "/open-apis/contact/v3/users";
pub const CONTACT_V3_USERS_BATCH: &str = "/open-apis/contact/v3/users/batch";
pub const CONTACT_V3_USERS_BATCH_GET_ID: &str = "/open-apis/contact/v3/users/batch_get_id";
pub const CONTACT_V3_USERS_FIND_BY_DEPARTMENT: &str =
    "/open-apis/contact/v3/users/find_by_department";
pub const CONTACT_V3_USERS_SEARCH: &str = "/open-apis/contact/v3/users/search";
pub const CONTACT_V3_USER_GET: &str = "/open-apis/contact/v3/users/{user_id}";
pub const CONTACT_V3_USER_UPDATE_ID: &str = "/open-apis/contact/v3/users/{user_id}/update_user_id";
pub const CONTACT_V3_USER_RESURRECT: &str = "/open-apis/contact/v3/users/{user_id}/resurrect";

// ==================== 部门管理 ====================
pub const CONTACT_V3_DEPARTMENTS: &str = "/open-apis/contact/v3/departments";
pub const CONTACT_V3_DEPARTMENTS_BATCH: &str = "/open-apis/contact/v3/departments/batch";
pub const CONTACT_V3_DEPARTMENTS_CHILDREN: &str = "/open-apis/contact/v3/departments/children";
pub const CONTACT_V3_DEPARTMENTS_PARENT: &str = "/open-apis/contact/v3/departments/parent";
pub const CONTACT_V3_DEPARTMENTS_SEARCH: &str = "/open-apis/contact/v3/departments/search";
pub const CONTACT_V3_DEPARTMENT_GET: &str = "/open-apis/contact/v3/departments/{department_id}";
pub const CONTACT_V3_DEPARTMENT_UPDATE_ID: &str =
    "/open-apis/contact/v3/departments/{department_id}/update_department_id";

// ==================== 职务/职位族群/职级 ====================
pub const CONTACT_V3_JOB_TITLES: &str = "/open-apis/contact/v3/job_titles";
pub const CONTACT_V3_JOB_TITLE_GET: &str = "/open-apis/contact/v3/job_titles/{job_title_id}";

pub const CONTACT_V3_JOB_FAMILIES: &str = "/open-apis/contact/v3/job_families";
pub const CONTACT_V3_JOB_FAMILY_GET: &str = "/open-apis/contact/v3/job_families/{job_family_id}";

pub const CONTACT_V3_JOB_LEVELS: &str = "/open-apis/contact/v3/job_levels";
pub const CONTACT_V3_JOB_LEVEL_GET: &str = "/open-apis/contact/v3/job_levels/{job_level_id}";

// ==================== 功能角色与成员 ====================
pub const CONTACT_V3_FUNCTIONAL_ROLES: &str = "/open-apis/contact/v3/functional_roles";
pub const CONTACT_V3_FUNCTIONAL_ROLE_GET: &str = "/open-apis/contact/v3/functional_roles/{role_id}";
pub const CONTACT_V3_FUNCTIONAL_ROLE_MEMBERS: &str =
    "/open-apis/contact/v3/functional_roles/{role_id}/members";
pub const CONTACT_V3_FUNCTIONAL_ROLE_MEMBERS_BATCH_CREATE: &str =
    "/open-apis/contact/v3/functional_roles/{role_id}/members/batch_create";
pub const CONTACT_V3_FUNCTIONAL_ROLE_MEMBERS_BATCH_DELETE: &str =
    "/open-apis/contact/v3/functional_roles/{role_id}/members/batch_delete";
pub const CONTACT_V3_FUNCTIONAL_ROLE_MEMBERS_SCOPES: &str =
    "/open-apis/contact/v3/functional_roles/{role_id}/members/scopes";
pub const CONTACT_V3_FUNCTIONAL_ROLE_MEMBER_GET: &str =
    "/open-apis/contact/v3/functional_roles/{role_id}/members/{member_id}";

// ==================== 群组与成员 ====================
pub const CONTACT_V3_GROUPS: &str = "/open-apis/contact/v3/groups";
pub const CONTACT_V3_GROUPS_SIMPLELIST: &str = "/open-apis/contact/v3/groups/simplelist";
pub const CONTACT_V3_GROUPS_MEMBER_BELONG: &str = "/open-apis/contact/v3/groups/member_belong";
pub const CONTACT_V3_GROUP_GET: &str = "/open-apis/contact/v3/groups/{group_id}";
pub const CONTACT_V3_GROUP_DETAIL: &str = "/open-apis/contact/v3/groups/{group_id}/detail";
pub const CONTACT_V3_GROUP_MEMBERS_ADD: &str =
    "/open-apis/contact/v3/groups/{group_id}/members/add";
pub const CONTACT_V3_GROUP_MEMBERS_BATCH_ADD: &str =
    "/open-apis/contact/v3/groups/{group_id}/members/batch_add";
pub const CONTACT_V3_GROUP_MEMBERS_REMOVE: &str =
    "/open-apis/contact/v3/groups/{group_id}/members/remove";
pub const CONTACT_V3_GROUP_MEMBERS_BATCH_REMOVE: &str =
    "/open-apis/contact/v3/groups/{group_id}/members/batch_remove";
pub const CONTACT_V3_GROUP_MEMBERS_SIMPLELIST: &str =
    "/open-apis/contact/v3/groups/{group_id}/members/simplelist";

// ==================== 单位/工作城市/枚举/自定义属性/权限范围 ====================
pub const CONTACT_V3_UNITS: &str = "/open-apis/contact/v3/units";
pub const CONTACT_V3_UNIT_GET: &str = "/open-apis/contact/v3/units/{unit_id}";
pub const CONTACT_V3_UNIT_BIND_DEPARTMENT: &str =
    "/open-apis/contact/v3/units/{unit_id}/bind_department";
pub const CONTACT_V3_UNIT_UNBIND_DEPARTMENT: &str =
    "/open-apis/contact/v3/units/{unit_id}/unbind_department";
pub const CONTACT_V3_UNIT_LIST_DEPARTMENT: &str =
    "/open-apis/contact/v3/units/{unit_id}/list_department";

pub const CONTACT_V3_WORK_CITIES: &str = "/open-apis/contact/v3/work_cities";
pub const CONTACT_V3_WORK_CITY_GET: &str = "/open-apis/contact/v3/work_cities/{work_city_id}";

pub const CONTACT_V3_EMPLOYEE_TYPE_ENUMS: &str = "/open-apis/contact/v3/employee_type_enums";
pub const CONTACT_V3_EMPLOYEE_TYPE_ENUM_GET: &str =
    "/open-apis/contact/v3/employee_type_enums/{enum_id}";

pub const CONTACT_V3_CUSTOM_ATTRS: &str = "/open-apis/contact/v3/custom_attrs";
pub const CONTACT_V3_SCOPES: &str = "/open-apis/contact/v3/scopes";
