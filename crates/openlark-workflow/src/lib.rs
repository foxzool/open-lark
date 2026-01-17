#![allow(missing_docs)]

// Core service
pub mod service;

// 项目模块（按 meta.Project）
pub mod task;
pub mod approval;
pub mod board;

pub mod prelude {
    pub use openlark_core::{config::Config, SDKResult};
}

use std::sync::Arc;
use service::WorkflowService;

/// WorkflowClient：统一入口，提供 project-version-resource 链式访问
#[derive(Clone)]
pub struct WorkflowClient {
    service: Arc<WorkflowService>,
}

impl WorkflowClient {
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self {
            service: Arc::new(WorkflowService::new(config)),
        }
    }

    pub fn task(&self) -> task::Task {
        task::Task::new(self.service.clone())
    }

    pub fn approval(&self) -> approval::Approval {
        approval::Approval::new(self.service.clone())
    }

    pub fn board(&self) -> board::Board {
        board::Board::new(self.service.clone())
    }
}
