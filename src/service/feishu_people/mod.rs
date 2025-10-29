// feishu_people - 飞书人员管理模块
//
// 该模块提供飞书人事管理相关的所有功能，包括：
// - 核心人事管理（员工、部门、职位、合同）
// - 假期管理（休假申请、假期余额）
// - 权限管理（用户授权、角色管理）
// - 基础数据管理（工时制度、地点、证件类型）
// - 异动管理（员工异动、离职流程）
//
// 覆盖105个API接口，是HR管理的核心功能模块

use crate::prelude::*;
use crate::service::feishu_people::core::CoreService;
use crate::service::feishu_people::leaves::LeavesService;
use crate::service::feishu_people::authorizations::AuthorizationsService;
use crate::service::feishu_people::basic_data::BasicDataService;

/// 飞书人员管理服务
#[cfg(feature = "feishu_people")]
#[derive(Debug, Clone)]
pub struct FeishuPeopleService {
    /// 核心人事管理服务
    pub core: CoreService,
    /// 假期管理服务
    pub leaves: LeavesService,
    /// 权限管理服务
    pub authorizations: AuthorizationsService,
    /// 基础数据管理服务
    pub basic_data: BasicDataService,
}

#[cfg(feature = "feishu_people")]
impl FeishuPeopleService {
    /// 创建新的人员管理服务实例
    pub fn new(client: std::sync::Arc<LarkClient>) -> Self {
        Self {
            core: CoreService::new(client.clone()),
            leaves: LeavesService::new(client.clone()),
            authorizations: AuthorizationsService::new(client.clone()),
            basic_data: BasicDataService::new(client.clone()),
        }
    }
}

#[cfg(not(feature = "feishu_people"))]
pub struct FeishuPeopleService;

/// 数据模型
pub mod models;

/// 各子模块
pub mod core;
pub mod leaves;
pub mod authorizations;
pub mod basic_data;