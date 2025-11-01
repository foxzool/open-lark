//! # 飞书汇报服务
//!
//! 飞书汇报 (Report) 服务提供汇报规则、看板和任务管理功能，支持以下核心能力：
//!
//! ## 功能特性
//!
//! - **规则管理**：查询汇报规则
//! - **规则看板管理**：移除规则看板
//! - **任务管理**：查询汇报任务
//!
//! ## 服务模块
//!
//! 该服务包含以下功能模块：
//!
//! - [`models`] - 数据模型和类型定义
//! - [`rule`] - 规则管理模块
//! - [`rule_view`] - 规则看板管理模块
//! - [`task`] - 任务管理模块
//!
//! ## 使用示例
//!
//! ```rust,no_run
//! use open_lark::prelude::*;
//! use open_lark::service::report::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = LarkClient::builder("app_id", "app_secret")
//!         .build();
//!
//!     // 查询规则
//!     let rules = client.report.rule.query(
//!         rule::RuleQueryRequest::default(), None
//!     ).await?;
//!     
//!     // 查询任务
//!     let tasks = client.report.task.query(
//!         task::TaskQueryRequest::default(), None
//!     ).await?;
//!     
//!     Ok(())
//! }
//! ```

pub mod models;
pub mod rule;
pub mod rule_view;
pub mod task;

use crate::{
    core::config::Config,
    service::report::{rule::RuleService, rule_view::RuleViewService, task::TaskService},
};

/// 飞书汇报服务
///
/// 提供完整的汇报功能，包括规则、看板和任务管理
pub struct ReportService {
    /// 规则管理服务
    pub rule: RuleService,
    /// 规则看板管理服务
    pub rule_view: RuleViewService,
    /// 任务管理服务
    pub task: TaskService,
}

impl ReportService {
    /// 创建飞书汇报服务实例
    pub fn new(config: Config) -> Self {
        Self {
            rule: RuleService::new(config.clone()),
            rule_view: RuleViewService::new(config.clone()),
            task: TaskService::new(config),
        }
    }

    /// 使用共享配置创建服务（实验性）
    pub fn new_from_shared(shared: std::sync::Arc<Config>) -> Self {
        Self {
            rule: RuleService::new(shared.as_ref().clone()),
            rule_view: RuleViewService::new(shared.as_ref().clone()),
            task: TaskService::new(shared.as_ref().clone()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_report_service_creation() {
        let config = Config::default();
        let service = ReportService::new(config.clone());

        assert_eq!(service.rule.config.app_id, config.app_id);
        assert_eq!(service.rule.config.app_secret, config.app_secret);
        assert_eq!(service.rule_view.config.app_id, config.app_id);
        assert_eq!(service.task.config.app_id, config.app_id);
    }

    #[test]
    fn test_report_service_with_custom_config() {
        let config = Config::builder()
            .app_id("report_test_app")
            .app_secret("report_test_secret")
            .req_timeout(Duration::from_secs(170))
            .build();

        let service = ReportService::new(config.clone());

        assert_eq!(service.rule.config.app_id, "report_test_app");
        assert_eq!(service.rule.config.app_secret, "report_test_secret");
        assert_eq!(
            service.rule.config.req_timeout,
            Some(Duration::from_secs(170))
        );
        assert_eq!(service.rule_view.config.app_id, "report_test_app");
        assert_eq!(
            service.task.config.req_timeout,
            Some(Duration::from_secs(170))
        );
    }

    #[test]
    fn test_report_service_config_independence() {
        let config1 = Config::builder().app_id("report_app_1").build();

        let config2 = Config::builder().app_id("report_app_2").build();

        let service1 = ReportService::new(config1);
        let service2 = ReportService::new(config2);

        assert_eq!(service1.rule.config.app_id, "report_app_1");
        assert_eq!(service2.rule.config.app_id, "report_app_2");
        assert_ne!(service1.rule.config.app_id, service2.rule.config.app_id);
        assert_ne!(
            service1.rule_view.config.app_id,
            service2.task.config.app_id
        );
    }

    #[test]
    fn test_report_service_sub_services_accessible() {
        let config = Config::default();
        let service = ReportService::new(config.clone());

        assert_eq!(service.rule.config.app_id, config.app_id);
        assert_eq!(service.rule_view.config.app_id, config.app_id);
        assert_eq!(service.task.config.app_id, config.app_id);
    }

    #[test]
    fn test_report_service_config_cloning() {
        let config = Config::builder()
            .app_id("clone_test_app")
            .app_secret("clone_test_secret")
            .build();

        let service = ReportService::new(config.clone());

        assert_eq!(service.rule.config.app_id, "clone_test_app");
        assert_eq!(service.rule.config.app_secret, "clone_test_secret");
        assert_eq!(service.rule_view.config.app_secret, "clone_test_secret");
        assert_eq!(service.task.config.app_id, "clone_test_app");
    }

    #[test]
    fn test_report_service_timeout_propagation() {
        let config = Config::builder()
            .req_timeout(Duration::from_secs(190))
            .build();

        let service = ReportService::new(config);

        assert_eq!(
            service.rule.config.req_timeout,
            Some(Duration::from_secs(190))
        );
        assert_eq!(
            service.rule_view.config.req_timeout,
            Some(Duration::from_secs(190))
        );
        assert_eq!(
            service.task.config.req_timeout,
            Some(Duration::from_secs(190))
        );
    }

    #[test]
    fn test_report_service_multiple_instances() {
        let config = Config::default();

        let service1 = ReportService::new(config.clone());
        let service2 = ReportService::new(config.clone());

        assert_eq!(service1.rule.config.app_id, service2.rule.config.app_id);
        assert_eq!(
            service1.rule.config.app_secret,
            service2.rule.config.app_secret
        );
        assert_eq!(
            service1.rule_view.config.app_id,
            service2.rule_view.config.app_id
        );
        assert_eq!(
            service1.task.config.app_secret,
            service2.task.config.app_secret
        );
    }

    #[test]
    fn test_report_service_config_consistency() {
        let config = Config::builder()
            .app_id("consistency_test")
            .app_secret("consistency_secret")
            .req_timeout(Duration::from_secs(140))
            .build();

        let service = ReportService::new(config);

        assert_eq!(service.rule.config.app_id, "consistency_test");
        assert_eq!(service.rule.config.app_secret, "consistency_secret");
        assert_eq!(
            service.rule.config.req_timeout,
            Some(Duration::from_secs(140))
        );
        assert_eq!(service.rule_view.config.app_id, "consistency_test");
        assert_eq!(service.task.config.app_secret, "consistency_secret");
    }
}
