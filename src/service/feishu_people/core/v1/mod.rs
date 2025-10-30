// core v1 - 核心人事管理v1版本API
//,
// 包含核心人事管理的完整功能，这是主要使用的版本
use crate::prelude::*;
use crate::service::feishu_people::core::v1::persons::PersonsService;
use crate::service::feishu_people::core::v1::departments::DepartmentsService;
use crate::service::feishu_people::core::v1::positions::PositionsService;
use crate::service::feishu_people::core::v1::contracts::ContractsService;
use crate::service::feishu_people::core::v1::companies::CompaniesService;
/// 核心人事管理v1版本服务
#[derive(Debug, Clone)]
pub struct CoreV1Service {
}

impl CoreV1Service {
}
    pub fn new(config: Config) -> Self {
        Self { config }
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
}