#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! Approval API v4版本
//!
//! 实现审批管理的核心功能：
//! - 审批实例创建和管理
//! - 审批任务处理（同意、拒绝、转交、退回）
//! - 审批定义配置和管理
//! - 审批查询和统计
//! - 外部审批集成

use config::Config;
/// Approval服务 v4版本
#[derive(Debug, Clone)]
pub struct ApprovalServiceV4 {
    pub config: Config,
    pub instance: InstanceService,
    pub task: TaskService,
    pub approval: ApprovalDefinitionService,
}

impl ApprovalServiceV4 {
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            instance: InstanceService::new(config.clone()),
            task: TaskService::new(config.clone()),
            approval: ApprovalDefinitionService::new(config),
        }
    }
}

// 导入所有子模块
pub mod approval;
pub mod instance;
pub mod models;
pub mod task;

// 重新导出所有模块和类型
pub use approval::*;
pub use instance::*;
pub use models::*;
pub use task::*;
