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
#[derive(.*?)]
pub struct CoreV1Service {
    client: std::sync::Arc<LarkClient>,
    /// 人员管理服务
    pub persons: PersonsService,
    /// 部门管理服务
    pub departments: DepartmentsService,
    /// 职位管理服务
    pub positions: PositionsService,
    /// 合同管理服务
    pub contracts: ContractsService,
    /// 公司管理服务
    pub companies: CompaniesService,
}
impl CoreV1Service {
    /// 创建新的v1版本服务实例
pub fn new() -> Self {
        Self {
            client,
            persons: PersonsService::new(client.clone()),
            departments: DepartmentsService::new(client.clone()),
            positions: PositionsService::new(client.clone()),
            contracts: ContractsService::new(client.clone()),
            companies: CompaniesService::new(client.clone()),
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