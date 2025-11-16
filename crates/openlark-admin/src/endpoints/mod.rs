//! OpenLark Admin 服务端点定义
//!
//! 此模块包含管理和治理相关的所有API端点常量，从 openlark-core 迁移而来。
//! 包含ACS门禁、后台管理、主数据、安全合规、租户管理等完整功能。
//!
//! # 服务模块包含
//!
//! - **acs**: ACS门禁控制系统（设备、访客、门禁记录、访问控制）
//! - **admin**: 后台管理（密码、勋章、数据报表、系统治理）
//! - **mdm**: 主数据管理（国家地区、用户认证数据关系）
//! - **security_and_compliance**: 安全合规（审计、日志、合规管理）
//! - **tenant**: 租户管理（企业信息、产品分配、配置管理）
//! - **trust_party**: 可信第三方管理
//! - **workplace**: 工作台管理（配置、应用推荐、数据分析）
//!
//! # 使用示例
//!
//! ```rust
//! use openlark_admin::endpoints::*;
//!
//! // ACS门禁系统
//! let devices_endpoint = ACS_V1_DEVICES;
//! let visitors_endpoint = ACS_V1_VISITORS;
//! let access_records_endpoint = ACS_V1_ACCESS_RECORDS;
//!
//! // 后台管理
//! let badges_endpoint = ADMIN_V1_BADGES_LIST;
//! let password_reset_endpoint = ADMIN_V1_PASSWORD_RESET;
//!
//! // 租户管理
//! let tenant_query_endpoint = TENANT_V2_QUERY;
//! let product_info_endpoint = TENANT_V2_PRODUCT_ASSIGN_INFO_QUERY;
//!
//! // 工作台管理
//! let workplace_data_endpoint = WORKPLACE_ACCESS_DATA_SEARCH;
//! let app_recommend_endpoint = WORKPLACE_APP_RECOMMEND_LIST;
//! ```

// 导入核心端点（auth, application等基础端点）
pub use openlark_core::endpoints::{auth, application, apass, platform_integration};
pub use openlark_core::endpoints::{Endpoints, EndpointBuilder};

// ==================== ACS (Access Control System) v1 ====================
// ACS门禁控制系统 - 访问控制、设备管理、访客管理

/// ACS设备管理
/// 门禁设备的注册、查询和管理
pub const ACS_V1_DEVICES: &str = "/open-apis/acs/v1/devices";

/// ACS访客管理
/// 访客信息的管理和访问控制
pub const ACS_V1_VISITORS: &str = "/open-apis/acs/v1/visitors";
/// 获取访客详情
pub const ACS_V1_VISITOR_GET: &str = "/open-apis/acs/v1/visitors/{visitor_id}";

/// ACS门禁记录
/// 门禁访问记录和事件日志
pub const ACS_V1_ACCESS_RECORDS: &str = "/open-apis/acs/v1/access_records";
/// 获取门禁记录人脸照片
pub const ACS_V1_ACCESS_RECORD_FACE_IMAGE: &str = "/open-apis/acs/v1/access_records/{access_record_id}/face_image";

/// ACS门禁规则
/// 访问控制规则和策略管理
pub const ACS_V1_RULE_EXTERNAL: &str = "/open-apis/acs/v1/rule_external";
pub const ACS_V1_RULE_EXTERNAL_OPERATION: &str = "/open-apis/acs/v1/rule_external/{rule_id}";
pub const ACS_V1_RULE_EXTERNAL_DEVICE_BIND: &str = "/open-apis/acs/v1/rule_external/{rule_id}/device_bind";

/// ACS用户管理
/// 门禁系统用户的认证和权限管理
pub const ACS_V1_USERS: &str = "/open-apis/acs/v1/users";
pub const ACS_V1_USER_OPERATION: &str = "/open-apis/acs/v1/users/{user_id}";
pub const ACS_V1_USER_FACE_IMAGE: &str = "/open-apis/acs/v1/users/{user_id}/face_image";

// ==================== Admin (后台管理) v1 ====================
// 后台管理系统 - 密码管理、勋章系统、数据报表

/// Admin登录密码管理
/// 管理员密码重置和安全管理
pub const ADMIN_V1_PASSWORD_RESET: &str = "/open-apis/admin/v1/password/reset";

/// Admin勋章管理
/// 企业勋章系统的创建、管理和授予
pub const ADMIN_V1_BADGES_CREATE: &str = "/open-apis/admin/v1/badges";
pub const ADMIN_V1_BADGES_LIST: &str = "/open-apis/admin/v1/badges";
pub const ADMIN_V1_BADGES_OPERATION: &str = "/open-apis/admin/v1/badges/{badge_id}";
pub const ADMIN_V1_BADGES_IMAGE_UPLOAD: &str = "/open-apis/admin/v1/badges/image";

