//! Payroll API v1版本
//!
//! 实现薪酬管理的核心功能：
//! - 薪资计算和发放
//! - 薪资组管理
//! - 算薪项配置
//! - 发薪活动和明细管理
//! - 成本分摊和报表生成
//! - 数据源管理

use crate::core::config::Config;
use open_lark_core::prelude::*;
use serde::{Deserialize, Serialize};

/// Payroll服务 v1版本
#[derive(Debug, Clone)]
pub struct PayrollServiceV1 {
    pub config: Config,
    pub calculation: CalculationService,
    pub payment: PaymentService,
    pub paygroup: PaygroupService,
    pub acct_item: AcctItemService,
    pub report: ReportService,
    pub datasource: DatasourceService,
}

impl PayrollServiceV1 {
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            calculation: CalculationService::new(config.clone()),
            payment: PaymentService::new(config.clone()),
            paygroup: PaygroupService::new(config.clone()),
            acct_item: AcctItemService::new(config.clone()),
            report: ReportService::new(config.clone()),
            datasource: DatasourceService::new(config),
        }
    }
}

// 导入所有子模块
pub mod calculation;
pub mod payment;
pub mod paygroup;
pub mod acct_item;
pub mod report;
pub mod datasource;

// 保留原有的事件处理模块
pub mod p2_payroll_payment_activity_approved_v1;
pub mod p2_payroll_payment_activity_status_changed_v1;

// 重新导出所有模块和类型
pub use calculation::*;
pub use payment::*;
pub use paygroup::*;
pub use acct_item::*;
pub use report::*;
pub use datasource::*;
