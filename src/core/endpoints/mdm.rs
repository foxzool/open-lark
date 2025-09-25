//! mdm 服务端点常量定义
//!
//! MDM (Master Data Management) 主数据管理相关 API 端点常量，包括：
//! - 国家地区管理
//! - 用户认证数据关系管理

/// 国家地区管理
pub const MDM_V1_COUNTRY_REGIONS: &str = "/open-apis/mdm/v1/country_regions";
pub const MDM_V1_COUNTRY_REGIONS_BATCH_GET: &str = "/open-apis/mdm/v1/country_regions/batch_get";

/// 用户认证数据关系管理
pub const MDM_V1_USER_AUTH_DATA_RELATIONS_BIND: &str =
    "/open-apis/mdm/v1/user_auth_data_relations/bind";
pub const MDM_V1_USER_AUTH_DATA_RELATIONS_UNBIND: &str =
    "/open-apis/mdm/v1/user_auth_data_relations/unbind";