/// Admin勋章授予名单管理
/// 勋章授予记录的管理和维护
pub const ADMIN_V1_BADGE_GRANTS_CREATE: &str = "/open-apis/admin/v1/badge_grants";
pub const ADMIN_V1_BADGE_GRANTS_LIST: &str = "/open-apis/admin/v1/badge_grants";
pub const ADMIN_V1_BADGE_GRANTS_OPERATION: &str = "/open-apis/admin/v1/badge_grants/{grant_id}";

/// Admin数据报表
/// 企业管理数据统计和分析报表
pub const ADMIN_V1_DATA_REPORT_DEPARTMENT: &str = "/open-apis/admin/v1/data_report/department";
pub const ADMIN_V1_DATA_REPORT_USER: &str = "/open-apis/admin/v1/data_report/user";

// ==================== MDM (Master Data Management) v1 ====================
// 主数据管理 - 国家地区、认证数据关系

/// MDM国家地区管理
/// 国家和地区信息的主数据管理
pub const MDM_V1_COUNTRY_REGIONS: &str = "/open-apis/mdm/v1/country_regions";
pub const MDM_V1_COUNTRY_REGIONS_BATCH_GET: &str = "/open-apis/mdm/v1/country_regions/batch_get";

/// MDM用户认证数据关系管理
/// 用户认证数据与其他业务数据的关联管理
pub const MDM_V1_USER_AUTH_DATA_RELATIONS_BIND: &str = "/open-apis/mdm/v1/user_auth_data_relations/bind";
pub const MDM_V1_USER_AUTH_DATA_RELATIONS_UNBIND: &str = "/open-apis/mdm/v1/user_auth_data_relations/unbind";

// ==================== Security & Compliance (安全合规) v1 ====================
// 安全合规管理 - 审计、日志、合规监控

/// Security合规审计数据
/// 企业安全审计数据的查询和管理
pub const SECURITY_AND_COMPLIANCE_V1_AUDIT_DATAS: &str = "/open-apis/security_and_compliance/v1/audit_datas";

/// Security开放接口日志
/// 开放平台API访问日志和监控数据
pub const SECURITY_AND_COMPLIANCE_V1_OPENAPI_LOGS_LIST_DATA: &str = "/open-apis/security_and_compliance/v1/openapi_logs/list_data";

// ==================== Tenant (租户管理) v2 ====================
// 租户管理 - 企业信息、产品分配、配置管理

/// Tenant企业信息查询
/// 企业租户基本信息和配置查询
pub const TENANT_V2_QUERY: &str = "/open-apis/tenant/v2/tenant/query";

/// Tenant产品分配信息查询
/// 租户产品授权和使用情况查询
pub const TENANT_V2_PRODUCT_ASSIGN_INFO_QUERY: &str = "/open-apis/tenant/v2/tenant_product_assign_info/query";

// ==================== Trust Party (可信第三方) v1 ====================
// 可信第三方管理 - 协作组织管理和可见性规则

/// Trust Party协作组织管理
/// 跨组织协作和关联组织管理
pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATIONS: &str = "/open-apis/trust_party/v1/collaboration_organizations";
pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_VISIBLE: &str = "/open-apis/trust_party/v1/collaboration_organizations/{org_id}/visible_organization";
pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_GET: &str = "/open-apis/trust_party/v1/collaboration_organizations/{org_id}";
pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_USER_GET: &str = "/open-apis/trust_party/v1/collaboration_organizations/{org_id}/users/{user_id}";
pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_DEPARTMENT_GET: &str = "/open-apis/trust_party/v1/collaboration_organizations/{org_id}/departments/{department_id}";
pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_SHARED_MEMBER_SCOPES: &str = "/open-apis/trust_party/v1/collaboration_organizations/{org_id}/shared_member_scopes";
pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATIONS_ADMIN: &str = "/open-apis/trust_party/v1/collaboration_organizations/admin";

/// Trust Party可搜可见规则管理
/// 跨组织搜索和可见性控制规则
pub const TRUST_PARTY_V1_SEARCHABLE_VISIBLE_RULES: &str = "/open-apis/trust_party/v1/searchable_visible_rules";
pub const TRUST_PARTY_V1_SEARCHABLE_VISIBLE_RULE_OPERATION: &str = "/open-apis/trust_party/v1/searchable_visible_rules/{rule_id}";

// ==================== Workplace (工作台管理) v1 ====================
// 工作台管理 - 配置、应用推荐、数据分析

/// Workplace访问数据搜索
/// 工作台使用数据的查询和分析
pub const WORKPLACE_ACCESS_DATA_SEARCH: &str = "/open-apis/workplace/v1/workplace_access_data/search";
pub const WORKPLACE_CUSTOM_ACCESS_DATA_SEARCH: &str = "/open-apis/workplace/v1/custom_workplace_access_data/search";
pub const WORKPLACE_WIDGET_ACCESS_DATA_SEARCH: &str = "/open-apis/workplace/v1/custom_workplace_widget_access_data/search";

