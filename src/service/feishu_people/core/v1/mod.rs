#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! Core v1 - 核心人事管理v1版本API
//!
//! 包含核心人事管理的完整功能，这是主要使用的版本

use crate::config::Config;

// 声明子模块
pub mod companies;
pub mod contracts;
pub mod departments;
pub mod persons;
pub mod positions;

// 重新导出服务类型
pub use companies::CompaniesService;
pub use contracts::ContractsService;
pub use departments::DepartmentsService;
pub use persons::PersonsService;
pub use positions::PositionsService;

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
