//! corehr 服务端点常量定义

/// 基础信息 - 枚举搜索
pub const COREHR_BASIC_INFO_ENUM_SEARCH: &str = "/open-apis/corehr/v1/basic_infos/enum_search";

/// 基础信息 - 地点搜索
pub const COREHR_BASIC_INFO_LOCATION_SEARCH: &str =
    "/open-apis/corehr/v1/basic_infos/location_search";

/// 基础信息 - 国籍搜索
pub const COREHR_BASIC_INFO_NATIONALITY_SEARCH: &str =
    "/open-apis/corehr/v1/basic_infos/nationality_search";

/// 通用数据 - ID转换
pub const COREHR_COMMON_DATA_ID_CONVERT: &str = "/open-apis/corehr/v1/common_data/id_convert";

/// 公司管理
pub const COREHR_COMPANIES: &str = "/open-apis/corehr/v1/companies";

/// 部门管理
pub const COREHR_DEPARTMENTS: &str = "/open-apis/corehr/v1/departments";

/// 部门批量获取
pub const COREHR_DEPARTMENTS_BATCH_GET: &str = "/open-apis/corehr/v1/departments/batch_get";

/// 部门树形结构
pub const COREHR_DEPARTMENTS_TREE: &str = "/open-apis/corehr/v1/departments/tree";

/// 员工批量获取
pub const COREHR_EMPLOYEES_BATCH_GET: &str = "/open-apis/corehr/v1/employees/batch_get";

/// 员工搜索
pub const COREHR_EMPLOYEES_SEARCH: &str = "/open-apis/corehr/v1/employees/search";

/// 职位管理
pub const COREHR_JOBS: &str = "/open-apis/corehr/v1/jobs";

/// 职级管理
pub const COREHR_JOB_LEVELS: &str = "/open-apis/corehr/v1/job_levels";

/// 职位族管理
pub const COREHR_JOB_FAMILIES: &str = "/open-apis/corehr/v1/job_families";

/// 职等管理
pub const COREHR_JOB_GRADES: &str = "/open-apis/corehr/v1/job_grades";

/// 职等查询
pub const COREHR_JOB_GRADES_QUERY: &str = "/open-apis/corehr/v1/job_grades/query";

/// 异动管理
pub const COREHR_JOB_CHANGES: &str = "/open-apis/corehr/v1/job_changes";

/// 异动搜索
pub const COREHR_JOB_CHANGES_SEARCH: &str = "/open-apis/corehr/v1/job_changes/search";

/// 离职管理
pub const COREHR_OFFBOARDINGS: &str = "/open-apis/corehr/v1/offboardings";

/// 离职搜索
pub const COREHR_OFFBOARDINGS_SEARCH: &str = "/open-apis/corehr/v1/offboardings/search";

/// 待入职管理
pub const COREHR_PRE_HIRES: &str = "/open-apis/corehr/v1/pre_hires";

/// 待入职搜索
pub const COREHR_PRE_HIRES_SEARCH: &str = "/open-apis/corehr/v1/pre_hires/search";