/// Workplace应用推荐规则
/// 工作台应用推荐策略和规则管理
pub const WORKPLACE_APP_RECOMMEND_FAVOURITE: &str = "/open-apis/workplace/v1/app_recommend_rule/favourite";
pub const WORKPLACE_APP_RECOMMEND_RECOMMEND: &str = "/open-apis/workplace/v1/app_recommend_rule/recommend";
pub const WORKPLACE_APP_RECOMMEND_LIST: &str = "/open-apis/workplace/v1/app_recommend_rule/list";

/// Workplace基础信息
/// 工作台标识和基础配置
pub const WORKPLACE_ID: &str = "/open-apis/workplace/v1/workplace_access_data/search";

// ==================== 兼容性别名 ====================

/// 为保持向后兼容性，提供一些简短的别名
/// ACS别名
pub const ACS_DEVICES: &str = ACS_V1_DEVICES;
pub const ACS_VISITORS: &str = ACS_V1_VISITORS;
pub const ACS_ACCESS_RECORDS: &str = ACS_V1_ACCESS_RECORDS;

/// Admin别名
pub const ADMIN_BADGES: &str = ADMIN_V1_BADGES_LIST;
pub const ADMIN_PASSWORD: &str = ADMIN_V1_PASSWORD_RESET;

/// Tenant别名
pub const TENANT_QUERY: &str = TENANT_V2_QUERY;

