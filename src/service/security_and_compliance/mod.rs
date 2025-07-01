//! 安全合规（Security and Compliance）服务
//!
//! 提供飞书安全合规的完整功能集，支持审计日志、OpenAPI日志、
//! 行为监控、合规检查等企业级安全管理能力。是企业安全体系的重要组成部分。
//!
//! # 核心功能
//!
//! ## OpenAPI审计日志
//! - 📋 OpenAPI调用日志记录
//! - 🔍 API访问行为分析
//! - 📊 API使用统计报告
//! - 🚨 异常API调用检测
//! - 📈 API性能监控分析
//!
//! ## 行为审计日志
//! - 👤 用户行为全程记录
//! - 🔒 敏感操作审计跟踪
//! - 📊 行为模式分析统计
//! - 🚨 异常行为告警机制
//! - 📋 合规性审查支持
//!
//! ## 安全监控
//! - 🔍 实时安全事件监控
//! - 🚨 安全威胁检测告警
//! - 📊 安全态势分析展示
//! - 🛡️ 安全策略执行监督
//! - 📈 安全指标统计分析
//!
//! ## 合规管理
//! - 📋 合规规则配置管理
//! - ✅ 合规状态检查验证
//! - 📊 合规报告自动生成
//! - 🔍 合规问题追溯分析
//! - 📈 合规趋势监控评估
//!
//! # 使用示例
//!
//! ```rust
//! use open_lark::prelude::*;
//!
//! let client = LarkClient::builder("app_id", "app_secret")
//!     .with_app_type(AppType::SelfBuild)
//!     .build();
//!
//! // 获取安全合规服务
//! let security = &client.security_and_compliance;
//!
//! // 查询OpenAPI日志
//! // let api_log_request = QueryOpenapiLogRequest::builder()
//! //     .start_time("2024-01-01T00:00:00Z")
//! //     .end_time("2024-01-31T23:59:59Z")
//! //     .api_path("/open-apis/im/v1/messages")
//! //     .page_size(100)
//! //     .build();
//! // let api_logs = security.openapi_log.query(api_log_request, None).await?;
//!
//! // 查询审计日志
//! // let audit_request = QueryAuditLogRequest::builder()
//! //     .start_time("2024-01-01T00:00:00Z")
//! //     .end_time("2024-01-31T23:59:59Z")
//! //     .event_type("user_login")
//! //     .user_id("user_123")
//! //     .build();
//! // let audit_logs = security.audit_log.query(audit_request, None).await?;
//!
//! // 获取安全统计
//! // let stats_request = GetSecurityStatsRequest::builder()
//! //     .date_range("last_30_days")
//! //     .metric_types(vec!["api_calls", "login_attempts", "data_access"])
//! //     .build();
//! // let stats = security.audit_log.get_stats(stats_request, None).await?;
//!
//! // 检查合规状态
//! // let compliance_request = CheckComplianceRequest::builder()
//! //     .check_type("data_retention")
//! //     .scope("tenant")
//! //     .build();
//! // let compliance_status = security.audit_log.check_compliance(compliance_request, None).await?;
//! ```
//!
//! # 安全管理特性
//!
//! - 🔐 全方位的安全监控
//! - 📋 完整的审计追踪
//! - 🚨 智能的异常检测
//! - 📊 详细的分析报告
//! - ✅ 自动化的合规检查
//!
//! # 企业应用
//!
//! - 🏢 企业安全管理体系
//! - 📋 合规性审查支持
//! - 🔍 安全事件调查分析
//! - 📊 安全风险评估
//! - 🛡️ 数据保护和隐私

use crate::core::config::Config;

// 子模块声明
pub mod audit_log;
pub mod models;
pub mod openapi_log;

// 重新导出服务类型
pub use audit_log::AuditLogService;
pub use openapi_log::OpenapiLogService;

/// 安全合规服务
///
/// 提供飞书安全合规相关功能，包括：
/// - OpenAPI 审计日志
/// - 行为审计日志
pub struct SecurityAndComplianceService {
    pub openapi_log: OpenapiLogService,
    pub audit_log: AuditLogService,
}

impl SecurityAndComplianceService {
    pub fn new(config: Config) -> Self {
        Self {
            openapi_log: OpenapiLogService::new(config.clone()),
            audit_log: AuditLogService::new(config),
        }
    }
}
