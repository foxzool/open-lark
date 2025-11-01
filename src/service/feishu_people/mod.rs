//! Feishu People 服务模块 - 人员管理
//!
//! 提供完整的企业人员管理功能，包括：
//! - 员工信息管理
//! - 组织架构管理
//! - 职位和合同管理
//! - 公司信息管理
//! - 人事统计分析

use crate::core::config::Config;

pub mod authorizations;
pub mod basic_data;
/// 重新导出核心模块
pub mod core;
pub mod leaves;

/// Feishu People 服务主入口
///
/// 提供对企业人员管理相关API的统一访问接口，
/// 包括员工、部门、职位、合同、公司等核心功能。
#[derive(Debug, Clone)]
pub struct FeishuPeopleService {
    pub config: Config,
    pub core: core::CoreService,
}

impl FeishuPeopleService {
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            core: core::CoreService::new(config),
        }
    }
}

// 重新导出所有类型和服务
pub use core::*;
