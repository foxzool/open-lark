#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! Security & Compliance安全与合规服务
//!
//! 提供企业级安全与合规管理的完整功能：
//! - 实时安全监控和威胁检测
//! - 法规合规性监控和自动化报告
//! - 智能风险评估和预警系统
//! - 细粒度访问控制和权限管理
//! - 完整的审计追踪和日志管理
//! - 安全策略配置和自动化执行

use config::Config;

/// Security & Compliance服务主入口
#[derive(Debug, Clone)]
pub struct SecurityAndComplianceService {
    pub config: Config,
    /// V1版本服务
    #[cfg(feature = "security_and_compliance")]
    pub v1: crate::service::security_and_compliance::v1::SecurityAndComplianceServiceV1,
}

impl SecurityAndComplianceService {
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            #[cfg(feature = "security_and_compliance")]
            v1: crate::service::security_and_compliance::v1::SecurityAndComplianceServiceV1::new(
                config,
            ),
        }
    }
}

// V1版本模块
#[cfg(feature = "security_and_compliance")]
pub mod v1;

// 重新导出V1版本的所有类型服务
#[cfg(feature = "security_and_compliance")]
pub use v1::*;

// 向后兼容的简化实现（保持兼容性）
#[derive(Debug, Clone)]
pub struct SimpleService {
    pub config: Config,
}

impl SimpleService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

/// Security_And_Compliance服务（向后兼容）
#[derive(Debug, Clone)]
pub struct SecurityandcomplianceService {
    pub service: SimpleService,
}

impl SecurityandcomplianceService {
    pub fn new(config: Config) -> Self {
        Self {
            service: SimpleService::new(config),
        }
    }
}

// Type alias for compatibility
pub type ServiceType = SecurityandcomplianceService;
