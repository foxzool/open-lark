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
pub use openlark_core::endpoints::{apass, application, auth, platform_integration};

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
pub const ACS_V1_ACCESS_RECORD_FACE_IMAGE: &str =
    "/open-apis/acs/v1/access_records/{access_record_id}/face_image";

/// ACS门禁规则
/// 访问控制规则和策略管理
pub const ACS_V1_RULE_EXTERNAL: &str = "/open-apis/acs/v1/rule_external";
pub const ACS_V1_RULE_EXTERNAL_OPERATION: &str = "/open-apis/acs/v1/rule_external/{rule_id}";
pub const ACS_V1_RULE_EXTERNAL_DEVICE_BIND: &str =
    "/open-apis/acs/v1/rule_external/{rule_id}/device_bind";

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
pub const MDM_V1_USER_AUTH_DATA_RELATIONS_BIND: &str =
    "/open-apis/mdm/v1/user_auth_data_relations/bind";
pub const MDM_V1_USER_AUTH_DATA_RELATIONS_UNBIND: &str =
    "/open-apis/mdm/v1/user_auth_data_relations/unbind";

// ==================== Security & Compliance (安全合规) v1 ====================
// 安全合规管理 - 审计、日志、合规监控

/// Security合规审计数据
/// 企业安全审计数据的查询和管理
pub const SECURITY_AND_COMPLIANCE_V1_AUDIT_DATAS: &str =
    "/open-apis/security_and_compliance/v1/audit_datas";

/// Security开放接口日志
/// 开放平台API访问日志和监控数据
pub const SECURITY_AND_COMPLIANCE_V1_OPENAPI_LOGS_LIST_DATA: &str =
    "/open-apis/security_and_compliance/v1/openapi_logs/list_data";

// ==================== Tenant (租户管理) v2 ====================
// 租户管理 - 企业信息、产品分配、配置管理

/// Tenant企业信息查询
/// 企业租户基本信息和配置查询
pub const TENANT_V2_QUERY: &str = "/open-apis/tenant/v2/tenant/query";

/// Tenant产品分配信息查询
/// 租户产品授权和使用情况查询
pub const TENANT_V2_PRODUCT_ASSIGN_INFO_QUERY: &str =
    "/open-apis/tenant/v2/tenant_product_assign_info/query";

// ==================== Trust Party (可信第三方) v1 ====================
// 可信第三方管理 - 协作组织管理和可见性规则

/// Trust Party协作组织管理
/// 跨组织协作和关联组织管理
pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATIONS: &str =
    "/open-apis/trust_party/v1/collaboration_organizations";
pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_VISIBLE: &str =
    "/open-apis/trust_party/v1/collaboration_organizations/{org_id}/visible_organization";
pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_GET: &str =
    "/open-apis/trust_party/v1/collaboration_organizations/{org_id}";
pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_USER_GET: &str =
    "/open-apis/trust_party/v1/collaboration_organizations/{org_id}/users/{user_id}";
pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_DEPARTMENT_GET: &str =
    "/open-apis/trust_party/v1/collaboration_organizations/{org_id}/departments/{department_id}";
pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_SHARED_MEMBER_SCOPES: &str =
    "/open-apis/trust_party/v1/collaboration_organizations/{org_id}/shared_member_scopes";
pub const TRUST_PARTY_V1_COLLABORATION_ORGANIZATIONS_ADMIN: &str =
    "/open-apis/trust_party/v1/collaboration_organizations/admin";

/// Trust Party可搜可见规则管理
/// 跨组织搜索和可见性控制规则
pub const TRUST_PARTY_V1_SEARCHABLE_VISIBLE_RULES: &str =
    "/open-apis/trust_party/v1/searchable_visible_rules";
pub const TRUST_PARTY_V1_SEARCHABLE_VISIBLE_RULE_OPERATION: &str =
    "/open-apis/trust_party/v1/searchable_visible_rules/{rule_id}";

// ==================== Workplace (工作台管理) v1 ====================
// 工作台管理 - 配置、应用推荐、数据分析

/// Workplace访问数据搜索
/// 工作台使用数据的查询和分析
pub const WORKPLACE_ACCESS_DATA_SEARCH: &str =
    "/open-apis/workplace/v1/workplace_access_data/search";
