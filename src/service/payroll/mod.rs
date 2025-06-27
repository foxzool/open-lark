pub mod acct_item;
pub mod cost_allocation_plan;
pub mod cost_allocation_report;
pub mod datasource;
pub mod datasource_record;
pub mod models;
pub mod paygroup;
pub mod payment_activity;
pub mod payment_detail;

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
