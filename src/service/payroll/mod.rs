//! 飞书发薪（Payroll）服务
//!
//! 提供飞书发薪的完整功能集，支持发薪活动管理、发薪明细查询、算薪项配置、
//! 成本分摊、薪资组管理等企业级薪酬管理能力。是企业薪酬体系的核心基础。
//!
//! # 核心功能
//!
//! ## 发薪活动管理
//! - 💰 发薪活动创建和管理
//! - 📊 发薪状态跟踪监控
//! - 🔒 发薪活动封存处理
//! - 📈 发薪数据统计分析
//! - 🔄 发薪流程自动化
//!
//! ## 发薪明细查询
//! - 📋 发薪明细批量查询
//! - 🔍 多维度筛选和检索
//! - 📊 薪资结构分析展示
//! - 📈 发薪数据统计汇总
//! - 📱 移动端明细查看
//!
//! ## 算薪项配置
//! - ⚙️ 算薪项参数配置管理
//! - 📊 算薪项批量查询
//! - 🔢 薪酬计算规则设置
//! - 📋 算薪项目分类管理
//! - 🔄 算薪逻辑优化调整
//!
//! ## 外部数据源管理
//! - 🔗 外部算薪数据集成
//! - 📊 数据源记录管理
//! - 🔄 数据同步和更新
//! - 📋 数据源配置信息
//! - 🛡️ 数据安全和验证
//!
//! ## 成本分摊管理
//! - 📊 成本分摊报表生成
//! - 🎯 成本分摊方案管理
//! - 📈 成本分摊规则配置
//! - 💼 成本中心分配
//! - 📋 分摊数据统计分析
//!
//! ## 薪资组管理
//! - 👥 薪资组基本信息管理
//! - ⚙️ 薪资组配置和设置
//! - 📊 薪资组统计分析
//! - 🔄 薪资组调整和优化
//! - 📋 薪资组成员管理
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
//! // 获取发薪服务
//! let payroll = &client.payroll;
//!
//! // 查询发薪活动
//! // let activity_request = ListPaymentActivityRequest::builder()
//! //     .page_size(50)
//! //     .status("active")
//! //     .year("2024")
//! //     .build();
//! // let activities = payroll.payment_activity.list(activity_request, None).await?;
//!
//! // 查询发薪明细
//! // let detail_request = ListPaymentDetailRequest::builder()
//! //     .payment_activity_id("activity_123")
//! //     .employee_id("emp_456")
//! //     .page_size(100)
//! //     .build();
//! // let details = payroll.payment_detail.list(detail_request, None).await?;
//!
//! // 查询算薪项
//! // let acct_request = ListAcctItemRequest::builder()
//! //     .paygroup_id("paygroup_789")
//! //     .category("basic_salary")
//! //     .build();
//! // let acct_items = payroll.acct_item.list(acct_request, None).await?;
//!
//! // 查询成本分摊报表
//! // let report_request = ListCostAllocationReportRequest::builder()
//! //     .start_date("2024-01-01")
//! //     .end_date("2024-12-31")
//! //     .cost_center_id("cc_123")
//! //     .build();
//! // let reports = payroll.cost_allocation_report.list(report_request, None).await?;
//!
//! // 管理外部数据源
//! // let datasource_request = CreateDatasourceRecordRequest::builder()
//! //     .datasource_id("ds_456")
//! //     .employee_id("emp_789")
//! //     .data(serde_json::json!({
//! //         "bonus": 5000,
//! //         "allowance": 1000
//! //     }))
//! //     .build();
//! // payroll.datasource_record.create(datasource_request, None).await?;
//! ```
//!
//! # 薪酬管理特性
//!
//! - 💰 完整的薪酬管理体系
//! - 📊 精准的成本分摊机制
//! - 🔗 灵活的外部数据集成
//! - 📋 标准化的薪酬处理流程
//! - 🛡️ 严格的数据安全保护
//!
//! # 企业应用
//!
//! - 💼 企业薪酬管理系统
//! - 📊 财务成本分析报告
//! - 🔗 HR系统数据集成
//! - 📋 合规性管理支持
//! - 📈 薪酬数据分析洞察