pub const WORKPLACE_CUSTOM_ACCESS_DATA_SEARCH: &str =
    "/open-apis/workplace/v1/custom_workplace_access_data/search";
pub const WORKPLACE_WIDGET_ACCESS_DATA_SEARCH: &str =
    "/open-apis/workplace/v1/custom_workplace_widget_access_data/search";

/// Workplace应用推荐规则
/// 工作台应用推荐策略和规则管理
pub const WORKPLACE_APP_RECOMMEND_FAVOURITE: &str =
    "/open-apis/workplace/v1/app_recommend_rule/favourite";
pub const WORKPLACE_APP_RECOMMEND_RECOMMEND: &str =
    "/open-apis/workplace/v1/app_recommend_rule/recommend";
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

// ==================== Zero Trust (零信任安全架构) ====================
// 零信任安全架构 - 身份验证增强、设备信任评估、微分段访问控制、持续监控

/// 设备信任评估
pub const DEVICE_TRUST_ASSESSMENT: &str = "/open-apis/zero_trust/v1/device/trust/assess";

/// 用户身份增强验证
pub const ENHANCED_IDENTITY_VERIFICATION: &str =
    "/open-apis/zero_trust/v1/identity/enhanced_verify";

/// 持续身份验证
pub const CONTINUOUS_IDENTITY_VERIFICATION: &str =
    "/open-apis/zero_trust/v1/identity/continuous_verify";

/// 微分段访问控制
pub const MICRO_SEGMENTATION_ACCESS: &str = "/open-apis/zero_trust/v1/access/micro_segment";

/// 上下文感知访问策略
pub const CONTEXT_AWARE_ACCESS_POLICY: &str = "/open-apis/zero_trust/v1/policy/context_aware";

/// 自适应访问控制
pub const ADAPTIVE_ACCESS_CONTROL: &str = "/open-apis/zero_trust/v1/access/adaptive";

/// 行为基线分析
pub const BEHAVIORAL_BASELINE_ANALYSIS: &str = "/open-apis/zero_trust/v1/behavior/baseline";

/// 异常行为检测
pub const ANOMALY_BEHAVIOR_DETECTION: &str = "/open-apis/zero_trust/v1/behavior/anomaly";

/// 实时安全审计
pub const REAL_TIME_SECURITY_AUDIT: &str = "/open-apis/zero_trust/v1/audit/realtime";

/// 合规状态检查
pub const COMPLIANCE_STATUS_CHECK: &str = "/open-apis/zero_trust/v1/compliance/status";

/// GDPR合规检查
pub const GDPR_COMPLIANCE_CHECK: &str = "/open-apis/zero_trust/v1/compliance/gdpr";

/// SOC2合规检查
pub const SOC2_COMPLIANCE_CHECK: &str = "/open-apis/zero_trust/v1/compliance/soc2";

/// 端到端加密管理
pub const END_TO_END_ENCRYPTION: &str = "/open-apis/zero_trust/v1/encryption/e2e";

/// 密钥轮换管理
pub const KEY_ROTATION_MANAGEMENT: &str = "/open-apis/zero_trust/v1/key/rotation";

/// 零知识证明
pub const ZERO_KNOWLEDGE_PROOF: &str = "/open-apis/zero_trust/v1/cryptography/zkp";

/// 威胁情报订阅
pub const THREAT_INTELLIGENCE_SUBSCRIPTION: &str = "/open-apis/zero_trust/v1/threat/intelligence";

/// 自动化威胁响应
pub const AUTOMATED_THREAT_RESPONSE: &str = "/open-apis/zero_trust/v1/threat/response";

/// 攻击链分析
pub const ATTACK_CHAIN_ANALYSIS: &str = "/open-apis/zero_trust/v1/threat/attack_chain";

/// 安全事件关联分析
pub const SECURITY_INCIDENT_CORRELATION: &str = "/open-apis/zero_trust/v1/incident/correlation";

/// 零信任网络网关
pub const ZERO_TRUST_NETWORK_GATEWAY: &str = "/open-apis/zero_trust/v1/network/gateway";

/// 安全访问隧道
pub const SECURE_ACCESS_TUNNEL: &str = "/open-apis/zero_trust/v1/network/tunnel";

/// 细粒度权限控制
pub const FINE_GRAINED_PERMISSION_CONTROL: &str =
    "/open-apis/zero_trust/v1/permission/fine_grained";

