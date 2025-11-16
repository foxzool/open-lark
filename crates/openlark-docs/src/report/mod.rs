//! Report 报告分析API模块
//!
//! 提供报告分析和数据统计相关的功能，包括：
//! - VC会议报告和用户统计
//! - 规则管理看板
//! - 任务查询和分析
//!
//! # 示例
//!
//! ```rust
//! use openlark_docs::report::{ReportService, GetDailyReportRequest};
//!
//! let service = ReportService::new(config);
//!
//! // 获取每日会议报告
//! let response = service
//!     .vc_report()
//!     .get_daily_report_builder()
//!     .start_date("2024-01-01")
//!     .end_date("2024-01-31")
//!     .execute(&service.vc_report())
//!     .await?;
//!
//! if let Some(report_data) = response.data {
//!     println!("报告数据: {:?}", report_data);
//! }
//! ```

/// VC会议报告
pub mod vc_report;

/// 报告规则管理
pub mod rule;

/// 报告任务管理
pub mod task;

// 重新导出主要类型
pub use rule::{QueryRuleRequestBuilder, RemoveRuleViewRequestBuilder, RuleService};
pub use task::{QueryTaskRequestBuilder, TaskService};
pub use vc_report::{
    GetDailyReportRequestBuilder, GetTopUserReportRequestBuilder, VcReportService,
};

/// 报告分析服务
#[derive(Debug, Clone)]
pub struct ReportService {
    config: openlark_core::config::Config,
}

impl ReportService {
    /// 创建新的报告分析服务实例
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self { config }
    }

    /// 获取VC会议报告服务
    pub fn vc_report(&self) -> vc_report::VcReportService {
        vc_report::VcReportService::new(self.config.clone())
    }

    /// 获取规则管理服务
    pub fn rule(&self) -> rule::RuleService {
        rule::RuleService::new(self.config.clone())
    }

    /// 获取任务管理服务
    pub fn task(&self) -> task::TaskService {
        task::TaskService::new(self.config.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_report_service_creation() {
        let config = openlark_core::config::Config::default();
        let service = ReportService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_service_submodules() {
        let config = openlark_core::config::Config::default();
        let service = ReportService::new(config);

        // 测试子模块访问
        let _vc_report_service = service.vc_report();
        let _rule_service = service.rule();
        let _task_service = service.task();
    }
}
