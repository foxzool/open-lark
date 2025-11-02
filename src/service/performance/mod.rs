//! Performance服务模块 - 企业绩效管理
//!
//! 提供全面的绩效管理功能：
//! - 绩效考核周期管理
//! - 目标设定与跟踪
//! - 绩效评估与反馈
//! - 绩效结果分析与统计
//! - 绩效流程审批

use crate::core::config::Config;

// 声明子模块
pub mod models;
pub mod v1;

// 重新导出服务类型
pub use v1::PerformanceV1Service;

/// Performance服务
#[derive(Debug, Clone)]
pub struct PerformanceService {
    pub config: Config,
    pub v1: PerformanceV1Service,
}

impl PerformanceService {
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            v1: v1::PerformanceV1Service::new(config),
        }
    }
}

// 重新导出常用类型
pub use models::*;
pub use models::PageResponse;

/// Performance服务类型别名（向后兼容）
pub type ServiceType = PerformanceService;