/// 动态权限调整
pub const DYNAMIC_PERMISSION_ADJUSTMENT: &str = "/open-apis/zero_trust/v1/permission/dynamic";

/// 权限审计追踪
pub const PERMISSION_AUDIT_TRACKING: &str = "/open-apis/zero_trust/v1/permission/audit";

// ==================== Analytics (数据分析) ====================
// 数据分析服务 - 业务数据统计、用户行为分析、趋势预测

/// 用户行为分析
pub const USER_BEHAVIOR_ANALYTICS: &str = "/open-apis/analytics/v1/user_behavior";

/// 应用使用统计
pub const APP_USAGE_STATISTICS: &str = "/open-apis/analytics/v1/app_usage";

/// 业务数据洞察
pub const BUSINESS_DATA_INSIGHTS: &str = "/open-apis/analytics/v1/business_insights";

/// 趋势预测分析
pub const TREND_PREDICTION_ANALYTICS: &str = "/open-apis/analytics/v1/trend_prediction";

/// 实时数据监控
pub const REAL_TIME_DATA_MONITORING: &str = "/open-apis/analytics/v1/realtime_monitoring";

/// 自定义报表生成
pub const CUSTOM_REPORT_GENERATION: &str = "/open-apis/analytics/v1/custom_report";

/// 数据导出服务
pub const DATA_EXPORT_SERVICE: &str = "/open-apis/analytics/v1/data_export";

/// 绩效指标分析
pub const PERFORMANCE_METRICS_ANALYTICS: &str = "/open-apis/analytics/v1/performance_metrics";

/// 转化漏斗分析
pub const CONVERSION_FUNNEL_ANALYTICS: &str = "/open-apis/analytics/v1/conversion_funnel";

/// 用户分群分析
pub const USER_SEGMENTATION_ANALYTICS: &str = "/open-apis/analytics/v1/user_segmentation";

/// A/B测试分析
pub const AB_TEST_ANALYTICS: &str = "/open-apis/analytics/v1/ab_test";

/// 数据质量评估
pub const DATA_QUALITY_ASSESSMENT: &str = "/open-apis/analytics/v1/data_quality";

/// 预测模型服务
pub const PREDICTIVE_MODEL_SERVICE: &str = "/open-apis/analytics/v1/predictive_model";

/// 数据可视化
pub const DATA_VISUALIZATION: &str = "/open-apis/analytics/v1/visualization";

/// 智能推荐引擎
pub const INTELLIGENT_RECOMMENDATION: &str = "/open-apis/analytics/v1/recommendation";

/// 异常检测分析
pub const ANOMALY_DETECTION_ANALYTICS: &str = "/open-apis/analytics/v1/anomaly_detection";

/// 根因分析
pub const ROOT_CAUSE_ANALYSIS: &str = "/open-apis/analytics/v1/root_cause";

/// 决策支持系统
pub const DECISION_SUPPORT_SYSTEM: &str = "/open-apis/analytics/v1/decision_support";

/// KPI监控仪表板
pub const KPI_DASHBOARD_MONITORING: &str = "/open-apis/analytics/v1/kpi_dashboard";

/// 资源利用率分析
pub const RESOURCE_UTILIZATION_ANALYTICS: &str = "/open-apis/analytics/v1/resource_utilization";

/// 成本效益分析
pub const COST_BENEFIT_ANALYTICS: &str = "/open-apis/analytics/v1/cost_benefit";

// ==================== Tenant Tag (租户标签) ====================
// 租户标签管理 - 租户标识、分类、权限管理

/// 租户标签管理
pub const TENANT_TAG_V1_TAGS: &str = "/open-apis/tenant_tag/v1/tags";

/// 租户标签获取
pub const TENANT_TAG_V1_TAG_GET: &str = "/open-apis/tenant_tag/v1/tags/{tag_id}";

/// 租户标签创建
pub const TENANT_TAG_V1_TAG_CREATE: &str = "/open-apis/tenant_tag/v1/tags";

/// 租户标签更新
pub const TENANT_TAG_V1_TAG_UPDATE: &str = "/open-apis/tenant_tag/v1/tags/{tag_id}";

/// 租户标签删除
pub const TENANT_TAG_V1_TAG_DELETE: &str = "/open-apis/tenant_tag/v1/tags/{tag_id}";

