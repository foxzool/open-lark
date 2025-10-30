//! Core v1 - 核心人事管理v1版本API
//!
//! 包含核心人事管理的完整功能，这是主要使用的版本

use crate::core::config::Config;

// 声明子模块
pub mod persons;
pub mod departments;
pub mod positions;
pub mod contracts;
pub mod companies;

// 重新导出服务类型
pub use persons::PersonsService;
pub use departments::DepartmentsService;
pub use positions::PositionsService;
pub use contracts::ContractsService;
pub use companies::CompaniesService;

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