pub mod acct_item;
pub mod cost_allocation_plan;
pub mod cost_allocation_report;
pub mod datasource;
pub mod datasource_record;
pub mod models;
pub mod paygroup;
pub mod payment_activity;
pub mod payment_detail;
pub mod v1;

use crate::core::config::Config;

use acct_item::AcctItemService;
use cost_allocation_plan::CostAllocationPlanService;
use cost_allocation_report::CostAllocationReportService;
use datasource::DatasourceService;
use datasource_record::DatasourceRecordService;
use paygroup::PaygroupService;
use payment_activity::PaymentActivityService;
use payment_detail::PaymentDetailService;

/// 飞书发薪服务
///
/// 飞书发薪为企业提供了完整的薪酬管理和发薪处理功能，包括发薪活动管理、
/// 发薪明细查询、算薪项配置、成本分摊等核心功能。本服务封装了相关API接口，支持：
///
/// ## 主要功能
///
/// ### 发薪明细管理
/// - 查询发薪活动明细列表
/// - 批量查询发薪明细
/// - 支持多维度筛选和分页查询
///
/// ### 发薪活动管理
/// - 查询发薪活动列表
/// - 封存发薪活动
/// - 发薪活动状态变更监听
///
/// ### 外部数据源管理
/// - 创建/更新外部算薪数据
/// - 批量查询外部算薪数据记录
/// - 获取外部数据源配置信息
///
/// ### 算薪项配置
/// - 批量查询算薪项
/// - 算薪项参数配置
///
/// ### 成本分摊管理
/// - 查询成本分摊报表汇总数据
/// - 批量查询成本分摊方案
/// - 成本分摊规则配置
///
/// ### 薪资组管理
/// - 获取薪资组基本信息
/// - 薪资组配置管理
///
/// ## 使用场景
///
/// - **薪酬管理系统**: 与第三方薪酬系统集成发薪数据
/// - **财务报表**: 基于发薪数据生成财务和成本分摊报表
/// - **合规管理**: 确保发薪流程符合法规要求
/// - **成本控制**: 通过成本分摊实现精细化成本管理
/// - **薪酬分析**: 分析薪酬结构和发薪趋势
///
/// ## 权限要求
///
/// 使用本服务需要相应的应用权限：
/// - `payroll:payment_detail`: 发薪明细查询权限
/// - `payroll:payment_activity`: 发薪活动管理权限
/// - `payroll:datasource`: 外部数据源管理权限
/// - `payroll:acct_item`: 算薪项查询权限
/// - `payroll:cost_allocation`: 成本分摊查询权限
/// - `payroll:paygroup`: 薪资组查询权限
///
/// ## 示例用法
///
/// ```ignore
/// use open_lark::prelude::*;
/// use open_lark::service::payroll::models::*;
///
/// // 创建客户端
/// let client = LarkClient::builder(app_id, app_secret)
///     .with_app_type(AppType::SelfBuild)
///     .build();
///
/// // 查询发薪活动列表
/// let activity_request = PaymentActivityListRequest {
///     page_size: Some(50),
///     page_token: None,
///     status: Some("active".to_string()),
///     ..Default::default()
/// };
///
/// let activities = client.payroll.payment_activity.list_activities(activity_request, None).await?;
///
/// // 查询发薪明细
/// let detail_request = PaymentDetailListRequest {
///     payment_activity_id: "activity_123".to_string(),
///     page_size: Some(100),
///     employee_id: Some("emp_456".to_string()),
///     ..Default::default()
/// };
///
/// let details = client.payroll.payment_detail.list_details(detail_request, None).await?;
///
/// // 查询成本分摊报表
/// let report_request = CostAllocationReportListRequest {
///     start_date: "2024-01-01".to_string(),
///     end_date: "2024-12-31".to_string(),
///     cost_center_id: Some("cc_789".to_string()),
///     ..Default::default()
/// };
///
/// let reports = client.payroll.cost_allocation_report.list_reports(report_request, None).await?;
/// ```
pub struct PayrollService {
    /// 发薪明细服务
    pub payment_detail: PaymentDetailService,
    /// 发薪活动服务
    pub payment_activity: PaymentActivityService,
    /// 外部数据源记录服务
    pub datasource_record: DatasourceRecordService,
    /// 外部数据源服务
    pub datasource: DatasourceService,
    /// 算薪项服务
    pub acct_item: AcctItemService,
    /// 成本分摊报表服务
    pub cost_allocation_report: CostAllocationReportService,
    /// 成本分摊方案服务
    pub cost_allocation_plan: CostAllocationPlanService,
    /// 薪资组服务
    pub paygroup: PaygroupService,
}

