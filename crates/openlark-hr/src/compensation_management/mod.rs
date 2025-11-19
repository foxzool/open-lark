//! 薪酬管理服务
//!
//! 提供企业薪酬管理的全面解决方案，包括：
//! - 薪酬档案管理
//! - 薪酬项目和类别管理
//! - 薪酬指标和计划管理
//! - 变更原因和社保管理
//! - 一次性及循环支付管理

// Re-export Config from the main crate for compatibility
pub use openlark_core::config::Config;

/// 薪酬管理服务
#[derive(Debug, Clone)]
pub struct CompensationManagementService {
    pub config: Config,
    pub v1: v1::CompensationManagementServiceV1,
}

impl CompensationManagementService {
    /// 创建薪酬管理服务实例
    pub fn new(config: Config) -> Self {
        let v1 = v1::CompensationManagementServiceV1::new(config.clone());
        Self { config, v1 }
    }
}

/// v1版本API
pub mod v1;

pub use v1::*;
