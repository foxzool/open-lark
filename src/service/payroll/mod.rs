//! Payroll薪酬管理服务模块
//!
//! 提供飞书薪酬管理的完整API功能，包括：
//! - 薪资计算和发放
//! - 薪资组管理
//! - 算薪项配置
//! - 发薪活动和明细
//! - 成本分摊和报表
//! - 税务和社保处理

use crate::core::config::Config;

/// Payroll薪酬管理服务
#[derive(Debug, Clone)]
pub struct PayrollService {
    pub config: Config,
    pub v1: v1::PayrollServiceV1,
}

impl PayrollService {
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            v1: v1::PayrollServiceV1::new(config),
        }
    }
}

pub mod v1;

// 重新导出所有模块和类型
pub use v1::*;

// 导入子模块
pub mod models;
pub mod payment_activity;
pub mod payment_detail;
pub mod paygroup;
pub mod datasource;
pub mod datasource_record;
pub mod acct_item;
pub mod cost_allocation_plan;
pub mod cost_allocation_report;

// 重新导出类型
pub use models::*;