/// Workplace别名
pub const WORKPLACE_SEARCH: &str = WORKPLACE_ACCESS_DATA_SEARCH;
pub const WORKPLACE_RECOMMEND: &str = WORKPLACE_APP_RECOMMEND_LIST;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acs_endpoints() {
        // 验证ACS端点
        assert!(ACS_V1_DEVICES.starts_with("/open-apis/acs/v1/"));
        assert!(ACS_V1_VISITORS.contains("visitors"));
        assert!(ACS_V1_ACCESS_RECORDS.contains("access_records"));
        assert!(ACS_V1_USERS.contains("users"));
        assert!(ACS_V1_RULE_EXTERNAL.contains("rule_external"));
    }

    #[test]
    fn test_admin_endpoints() {
        // 验证Admin端点
        assert!(ADMIN_V1_PASSWORD_RESET.starts_with("/open-apis/admin/v1/"));
        assert!(ADMIN_V1_BADGES_LIST.contains("badges"));
        assert!(ADMIN_V1_BADGE_GRANTS_LIST.contains("badge_grants"));
        assert!(ADMIN_V1_DATA_REPORT_DEPARTMENT.contains("data_report"));
        assert!(ADMIN_V1_BADGES_IMAGE_UPLOAD.contains("image"));
    }

    #[test]
    fn test_mdm_endpoints() {
        // 验证MDM端点
        assert!(MDM_V1_COUNTRY_REGIONS.starts_with("/open-apis/mdm/v1/"));
        assert!(MDM_V1_COUNTRY_REGIONS.contains("country_regions"));
        assert!(MDM_V1_USER_AUTH_DATA_RELATIONS_BIND.contains("user_auth_data_relations"));
        assert!(MDM_V1_USER_AUTH_DATA_RELATIONS_UNBIND.contains("unbind"));
    }

    #[test]
    fn test_security_compliance_endpoints() {
        // 验证安全合规端点
        assert!(SECURITY_AND_COMPLIANCE_V1_AUDIT_DATAS.starts_with("/open-apis/security_and_compliance/v1/"));
        assert!(SECURITY_AND_COMPLIANCE_V1_AUDIT_DATAS.contains("audit_datas"));
        assert!(SECURITY_AND_COMPLIANCE_V1_OPENAPI_LOGS_LIST_DATA.contains("openapi_logs"));
    }

    #[test]
    fn test_tenant_endpoints() {
        // 验证Tenant端点
        assert!(TENANT_V2_QUERY.starts_with("/open-apis/tenant/v2/"));
        assert!(TENANT_V2_QUERY.contains("tenant"));
        assert!(TENANT_V2_PRODUCT_ASSIGN_INFO_QUERY.contains("tenant_product_assign_info"));
    }

    #[test]
    fn test_workplace_endpoints() {
        // 验证Workplace端点
        assert!(WORKPLACE_ACCESS_DATA_SEARCH.starts_with("/open-apis/workplace/v1/"));
        assert!(WORKPLACE_ACCESS_DATA_SEARCH.contains("workplace_access_data"));
        assert!(WORKPLACE_APP_RECOMMEND_LIST.contains("app_recommend_rule"));
        assert!(WORKPLACE_WIDGET_ACCESS_DATA_SEARCH.contains("widget"));
    }

    #[test]
    fn test_trust_party_endpoints() {
        // 验证Trust Party端点
        assert!(TRUST_PARTY_V1_COLLABORATION_ORGANIZATIONS.starts_with("/open-apis/trust_party/v1/"));
        assert!(TRUST_PARTY_V1_COLLABORATION_ORGANIZATIONS.contains("collaboration_organizations"));
        assert!(TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_VISIBLE.contains("visible_organization"));
        assert!(TRUST_PARTY_V1_SEARCHABLE_VISIBLE_RULES.contains("searchable_visible_rules"));
        assert!(TRUST_PARTY_V1_COLLABORATION_ORGANIZATIONS_ADMIN.contains("admin"));
    }

    #[test]
    fn test_backward_compatibility() {
        // 验证兼容性别名
        assert_eq!(ACS_DEVICES, ACS_V1_DEVICES);
        assert_eq!(ACS_VISITORS, ACS_V1_VISITORS);
        assert_eq!(ADMIN_BADGES, ADMIN_V1_BADGES_LIST);
        assert_eq!(TENANT_QUERY, TENANT_V2_QUERY);
        assert_eq!(WORKPLACE_SEARCH, WORKPLACE_ACCESS_DATA_SEARCH);
    }

    #[test]
    fn test_endpoint_parameter_placeholders() {
        // 测试关键端点是否包含正确的参数占位符
        assert!(ACS_V1_VISITOR_GET.contains("{visitor_id}"));
        assert!(ACS_V1_ACCESS_RECORD_FACE_IMAGE.contains("{access_record_id}"));
        assert!(ACS_V1_RULE_EXTERNAL_OPERATION.contains("{rule_id}"));
        assert!(ACS_V1_USER_OPERATION.contains("{user_id}"));
        assert!(ADMIN_V1_BADGES_OPERATION.contains("{badge_id}"));
        assert!(ADMIN_V1_BADGE_GRANTS_OPERATION.contains("{grant_id}"));
        assert!(TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_VISIBLE.contains("{org_id}"));
        assert!(TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_USER_GET.contains("{org_id}"));
        assert!(TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_USER_GET.contains("{user_id}"));
        assert!(TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_DEPARTMENT_GET.contains("{org_id}"));
        assert!(TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_DEPARTMENT_GET.contains("{department_id}"));
        assert!(TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_SHARED_MEMBER_SCOPES.contains("{org_id}"));
        assert!(TRUST_PARTY_V1_SEARCHABLE_VISIBLE_RULE_OPERATION.contains("{rule_id}"));
    }

    #[test]
    fn test_service_grouping() {
        // 测试服务分组的正确性
        let acs_endpoints = [ACS_V1_DEVICES, ACS_V1_VISITORS, ACS_V1_ACCESS_RECORDS];
        for endpoint in acs_endpoints {
            assert!(endpoint.contains("/acs/"), "{} 应该包含 /acs/", endpoint);
        }

        let admin_endpoints = [ADMIN_V1_PASSWORD_RESET, ADMIN_V1_BADGES_LIST];
        for endpoint in admin_endpoints {
            assert!(endpoint.contains("/admin/"), "{} 应该包含 /admin/", endpoint);
        }

        let mdm_endpoints = [MDM_V1_COUNTRY_REGIONS, MDM_V1_USER_AUTH_DATA_RELATIONS_BIND];
        for endpoint in mdm_endpoints {
            assert!(endpoint.contains("/mdm/"), "{} 应该包含 /mdm/", endpoint);
        }

        let security_endpoints = [SECURITY_AND_COMPLIANCE_V1_AUDIT_DATAS];
        for endpoint in security_endpoints {
            assert!(endpoint.contains("/security_and_compliance/"), "{} 应该包含 /security_and_compliance/", endpoint);
        }

        let tenant_endpoints = [TENANT_V2_QUERY];
        for endpoint in tenant_endpoints {
            assert!(endpoint.contains("/tenant/"), "{} 应该包含 /tenant/", endpoint);
        }

        let workplace_endpoints = [WORKPLACE_ACCESS_DATA_SEARCH, WORKPLACE_APP_RECOMMEND_LIST];
        for endpoint in workplace_endpoints {
            assert!(endpoint.contains("/workplace/"), "{} 应该包含 /workplace/", endpoint);
        }

        let trust_party_endpoints = [TRUST_PARTY_V1_COLLABORATION_ORGANIZATIONS, TRUST_PARTY_V1_SEARCHABLE_VISIBLE_RULES];
        for endpoint in trust_party_endpoints {
            assert!(endpoint.contains("/trust_party/"), "{} 应该包含 /trust_party/", endpoint);
        }
    }
}