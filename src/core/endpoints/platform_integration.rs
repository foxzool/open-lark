//! platform_integration 服务端点常量定义
//!
//! 平台集成相关 API 端点常量，包括：
//! - 第三方服务连接
//! - 低代码/无代码平台
//! - DevOps集成
//! - API网关管理

/// 第三方服务连接管理
pub const THIRD_PARTY_SERVICE_CONNECTION: &str =
    "/open-apis/platform_integration/v1/third_party/connection";

/// API网关配置
pub const API_GATEWAY_CONFIG: &str = "/open-apis/platform_integration/v1/gateway/config";

/// API流量管理
pub const API_TRAFFIC_MANAGEMENT: &str = "/open-apis/platform_integration/v1/traffic/management";

/// 服务网格集成
pub const SERVICE_MESH_INTEGRATION: &str = "/open-apis/platform_integration/v1/service_mesh";

/// 低代码平台连接器
pub const LOW_CODE_CONNECTOR: &str = "/open-apis/platform_integration/v1/low_code/connector";

/// 无代码应用构建器
pub const NO_CODE_APP_BUILDER: &str = "/open-apis/platform_integration/v1/no_code/app_builder";

/// 工作流引擎集成
pub const WORKFLOW_ENGINE_INTEGRATION: &str = "/open-apis/platform_integration/v1/workflow/engine";

/// DevOps和CI/CD集成
pub const CONTINUOUS_INTEGRATION_PIPELINE: &str =
    "/open-apis/platform_integration/v1/devops/ci_pipeline";

/// 持续部署管理
pub const CONTINUOUS_DEPLOYMENT_MANAGEMENT: &str =
    "/open-apis/platform_integration/v1/devops/cd_management";

/// 基础设施即代码
pub const INFRASTRUCTURE_AS_CODE: &str =
    "/open-apis/platform_integration/v1/devops/infrastructure_as_code";

/// 容器编排集成
pub const CONTAINER_ORCHESTRATION: &str =
    "/open-apis/platform_integration/v1/devops/container_orchestration";

/// 监控和可观测性
/// 应用性能监控
pub const APPLICATION_PERFORMANCE_MONITORING: &str =
    "/open-apis/platform_integration/v1/monitoring/apm";

/// 分布式追踪
pub const DISTRIBUTED_TRACING: &str = "/open-apis/platform_integration/v1/monitoring/tracing";

/// 日志聚合和分析
pub const LOG_AGGREGATION: &str = "/open-apis/platform_integration/v1/monitoring/log_aggregation";

/// 指标收集和告警
pub const METRICS_COLLECTION_ALERTING: &str =
    "/open-apis/platform_integration/v1/monitoring/metrics";

/// 数据集成和同步
/// 实时数据同步
pub const REAL_TIME_DATA_SYNC: &str = "/open-apis/platform_integration/v1/data/sync";

/// 数据湖集成
pub const DATA_LAKE_INTEGRATION: &str = "/open-apis/platform_integration/v1/data/lake";

/// ETL管道管理
pub const ETL_PIPELINE_MANAGEMENT: &str = "/open-apis/platform_integration/v1/data/etl";

/// 企业系统连接器
/// ERP系统集成
pub const ERP_SYSTEM_INTEGRATION: &str = "/open-apis/platform_integration/v1/enterprise/erp";

/// CRM系统集成
pub const CRM_SYSTEM_INTEGRATION: &str = "/open-apis/platform_integration/v1/enterprise/crm";

/// HR系统集成
pub const HR_SYSTEM_INTEGRATION: &str = "/open-apis/platform_integration/v1/enterprise/hr";

/// 财务系统集成
pub const FINANCE_SYSTEM_INTEGRATION: &str =
    "/open-apis/platform_integration/v1/enterprise/finance";

/// API市场和管理
/// API市场发布
pub const API_MARKETPLACE_PUBLISH: &str = "/open-apis/platform_integration/v1/marketplace/publish";

/// API使用统计
pub const API_USAGE_ANALYTICS: &str = "/open-apis/platform_integration/v1/marketplace/usage";

/// API版本管理
pub const API_VERSION_MANAGEMENT: &str = "/open-apis/platform_integration/v1/api/version";

/// 安全和合规集成
/// 统一身份认证
pub const UNIFIED_IDENTITY_AUTHENTICATION: &str =
    "/open-apis/platform_integration/v1/security/unified_auth";

/// OAuth2.0和SAML集成
pub const OAUTH_SAML_INTEGRATION: &str = "/open-apis/platform_integration/v1/security/oauth_saml";

/// Webhook管理
pub const WEBHOOK_MANAGEMENT: &str = "/open-apis/platform_integration/v1/webhook/management";

/// 开发者工具
/// API测试套件
pub const API_TESTING_SUITE: &str = "/open-apis/platform_integration/v1/developer/api_testing";

/// 文档自动生成
pub const AUTO_DOCUMENTATION_GENERATION: &str =
    "/open-apis/platform_integration/v1/developer/doc_generation";

/// SDK生成器
pub const SDK_GENERATOR: &str = "/open-apis/platform_integration/v1/developer/sdk_generator";
