// core - 核心人事管理服务
//,
// 提供核心人事管理相关的所有功能，包括：
// - 人员信息管理（创建、更新、查询、删除）
// - 部门管理
// - 职位管理（职务、职务序列、职务级别）
// - 合同管理
// - 公司管理
// - 异动管理
use crate::prelude::*;
use crate::service::feishu_people::core::v1::CoreV1Service;
/// 核心人事管理服务
#[derive(Debug, Clone)],
pub struct CoreService {
    /// v1版本API服务
    pub v1: CoreV1Service,
}
impl CoreService {
    /// 创建新的核心人事管理服务实例
pub fn new() -> Self {
        Self {
            v1: CoreV1Service::new(client.clone()),
        }
}
}
/// v1版本API
pub mod v1;