/// 租户标签搜索
pub const TENANT_TAG_V1_TAGS_SEARCH: &str = "/open-apis/tenant_tag/v1/tags/search";

// ==================== 兼容性别名 ====================

/// 为保持向后兼容性，提供一些简短的别名
/// Zero Trust别名
pub const TRUST_ASSESSMENT: &str = DEVICE_TRUST_ASSESSMENT;
pub const IDENTITY_VERIFY: &str = ENHANCED_IDENTITY_VERIFICATION;

/// Analytics别名
pub const USER_ANALYTICS: &str = USER_BEHAVIOR_ANALYTICS;
pub const APP_STATS: &str = APP_USAGE_STATISTICS;

/// Tenant Tag别名
pub const TAGS: &str = TENANT_TAG_V1_TAGS;

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
        assert!(SECURITY_AND_COMPLIANCE_V1_AUDIT_DATAS
            .starts_with("/open-apis/security_and_compliance/v1/"));
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
        assert!(
            TRUST_PARTY_V1_COLLABORATION_ORGANIZATIONS.starts_with("/open-apis/trust_party/v1/")
        );
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
        assert!(
            TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_DEPARTMENT_GET.contains("{department_id}")
        );
        assert!(
            TRUST_PARTY_V1_COLLABORATION_ORGANIZATION_SHARED_MEMBER_SCOPES.contains("{org_id}")
        );
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
            assert!(
                endpoint.contains("/admin/"),
                "{} 应该包含 /admin/",
                endpoint
            );
        }

        let mdm_endpoints = [MDM_V1_COUNTRY_REGIONS, MDM_V1_USER_AUTH_DATA_RELATIONS_BIND];
        for endpoint in mdm_endpoints {
            assert!(endpoint.contains("/mdm/"), "{} 应该包含 /mdm/", endpoint);
        }

        let security_endpoints = [SECURITY_AND_COMPLIANCE_V1_AUDIT_DATAS];
        for endpoint in security_endpoints {
            assert!(
                endpoint.contains("/security_and_compliance/"),
                "{} 应该包含 /security_and_compliance/",
                endpoint
            );
        }

        let tenant_endpoints = [TENANT_V2_QUERY];
        for endpoint in tenant_endpoints {
            assert!(
                endpoint.contains("/tenant/"),
                "{} 应该包含 /tenant/",
                endpoint
            );
        }

        let workplace_endpoints = [WORKPLACE_ACCESS_DATA_SEARCH, WORKPLACE_APP_RECOMMEND_LIST];
        for endpoint in workplace_endpoints {
            assert!(
                endpoint.contains("/workplace/"),
                "{} 应该包含 /workplace/",
                endpoint
            );
        }

        let trust_party_endpoints = [
            TRUST_PARTY_V1_COLLABORATION_ORGANIZATIONS,
            TRUST_PARTY_V1_SEARCHABLE_VISIBLE_RULES,
        ];
        for endpoint in trust_party_endpoints {
            assert!(
                endpoint.contains("/trust_party/"),
                "{} 应该包含 /trust_party/",
                endpoint
            );
        }
    }

    #[test]
    fn test_zero_trust_endpoints() {
        // 验证Zero Trust端点
        assert!(DEVICE_TRUST_ASSESSMENT.starts_with("/open-apis/zero_trust/"));
        assert!(DEVICE_TRUST_ASSESSMENT.contains("device"));
        assert!(ENHANCED_IDENTITY_VERIFICATION.contains("identity"));
        assert!(MICRO_SEGMENTATION_ACCESS.contains("access"));
        assert!(CONTEXT_AWARE_ACCESS_POLICY.contains("policy"));
        assert!(BEHAVIORAL_BASELINE_ANALYSIS.contains("behavior"));
        assert!(REAL_TIME_SECURITY_AUDIT.contains("audit"));
        assert!(COMPLIANCE_STATUS_CHECK.contains("compliance"));
        assert!(END_TO_END_ENCRYPTION.contains("encryption"));
        assert!(THREAT_INTELLIGENCE_SUBSCRIPTION.contains("threat"));
        assert!(ZERO_TRUST_NETWORK_GATEWAY.contains("network"));
        assert!(FINE_GRAINED_PERMISSION_CONTROL.contains("permission"));
    }

    #[test]
    fn test_analytics_endpoints() {
        // 验证Analytics端点
        assert!(USER_BEHAVIOR_ANALYTICS.starts_with("/open-apis/analytics/"));
        assert!(USER_BEHAVIOR_ANALYTICS.contains("user_behavior"));
        assert!(APP_USAGE_STATISTICS.contains("app_usage"));
        assert!(BUSINESS_DATA_INSIGHTS.contains("business_insights"));
        assert!(TREND_PREDICTION_ANALYTICS.contains("trend_prediction"));
        assert!(REAL_TIME_DATA_MONITORING.contains("realtime_monitoring"));
        assert!(CUSTOM_REPORT_GENERATION.contains("custom_report"));
        assert!(PERFORMANCE_METRICS_ANALYTICS.contains("performance_metrics"));
        assert!(CONVERSION_FUNNEL_ANALYTICS.contains("conversion_funnel"));
        assert!(DATA_VISUALIZATION.contains("visualization"));
        assert!(INTELLIGENT_RECOMMENDATION.contains("recommendation"));
        assert!(KPI_DASHBOARD_MONITORING.contains("kpi_dashboard"));
    }

    #[test]
    fn test_tenant_tag_endpoints() {
        // 验证Tenant Tag端点
        assert!(TENANT_TAG_V1_TAGS.starts_with("/open-apis/tenant_tag/"));
        assert!(TENANT_TAG_V1_TAGS.contains("tags"));
        assert!(TENANT_TAG_V1_TAG_GET.contains("{tag_id}"));
        assert!(TENANT_TAG_V1_TAG_UPDATE.contains("{tag_id}"));
        assert!(TENANT_TAG_V1_TAG_DELETE.contains("{tag_id}"));
        assert!(TENANT_TAG_V1_TAGS_SEARCH.contains("search"));
    }

    #[test]
    fn test_new_service_grouping() {
        // 测试新增服务的分组正确性
        let zero_trust_endpoints = [
            DEVICE_TRUST_ASSESSMENT,
            ENHANCED_IDENTITY_VERIFICATION,
            MICRO_SEGMENTATION_ACCESS,
        ];
        for endpoint in zero_trust_endpoints {
            assert!(
                endpoint.contains("/zero_trust/"),
                "{} 应该包含 /zero_trust/",
                endpoint
            );
        }

        let analytics_endpoints = [
            USER_BEHAVIOR_ANALYTICS,
            APP_USAGE_STATISTICS,
            BUSINESS_DATA_INSIGHTS,
        ];
        for endpoint in analytics_endpoints {
            assert!(
                endpoint.contains("/analytics/"),
                "{} 应该包含 /analytics/",
                endpoint
            );
        }

        let tenant_tag_endpoints = [
            TENANT_TAG_V1_TAGS,
            TENANT_TAG_V1_TAG_CREATE,
            TENANT_TAG_V1_TAGS_SEARCH,
        ];
        for endpoint in tenant_tag_endpoints {
            assert!(
                endpoint.contains("/tenant_tag/"),
                "{} 应该包含 /tenant_tag/",
                endpoint
            );
        }
    }

    #[test]
    fn test_new_backward_compatibility() {
        // 验证新增端点的兼容性别名
        assert_eq!(TRUST_ASSESSMENT, DEVICE_TRUST_ASSESSMENT);
        assert_eq!(IDENTITY_VERIFY, ENHANCED_IDENTITY_VERIFICATION);
        assert_eq!(USER_ANALYTICS, USER_BEHAVIOR_ANALYTICS);
        assert_eq!(APP_STATS, APP_USAGE_STATISTICS);
        assert_eq!(TAGS, TENANT_TAG_V1_TAGS);
    }

    #[test]
    fn test_version_consistency_new_endpoints() {
        // 测试新增端点的版本一致性
        let v1_endpoints = [
            DEVICE_TRUST_ASSESSMENT,
            USER_BEHAVIOR_ANALYTICS,
            TENANT_TAG_V1_TAGS,
        ];
        for endpoint in v1_endpoints {
            assert!(endpoint.contains("/v1/"), "{} 应该包含 /v1/", endpoint);
        }
    }
} // Endpoints and EndpointBuilder are now available directly from openlark_core::endpoints
