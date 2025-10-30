//! Core v1 - 核心人事管理v1版本API
//!
//! 包含核心人事管理的完整功能，这是主要使用的版本

use crate::core::config::Config;
use open_lark_core::prelude::*;

/// 核心人事管理v1版本服务
#[derive(Debug, Clone)]
pub struct CoreV1Service {
    pub config: Config,
    pub persons: PersonsService,
    pub departments: DepartmentsService,
    pub positions: PositionsService,
    pub contracts: ContractsService,
    pub companies: CompaniesService,
}

impl CoreV1Service {
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            persons: PersonsService::new(config.clone()),
            departments: DepartmentsService::new(config.clone()),
            positions: PositionsService::new(config.clone()),
            contracts: ContractsService::new(config.clone()),
            companies: CompaniesService::new(config),
        }
    }
}

/// 人员管理服务
pub mod persons;
/// 部门管理服务
pub mod departments;
/// 职位管理服务
pub mod positions;
/// 合同管理服务
pub mod contracts;
/// 公司管理服务
pub mod companies;