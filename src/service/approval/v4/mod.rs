//! Approval API v4版本
//!
//! 实现审批管理的核心功能：
//! - 审批实例创建和管理
//! - 审批任务处理（同意、拒绝、转交、退回）
//! - 审批定义配置和管理
//! - 审批查询和统计
//! - 外部审批集成

use crate::core::config::Config;
use open_lark_core::prelude::*;
use serde::{Deserialize, Serialize};

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
pub mod instance;
pub mod task;
pub mod approval;
pub mod models;

// 重新导出所有模块和类型
pub use instance::*;
pub use task::*;
pub use approval::*;
pub use models::*;

// 暂时注释掉有语法错误的子模块
// pub mod external_approval;
// pub mod external_instance;
// pub mod external_task;
// pub mod file;
// pub mod message;
// pub mod search;
// pub mod instance_comment;
// pub mod p2_approval_instance_created_v4;
// pub mod p2_approval_instance_approved_v4;
// pub mod p2_approval_instance_rejected_v4;

// 重新导出所有模块和类型
pub use instance::*;
pub use task::*;
pub use approval::*;
pub use models::*;