impl PayrollService {
    pub fn new(config: Config) -> Self {
        Self {
            payment_detail: PaymentDetailService::new(config.clone()),
            payment_activity: PaymentActivityService::new(config.clone()),
            datasource_record: DatasourceRecordService::new(config.clone()),
            datasource: DatasourceService::new(config.clone()),
            acct_item: AcctItemService::new(config.clone()),
            cost_allocation_report: CostAllocationReportService::new(config.clone()),
            cost_allocation_plan: CostAllocationPlanService::new(config.clone()),
            paygroup: PaygroupService::new(config),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_payroll_service_creation() {
        let config = Config::default();
        let service = PayrollService::new(config.clone());

        assert_eq!(service.payment_detail.config.app_id, config.app_id);
        assert_eq!(service.payment_detail.config.app_secret, config.app_secret);
        assert_eq!(service.payment_activity.config.app_id, config.app_id);
        assert_eq!(service.datasource_record.config.app_id, config.app_id);
        assert_eq!(service.datasource.config.app_id, config.app_id);
        assert_eq!(service.acct_item.config.app_id, config.app_id);
        assert_eq!(service.cost_allocation_report.config.app_id, config.app_id);
        assert_eq!(service.cost_allocation_plan.config.app_id, config.app_id);
        assert_eq!(service.paygroup.config.app_id, config.app_id);
    }

    #[test]
    fn test_payroll_service_with_custom_config() {
        let config = Config::builder()
            .app_id("payroll_test_app")
            .app_secret("payroll_test_secret")
            .req_timeout(Duration::from_secs(240))
            .build();

        let service = PayrollService::new(config.clone());

        assert_eq!(service.payment_detail.config.app_id, "payroll_test_app");
        assert_eq!(
            service.payment_detail.config.app_secret,
            "payroll_test_secret"
        );
        assert_eq!(
            service.payment_detail.config.req_timeout,
            Some(Duration::from_secs(240))
        );
        assert_eq!(service.payment_activity.config.app_id, "payroll_test_app");
        assert_eq!(
            service.datasource.config.req_timeout,
            Some(Duration::from_secs(240))
        );
        assert_eq!(service.acct_item.config.app_secret, "payroll_test_secret");
        assert_eq!(
            service.cost_allocation_report.config.req_timeout,
            Some(Duration::from_secs(240))
        );
        assert_eq!(service.paygroup.config.app_id, "payroll_test_app");
    }

    #[test]
    fn test_payroll_service_config_independence() {
        let config1 = Config::builder().app_id("payroll_app_1").build();

        let config2 = Config::builder().app_id("payroll_app_2").build();

        let service1 = PayrollService::new(config1);
        let service2 = PayrollService::new(config2);

        assert_eq!(service1.payment_detail.config.app_id, "payroll_app_1");
        assert_eq!(service2.payment_detail.config.app_id, "payroll_app_2");
        assert_ne!(
            service1.payment_detail.config.app_id,
            service2.payment_detail.config.app_id
        );
        assert_ne!(
            service1.datasource.config.app_id,
            service2.paygroup.config.app_id
        );
    }

    #[test]
    fn test_payroll_service_sub_services_accessible() {
        let config = Config::default();
        let service = PayrollService::new(config.clone());

        assert_eq!(service.payment_detail.config.app_id, config.app_id);
        assert_eq!(service.payment_activity.config.app_id, config.app_id);
        assert_eq!(service.datasource_record.config.app_id, config.app_id);
        assert_eq!(service.datasource.config.app_id, config.app_id);
        assert_eq!(service.acct_item.config.app_id, config.app_id);
        assert_eq!(service.cost_allocation_report.config.app_id, config.app_id);
        assert_eq!(service.cost_allocation_plan.config.app_id, config.app_id);
        assert_eq!(service.paygroup.config.app_id, config.app_id);
    }

    #[test]
    fn test_payroll_service_config_cloning() {
        let config = Config::builder()
            .app_id("clone_test_app")
            .app_secret("clone_test_secret")
            .build();

        let service = PayrollService::new(config.clone());

        assert_eq!(service.payment_detail.config.app_id, "clone_test_app");
        assert_eq!(
            service.payment_detail.config.app_secret,
            "clone_test_secret"
        );
        assert_eq!(
            service.payment_activity.config.app_secret,
            "clone_test_secret"
        );
        assert_eq!(service.datasource_record.config.app_id, "clone_test_app");
        assert_eq!(service.datasource.config.app_secret, "clone_test_secret");
        assert_eq!(service.acct_item.config.app_id, "clone_test_app");
        assert_eq!(
            service.cost_allocation_plan.config.app_secret,
            "clone_test_secret"
        );
        assert_eq!(service.paygroup.config.app_id, "clone_test_app");
    }

    #[test]
    fn test_payroll_service_timeout_propagation() {
        let config = Config::builder()
            .req_timeout(Duration::from_secs(220))
            .build();

        let service = PayrollService::new(config);

        assert_eq!(
            service.payment_detail.config.req_timeout,
            Some(Duration::from_secs(220))
        );
        assert_eq!(
            service.payment_activity.config.req_timeout,
            Some(Duration::from_secs(220))
        );
        assert_eq!(
            service.datasource_record.config.req_timeout,
            Some(Duration::from_secs(220))
        );
        assert_eq!(
            service.datasource.config.req_timeout,
            Some(Duration::from_secs(220))
        );
        assert_eq!(
            service.acct_item.config.req_timeout,
            Some(Duration::from_secs(220))
        );
        assert_eq!(
            service.cost_allocation_report.config.req_timeout,
            Some(Duration::from_secs(220))
        );
        assert_eq!(
            service.cost_allocation_plan.config.req_timeout,
            Some(Duration::from_secs(220))
        );
        assert_eq!(
            service.paygroup.config.req_timeout,
            Some(Duration::from_secs(220))
        );
    }

    #[test]
    fn test_payroll_service_multiple_instances() {
        let config = Config::default();

        let service1 = PayrollService::new(config.clone());
        let service2 = PayrollService::new(config.clone());

        assert_eq!(
            service1.payment_detail.config.app_id,
            service2.payment_detail.config.app_id
        );
        assert_eq!(
            service1.payment_detail.config.app_secret,
            service2.payment_detail.config.app_secret
        );
        assert_eq!(
            service1.payment_activity.config.app_id,
            service2.payment_activity.config.app_id
        );
        assert_eq!(
            service1.datasource.config.app_secret,
            service2.datasource.config.app_secret
        );
        assert_eq!(
            service1.acct_item.config.app_id,
            service2.acct_item.config.app_id
        );
        assert_eq!(
            service1.cost_allocation_report.config.app_secret,
            service2.cost_allocation_report.config.app_secret
        );
        assert_eq!(
            service1.paygroup.config.app_id,
            service2.paygroup.config.app_id
        );
    }

    #[test]
    fn test_payroll_service_config_consistency() {
        let config = Config::builder()
            .app_id("consistency_test")
            .app_secret("consistency_secret")
            .req_timeout(Duration::from_secs(190))
            .build();

        let service = PayrollService::new(config);

        assert_eq!(service.payment_detail.config.app_id, "consistency_test");
        assert_eq!(
            service.payment_detail.config.app_secret,
            "consistency_secret"
        );
        assert_eq!(
            service.payment_detail.config.req_timeout,
            Some(Duration::from_secs(190))
        );
        assert_eq!(service.payment_activity.config.app_id, "consistency_test");
        assert_eq!(
            service.datasource_record.config.app_secret,
            "consistency_secret"
        );
        assert_eq!(
            service.datasource.config.req_timeout,
            Some(Duration::from_secs(190))
        );
        assert_eq!(service.acct_item.config.app_id, "consistency_test");
        assert_eq!(
            service.cost_allocation_plan.config.app_secret,
            "consistency_secret"
        );
        assert_eq!(
            service.paygroup.config.req_timeout,
            Some(Duration::from_secs(190))
        );
    }
}
