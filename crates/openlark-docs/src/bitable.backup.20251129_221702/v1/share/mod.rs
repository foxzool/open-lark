//! Bitable V1 协作者管理模块
//!
//! 提供多维表格协作者管理功能，包括：
//! - 新增协作者
//! - 删除协作者
//! - 列出协作者
//! - 批量新增协作者
//! - 批量删除协作者

use openlark_core::config::Config;

pub mod batch_create;
pub mod batch_delete;
pub mod create;
pub mod delete;
pub mod list;

pub use batch_create::*;
pub use batch_delete::*;
pub use create::*;
pub use delete::*;
pub use list::*;

/// 协作者管理服务
pub struct CollaboratorService {
    pub config: Config,
}

impl CollaboratorService {
    /// 创建协作者管理服务
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置
    pub fn config(&self) -> &Config {
        &self.config
    }
}